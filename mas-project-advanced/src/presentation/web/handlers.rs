use axum::{
    extract::{Path, State, Form, Query, Multipart},
    response::Redirect,
    http::StatusCode,
};
use serde::Deserialize;
use rust_decimal::Decimal;
use std::collections::HashMap;

use crate::application::dto::{CreatePagoDto, CreateProyectoDto, UpdateProyectoDto, CreateIngresoDto, CreateGastoDto, CreateCreditoDto, CreateDocumentoDto};
use crate::presentation::middleware::AuthUser;
use super::templates::*;
use super::server::AppState;

#[derive(Deserialize)]
pub struct Pagination {
    #[serde(default = "default_page")]
    page: u32,
}

fn default_page() -> u32 {
    1
}

pub async fn home(State(state): State<AppState>, user: AuthUser) -> Result<DashboardTemplate, StatusCode> {
    dashboard(State(state), user).await
}

pub async fn dashboard(State(state): State<AppState>, user: AuthUser) -> Result<DashboardTemplate, StatusCode> {
    // Obtener datos de todos los módulos
    let (ingresos, _total_ing) = state.ingreso_service.listar_ingresos(user.id, 1, 10000).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let (gastos, _total_gas) = state.gasto_service.listar_gastos(user.id, 1, 10000).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let (creditos, _total_cred) = state.credito_service.listar_creditos(user.id, 1, 10000).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let (_, summary) = state.proyecto_service.list_all_proyectos(user.id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let documentos = state.documento_service.listar_documentos(user.id, 1, 10000).await
        .unwrap_or_else(|_| (vec![], 0)).0;

    // Calcular métricas
    let total_ingresos: Decimal = ingresos.iter().map(|i| i.monto).sum();
    let total_gastos: Decimal = gastos.iter().map(|g| g.monto).sum();
    let balance = total_ingresos - total_gastos;
    let deuda_total: Decimal = creditos.iter().map(|c| c.saldo_pendiente).sum();

    // === GRÁFICOS: Agrupar por mes ===
    let meses = ["Ene", "Feb", "Mar", "Abr", "May", "Jun", "Jul", "Ago", "Sep", "Oct", "Nov", "Dic"];
    let mut ingresos_por_mes = vec![Decimal::ZERO; 12];
    let mut gastos_por_mes = vec![Decimal::ZERO; 12];

    for ingreso in &ingresos {
        if let Some(mes_idx) = extract_month(&ingreso.fecha) {
            ingresos_por_mes[mes_idx] += ingreso.monto;
        }
    }
    for gasto in &gastos {
        if let Some(mes_idx) = extract_month(&gasto.fecha) {
            gastos_por_mes[mes_idx] += gasto.monto;
        }
    }

    // Construir JSON para el gráfico mensual
    let chart_labels_json = serde_json::to_string(&meses).unwrap_or_else(|_| "[]".to_string());
    let chart_ingresos_json = serde_json::to_string(
        &ingresos_por_mes.iter().map(|d| d.to_string().parse::<f64>().unwrap_or(0.0)).collect::<Vec<f64>>()
    ).unwrap_or_else(|_| "[]".to_string());
    let chart_gastos_json = serde_json::to_string(
        &gastos_por_mes.iter().map(|d| d.to_string().parse::<f64>().unwrap_or(0.0)).collect::<Vec<f64>>()
    ).unwrap_or_else(|_| "[]".to_string());

    // === GRÁFICO: Distribución de gastos por categoría ===
    let mut categorias_map: HashMap<String, Decimal> = HashMap::new();
    for gasto in &gastos {
        let cat = gasto.categoria.to_string();
        *categorias_map.entry(cat).or_insert(Decimal::ZERO) += gasto.monto;
    }
    let cat_labels: Vec<String> = categorias_map.keys().cloned().collect();
    let cat_data: Vec<f64> = cat_labels.iter()
        .map(|k| categorias_map[k].to_string().parse::<f64>().unwrap_or(0.0))
        .collect();
    let chart_categorias_gastos_labels_json = serde_json::to_string(&cat_labels).unwrap_or_else(|_| "[]".to_string());
    let chart_categorias_gastos_data_json = serde_json::to_string(&cat_data).unwrap_or_else(|_| "[]".to_string());

    // === ALERTAS: Documentos próximos a vencer (30 días) ===
    let hoy = chrono::Local::now().format("%Y-%m-%d").to_string();
    let en_30_dias = (chrono::Local::now() + chrono::Duration::days(30)).format("%Y-%m-%d").to_string();
    let documentos_por_vencer: Vec<_> = documentos.into_iter()
        .filter(|d| {
            if let Some(ref fecha_v) = d.fecha_vencimiento {
                fecha_v >= &hoy && fecha_v <= &en_30_dias
            } else {
                false
            }
        })
        .collect();

    // Últimos 5 ingresos y gastos
    let ultimos_ingresos: Vec<_> = ingresos.into_iter().take(5).collect();
    let ultimos_gastos: Vec<_> = gastos.into_iter().take(5).collect();

    // Créditos activos (no pagados)
    let creditos_activos: Vec<_> = creditos.into_iter()
        .filter(|c| c.estado.to_string() != "Pagado")
        .collect();
    let num_creditos = creditos_activos.len();

    Ok(DashboardTemplate {
        title: "Dashboard - MAS Finance".to_string(),
        balance,
        total_ingresos,
        total_gastos,
        deuda_total,
        num_ingresos: ultimos_ingresos.len(),
        num_gastos: ultimos_gastos.len(),
        num_creditos,
        proyectos_activos: summary.proyectos_activos,
        ultimos_ingresos,
        ultimos_gastos,
        creditos_activos,
        chart_labels_json,
        chart_ingresos_json,
        chart_gastos_json,
        chart_categorias_gastos_labels_json,
        chart_categorias_gastos_data_json,
        documentos_por_vencer,
    })
}

/// Extrae el índice del mes (0-11) de una fecha en formato "YYYY-MM-DD"
fn extract_month(fecha: &str) -> Option<usize> {
    let parts: Vec<&str> = fecha.split('-').collect();
    if parts.len() >= 2 {
        parts[1].parse::<usize>().ok().map(|m| if m >= 1 && m <= 12 { m - 1 } else { 0 })
    } else {
        None
    }
}

pub async fn list_proyectos(State(state): State<AppState>, user: AuthUser) -> Result<ProyectosListTemplate, StatusCode> {
    let (proyectos, summary) = state.proyecto_service.list_all_proyectos(user.id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(ProyectosListTemplate {
        title: "Lista de Proyectos".to_string(),
        proyectos,
        summary,
    })
}

pub async fn list_pagos_proyecto(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
    Query(pagination): Query<Pagination>,
) -> Result<PagosProyectoTemplate, StatusCode> {
    let page_size = 10;
    let (proyecto, pagos, total_pagos) = state.proyecto_service.get_proyecto_with_pagos(user.id, id, pagination.page, page_size).await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    // Obtener totales globales (de TODOS los pagos, no solo la página actual)
    let (total_valor, total_saldo, pagos_completados, _) = state.pago_service.obtener_totales_proyecto(user.id, id).await
        .unwrap_or((Decimal::ZERO, Decimal::ZERO, 0, 0));

    let total_abonado = total_valor - total_saldo;
    let saldo_pendiente_proyecto = proyecto.presupuesto.unwrap_or_default() - total_abonado;
    
    let total_paginas = (total_pagos as f64 / page_size as f64).ceil() as u32;

    Ok(PagosProyectoTemplate {
        title: format!("Plan de Pagos - {}", proyecto.nombre),
        proyecto,
        pagos,
        total_valor,
        total_abonado,
        saldo_pendiente_proyecto,
        total_pagos: total_pagos as usize,
        pagos_completados: pagos_completados as usize,
        current_page: pagination.page,
        total_pages: total_paginas,
    })
}

pub async fn new_proyecto_form(_user: AuthUser) -> NewProyectoTemplate {
    NewProyectoTemplate {
        title: "Nuevo Proyecto".to_string(),
    }
}

#[derive(Deserialize)]
pub struct NewPagoQuery {
    pub error: Option<String>,
}

pub async fn new_pago_proyecto_form(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
    Query(query): Query<NewPagoQuery>,
) -> Result<NewPagoProyectoTemplate, StatusCode> {
    let (proyecto, pagos, _) = state.proyecto_service.get_proyecto_with_pagos(user.id, id, 1, 10000).await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    
    let presupuesto = proyecto.presupuesto.unwrap_or(Decimal::ZERO);
    let total_pagos: Decimal = pagos.iter().map(|p| p.valor).sum();
    let saldo_disponible = presupuesto - total_pagos;
    
    let error = match query.error.as_deref() {
        Some("excede_presupuesto") => Some(format!(
            "El pago excede el presupuesto disponible. Máximo permitido: ${}",
            saldo_disponible
        )),
        _ => None,
    };
    
    Ok(NewPagoProyectoTemplate {
        title: format!("Nuevo Pago - {}", proyecto.nombre),
        proyecto,
        error,
        saldo_disponible,
    })
}

#[derive(Deserialize)]
pub struct CambiarEstadoForm {
    pub estado: String,
}

pub async fn cambiar_estado_proyecto(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
    Form(form): Form<CambiarEstadoForm>,
) -> Result<Redirect, StatusCode> {
    state.proyecto_service.cambiar_estado_proyecto(user.id, id, &form.estado).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Redirect::to("/proyectos"))
}

#[derive(Deserialize)]
pub struct CreatePagoProyectoForm {
    pub descripcion: String,
    pub valor: String,
    pub mes: String,
    pub anio: String,
}

pub async fn create_pago_proyecto(
    State(state): State<AppState>,
    user: AuthUser,
    Path(proyecto_id): Path<i32>,
    mut multipart: Multipart,
) -> Result<Redirect, StatusCode> {
    let mut descripcion: Option<String> = None;
    let mut valor: Option<String> = None;
    let mut mes: Option<String> = None;
    let mut anio: Option<String> = None;
    let mut evidencia_cliente: Option<(Vec<u8>, String)> = None;
    let mut evidencia_constructora: Option<(Vec<u8>, String)> = None;

    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let name = field.name().unwrap_or("").to_string();
        
        match name.as_str() {
            "descripcion" => descripcion = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?),
            "valor" => valor = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?),
            "mes" => mes = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?),
            "anio" => anio = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?),
            "evidencia_cliente" => {
                let file_name = field.file_name().unwrap_or("").to_string();
                if !file_name.is_empty() {
                    let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?.to_vec();
                    if !data.is_empty() {
                        evidencia_cliente = Some((data, file_name));
                    }
                }
            },
            "evidencia_constructora" => {
                let file_name = field.file_name().unwrap_or("").to_string();
                if !file_name.is_empty() {
                    let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?.to_vec();
                    if !data.is_empty() {
                        evidencia_constructora = Some((data, file_name));
                    }
                }
            },
            _ => {}
        }
    }

    let descripcion = descripcion.ok_or(StatusCode::BAD_REQUEST)?;
    let valor_decimal: Decimal = valor.ok_or(StatusCode::BAD_REQUEST)?.parse().map_err(|_| StatusCode::BAD_REQUEST)?;
    let mes = mes.ok_or(StatusCode::BAD_REQUEST)?;
    let anio = anio.ok_or(StatusCode::BAD_REQUEST)?;

    let dto = CreatePagoDto {
        descripcion,
        valor: valor_decimal,
        mes,
        anio,
        proyecto_id: Some(proyecto_id),
    };
    
    match state.pago_service.crear_pago(user.id, dto).await {
        Ok(pago) => {
            // Subir evidencias si se proporcionaron
            if let Some((data, name)) = evidencia_cliente {
                let _ = state.pago_service.subir_evidencia(user.id, pago.id, data, &name, "cliente").await;
            }
            if let Some((data, name)) = evidencia_constructora {
                let _ = state.pago_service.subir_evidencia(user.id, pago.id, data, &name, "constructora").await;
            }
            Ok(Redirect::to(&format!("/proyectos/{}/pagos", proyecto_id)))
        }
        Err(_e) => {
            // Redirigir con error - el usuario verá el saldo disponible
            Ok(Redirect::to(&format!("/proyectos/{}/pagos/new?error=excede_presupuesto", proyecto_id)))
        }
    }
}

