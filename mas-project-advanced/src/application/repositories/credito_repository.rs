use async_trait::async_trait;
use crate::domain::entities::Credito;
use crate::application::dto::CreateCreditoDto;
use anyhow::Result;

#[async_trait]
pub trait ICreditoRepository: Send + Sync {
    async fn create(&self, usuario_id: i64, dto: CreateCreditoDto) -> Result<Credito>;
    async fn find_by_id(&self, usuario_id: i64, id: i32) -> Result<Option<Credito>>;
    async fn list_all(&self, usuario_id: i64, page: u32, page_size: u32) -> Result<(Vec<Credito>, i64)>;
    async fn registrar_cuota(&self, usuario_id: i64, id: i32) -> Result<Option<Credito>>;
    async fn delete(&self, usuario_id: i64, id: i32) -> Result<Option<Credito>>;
    async fn update(&self, usuario_id: i64, id: i32, dto: CreateCreditoDto) -> Result<Option<Credito>>;
}
