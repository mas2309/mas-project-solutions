use axum::{
    extract::{Path, State, Query, Json, Multipart},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

use crate::application::dto::*;
use crate::domain::entities::*;
use crate::presentation::web::server::AppState;
use crate::presentation::middleware::AuthUser;
use crate::application::services::auth_service::LoginResponse;

// === RESPUESTAS API ===

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    fn ok(data: T) -> Json<Self> {
        Json(Self { success: true, data: Some(data), message: None })
    }

    fn error(msg: &str) -> Json<Self> {
        Json(Self { success: false, data: None, message: Some(msg.to_string()) })
    }
}

#[derive(Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_page_size")]
    pub page_size: u32,
}

fn default_page() -> u32 { 1 }
fn default_page_size() -> u32 { 20 }

#[derive(Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: u32,
    pub page_size: u32,
}

// === DASHBOARD ===

#[derive(Serialize)]
pub struct DashboardData {
    pub total_ingresos: Decimal,
    pub total_gastos: Decimal,
    pub balance: Decimal,
    pub deuda_total: Decimal,
    pub proyectos_activos: i64,
}

pub async fn api_dashboard(
    State(state): State<AppState>,
    user: AuthUser,
) -> Result<Json<ApiResponse<DashboardData>>, StatusCode> {
    let (ingresos, _) = state.ingreso_service.listar_ingresos(user.id, 1, 10000).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let (gastos, _) = state.gasto_service.listar_gastos(user.id, 1, 10000).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let (creditos, _) = state.credito_service.listar_creditos(user.id, 1, 10000).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let (_, summary) = state.proyecto_service.list_all_proyectos(user.id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let total_ingresos: Decimal = ingresos.iter().map(|i| i.monto).sum();
    let total_gastos: Decimal = gastos.iter().map(|g| g.monto).sum();
    let deuda_total: Decimal = creditos.iter().map(|c| c.saldo_pendiente).sum();

    Ok(ApiResponse::ok(DashboardData {
        total_ingresos,
        total_gastos,
        balance: total_ingresos - total_gastos,
        deuda_total,
        proyectos_activos: summary.proyectos_activos,
    }))
}

// === INGRESOS ===

pub async fn api_list_ingresos(
    State(state): State<AppState>,
    user: AuthUser,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<ApiResponse<PaginatedResponse<Ingreso>>>, StatusCode> {
    let (ingresos, total) = state.ingreso_service.listar_ingresos(user.id, pagination.page, pagination.page_size).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(ApiResponse::ok(PaginatedResponse {
        items: ingresos,
        total,
        page: pagination.page,
        page_size: pagination.page_size,
    }))
}

pub async fn api_get_ingreso(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<Ingreso>>, StatusCode> {
    match state.ingreso_service.obtener_ingreso(user.id, id).await {
        Ok(ingreso) => Ok(ApiResponse::ok(ingreso)),
        Err(_) => Ok(ApiResponse::error("Ingreso no encontrado")),
    }
}

pub async fn api_create_ingreso(
    State(state): State<AppState>,
    user: AuthUser,
    Json(dto): Json<CreateIngresoDto>,
) -> Result<Json<ApiResponse<Ingreso>>, StatusCode> {
    match state.ingreso_service.crear_ingreso(user.id, dto).await {
        Ok(ingreso) => Ok(ApiResponse::ok(ingreso)),
        Err(e) => Ok(ApiResponse::error(&e.to_string())),
    }
}

pub async fn api_update_ingreso(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
    Json(dto): Json<CreateIngresoDto>,
) -> Result<Json<ApiResponse<Ingreso>>, StatusCode> {
    match state.ingreso_service.editar_ingreso(user.id, id, dto).await {
        Ok(ingreso) => Ok(ApiResponse::ok(ingreso)),
        Err(e) => Ok(ApiResponse::error(&e.to_string())),
    }
}

pub async fn api_delete_ingreso(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<Ingreso>>, StatusCode> {
    match state.ingreso_service.eliminar_ingreso(user.id, id).await {
        Ok(ingreso) => Ok(ApiResponse::ok(ingreso)),
        Err(e) => Ok(ApiResponse::error(&e.to_string())),
    }
}

// === GASTOS ===

pub async fn api_list_gastos(
    State(state): State<AppState>,
    user: AuthUser,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<ApiResponse<PaginatedResponse<Gasto>>>, StatusCode> {
    let _ = state.credito_service.auto_generar_cuotas(user.id).await;

    // Igual que la web: auto-generar los gastos fijos cuyo día de facturación ya pasó.
    let _ = state.gasto_recurrente_service.auto_generar_fijos(user.id).await;

    let (gastos, total) = state.gasto_service.listar_gastos(user.id, pagination.page, pagination.page_size).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(ApiResponse::ok(PaginatedResponse {
        items: gastos,
        total,
        page: pagination.page,
        page_size: pagination.page_size,
    }))
}

pub async fn api_get_gasto(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<Gasto>>, StatusCode> {
    match state.gasto_service.obtener_gasto(user.id, id).await {
        Ok(gasto) => Ok(ApiResponse::ok(gasto)),
        Err(_) => Ok(ApiResponse::error("Gasto no encontrado")),
    }
}

pub async fn api_create_gasto(
    State(state): State<AppState>,
    user: AuthUser,
    Json(dto): Json<CreateGastoDto>,
) -> Result<Json<ApiResponse<Gasto>>, StatusCode> {
    match state.gasto_service.crear_gasto(user.id, dto).await {
        Ok(gasto) => Ok(ApiResponse::ok(gasto)),
        Err(e) => Ok(ApiResponse::error(&e.to_string())),
    }
}

pub async fn api_update_gasto(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
    Json(dto): Json<CreateGastoDto>,
) -> Result<Json<ApiResponse<Gasto>>, StatusCode> {
    match state.gasto_service.editar_gasto(user.id, id, dto).await {
        Ok(gasto) => Ok(ApiResponse::ok(gasto)),
        Err(e) => Ok(ApiResponse::error(&e.to_string())),
    }
}

pub async fn api_delete_gasto(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<Gasto>>, StatusCode> {
    match state.gasto_service.eliminar_gasto(user.id, id).await {
        Ok(gasto) => Ok(ApiResponse::ok(gasto)),
        Err(e) => Ok(ApiResponse::error(&e.to_string())),
    }
}

pub async fn api_marcar_gasto_pagado(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<Gasto>>, StatusCode> {
    match state.gasto_service.marcar_pagado(user.id, id).await {
        Ok(gasto) => Ok(ApiResponse::ok(gasto)),
        Err(e) => Ok(ApiResponse::error(&e.to_string())),
    }
}

// === CRÉDITOS ===

pub async fn api_list_creditos(
    State(state): State<AppState>,
    user: AuthUser,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<ApiResponse<PaginatedResponse<Credito>>>, StatusCode> {
    let (creditos, total) = state.credito_service.listar_creditos(user.id, pagination.page, pagination.page_size).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(ApiResponse::ok(PaginatedResponse {
        items: creditos,
        total,
        page: pagination.page,
        page_size: pagination.page_size,
    }))
}

pub async fn api_get_credito(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<Credito>>, StatusCode> {
    match state.credito_service.obtener_credito(user.id, id).await {
        Ok(credito) => Ok(ApiResponse::ok(credito)),
        Err(_) => Ok(ApiResponse::error("Crédito no encontrado")),
    }
}

pub async fn api_create_credito(
    State(state): State<AppState>,
    user: AuthUser,
    Json(dto): Json<CreateCreditoDto>,
) -> Result<Json<ApiResponse<Credito>>, StatusCode> {
    match state.credito_service.crear_credito(user.id, dto).await {
        Ok(credito) => Ok(ApiResponse::ok(credito)),
        Err(e) => Ok(ApiResponse::error(&e.to_string())),
    }
}

pub async fn api_update_credito(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
    Json(dto): Json<CreateCreditoDto>,
) -> Result<Json<ApiResponse<Credito>>, StatusCode> {
    match state.credito_service.editar_credito(user.id, id, dto).await {
        Ok(credito) => Ok(ApiResponse::ok(credito)),
        Err(e) => Ok(ApiResponse::error(&e.to_string())),
    }
}

pub async fn api_delete_credito(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<Credito>>, StatusCode> {
    match state.credito_service.eliminar_credito(user.id, id).await {
        Ok(credito) => Ok(ApiResponse::ok(credito)),
        Err(e) => Ok(ApiResponse::error(&e.to_string())),
    }
}

pub async fn api_registrar_cuota(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<Credito>>, StatusCode> {
    match state.credito_service.registrar_cuota(user.id, id).await {
        Ok(credito) => Ok(ApiResponse::ok(credito)),
        Err(e) => Ok(ApiResponse::error(&e.to_string())),
    }
}

pub async fn api_finalizar_credito(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<Credito>>, StatusCode> {
    match state.credito_service.finalizar_credito(user.id, id).await {
        Ok(credito) => Ok(ApiResponse::ok(credito)),
        Err(e) => Ok(ApiResponse::error(&e.to_string())),
    }
}

// === DOCUMENTOS ===

pub async fn api_list_documentos(
    State(state): State<AppState>,
    user: AuthUser,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<ApiResponse<PaginatedResponse<Documento>>>, StatusCode> {
    let (documentos, total) = state.documento_service.listar_documentos(user.id, pagination.page, pagination.page_size).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(ApiResponse::ok(PaginatedResponse {
        items: documentos,
        total,
        page: pagination.page,
        page_size: pagination.page_size,
    }))
}

pub async fn api_delete_documento(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<Documento>>, StatusCode> {
    match state.documento_service.eliminar_documento(user.id, id).await {
        Ok(doc) => Ok(ApiResponse::ok(doc)),
        Err(e) => Ok(ApiResponse::error(&e.to_string())),
    }
}

// === PROYECTOS ===

pub async fn api_list_proyectos(
    State(state): State<AppState>,
    user: AuthUser,
) -> Result<Json<ApiResponse<Vec<Proyecto>>>, StatusCode> {
    let (proyectos, _) = state.proyecto_service.list_all_proyectos(user.id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(ApiResponse::ok(proyectos))
}

pub async fn api_get_proyecto(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<Proyecto>>, StatusCode> {
    match state.proyecto_service.get_proyecto_by_id(user.id, id).await {
        Ok(proyecto) => Ok(ApiResponse::ok(proyecto)),
        Err(_) => Ok(ApiResponse::error("Proyecto no encontrado")),
    }
}

pub async fn api_list_pagos_proyecto(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<ApiResponse<PaginatedResponse<PagoExistente>>>, StatusCode> {
    let (_, pagos, total) = state.proyecto_service.get_proyecto_with_pagos(user.id, id, pagination.page, pagination.page_size).await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(ApiResponse::ok(PaginatedResponse {
        items: pagos,
        total,
        page: pagination.page,
        page_size: pagination.page_size,
    }))
}

// === AUTH ===

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Login para clientes API (app móvil): devuelve el JWT en el body en lugar de cookie.
pub async fn api_login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> (StatusCode, Json<ApiResponse<LoginResponse>>) {
    match state.auth_service.login(body.username.trim(), &body.password).await {
        Ok(login) => (StatusCode::OK, ApiResponse::ok(login)),
        Err(e) => (StatusCode::UNAUTHORIZED, ApiResponse::error(&e.to_string())),
    }
}

// === GASTOS RECURRENTES ===

/// Listado de plantillas + resumen, igual que la vista web `/gastos-recurrentes`.
#[derive(Serialize)]
pub struct GastosRecurrentesResponse {
    pub items: Vec<GastoRecurrente>,
    /// Suma del monto de referencia de las plantillas activas.
    pub monto_mensual_estimado: Decimal,
    /// Plantillas Fijo Variable activas que aún no se han generado en el mes actual.
    pub pendientes_generar: u32,
    pub mes: u32,
    pub anio: i32,
}

/// Validación mínima del payload (el formulario web la hace con HTML: día 1-28, monto >= 0).
fn validar_gasto_recurrente(dto: &CreateGastoRecurrenteDto) -> Result<(), &'static str> {
    if dto.descripcion.trim().is_empty() {
        return Err("La descripción es obligatoria");
    }
    if dto.monto_referencia < Decimal::ZERO {
        return Err("El monto de referencia no puede ser negativo");
    }
    if !(1..=28).contains(&dto.dia_facturacion) {
        return Err("El día de facturación debe estar entre 1 y 28");
    }
    Ok(())
}

pub async fn api_list_gastos_recurrentes(
    State(state): State<AppState>,
    user: AuthUser,
) -> Result<Json<ApiResponse<GastosRecurrentesResponse>>, StatusCode> {
    use chrono::Datelike;

    // Igual que la web: al entrar se generan los gastos fijos cuyo día de facturación ya pasó.
    let _ = state.gasto_recurrente_service.auto_generar_fijos(user.id).await;

    let items = state.gasto_recurrente_service.listar_todos(user.id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let monto_mensual_estimado = items.iter().filter(|g| g.activo).map(|g| g.monto_referencia).sum();

    let now = chrono::Local::now();
    let (mes, anio) = (now.month(), now.year());
    let pendientes_generar = state.gasto_recurrente_service
        .variables_pendientes_por_generar(user.id, anio, mes).await
        .unwrap_or(0);

    Ok(ApiResponse::ok(GastosRecurrentesResponse { items, monto_mensual_estimado, pendientes_generar, mes, anio }))
}

pub async fn api_get_gasto_recurrente(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<GastoRecurrente>>, StatusCode> {
    match state.gasto_recurrente_service.obtener(user.id, id).await {
        Ok(g) => Ok(ApiResponse::ok(g)),
        Err(_) => Ok(ApiResponse::error("Gasto recurrente no encontrado")),
    }
}

pub async fn api_create_gasto_recurrente(
    State(state): State<AppState>,
    user: AuthUser,
    Json(dto): Json<CreateGastoRecurrenteDto>,
) -> Result<Json<ApiResponse<GastoRecurrente>>, StatusCode> {
    if let Err(msg) = validar_gasto_recurrente(&dto) {
        return Ok(ApiResponse::error(msg));
    }
    match state.gasto_recurrente_service.crear(user.id, dto).await {
        Ok(g) => Ok(ApiResponse::ok(g)),
        Err(e) => Ok(ApiResponse::error(&e.to_string())),
    }
}

pub async fn api_update_gasto_recurrente(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
    Json(dto): Json<CreateGastoRecurrenteDto>,
) -> Result<Json<ApiResponse<GastoRecurrente>>, StatusCode> {
    if let Err(msg) = validar_gasto_recurrente(&dto) {
        return Ok(ApiResponse::error(msg));
    }
    match state.gasto_recurrente_service.editar(user.id, id, dto).await {
        Ok(g) => Ok(ApiResponse::ok(g)),
        Err(e) => Ok(ApiResponse::error(&e.to_string())),
    }
}

pub async fn api_toggle_gasto_recurrente(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<GastoRecurrente>>, StatusCode> {
    match state.gasto_recurrente_service.toggle_activo(user.id, id).await {
        Ok(g) => Ok(ApiResponse::ok(g)),
        Err(e) => Ok(ApiResponse::error(&e.to_string())),
    }
}

pub async fn api_delete_gasto_recurrente(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<GastoRecurrente>>, StatusCode> {
    match state.gasto_recurrente_service.eliminar(user.id, id).await {
        Ok(g) => Ok(ApiResponse::ok(g)),
        Err(e) => Ok(ApiResponse::error(&e.to_string())),
    }
}

/// Genera los gastos Fijo Variable del mes indicado (botón "Generar" de la web). Devuelve los gastos creados.
pub async fn api_generar_gastos_mes(
    State(state): State<AppState>,
    user: AuthUser,
    Json(dto): Json<GenerarGastosDto>,
) -> Result<Json<ApiResponse<Vec<Gasto>>>, StatusCode> {
    if !(1..=12).contains(&dto.mes) {
        return Ok(ApiResponse::error("El mes debe estar entre 1 y 12"));
    }
    match state.gasto_recurrente_service.generar_variables_del_mes(user.id, dto).await {
        Ok(gastos) => Ok(ApiResponse::ok(gastos)),
        Err(e) => Ok(ApiResponse::error(&e.to_string())),
    }
}

// === PROYECTOS: ESCRITURA ===

fn validar_proyecto(nombre: Option<&str>, presupuesto: Option<Decimal>) -> Result<(), &'static str> {
    if let Some(n) = nombre {
        if n.trim().is_empty() {
            return Err("El nombre del proyecto es obligatorio");
        }
    }
    if presupuesto.is_some_and(|p| p < Decimal::ZERO) {
        return Err("El presupuesto no puede ser negativo");
    }
    Ok(())
}

/// El repositorio parsea `fecha_fin_estimada` como "%Y-%m-%d %H:%M:%S" y descarta en silencio otro formato.
/// El API recibe "YYYY-MM-DD" (o fecha-hora completa) y lo normaliza; si no es válida, devuelve error.
fn normalizar_fecha_fin(fecha: Option<String>) -> Result<Option<String>, &'static str> {
    let Some(f) = fecha.map(|f| f.trim().to_string()).filter(|f| !f.is_empty()) else {
        return Ok(None);
    };
    if chrono::NaiveDateTime::parse_from_str(&f, "%Y-%m-%d %H:%M:%S").is_ok() {
        return Ok(Some(f));
    }
    chrono::NaiveDate::parse_from_str(&f, "%Y-%m-%d")
        .map(|d| Some(format!("{} 00:00:00", d)))
        .map_err(|_| "La fecha fin estimada debe tener formato AAAA-MM-DD")
}

pub async fn api_create_proyecto(
    State(state): State<AppState>,
    user: AuthUser,
    Json(mut dto): Json<CreateProyectoDto>,
) -> Result<Json<ApiResponse<Proyecto>>, StatusCode> {
    if let Err(msg) = validar_proyecto(Some(&dto.nombre), dto.presupuesto) {
        return Ok(ApiResponse::error(msg));
    }
    dto.fecha_fin_estimada = match normalizar_fecha_fin(dto.fecha_fin_estimada.take()) {
        Ok(f) => f,
        Err(msg) => return Ok(ApiResponse::error(msg)),
    };
    match state.proyecto_service.crear_proyecto(user.id, dto).await {
        Ok(p) => Ok(ApiResponse::ok(p)),
        Err(e) => Ok(ApiResponse::error(&e.to_string())),
    }
}

/// Campos en `null` se conservan (COALESCE en el repositorio); un texto vacío los limpia, igual que el formulario web.
pub async fn api_update_proyecto(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
    Json(mut dto): Json<UpdateProyectoDto>,
) -> Result<Json<ApiResponse<Proyecto>>, StatusCode> {
    if let Err(msg) = validar_proyecto(dto.nombre.as_deref(), dto.presupuesto) {
        return Ok(ApiResponse::error(msg));
    }
    dto.fecha_fin_estimada = match normalizar_fecha_fin(dto.fecha_fin_estimada.take()) {
        Ok(f) => f,
        Err(msg) => return Ok(ApiResponse::error(msg)),
    };
    match state.proyecto_service.actualizar_proyecto(user.id, id, dto).await {
        Ok(p) => Ok(ApiResponse::ok(p)),
        Err(e) => Ok(ApiResponse::error(&e.to_string())),
    }
}

#[derive(Deserialize)]
pub struct CambiarEstadoRequest {
    pub estado: String,
}

/// Acepta el nombre de la variante ("EnProgreso", como lo devuelve el API) o el valor de BD ("En_Progreso").
/// Al pasar a En_Progreso se fija la fecha de inicio y a Completado la fecha fin real (lógica del repositorio).
pub async fn api_cambiar_estado_proyecto(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
    Json(body): Json<CambiarEstadoRequest>,
) -> Result<Json<ApiResponse<Proyecto>>, StatusCode> {
    let estado_bd = match body.estado.to_lowercase().replace('_', "").as_str() {
        "planificacion" => "Planificacion",
        "enprogreso" => "En_Progreso",
        "pausado" => "Pausado",
        "completado" => "Completado",
        "cancelado" => "Cancelado",
        _ => return Ok(ApiResponse::error("Estado de proyecto no válido")),
    };
    if let Err(e) = state.proyecto_service.cambiar_estado_proyecto(user.id, id, estado_bd).await {
        return Ok(ApiResponse::error(&e.to_string()));
    }
    match state.proyecto_service.get_proyecto_by_id(user.id, id).await {
        Ok(p) => Ok(ApiResponse::ok(p)),
        Err(_) => Ok(ApiResponse::error("Proyecto no encontrado")),
    }
}

/// Totales de pagos del proyecto, con el mismo cálculo que la vista web "Plan de pagos".
#[derive(Serialize)]
pub struct ProyectoTotales {
    pub total_valor: Decimal,
    pub total_abonado: Decimal,
    /// Presupuesto - abonado; null si el proyecto no tiene presupuesto.
    pub saldo_pendiente_proyecto: Option<Decimal>,
    pub pagos_completados: i64,
    pub total_pagos: i64,
}

pub async fn api_totales_proyecto(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<ProyectoTotales>>, StatusCode> {
    let proyecto = match state.proyecto_service.get_proyecto_by_id(user.id, id).await {
        Ok(p) => p,
        Err(_) => return Ok(ApiResponse::error("Proyecto no encontrado")),
    };
    let (total_valor, total_saldo, pagos_completados, total_pagos) = state.pago_service
        .obtener_totales_proyecto(user.id, id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let total_abonado = total_valor - total_saldo;
    Ok(ApiResponse::ok(ProyectoTotales {
        total_valor,
        total_abonado,
        saldo_pendiente_proyecto: proyecto.presupuesto.map(|p| p - total_abonado),
        pagos_completados,
        total_pagos,
    }))
}

// === PAGOS DE PROYECTO ===

const MESES: [&str; 12] = [
    "enero", "febrero", "marzo", "abril", "mayo", "junio",
    "julio", "agosto", "septiembre", "octubre", "noviembre", "diciembre",
];

#[derive(Deserialize)]
pub struct PagoRequest {
    pub descripcion: String,
    pub valor: Decimal,
    /// En minúscula, como el formulario web: "enero".."diciembre".
    pub mes: String,
    pub anio: String,
}

impl PagoRequest {
    fn validar(&self) -> Result<(), &'static str> {
        if self.descripcion.trim().is_empty() {
            return Err("La descripción es obligatoria");
        }
        if self.valor <= Decimal::ZERO {
            return Err("El valor debe ser mayor a cero");
        }
        if !MESES.contains(&self.mes.to_lowercase().as_str()) {
            return Err("Mes no válido");
        }
        if self.anio.len() != 4 || !self.anio.chars().all(|c| c.is_ascii_digit()) {
            return Err("Año no válido");
        }
        Ok(())
    }
}

/// Crea un pago del proyecto. Falla si excede el presupuesto (misma validación que la web).
pub async fn api_create_pago_proyecto(
    State(state): State<AppState>,
    user: AuthUser,
    Path(proyecto_id): Path<i32>,
    Json(body): Json<PagoRequest>,
) -> Result<Json<ApiResponse<PagoExistente>>, StatusCode> {
    if let Err(msg) = body.validar() {
        return Ok(ApiResponse::error(msg));
    }
    let dto = CreatePagoDto {
        descripcion: body.descripcion.trim().to_string(),
        valor: body.valor,
        mes: body.mes.to_lowercase(),
        anio: body.anio,
        proyecto_id: Some(proyecto_id),
    };
    match state.pago_service.crear_pago(user.id, dto).await {
        Ok(p) => Ok(ApiResponse::ok(p)),
        Err(e) => Ok(ApiResponse::error(&e.to_string())),
    }
}

pub async fn api_get_pago(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<PagoExistente>>, StatusCode> {
    match state.pago_service.obtener_pago(user.id, id).await {
        Ok(p) => Ok(ApiResponse::ok(p)),
        Err(_) => Ok(ApiResponse::error("Pago no encontrado")),
    }
}

/// Edita descripción, valor y periodo. El saldo se ajusta por la diferencia de valor (lógica del repositorio).
pub async fn api_update_pago(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
    Json(body): Json<PagoRequest>,
) -> Result<Json<ApiResponse<PagoExistente>>, StatusCode> {
    if let Err(msg) = body.validar() {
        return Ok(ApiResponse::error(msg));
    }
    match state.pago_service
        .editar_pago(user.id, id, body.descripcion.trim(), body.valor, &body.mes.to_lowercase(), &body.anio)
        .await
    {
        Ok(p) => Ok(ApiResponse::ok(p)),
        Err(e) => Ok(ApiResponse::error(&e.to_string())),
    }
}

pub async fn api_marcar_pago_pagado(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<PagoExistente>>, StatusCode> {
    match state.pago_service.marcar_pagado(user.id, id).await {
        Ok(p) => Ok(ApiResponse::ok(p)),
        Err(e) => Ok(ApiResponse::error(&e.to_string())),
    }
}

/// Elimina el pago y sus evidencias en el storage.
pub async fn api_delete_pago(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<PagoExistente>>, StatusCode> {
    match state.pago_service.eliminar_pago(user.id, id).await {
        Ok(p) => Ok(ApiResponse::ok(p)),
        Err(e) => Ok(ApiResponse::error(&e.to_string())),
    }
}

// === SUBIDA DE ARCHIVOS (multipart, mismos nombres de campo que los formularios web) ===

/// Campos de texto y archivos de un multipart. Los archivos vacíos o sin nombre se ignoran, igual que en la web.
#[derive(Default)]
struct MultipartData {
    texts: std::collections::HashMap<String, String>,
    files: std::collections::HashMap<String, (Vec<u8>, String)>,
}

impl MultipartData {
    fn text(&self, name: &str) -> Option<String> {
        self.texts.get(name).map(|t| t.trim().to_string()).filter(|t| !t.is_empty())
    }
}

async fn read_multipart(mut multipart: Multipart) -> Result<MultipartData, &'static str> {
    const ERROR: &str = "No se pudo leer el archivo enviado (tamaño máximo 20 MB)";
    let mut data = MultipartData::default();
    while let Some(field) = multipart.next_field().await.map_err(|_| ERROR)? {
        let name = field.name().unwrap_or("").to_string();
        match field.file_name().map(|f| f.to_string()) {
            Some(file_name) if !file_name.is_empty() => {
                let bytes = field.bytes().await.map_err(|_| ERROR)?.to_vec();
                if !bytes.is_empty() {
                    data.files.insert(name, (bytes, file_name));
                }
            }
            _ => {
                let text = field.text().await.map_err(|_| ERROR)?;
                data.texts.insert(name, text);
            }
        }
    }
    Ok(data)
}

/// Sube o reemplaza el soporte de un gasto. Campo: `soporte`.
pub async fn api_upload_soporte_gasto(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
    multipart: Multipart,
) -> Result<Json<ApiResponse<Gasto>>, StatusCode> {
    let mut data = match read_multipart(multipart).await {
        Ok(d) => d,
        Err(msg) => return Ok(ApiResponse::error(msg)),
    };
    let Some((bytes, file_name)) = data.files.remove("soporte") else {
        return Ok(ApiResponse::error("Adjunta el archivo del soporte"));
    };
    // Validar antes de subir para no dejar archivos huérfanos en el storage.
    if state.gasto_service.obtener_gasto(user.id, id).await.is_err() {
        return Ok(ApiResponse::error("Gasto no encontrado"));
    }
    match state.gasto_service.subir_soporte(user.id, id, bytes, &file_name).await {
        Ok(g) => Ok(ApiResponse::ok(g)),
        Err(e) => Ok(ApiResponse::error(&format!("No se pudo subir el soporte: {}", e))),
    }
}

/// Crea un documento con su archivo. Campos: `nombre`, `categoria`, `descripcion`?, `fecha_vencimiento`? (AAAA-MM-DD), `archivo`.
pub async fn api_create_documento(
    State(state): State<AppState>,
    user: AuthUser,
    multipart: Multipart,
) -> Result<Json<ApiResponse<Documento>>, StatusCode> {
    let mut data = match read_multipart(multipart).await {
        Ok(d) => d,
        Err(msg) => return Ok(ApiResponse::error(msg)),
    };
    let Some(nombre) = data.text("nombre") else {
        return Ok(ApiResponse::error("El nombre del documento es obligatorio"));
    };
    let Some(categoria) = data.text("categoria") else {
        return Ok(ApiResponse::error("La categoría es obligatoria"));
    };
    let fecha_vencimiento = data.text("fecha_vencimiento");
    // El repositorio descarta en silencio una fecha con otro formato: mejor rechazarla.
    if let Some(f) = &fecha_vencimiento {
        if chrono::NaiveDate::parse_from_str(f, "%Y-%m-%d").is_err() {
            return Ok(ApiResponse::error("La fecha de vencimiento debe tener formato AAAA-MM-DD"));
        }
    }
    let Some((bytes, file_name)) = data.files.remove("archivo") else {
        return Ok(ApiResponse::error("Adjunta el archivo del documento"));
    };
    let dto = CreateDocumentoDto {
        nombre,
        descripcion: data.text("descripcion"),
        categoria,
        fecha_vencimiento,
    };
    match state.documento_service.crear_documento(user.id, dto, bytes, &file_name).await {
        Ok(d) => Ok(ApiResponse::ok(d)),
        Err(e) => Ok(ApiResponse::error(&format!("No se pudo guardar el documento: {}", e))),
    }
}

/// Sube o reemplaza un comprobante de pago. Campo `evidencia_cliente` o `evidencia_constructora`.
pub async fn api_upload_evidencia_pago(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
    multipart: Multipart,
) -> Result<Json<ApiResponse<PagoExistente>>, StatusCode> {
    let mut data = match read_multipart(multipart).await {
        Ok(d) => d,
        Err(msg) => return Ok(ApiResponse::error(msg)),
    };
    let (tipo, (bytes, file_name)) = if let Some(f) = data.files.remove("evidencia_cliente") {
        ("cliente", f)
    } else if let Some(f) = data.files.remove("evidencia_constructora") {
        ("constructora", f)
    } else {
        return Ok(ApiResponse::error("Adjunta el comprobante"));
    };
    if state.pago_service.obtener_pago(user.id, id).await.is_err() {
        return Ok(ApiResponse::error("Pago no encontrado"));
    }
    match state.pago_service.subir_evidencia(user.id, id, bytes, &file_name, tipo).await {
        Ok(p) => Ok(ApiResponse::ok(p)),
        Err(e) => Ok(ApiResponse::error(&format!("No se pudo subir el comprobante: {}", e))),
    }
}
