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
        // Créditos
        .route("/creditos", get(handlers::api_list_creditos))
        .route("/creditos", post(handlers::api_create_credito))
        .route("/creditos/:id", get(handlers::api_get_credito))
        .route("/creditos/:id", put(handlers::api_update_credito))
        .route("/creditos/:id", delete(handlers::api_delete_credito))
        .route("/creditos/:id/cuota", post(handlers::api_registrar_cuota))
        // Documentos
        .route("/documentos", get(handlers::api_list_documentos))
        .route("/documentos/:id", delete(handlers::api_delete_documento))
        // Proyectos
        .route("/proyectos", get(handlers::api_list_proyectos))
        .route("/proyectos/:id", get(handlers::api_get_proyecto))
        .route("/proyectos/:id/pagos", get(handlers::api_list_pagos_proyecto))
        // Dashboard
        .route("/dashboard", get(handlers::api_dashboard))
}