pub async fn upload_evidencia_pago(
    State(state): State<AppState>,
    user: AuthUser,
    Path(pago_id): Path<i32>,
    mut multipart: Multipart,
) -> Result<Redirect, StatusCode> {
    let mut file_data: Option<Vec<u8>> = None;
    let mut file_name: Option<String> = None;
    let mut tipo: Option<String> = None;
    let mut proyecto_id: Option<i32> = None;

    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let name = field.name().unwrap_or("").to_string();
        
        match name.as_str() {
            "evidencia_cliente" => {
                file_name = field.file_name().map(|s| s.to_string());
                file_data = Some(field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?.to_vec());
                tipo = Some("cliente".to_string());
            },
            "evidencia_constructora" => {
                file_name = field.file_name().map(|s| s.to_string());
                file_data = Some(field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?.to_vec());
                tipo = Some("constructora".to_string());
            },
            "proyecto_id" => {
                let text = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                proyecto_id = text.parse::<i32>().ok();
            },
            _ => {}
        }
    }

    let file_data = file_data.ok_or(StatusCode::BAD_REQUEST)?;
    let file_name = file_name.ok_or(StatusCode::BAD_REQUEST)?;
    let tipo = tipo.ok_or(StatusCode::BAD_REQUEST)?;
    
    state.pago_service.subir_evidencia(user.id, pago_id, file_data, &file_name, &tipo).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if let Some(pid) = proyecto_id {
        Ok(Redirect::to(&format!("/proyectos/{}/pagos", pid)))
    } else {
        Ok(Redirect::to("/proyectos"))
    }
}

