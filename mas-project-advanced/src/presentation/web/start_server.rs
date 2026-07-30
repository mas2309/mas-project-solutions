use crate::shared::config::AppConfig;
use crate::infrastructure::database::{DatabaseManager, ProyectoRepository, PagoRepository, IngresoRepository, GastoRepository, CreditoRepository, DocumentoRepository, GastoRecurrenteRepository};
use crate::infrastructure::storage::s3_storage_service::ContaboStorageService;
use crate::application::services::{
    proyecto_service::ProyectoService,
    pago_service::PagoService,
    ingreso_service::IngresoService,
    gasto_service::GastoService,
    credito_service::CreditoService,
    documento_service::DocumentoService,
    gasto_recurrente_service::GastoRecurrenteService,
};
use super::server::{create_app, AppState};
use anyhow::Result;
use std::sync::Arc;

pub async fn start_web_server(config: &AppConfig) -> Result<()> {
    let pool = DatabaseManager::create_pool(config).await?;
    
    // Crear instancias de repositorios
    let proyecto_repo = Arc::new(ProyectoRepository::new(pool.clone()));
    let pago_repo = Arc::new(PagoRepository::new(pool.clone()));
    let ingreso_repo = Arc::new(IngresoRepository::new(pool.clone()));
    let gasto_repo = Arc::new(GastoRepository::new(pool.clone()));
    let credito_repo = Arc::new(CreditoRepository::new(pool.clone()));
    let documento_repo = Arc::new(DocumentoRepository::new(pool.clone()));
    let gasto_recurrente_repo = Arc::new(GastoRecurrenteRepository::new(pool.clone()));
    
    // Crear servicio de almacenamiento Contabo Object Storage
    let storage_service = Arc::new(
        ContaboStorageService::new(&config.storage).await?
    );
    
    // Crear servicios
    let proyecto_service = Arc::new(ProyectoService::new(proyecto_repo.clone()));
    let pago_service = Arc::new(PagoService::new(pago_repo, proyecto_repo, storage_service.clone(), config.storage.bucket_proyectos.clone()));
    let ingreso_service = Arc::new(IngresoService::new(ingreso_repo));
    let gasto_service = Arc::new(GastoService::new(gasto_repo.clone(), storage_service.clone(), config.storage.bucket_gastos.clone()));
    let credito_service = Arc::new(CreditoService::new(credito_repo));
    let documento_service = Arc::new(DocumentoService::new(documento_repo, storage_service, config.storage.bucket_documentos.clone()));
    let gasto_recurrente_service = Arc::new(GastoRecurrenteService::new(gasto_recurrente_repo, gasto_repo));
    
    // Crear el estado de la aplicación
    let app_state = AppState {
        proyecto_service,
        pago_service,
        ingreso_service,
        gasto_service,
        credito_service,
        documento_service,
        gasto_recurrente_service,
    };

    let app = create_app(app_state);
    
    let listener = tokio::net::TcpListener::bind(&config.server_address()).await?;
    println!("🌐 Servidor ejecutándose en http://{}", config.server_address());
    println!("📋 Visita http://{}/proyectos para gestionar proyectos", config.server_address());
    
    axum::serve(listener, app).await?;
    Ok(())
}