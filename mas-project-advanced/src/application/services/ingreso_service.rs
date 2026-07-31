use std::sync::Arc;
use crate::application::repositories::ingreso_repository::IIngresoRepository;
use crate::domain::entities::Ingreso;
use crate::application::dto::CreateIngresoDto;
use anyhow::{Result, anyhow};

pub struct IngresoService {
    repository: Arc<dyn IIngresoRepository>,
}

impl IngresoService {
    pub fn new(repository: Arc<dyn IIngresoRepository>) -> Self {
        Self { repository }
    }

    pub async fn crear_ingreso(&self, usuario_id: i64, dto: CreateIngresoDto) -> Result<Ingreso> {
        self.repository.create(usuario_id, dto).await
    }

    pub async fn listar_ingresos(&self, usuario_id: i64, page: u32, page_size: u32) -> Result<(Vec<Ingreso>, i64)> {
        self.repository.list_all(usuario_id, page, page_size).await
    }

    pub async fn eliminar_ingreso(&self, usuario_id: i64, id: i32) -> Result<Ingreso> {
        self.repository.delete(usuario_id, id).await?
            .ok_or_else(|| anyhow!("Ingreso no encontrado"))
    }

    pub async fn obtener_ingreso(&self, usuario_id: i64, id: i32) -> Result<Ingreso> {
        self.repository.find_by_id(usuario_id, id).await?
            .ok_or_else(|| anyhow!("Ingreso no encontrado"))
    }

    pub async fn editar_ingreso(&self, usuario_id: i64, id: i32, dto: CreateIngresoDto) -> Result<Ingreso> {
        self.repository.update(usuario_id, id, dto).await?
            .ok_or_else(|| anyhow!("Ingreso no encontrado"))
    }
}