// Aquí van los handlers de proyectos

#[derive(Deserialize)]
pub struct CreateProyectoForm {
    pub nombre: String,
    pub descripcion: Option<String>,
    pub presupuesto: Option<String>,
    pub cliente: Option<String>,
    pub responsable: Option<String>,
}

pub async fn create_proyecto(
    State(state): State<AppState>,
    user: AuthUser,
    Form(form): Form<CreateProyectoForm>,
) -> Result<Redirect, StatusCode> {
    let dto = CreateProyectoDto {
        nombre: form.nombre,
        descripcion: form.descripcion,
        presupuesto: form.presupuesto.and_then(|p| p.parse().ok()),
        fecha_fin_estimada: None,
        cliente: form.cliente,
        responsable: form.responsable,
    };
    
    state.proyecto_service.crear_proyecto(user.id, dto).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Redirect::to("/proyectos"))
}

pub async fn show_proyecto(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<ShowProyectoTemplate, StatusCode> {
    let proyecto = state.proyecto_service.get_proyecto_by_id(user.id, id).await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    
    Ok(ShowProyectoTemplate {
        title: format!("Proyecto: {}", proyecto.nombre),
        proyecto,
    })
}

pub async fn edit_proyecto_form(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<EditProyectoTemplate, StatusCode> {
    let proyecto = state.proyecto_service.get_proyecto_by_id(user.id, id).await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    
    Ok(EditProyectoTemplate {
        title: format!("Editar: {}", proyecto.nombre),
        proyecto,
    })
}

#[derive(Deserialize)]
pub struct UpdateProyectoForm {
    pub nombre: Option<String>,
    pub descripcion: Option<String>,
    pub presupuesto: Option<String>,
    pub cliente: Option<String>,
    pub responsable: Option<String>,
}

pub async fn update_proyecto(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
    Form(form): Form<UpdateProyectoForm>,
) -> Result<Redirect, StatusCode> {
    let dto = UpdateProyectoDto {
        nombre: form.nombre,
        descripcion: form.descripcion,
        presupuesto: form.presupuesto.and_then(|p| p.parse().ok()),
        fecha_fin_estimada: None,
        cliente: form.cliente,
        responsable: form.responsable,
    };
    
    state.proyecto_service.actualizar_proyecto(user.id, id, dto).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Redirect::to(&format!("/proyectos/{}", id)))
}

#[derive(Deserialize)]
pub struct PagoRedirectForm {
    pub proyecto_id: Option<i32>,
}

pub async fn marcar_pago_pagado(
    State(state): State<AppState>,
    user: AuthUser,
    Path(pago_id): Path<i32>,
    Form(form): Form<PagoRedirectForm>,
) -> Result<Redirect, StatusCode> {
    state.pago_service.marcar_pagado(user.id, pago_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if let Some(proyecto_id) = form.proyecto_id {
        Ok(Redirect::to(&format!("/proyectos/{}/pagos", proyecto_id)))
    } else {
        Ok(Redirect::to("/proyectos"))
    }
}

pub async fn eliminar_pago(
    State(state): State<AppState>,
    user: AuthUser,
    Path(pago_id): Path<i32>,
    Form(form): Form<PagoRedirectForm>,
) -> Result<Redirect, StatusCode> {
    state.pago_service.eliminar_pago(user.id, pago_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if let Some(proyecto_id) = form.proyecto_id {
        Ok(Redirect::to(&format!("/proyectos/{}/pagos", proyecto_id)))
    } else {
        Ok(Redirect::to("/proyectos"))
    }
}

pub async fn edit_pago_form(
    State(state): State<AppState>,
    user: AuthUser,
    Path(pago_id): Path<i32>,
) -> Result<EditPagoTemplate, StatusCode> {
    let pago = state.pago_service.obtener_pago(user.id, pago_id).await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    
    Ok(EditPagoTemplate {
        title: format!("Editar Pago - {}", pago.descripcion),
        pago,
    })
}

pub async fn update_pago(
    State(state): State<AppState>,
    user: AuthUser,
    Path(pago_id): Path<i32>,
    mut multipart: Multipart,
) -> Result<Redirect, StatusCode> {
    let mut descripcion: Option<String> = None;
    let mut valor: Option<String> = None;
    let mut mes: Option<String> = None;
    let mut anio: Option<String> = None;
    let mut proyecto_id: Option<i32> = None;
    let mut evidencia_cliente: Option<(Vec<u8>, String)> = None;
    let mut evidencia_constructora: Option<(Vec<u8>, String)> = None;

    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let name = field.name().unwrap_or("").to_string();
        
        match name.as_str() {
            "descripcion" => descripcion = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?),
            "valor" => valor = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?),
            "mes" => mes = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?),
            "anio" => anio = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?),
            "proyecto_id" => {
                let text = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                proyecto_id = text.parse::<i32>().ok();
            },
            "evidencia_cliente" => {
                let file_name = field.file_name().unwrap_or("").to_string();
                if !file_name.is_empty() {
                    let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?.to_vec();
                    if !data.is_empty() {
                        evidencia_cliente = Some((data, file_name));
                    }
                }
            },
            "evidencia_constructora" => {
                let file_name = field.file_name().unwrap_or("").to_string();
                if !file_name.is_empty() {
                    let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?.to_vec();
                    if !data.is_empty() {
                        evidencia_constructora = Some((data, file_name));
                    }
                }
            },
            _ => {}
        }
    }

    let descripcion = descripcion.ok_or(StatusCode::BAD_REQUEST)?;
    let valor: Decimal = valor.ok_or(StatusCode::BAD_REQUEST)?.parse().map_err(|_| StatusCode::BAD_REQUEST)?;
    let mes = mes.ok_or(StatusCode::BAD_REQUEST)?;
    let anio = anio.ok_or(StatusCode::BAD_REQUEST)?;

    state.pago_service.editar_pago_con_evidencias(
        user.id, pago_id, &descripcion, valor, &mes, &anio,
        evidencia_cliente, evidencia_constructora,
    ).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(pid) = proyecto_id {
        Ok(Redirect::to(&format!("/proyectos/{}/pagos", pid)))
    } else {
        Ok(Redirect::to("/proyectos"))
    }
}

