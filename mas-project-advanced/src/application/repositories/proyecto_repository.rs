use async_trait::async_trait;
use crate::domain::entities::{Proyecto, PagoExistente};
use crate::application::dto::proyecto_dto::{ProyectoSummaryDto, CreateProyectoDto, UpdateProyectoDto};
use anyhow::Result;

#[async_trait]
pub trait IProyectoRepository: Send + Sync {
    async fn list_all(&self, usuario_id: i64) -> Result<Vec<Proyecto>>;
    async fn get_summary(&self, usuario_id: i64) -> Result<ProyectoSummaryDto>;
    async fn find_by_id(&self, usuario_id: i64, id: i32) -> Result<Option<Proyecto>>;
    async fn create(&self, usuario_id: i64, dto: CreateProyectoDto) -> Result<Proyecto>;
    async fn update(&self, usuario_id: i64, id: i32, dto: UpdateProyectoDto) -> Result<Option<Proyecto>>;
    async fn get_pagos_by_proyecto(&self, usuario_id: i64, proyecto_id: i32, page: u32, page_size: u32) -> Result<(Vec<PagoExistente>, i64)>;
    async fn cambiar_estado(&self, usuario_id: i64, id: i32, nuevo_estado: &str) -> Result<Option<Proyecto>>;
}
