use async_trait::async_trait;
use crate::domain::entities::Credito;
use crate::application::dto::CreateCreditoDto;
use anyhow::Result;

#[async_trait]
pub trait ICreditoRepository: Send + Sync {
    async fn create(&self, dto: CreateCreditoDto) -> Result<Credito>;
    async fn find_by_id(&self, id: i32) -> Result<Option<Credito>>;
    async fn list_all(&self, page: u32, page_size: u32) -> Result<(Vec<Credito>, i64)>;
    async fn registrar_cuota(&self, id: i32) -> Result<Option<Credito>>;
    async fn delete(&self, id: i32) -> Result<Option<Credito>>;
    async fn update(&self, id: i32, dto: CreateCreditoDto) -> Result<Option<Credito>>;
}