use std::sync::Arc;
use crate::application::repositories::documento_repository::IDocumentoRepository;
use crate::application::services::storage_service::IStorageService;
use crate::domain::entities::Documento;
use crate::application::dto::CreateDocumentoDto;
use anyhow::{Result, anyhow};

pub struct DocumentoService {
    repository: Arc<dyn IDocumentoRepository>,
    storage_service: Arc<dyn IStorageService>,
    bucket: String,
}

impl DocumentoService {
    pub fn new(repository: Arc<dyn IDocumentoRepository>, storage_service: Arc<dyn IStorageService>, bucket: String) -> Self {
        Self { repository, storage_service, bucket }
    }

    pub async fn crear_documento(&self, usuario_id: i64, dto: CreateDocumentoDto, file_data: Vec<u8>, file_name: &str) -> Result<Documento> {
        let extension = file_name.split('.').last().unwrap_or("bin");
        let clean_name = slug::slugify(&dto.nombre);
        let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S");
        let storage_name = format!("documentos/{}-{}.{}", clean_name, timestamp, extension);
        let file_url = self.storage_service.upload_file(file_data, &storage_name, &self.bucket).await?;
        self.repository.create(usuario_id, dto, &file_url, file_name).await
    }

    pub async fn listar_documentos(&self, usuario_id: i64, page: u32, page_size: u32) -> Result<(Vec<Documento>, i64)> {
        self.repository.list_all(usuario_id, page, page_size).await
    }

    pub async fn eliminar_documento(&self, usuario_id: i64, id: i32) -> Result<Documento> {
        let doc = self.repository.find_by_id(usuario_id, id).await?
            .ok_or_else(|| anyhow!("Documento no encontrado"))?;

        if let Err(e) = self.storage_service.delete_file(&doc.archivo_url).await {
            println!("⚠️ Error eliminando archivo: {}", e);
        }

        self.repository.delete(usuario_id, id).await?
            .ok_or_else(|| anyhow!("Error eliminando documento"))
    }
}