// === HANDLERS DE INGRESOS ===

pub async fn list_ingresos(
    State(state): State<AppState>,
    user: AuthUser,
    Query(pagination): Query<Pagination>,
) -> Result<IngresosListTemplate, StatusCode> {
    let page_size = 20;
    let (ingresos, total) = state.ingreso_service.listar_ingresos(user.id, pagination.page, page_size).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let monto_total: Decimal = state.ingreso_service.obtener_total_monto(user.id).await
        .unwrap_or(Decimal::ZERO);
    let total_pages = (total as f64 / page_size as f64).ceil() as u32;

    Ok(IngresosListTemplate {
        title: "Ingresos".to_string(),
        total_ingresos: total as usize,
        monto_total,
        ingresos,
        current_page: pagination.page,
        total_pages,
    })
}

pub async fn new_ingreso_form(_user: AuthUser) -> NewIngresoTemplate {
    NewIngresoTemplate {
        title: "Nuevo Ingreso".to_string(),
    }
}

#[derive(Deserialize)]
pub struct CreateIngresoForm {
    pub descripcion: String,
    pub monto: String,
    pub categoria: String,
    pub fuente: Option<String>,
    pub fecha: String,
    pub recurrente: Option<String>,
}

pub async fn create_ingreso(
    State(state): State<AppState>,
    user: AuthUser,
    Form(form): Form<CreateIngresoForm>,
) -> Result<Redirect, StatusCode> {
    let dto = CreateIngresoDto {
        descripcion: form.descripcion,
        monto: form.monto.parse().map_err(|_| StatusCode::BAD_REQUEST)?,
        categoria: form.categoria,
        fuente: form.fuente,
        fecha: form.fecha,
        recurrente: form.recurrente.is_some(),
    };

    state.ingreso_service.crear_ingreso(user.id, dto).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to("/ingresos"))
}

