use std::sync::Arc;
use crate::application::repositories::pago_repository::IPagoRepository;
use crate::application::repositories::proyecto_repository::IProyectoRepository;
use crate::application::services::storage_service::IStorageService;
use crate::domain::entities::PagoExistente;
use crate::application::dto::{CreatePagoDto, PagosSummaryDto};
use anyhow::{Result, anyhow};
use rust_decimal::Decimal;

pub struct PagoService {
    repository: Arc<dyn IPagoRepository>,
    proyecto_repository: Arc<dyn IProyectoRepository>,
    storage_service: Arc<dyn IStorageService>,
    bucket: String,
}

impl PagoService {
    pub fn new(
        repository: Arc<dyn IPagoRepository>,
        proyecto_repository: Arc<dyn IProyectoRepository>,
        storage_service: Arc<dyn IStorageService>,
        bucket: String,
    ) -> Self {
        Self { repository, proyecto_repository, storage_service, bucket }
    }

    async fn validar_presupuesto(&self, proyecto_id: i32, monto_nuevo: Decimal) -> Result<()> {
        let proyecto = self.proyecto_repository.find_by_id(proyecto_id).await?
            .ok_or_else(|| anyhow!("Proyecto no encontrado"))?;
        
        let presupuesto = proyecto.presupuesto.unwrap_or(Decimal::ZERO);
        if presupuesto == Decimal::ZERO {
            return Ok(()); // Sin presupuesto definido, no validar
        }

        // Obtener total de pagos existentes del proyecto
        let (pagos, _) = self.proyecto_repository.get_pagos_by_proyecto(proyecto_id, 1, 10000).await?;
        let total_pagos_existentes: Decimal = pagos.iter().map(|p| p.valor).sum();
        
        let total_con_nuevo = total_pagos_existentes + monto_nuevo;
        if total_con_nuevo > presupuesto {
            let disponible = presupuesto - total_pagos_existentes;
            return Err(anyhow!(
                "El pago excede el presupuesto del proyecto. Presupuesto: ${}, Total pagos: ${}, Disponible: ${}",
                presupuesto, total_pagos_existentes, disponible
            ));
        }
        
        Ok(())
    }

    pub async fn crear_pago(&self, dto: CreatePagoDto) -> Result<PagoExistente> {
        // Validar que el pago no exceda el presupuesto del proyecto
        if let Some(proyecto_id) = dto.proyecto_id {
            self.validar_presupuesto(proyecto_id, dto.valor).await?;
        }
        self.repository.create(dto).await
    }

    pub async fn obtener_pago(&self, id: i32) -> Result<PagoExistente> {
        self.repository.find_by_id(id).await?
            .ok_or_else(|| anyhow!("Pago no encontrado"))
    }

    pub async fn registrar_pago(&self, id: i32, monto: Decimal) -> Result<PagoExistente> {
        if monto <= Decimal::ZERO {
            return Err(anyhow!("El monto debe ser mayor a cero"));
        }

        self.repository.registrar_pago(id, monto).await?
            .ok_or_else(|| anyhow!("Pago no encontrado"))
    }

    pub async fn obtener_resumen(&self, anio: &str) -> Result<PagosSummaryDto> {
        self.repository.get_summary(anio).await
    }

    pub async fn subir_evidencia(&self, pago_id: i32, file_data: Vec<u8>, file_name: &str, tipo: &str) -> Result<PagoExistente> {
        let extension = file_name.split('.').last().unwrap_or("bin");
        let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S");
        let prefix = if tipo == "cliente" { "evidencia-pagos" } else { "evidencia-constructora" };
        let storage_name = format!("{}/pago-{}-{}.{}", prefix, pago_id, timestamp, extension);
        
        let file_url = self.storage_service.upload_file(file_data, &storage_name, &self.bucket).await?;
        
        self.repository.actualizar_evidencia(pago_id, &file_url, tipo).await?
            .ok_or_else(|| anyhow!("Pago no encontrado"))
    }

    pub async fn marcar_pagado(&self, id: i32) -> Result<PagoExistente> {
        self.repository.marcar_pagado(id).await?
            .ok_or_else(|| anyhow!("Pago no encontrado"))
    }

    pub async fn eliminar_pago(&self, id: i32) -> Result<PagoExistente> {
        let pago = self.repository.find_by_id(id).await?
            .ok_or_else(|| anyhow!("Pago no encontrado"))?;
        
        // Eliminar archivos del bucket si existen
        if let Some(ref evidencia) = pago.evidencia {
            if let Err(e) = self.storage_service.delete_file(evidencia).await {
                println!("⚠️ Error eliminando evidencia cliente: {}", e);
            }
        }
        if let Some(ref evidencia) = pago.evidencia_constructora {
            if let Err(e) = self.storage_service.delete_file(evidencia).await {
                println!("⚠️ Error eliminando evidencia constructora: {}", e);
            }
        }
        
        self.repository.delete(id).await?
            .ok_or_else(|| anyhow!("Error eliminando pago"))
    }

    pub async fn editar_pago(&self, id: i32, descripcion: &str, valor: Decimal, mes: &str, anio: &str) -> Result<PagoExistente> {
        self.repository.update(id, descripcion, valor, mes, anio).await?
            .ok_or_else(|| anyhow!("Pago no encontrado"))
    }

    pub async fn editar_pago_con_evidencias(
        &self, id: i32, descripcion: &str, valor: Decimal, mes: &str, anio: &str,
        evidencia_cliente: Option<(Vec<u8>, String)>,
        evidencia_constructora: Option<(Vec<u8>, String)>,
    ) -> Result<PagoExistente> {
        // Actualizar datos básicos
        let pago = self.repository.update(id, descripcion, valor, mes, anio).await?
            .ok_or_else(|| anyhow!("Pago no encontrado"))?;
        
        // Subir evidencia cliente si se proporcionó
        let pago = if let Some((data, name)) = evidencia_cliente {
            self.subir_evidencia(id, data, &name, "cliente").await?
        } else {
            pago
        };
        
        // Subir evidencia constructora si se proporcionó
        let pago = if let Some((data, name)) = evidencia_constructora {
            self.subir_evidencia(id, data, &name, "constructora").await?
        } else {
            pago
        };
        
        Ok(pago)
    }
}
