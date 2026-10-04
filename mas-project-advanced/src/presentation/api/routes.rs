use axum::{
    routing::{get, post, put, delete},
    Router,
};
use super::handlers;
use crate::presentation::web::server::AppState;

pub fn api_routes() -> Router<AppState> {
    Router::new()
        // Ingresos
        .route("/ingresos", get(handlers::api_list_ingresos))
        .route("/ingresos", post(handlers::api_create_ingreso))
        .route("/ingresos/:id", get(handlers::api_get_ingreso))
        .route("/ingresos/:id", put(handlers::api_update_ingreso))
        .route("/ingresos/:id", delete(handlers::api_delete_ingreso))
        // Gastos
        .route("/gastos", get(handlers::api_list_gastos))
        .route("/gastos", post(handlers::api_create_gasto))
        .route("/gastos/:id", get(handlers::api_get_gasto))
        .route("/gastos/:id", put(handlers::api_update_gasto))
        .route("/gastos/:id", delete(handlers::api_delete_gasto))
        .route("/gastos/:id/pagado", post(handlers::api_marcar_gasto_pagado))
        .route("/gastos/:id/soporte", post(handlers::api_upload_soporte_gasto))
        // Gastos recurrentes
        .route("/gastos-recurrentes", get(handlers::api_list_gastos_recurrentes))
        .route("/gastos-recurrentes", post(handlers::api_create_gasto_recurrente))
        .route("/gastos-recurrentes/generar", post(handlers::api_generar_gastos_mes))
        .route("/gastos-recurrentes/:id", get(handlers::api_get_gasto_recurrente))
        .route("/gastos-recurrentes/:id", put(handlers::api_update_gasto_recurrente))
        .route("/gastos-recurrentes/:id", delete(handlers::api_delete_gasto_recurrente))
        .route("/gastos-recurrentes/:id/toggle", post(handlers::api_toggle_gasto_recurrente))
        // Créditos
        .route("/creditos", get(handlers::api_list_creditos))
        .route("/creditos", post(handlers::api_create_credito))
        .route("/creditos/:id", get(handlers::api_get_credito))
        .route("/creditos/:id", put(handlers::api_update_credito))
        .route("/creditos/:id", delete(handlers::api_delete_credito))
        .route("/creditos/:id/cuota", post(handlers::api_registrar_cuota))
        .route("/creditos/:id/finalizar", post(handlers::api_finalizar_credito))
        // Documentos
        .route("/documentos", get(handlers::api_list_documentos))
        .route("/documentos", post(handlers::api_create_documento))
        .route("/documentos/:id", delete(handlers::api_delete_documento))
        // Proyectos
        .route("/proyectos", get(handlers::api_list_proyectos))
        .route("/proyectos", post(handlers::api_create_proyecto))
        .route("/proyectos/:id", get(handlers::api_get_proyecto))
        .route("/proyectos/:id", put(handlers::api_update_proyecto))
        .route("/proyectos/:id/estado", post(handlers::api_cambiar_estado_proyecto))
        .route("/proyectos/:id/totales", get(handlers::api_totales_proyecto))
        .route("/proyectos/:id/pagos", get(handlers::api_list_pagos_proyecto))
        .route("/proyectos/:id/pagos", post(handlers::api_create_pago_proyecto))
        // Pagos de proyecto
        .route("/pagos/:id", get(handlers::api_get_pago))
        .route("/pagos/:id", put(handlers::api_update_pago))
        .route("/pagos/:id", delete(handlers::api_delete_pago))
        .route("/pagos/:id/pagado", post(handlers::api_marcar_pago_pagado))
        .route("/pagos/:id/evidencia", post(handlers::api_upload_evidencia_pago))
        // Dashboard
        .route("/dashboard", get(handlers::api_dashboard))
}

/// Rutas API públicas (sin auth_guard).
pub fn api_public_routes() -> Router<AppState> {
    Router::new()
        .route("/auth/login", post(handlers::api_login))
}
