use axum::{
    extract::{Path, State, Query, Json},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

use crate::application::dto::*;
use crate::domain::entities::*;
use crate::presentation::web::server::AppState;
use crate::presentation::middleware::AuthUser;

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