pub async fn eliminar_ingreso(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<Redirect, StatusCode> {
    state.ingreso_service.eliminar_ingreso(user.id, id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to("/ingresos"))
}

// === HANDLERS DE GASTOS ===

pub async fn list_gastos(
    State(state): State<AppState>,
    user: AuthUser,
    Query(pagination): Query<Pagination>,
) -> Result<GastosListTemplate, StatusCode> {
    // Auto-generar los gastos fijos cuyo día de facturación ya pasó
    let _ = state.gasto_recurrente_service.auto_generar_fijos(user.id).await;

    let page_size = 20;
    let (gastos, total) = state.gasto_service.listar_gastos(user.id, pagination.page, page_size).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let monto_total: Decimal = state.gasto_service.obtener_total_monto(user.id).await
        .unwrap_or(Decimal::ZERO);
    let total_pages = (total as f64 / page_size as f64).ceil() as u32;

    Ok(GastosListTemplate {
        title: "Gastos".to_string(),
        total_gastos: total as usize,
        monto_total,
        gastos,
        current_page: pagination.page,
        total_pages,
    })
}

pub async fn new_gasto_form(_user: AuthUser) -> NewGastoTemplate {
    NewGastoTemplate {
        title: "Nuevo Gasto".to_string(),
    }
}

pub async fn create_gasto(
    State(state): State<AppState>,
    user: AuthUser,
    mut multipart: Multipart,
) -> Result<Redirect, StatusCode> {
    let mut descripcion: Option<String> = None;
    let mut monto: Option<String> = None;
    let mut categoria: Option<String> = None;
    let mut responsable: Option<String> = None;
    let mut fecha: Option<String> = None;
    let mut soporte: Option<(Vec<u8>, String)> = None;

    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "descripcion" => descripcion = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?),
            "monto" => monto = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?),
            "categoria" => categoria = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?),
            "responsable" => {
                let text = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                if !text.is_empty() { responsable = Some(text); }
            },
            "fecha" => fecha = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?),
            "soporte" => {
                let file_name = field.file_name().unwrap_or("").to_string();
                if !file_name.is_empty() {
                    let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?.to_vec();
                    if !data.is_empty() {
                        soporte = Some((data, file_name));
                    }
                }
            },
            _ => {}
        }
    }

    let dto = CreateGastoDto {
        descripcion: descripcion.ok_or(StatusCode::BAD_REQUEST)?,
        monto: monto.ok_or(StatusCode::BAD_REQUEST)?.parse().map_err(|_| StatusCode::BAD_REQUEST)?,
        categoria: categoria.ok_or(StatusCode::BAD_REQUEST)?,
        responsable,
        fecha: fecha.ok_or(StatusCode::BAD_REQUEST)?,
    };

    let gasto = state.gasto_service.crear_gasto(user.id, dto).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some((data, name)) = soporte {
        let _ = state.gasto_service.subir_soporte(user.id, gasto.id, data, &name).await;
    }

    Ok(Redirect::to("/gastos"))
}

