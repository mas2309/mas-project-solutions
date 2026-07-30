use std::sync::Arc;
use crate::application::repositories::gasto_repository::IGastoRepository;
use crate::application::services::storage_service::IStorageService;
use crate::domain::entities::Gasto;
use crate::application::dto::CreateGastoDto;
use anyhow::{Result, anyhow};

pub struct GastoService {
    repository: Arc<dyn IGastoRepository>,
    storage_service: Arc<dyn IStorageService>,
    bucket: String,
}

impl GastoService {
    pub fn new(repository: Arc<dyn IGastoRepository>, storage_service: Arc<dyn IStorageService>, bucket: String) -> Self {
        Self { repository, storage_service, bucket }
    }

    pub async fn crear_gasto(&self, dto: CreateGastoDto) -> Result<Gasto> {
        self.repository.create(dto).await
    }

    pub async fn listar_gastos(&self, page: u32, page_size: u32) -> Result<(Vec<Gasto>, i64)> {
        self.repository.list_all(page, page_size).await
    }

    pub async fn marcar_pagado(&self, id: i32) -> Result<Gasto> {
        self.repository.marcar_pagado(id).await?
            .ok_or_else(|| anyhow!("Gasto no encontrado"))
    }

    pub async fn eliminar_gasto(&self, id: i32) -> Result<Gasto> {
        let gasto = self.repository.find_by_id(id).await?
            .ok_or_else(|| anyhow!("Gasto no encontrado"))?;

        if let Some(ref soporte) = gasto.soporte {
            if let Err(e) = self.storage_service.delete_file(soporte).await {
                println!("⚠️ Error eliminando soporte: {}", e);
            }
        }

        self.repository.delete(id).await?
            .ok_or_else(|| anyhow!("Error eliminando gasto"))
    }

    pub async fn subir_soporte(&self, id: i32, file_data: Vec<u8>, file_name: &str) -> Result<Gasto> {
        let extension = file_name.split('.').last().unwrap_or("bin");
        let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S");
        let storage_name = format!("soportes-gastos/gasto-{}-{}.{}", id, timestamp, extension);
        let file_url = self.storage_service.upload_file(file_data, &storage_name, &self.bucket).await?;
        
        self.repository.actualizar_soporte(id, &file_url).await?
            .ok_or_else(|| anyhow!("Gasto no encontrado"))
    }

    pub async fn obtener_gasto(&self, id: i32) -> Result<Gasto> {
        self.repository.find_by_id(id).await?
            .ok_or_else(|| anyhow!("Gasto no encontrado"))
    }

    pub async fn editar_gasto(&self, id: i32, dto: CreateGastoDto) -> Result<Gasto> {
        self.repository.update(id, dto).await?
            .ok_or_else(|| anyhow!("Gasto no encontrado"))
    }
}