use async_trait::async_trait;
use crate::domain::entities::Ingreso;
use crate::application::dto::CreateIngresoDto;
use anyhow::Result;
use rust_decimal::Decimal;

#[async_trait]
pub trait IIngresoRepository: Send + Sync {
    async fn create(&self, usuario_id: i64, dto: CreateIngresoDto) -> Result<Ingreso>;
    async fn find_by_id(&self, usuario_id: i64, id: i32) -> Result<Option<Ingreso>>;
    async fn list_by_month(&self, usuario_id: i64, anio: &str, mes: &str) -> Result<Vec<Ingreso>>;
    async fn list_all(&self, usuario_id: i64, page: u32, page_size: u32) -> Result<(Vec<Ingreso>, i64)>;
    async fn delete(&self, usuario_id: i64, id: i32) -> Result<Option<Ingreso>>;
    async fn update(&self, usuario_id: i64, id: i32, dto: CreateIngresoDto) -> Result<Option<Ingreso>>;
    async fn get_total_monto(&self, usuario_id: i64) -> Result<Decimal>;
}
