use async_trait::async_trait;
use crate::domain::entities::GastoRecurrente;
use crate::application::dto::CreateGastoRecurrenteDto;
use anyhow::Result;

#[async_trait]
pub trait IGastoRecurrenteRepository: Send + Sync {
    async fn create(&self, dto: CreateGastoRecurrenteDto) -> Result<GastoRecurrente>;
    async fn find_by_id(&self, id: i32) -> Result<Option<GastoRecurrente>>;
    async fn list_activos(&self) -> Result<Vec<GastoRecurrente>>;
    async fn list_all(&self) -> Result<Vec<GastoRecurrente>>;
    async fn update(&self, id: i32, dto: CreateGastoRecurrenteDto) -> Result<Option<GastoRecurrente>>;
    async fn toggle_activo(&self, id: i32) -> Result<Option<GastoRecurrente>>;
    async fn delete(&self, id: i32) -> Result<Option<GastoRecurrente>>;
    async fn ya_generados_en_mes(&self, anio: i32, mes: u32) -> Result<Vec<i32>>;
}
