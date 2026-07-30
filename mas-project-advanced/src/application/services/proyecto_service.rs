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

    pub async fn list_all_proyectos(&self) -> Result<(Vec<Proyecto>, ProyectoSummaryDto)> {
        let proyectos = self.repository.list_all().await?;
        let summary = self.repository.get_summary().await?;
        Ok((proyectos, summary))
    }

    pub async fn get_proyecto_with_pagos(&self, proyecto_id: i32, page: u32, page_size: u32) -> Result<(Proyecto, Vec<PagoExistente>, i64)> {
        let proyecto = self.repository.find_by_id(proyecto_id).await?
            .ok_or_else(|| anyhow!("Proyecto no encontrado"))?;
        let (pagos, total_pagos) = self.repository.get_pagos_by_proyecto(proyecto_id, page, page_size).await?;
        Ok((proyecto, pagos, total_pagos))
    }

    pub async fn get_proyecto_by_id(&self, proyecto_id: i32) -> Result<Proyecto> {
        self.repository.find_by_id(proyecto_id).await?
            .ok_or_else(|| anyhow!("Proyecto no encontrado"))
    }

    pub async fn cambiar_estado_proyecto(&self, proyecto_id: i32, nuevo_estado: &str) -> Result<()> {
        self.repository.cambiar_estado(proyecto_id, nuevo_estado).await?;
        Ok(())
    }

    pub async fn crear_proyecto(&self, dto: CreateProyectoDto) -> Result<Proyecto> {
        self.repository.create(dto).await
    }

    pub async fn actualizar_proyecto(&self, id: i32, dto: UpdateProyectoDto) -> Result<Proyecto> {
        self.repository.update(id, dto).await?
            .ok_or_else(|| anyhow!("Proyecto no encontrado"))
    }
}
