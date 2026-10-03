use std::sync::Arc;
use crate::application::repositories::proyecto_repository::IProyectoRepository;
use crate::domain::entities::{Proyecto, PagoExistente};
use crate::application::dto::proyecto_dto::{ProyectoSummaryDto, CreateProyectoDto, UpdateProyectoDto};
use anyhow::{Result, anyhow};

pub struct ProyectoService {
    repository: Arc<dyn IProyectoRepository>,
}

impl ProyectoService {
    pub fn new(repository: Arc<dyn IProyectoRepository>) -> Self {
        Self { repository }
    }

    pub async fn list_all_proyectos(&self, usuario_id: i64) -> Result<(Vec<Proyecto>, ProyectoSummaryDto)> {
        let proyectos = self.repository.list_all(usuario_id).await?;
        let summary = self.repository.get_summary(usuario_id).await?;
        Ok((proyectos, summary))
    }

    pub async fn get_proyecto_with_pagos(&self, usuario_id: i64, proyecto_id: i32, page: u32, page_size: u32) -> Result<(Proyecto, Vec<PagoExistente>, i64)> {
        let proyecto = self.repository.find_by_id(usuario_id, proyecto_id).await?
            .ok_or_else(|| anyhow!("Proyecto no encontrado"))?;
        let (pagos, total_pagos) = self.repository.get_pagos_by_proyecto(usuario_id, proyecto_id, page, page_size).await?;
        Ok((proyecto, pagos, total_pagos))
    }

    pub async fn get_proyecto_by_id(&self, usuario_id: i64, proyecto_id: i32) -> Result<Proyecto> {
        self.repository.find_by_id(usuario_id, proyecto_id).await?
            .ok_or_else(|| anyhow!("Proyecto no encontrado"))
    }

    pub async fn cambiar_estado_proyecto(&self, usuario_id: i64, proyecto_id: i32, nuevo_estado: &str) -> Result<()> {
        self.repository.cambiar_estado(usuario_id, proyecto_id, nuevo_estado).await?;
        Ok(())
    }

    pub async fn crear_proyecto(&self, usuario_id: i64, dto: CreateProyectoDto) -> Result<Proyecto> {
        self.repository.create(usuario_id, dto).await
    }

    pub async fn actualizar_proyecto(&self, usuario_id: i64, id: i32, dto: UpdateProyectoDto) -> Result<Proyecto> {
        self.repository.update(usuario_id, id, dto).await?
            .ok_or_else(|| anyhow!("Proyecto no encontrado"))
    }
}