pub async fn marcar_gasto_pagado(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<Redirect, StatusCode> {
    state.gasto_service.marcar_pagado(user.id, id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to("/gastos"))
}

pub async fn eliminar_gasto(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<Redirect, StatusCode> {
    state.gasto_service.eliminar_gasto(user.id, id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to("/gastos"))
}

pub async fn upload_soporte_gasto(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
    mut multipart: Multipart,
) -> Result<Redirect, StatusCode> {
    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let name = field.name().unwrap_or("").to_string();
        if name == "soporte" {
            let file_name = field.file_name().unwrap_or("").to_string();
            if !file_name.is_empty() {
                let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?.to_vec();
                if !data.is_empty() {
                    state.gasto_service.subir_soporte(user.id, id, data, &file_name).await
                        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                }
            }
        }
    }

    Ok(Redirect::to("/gastos"))
}

// === HANDLERS EDITAR INGRESOS ===

pub async fn edit_ingreso_form(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<EditIngresoTemplate, StatusCode> {
    let ingreso = state.ingreso_service.obtener_ingreso(user.id, id).await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(EditIngresoTemplate {
        title: format!("Editar Ingreso - {}", ingreso.descripcion),
        ingreso,
    })
}

pub async fn update_ingreso(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
    Form(form): Form<CreateIngresoForm>,
) -> Result<Redirect, StatusCode> {
    let dto = CreateIngresoDto {
        descripcion: form.descripcion,
        monto: form.monto.parse().map_err(|_| StatusCode::BAD_REQUEST)?,
        categoria: form.categoria,
        fuente: form.fuente,
        fecha: form.fecha,
        recurrente: form.recurrente.is_some(),
    };

    state.ingreso_service.editar_ingreso(user.id, id, dto).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to("/ingresos"))
}

// === HANDLERS EDITAR GASTOS ===

pub async fn edit_gasto_form(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<EditGastoTemplate, StatusCode> {
    let gasto = state.gasto_service.obtener_gasto(user.id, id).await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(EditGastoTemplate {
        title: format!("Editar Gasto - {}", gasto.descripcion),
        gasto,
    })
}

pub async fn update_gasto(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
    mut multipart: Multipart,
) -> Result<Redirect, StatusCode> {
    let mut descripcion: Option<String> = None;
    let mut monto: Option<String> = None;
    let mut categoria: Option<String> = None;
    let mut responsable: Option<String> = None;
    let mut fecha: Option<String> = None;
    let mut soporte: Option<(Vec<u8>, String)> = None;

    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "descripcion" => descripcion = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?),
            "monto" => monto = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?),
            "categoria" => categoria = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?),
            "responsable" => {
                let text = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                if !text.is_empty() { responsable = Some(text); }
            },
            "fecha" => fecha = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?),
            "soporte" => {
                let file_name = field.file_name().unwrap_or("").to_string();
                if !file_name.is_empty() {
                    let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?.to_vec();
                    if !data.is_empty() {
                        soporte = Some((data, file_name));
                    }
                }
            },
            _ => {}
        }
    }

    let dto = CreateGastoDto {
        descripcion: descripcion.ok_or(StatusCode::BAD_REQUEST)?,
        monto: monto.ok_or(StatusCode::BAD_REQUEST)?.parse().map_err(|_| StatusCode::BAD_REQUEST)?,
        categoria: categoria.ok_or(StatusCode::BAD_REQUEST)?,
        responsable,
        fecha: fecha.ok_or(StatusCode::BAD_REQUEST)?,
    };

    state.gasto_service.editar_gasto(user.id, id, dto).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some((data, name)) = soporte {
        let _ = state.gasto_service.subir_soporte(user.id, id, data, &name).await;
    }

    Ok(Redirect::to("/gastos"))
}

// === HANDLERS DE GASTOS RECURRENTES ===

