use async_trait::async_trait;
use crate::domain::entities::Gasto;
use crate::application::dto::CreateGastoDto;
use anyhow::Result;

#[async_trait]
pub trait IGastoRepository: Send + Sync {
    async fn create(&self, dto: CreateGastoDto) -> Result<Gasto>;
    async fn create_from_recurrente(&self, dto: CreateGastoDto, gasto_recurrente_id: i32) -> Result<Gasto>;
    async fn find_by_id(&self, id: i32) -> Result<Option<Gasto>>;
    async fn list_by_month(&self, anio: &str, mes: &str) -> Result<Vec<Gasto>>;
    async fn list_all(&self, page: u32, page_size: u32) -> Result<(Vec<Gasto>, i64)>;
    async fn marcar_pagado(&self, id: i32) -> Result<Option<Gasto>>;
    async fn delete(&self, id: i32) -> Result<Option<Gasto>>;
    async fn actualizar_soporte(&self, id: i32, url: &str) -> Result<Option<Gasto>>;
    async fn update(&self, id: i32, dto: CreateGastoDto) -> Result<Option<Gasto>>;
}