use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;
use tower_http::cors::{CorsLayer, Any};
use std::sync::Arc;
use crate::application::services::proyecto_service::ProyectoService;
use crate::application::services::pago_service::PagoService;
use crate::application::services::ingreso_service::IngresoService;
use crate::application::services::gasto_service::GastoService;
use crate::application::services::credito_service::CreditoService;
use crate::application::services::documento_service::DocumentoService;
use crate::application::services::gasto_recurrente_service::GastoRecurrenteService;

use super::handlers::*;
use crate::presentation::api::routes::api_routes;

#[derive(Clone)]
pub struct AppState {
    pub proyecto_service: Arc<ProyectoService>,
    pub pago_service: Arc<PagoService>,
    pub ingreso_service: Arc<IngresoService>,
    pub gasto_service: Arc<GastoService>,
    pub credito_service: Arc<CreditoService>,
    pub documento_service: Arc<DocumentoService>,
    pub gasto_recurrente_service: Arc<GastoRecurrenteService>,
}

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/", get(home))
        .route("/dashboard", get(dashboard))
        .route("/proyectos", get(list_proyectos))
        .route("/proyectos/new", get(new_proyecto_form))
        .route("/proyectos", post(create_proyecto))
        .route("/proyectos/:id", get(show_proyecto))
        .route("/proyectos/:id/edit", get(edit_proyecto_form))
        .route("/proyectos/:id/update", post(update_proyecto))
        .route("/proyectos/:id/estado", post(cambiar_estado_proyecto))
        .route("/proyectos/:id/pagos", get(list_pagos_proyecto))
        .route("/proyectos/:id/pagos/new", get(new_pago_proyecto_form))
        .route("/proyectos/:id/pagos", post(create_pago_proyecto))
        .route("/pagos/:id/evidencia", post(upload_evidencia_pago))
        .route("/pagos/:id/pagado", post(marcar_pago_pagado))
        .route("/pagos/:id/eliminar", post(eliminar_pago))
        .route("/pagos/:id/editar", get(edit_pago_form))
        .route("/pagos/:id/editar", post(update_pago))
        // Ingresos
        .route("/ingresos", get(list_ingresos))
        .route("/ingresos/new", get(new_ingreso_form))
        .route("/ingresos", post(create_ingreso))
        .route("/ingresos/:id/eliminar", post(eliminar_ingreso))
        .route("/ingresos/:id/editar", get(edit_ingreso_form))
        .route("/ingresos/:id/editar", post(update_ingreso))
        // Gastos
        .route("/gastos", get(list_gastos))
        .route("/gastos/new", get(new_gasto_form))
        .route("/gastos", post(create_gasto))
        .route("/gastos/:id/pagado", post(marcar_gasto_pagado))
        .route("/gastos/:id/eliminar", post(eliminar_gasto))
        .route("/gastos/:id/editar", get(edit_gasto_form))
        .route("/gastos/:id/editar", post(update_gasto))
        .route("/gastos/:id/soporte", post(upload_soporte_gasto))
        // Gastos Recurrentes
        .route("/gastos-recurrentes", get(list_gastos_recurrentes))
        .route("/gastos-recurrentes/new", get(new_gasto_recurrente_form))
        .route("/gastos-recurrentes", post(create_gasto_recurrente))
        .route("/gastos-recurrentes/:id/editar", get(edit_gasto_recurrente_form))
        .route("/gastos-recurrentes/:id/editar", post(update_gasto_recurrente))
        .route("/gastos-recurrentes/:id/toggle", post(toggle_gasto_recurrente))
        .route("/gastos-recurrentes/:id/eliminar", post(eliminar_gasto_recurrente))
        .route("/gastos-recurrentes/generar", post(generar_gastos_mes))
        // Créditos
        .route("/creditos", get(list_creditos))
        .route("/creditos/new", get(new_credito_form))
        .route("/creditos", post(create_credito))
        .route("/creditos/:id/cuota", post(registrar_cuota_credito))
        .route("/creditos/:id/editar", get(edit_credito_form))
        .route("/creditos/:id/editar", post(update_credito))
        .route("/creditos/:id/eliminar", post(eliminar_credito))
        // Documentos
        .route("/documentos", get(list_documentos))
        .route("/documentos/new", get(new_documento_form))
        .route("/documentos", post(create_documento))
        .route("/documentos/:id/eliminar", post(eliminar_documento))
        // API REST para app móvil
        .nest("/api/v1", api_routes())
        .nest_service("/static", ServeDir::new("static"))
        .layer(CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
        )
        .with_state(state)
}