pub async fn list_gastos_recurrentes(
    State(state): State<AppState>,
    user: AuthUser,
) -> Result<GastosRecurrentesListTemplate, StatusCode> {
    // Auto-generar los gastos fijos cuyo día de facturación ya pasó
    let _ = state.gasto_recurrente_service.auto_generar_fijos(user.id).await;

    let gastos_recurrentes = state.gasto_recurrente_service.listar_todos(user.id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let monto_mensual_estimado: Decimal = gastos_recurrentes.iter()
        .filter(|g| g.activo)
        .map(|g| g.monto_referencia)
        .sum();

    let total_recurrentes = gastos_recurrentes.len();

    // Calcular variables pendientes del mes actual
    let now = chrono::Local::now();
    let mes_actual = format!("{}/{}", now.format("%m"), now.format("%Y"));
    let pendientes_generar = state.gasto_recurrente_service
        .variables_pendientes_por_generar(
            user.id,
            now.format("%Y").to_string().parse().unwrap_or(2026),
            now.format("%m").to_string().parse().unwrap_or(1),
        )
        .await.unwrap_or(0);

    Ok(GastosRecurrentesListTemplate {
        title: "Gastos Recurrentes".to_string(),
        gastos_recurrentes,
        total_recurrentes,
        monto_mensual_estimado,
        pendientes_generar,
        mes_actual,
    })
}

pub async fn new_gasto_recurrente_form(_user: AuthUser) -> NewGastoRecurrenteTemplate {
    NewGastoRecurrenteTemplate { title: "Nuevo Gasto Recurrente".to_string() }
}

#[derive(Deserialize)]
pub struct CreateGastoRecurrenteForm {
    pub descripcion: String,
    pub monto_referencia: String,
    pub categoria: String,
    pub tipo: String,
    pub responsable: Option<String>,
    pub dia_facturacion: String,
}

pub async fn create_gasto_recurrente(
    State(state): State<AppState>,
    user: AuthUser,
    Form(form): Form<CreateGastoRecurrenteForm>,
) -> Result<Redirect, StatusCode> {
    use crate::application::dto::CreateGastoRecurrenteDto;

    let dto = CreateGastoRecurrenteDto {
        descripcion: form.descripcion,
        monto_referencia: form.monto_referencia.parse().map_err(|_| StatusCode::BAD_REQUEST)?,
        categoria: form.categoria,
        tipo: form.tipo,
        responsable: form.responsable.filter(|s| !s.is_empty()),
        dia_facturacion: form.dia_facturacion.parse().unwrap_or(1),
    };

    state.gasto_recurrente_service.crear(user.id, dto).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to("/gastos-recurrentes"))
}

pub async fn edit_gasto_recurrente_form(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<EditGastoRecurrenteTemplate, StatusCode> {
    let gasto_recurrente = state.gasto_recurrente_service.obtener(user.id, id).await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(EditGastoRecurrenteTemplate {
        title: format!("Editar - {}", gasto_recurrente.descripcion),
        gasto_recurrente,
    })
}

pub async fn update_gasto_recurrente(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
    Form(form): Form<CreateGastoRecurrenteForm>,
) -> Result<Redirect, StatusCode> {
    use crate::application::dto::CreateGastoRecurrenteDto;

    let dto = CreateGastoRecurrenteDto {
        descripcion: form.descripcion,
        monto_referencia: form.monto_referencia.parse().map_err(|_| StatusCode::BAD_REQUEST)?,
        categoria: form.categoria,
        tipo: form.tipo,
        responsable: form.responsable.filter(|s| !s.is_empty()),
        dia_facturacion: form.dia_facturacion.parse().unwrap_or(1),
    };

    state.gasto_recurrente_service.editar(user.id, id, dto).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to("/gastos-recurrentes"))
}

pub async fn toggle_gasto_recurrente(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<Redirect, StatusCode> {
    state.gasto_recurrente_service.toggle_activo(user.id, id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to("/gastos-recurrentes"))
}

pub async fn eliminar_gasto_recurrente(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<Redirect, StatusCode> {
    state.gasto_recurrente_service.eliminar(user.id, id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to("/gastos-recurrentes"))
}

#[derive(Deserialize)]
pub struct GenerarGastosForm {
    pub mes: String,
    pub anio: String,
}

pub async fn generar_gastos_mes(
    State(state): State<AppState>,
    user: AuthUser,
    Form(form): Form<GenerarGastosForm>,
) -> Result<Redirect, StatusCode> {
    use crate::application::dto::GenerarGastosDto;

    let dto = GenerarGastosDto {
        mes: form.mes.parse().map_err(|_| StatusCode::BAD_REQUEST)?,
        anio: form.anio.parse().map_err(|_| StatusCode::BAD_REQUEST)?,
    };

    state.gasto_recurrente_service.generar_variables_del_mes(user.id, dto).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to("/gastos"))
}

// === HANDLERS DE CRÉDITOS ===

pub async fn list_creditos(
    State(state): State<AppState>,
    user: AuthUser,
    Query(pagination): Query<Pagination>,
) -> Result<CreditosListTemplate, StatusCode> {
    let page_size = 20;
    let (creditos, total) = state.credito_service.listar_creditos(user.id, pagination.page, page_size).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let deuda_total: Decimal = state.credito_service.obtener_deuda_total(user.id).await
        .unwrap_or(Decimal::ZERO);
    let total_pages = (total as f64 / page_size as f64).ceil() as u32;

    Ok(CreditosListTemplate {
        title: "Créditos".to_string(),
        total_creditos: total as usize,
        deuda_total,
        creditos,
        current_page: pagination.page,
        total_pages,
    })
}

pub async fn new_credito_form(_user: AuthUser) -> NewCreditoTemplate {
    NewCreditoTemplate { title: "Nuevo Crédito".to_string() }
}

#[derive(Deserialize)]
pub struct CreateCreditoForm {
    pub entidad: String,
    pub descripcion: String,
    pub monto_total: String,
    pub tasa_interes: String,
    pub tipo_tasa: String,
    pub cuotas_totales: String,
    pub valor_cuota: String,
    pub fecha_inicio: String,
    pub fecha_fin_estimada: Option<String>,
}

pub async fn create_credito(
    State(state): State<AppState>,
    user: AuthUser,
    Form(form): Form<CreateCreditoForm>,
) -> Result<Redirect, StatusCode> {
    let dto = CreateCreditoDto {
        entidad: form.entidad,
        descripcion: form.descripcion,
        monto_total: form.monto_total.parse().map_err(|_| StatusCode::BAD_REQUEST)?,
        tasa_interes: form.tasa_interes.parse().map_err(|_| StatusCode::BAD_REQUEST)?,
        tipo_tasa: form.tipo_tasa,
        cuotas_totales: form.cuotas_totales.parse().map_err(|_| StatusCode::BAD_REQUEST)?,
        valor_cuota: form.valor_cuota.parse().map_err(|_| StatusCode::BAD_REQUEST)?,
        fecha_inicio: form.fecha_inicio,
        fecha_fin_estimada: form.fecha_fin_estimada.filter(|s| !s.is_empty()),
    };

    state.credito_service.crear_credito(user.id, dto).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to("/creditos"))
}

