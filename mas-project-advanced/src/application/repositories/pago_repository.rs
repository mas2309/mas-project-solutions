use async_trait::async_trait;
use crate::domain::entities::PagoExistente;
use crate::application::dto::{CreatePagoDto, PagosSummaryDto};
use anyhow::Result;
use rust_decimal::Decimal;

#[async_trait]
pub trait IPagoRepository: Send + Sync {
    async fn create(&self, dto: CreatePagoDto) -> Result<PagoExistente>;
    async fn find_by_id(&self, id: i32) -> Result<Option<PagoExistente>>;
    async fn registrar_pago(&self, id: i32, monto: Decimal) -> Result<Option<PagoExistente>>;
    async fn get_summary(&self, anio: &str) -> Result<PagosSummaryDto>;
    async fn actualizar_evidencia(&self, id: i32, url: &str, tipo: &str) -> Result<Option<PagoExistente>>;
    async fn marcar_pagado(&self, id: i32) -> Result<Option<PagoExistente>>;
    async fn delete(&self, id: i32) -> Result<Option<PagoExistente>>;
    async fn update(&self, id: i32, descripcion: &str, valor: Decimal, mes: &str, anio: &str) -> Result<Option<PagoExistente>>;
}
