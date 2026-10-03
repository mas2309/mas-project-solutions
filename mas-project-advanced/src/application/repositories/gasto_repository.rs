use async_trait::async_trait;
use crate::domain::entities::Gasto;
use crate::application::dto::CreateGastoDto;
use anyhow::Result;
use rust_decimal::Decimal;

#[async_trait]
pub trait IGastoRepository: Send + Sync {
    async fn create(&self, usuario_id: i64, dto: CreateGastoDto) -> Result<Gasto>;
    async fn create_from_recurrente(&self, usuario_id: i64, dto: CreateGastoDto, gasto_recurrente_id: i32) -> Result<Gasto>;
    async fn find_by_id(&self, usuario_id: i64, id: i32) -> Result<Option<Gasto>>;
    async fn list_by_month(&self, usuario_id: i64, anio: &str, mes: &str) -> Result<Vec<Gasto>>;
    async fn list_all(&self, usuario_id: i64, page: u32, page_size: u32) -> Result<(Vec<Gasto>, i64)>;
    async fn marcar_pagado(&self, usuario_id: i64, id: i32) -> Result<Option<Gasto>>;
    async fn delete(&self, usuario_id: i64, id: i32) -> Result<Option<Gasto>>;
    async fn actualizar_soporte(&self, usuario_id: i64, id: i32, url: &str) -> Result<Option<Gasto>>;
    async fn update(&self, usuario_id: i64, id: i32, dto: CreateGastoDto) -> Result<Option<Gasto>>;
    async fn get_total_monto(&self, usuario_id: i64) -> Result<Decimal>;

    // Cuotas de créditos
    async fn create_from_credito(&self, usuario_id: i64, dto: CreateGastoDto, credito_id: i32, estado: &str) -> Result<Gasto>;
    async fn creditos_con_cuota_en_mes(&self, usuario_id: i64, anio: i32, mes: u32) -> Result<Vec<i32>>;
    async fn contar_pendientes_por_credito(&self, usuario_id: i64, credito_id: i32) -> Result<i64>;
    async fn find_pendiente_por_credito(&self, usuario_id: i64, credito_id: i32) -> Result<Option<Gasto>>;
    async fn eliminar_pendientes_por_credito(&self, usuario_id: i64, credito_id: i32) -> Result<u64>;
}