pub async fn registrar_cuota_credito(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<Redirect, StatusCode> {
    state.credito_service.registrar_cuota(user.id, id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Redirect::to("/creditos"))
}

pub async fn edit_credito_form(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<EditCreditoTemplate, StatusCode> {
    let credito = state.credito_service.obtener_credito(user.id, id).await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(EditCreditoTemplate {
        title: format!("Editar Crédito - {}", credito.descripcion),
        credito,
    })
}

pub async fn update_credito(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
    Form(form): Form<CreateCreditoForm>,
) -> Result<Redirect, StatusCode> {
    let dto = CreateCreditoDto {
        entidad: form.entidad,
        descripcion: form.descripcion,
        monto_total: form.monto_total.parse().map_err(|_| StatusCode::BAD_REQUEST)?,
        tasa_interes: form.tasa_interes.parse().map_err(|_| StatusCode::BAD_REQUEST)?,
        tipo_tasa: form.tipo_tasa,
        cuotas_totales: form.cuotas_totales.parse().map_err(|_| StatusCode::BAD_REQUEST)?,
        valor_cuota: form.valor_cuota.parse().map_err(|_| StatusCode::BAD_REQUEST)?,
        fecha_inicio: form.fecha_inicio,
        fecha_fin_estimada: form.fecha_fin_estimada.filter(|s| !s.is_empty()),
    };

    state.credito_service.editar_credito(user.id, id, dto).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to("/creditos"))
}

pub async fn eliminar_credito(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<Redirect, StatusCode> {
    state.credito_service.eliminar_credito(user.id, id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Redirect::to("/creditos"))
}

// === HANDLERS DE DOCUMENTOS ===

pub async fn list_documentos(
    State(state): State<AppState>,
    user: AuthUser,
    Query(pagination): Query<Pagination>,
) -> Result<DocumentosListTemplate, StatusCode> {
    let page_size = 20;
    let (documentos, total) = state.documento_service.listar_documentos(user.id, pagination.page, page_size).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let total_pages = (total as f64 / page_size as f64).ceil() as u32;

    Ok(DocumentosListTemplate {
        title: "Documentos Importantes".to_string(),
        documentos,
        total_documentos: total as usize,
        current_page: pagination.page,
        total_pages,
    })
}

pub async fn new_documento_form(_user: AuthUser) -> NewDocumentoTemplate {
    NewDocumentoTemplate { title: "Nuevo Documento".to_string() }
}

pub async fn create_documento(
    State(state): State<AppState>,
    user: AuthUser,
    mut multipart: Multipart,
) -> Result<Redirect, StatusCode> {
    let mut nombre: Option<String> = None;
    let mut descripcion: Option<String> = None;
    let mut categoria: Option<String> = None;
    let mut fecha_vencimiento: Option<String> = None;
    let mut archivo: Option<(Vec<u8>, String)> = None;

    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "nombre" => nombre = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?),
            "descripcion" => {
                let text = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                if !text.is_empty() { descripcion = Some(text); }
            },
            "categoria" => categoria = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?),
            "fecha_vencimiento" => {
                let text = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                if !text.is_empty() { fecha_vencimiento = Some(text); }
            },
            "archivo" => {
                let file_name = field.file_name().unwrap_or("").to_string();
                if !file_name.is_empty() {
                    let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?.to_vec();
                    if !data.is_empty() {
                        archivo = Some((data, file_name));
                    }
                }
            },
            _ => {}
        }
    }

    let (file_data, file_name) = archivo.ok_or(StatusCode::BAD_REQUEST)?;

    let dto = CreateDocumentoDto {
        nombre: nombre.ok_or(StatusCode::BAD_REQUEST)?,
        descripcion,
        categoria: categoria.ok_or(StatusCode::BAD_REQUEST)?,
        fecha_vencimiento,
    };

    state.documento_service.crear_documento(user.id, dto, file_data, &file_name).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to("/documentos"))
}

pub async fn eliminar_documento(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<Redirect, StatusCode> {
    state.documento_service.eliminar_documento(user.id, id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Redirect::to("/documentos"))
}
