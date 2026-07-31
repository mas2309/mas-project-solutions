use async_trait::async_trait;
use crate::domain::entities::Documento;
use crate::application::dto::CreateDocumentoDto;
use anyhow::Result;

#[async_trait]
pub trait IDocumentoRepository: Send + Sync {
    async fn create(&self, usuario_id: i64, dto: CreateDocumentoDto, archivo_url: &str, nombre_archivo: &str) -> Result<Documento>;
    async fn find_by_id(&self, usuario_id: i64, id: i32) -> Result<Option<Documento>>;
    async fn list_all(&self, usuario_id: i64, page: u32, page_size: u32) -> Result<(Vec<Documento>, i64)>;
    async fn delete(&self, usuario_id: i64, id: i32) -> Result<Option<Documento>>;
}
