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

    async fn validar_presupuesto(&self, usuario_id: i64, proyecto_id: i32, monto_nuevo: Decimal) -> Result<()> {
        let proyecto = self.proyecto_repository.find_by_id(usuario_id, proyecto_id).await?
            .ok_or_else(|| anyhow!("Proyecto no encontrado"))?;
        
        let presupuesto = proyecto.presupuesto.unwrap_or(Decimal::ZERO);
        if presupuesto == Decimal::ZERO {
            return Ok(());
        }

        let (pagos, _) = self.proyecto_repository.get_pagos_by_proyecto(usuario_id, proyecto_id, 1, 10000).await?;
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

    pub async fn crear_pago(&self, usuario_id: i64, dto: CreatePagoDto) -> Result<PagoExistente> {
        if let Some(proyecto_id) = dto.proyecto_id {
            self.validar_presupuesto(usuario_id, proyecto_id, dto.valor).await?;
        }
        self.repository.create(usuario_id, dto).await
    }

    pub async fn obtener_pago(&self, usuario_id: i64, id: i32) -> Result<PagoExistente> {
        self.repository.find_by_id(usuario_id, id).await?
            .ok_or_else(|| anyhow!("Pago no encontrado"))
    }

    pub async fn registrar_pago(&self, usuario_id: i64, id: i32, monto: Decimal) -> Result<PagoExistente> {
        if monto <= Decimal::ZERO {
            return Err(anyhow!("El monto debe ser mayor a cero"));
        }

        self.repository.registrar_pago(usuario_id, id, monto).await?
            .ok_or_else(|| anyhow!("Pago no encontrado"))
    }

    pub async fn obtener_resumen(&self, usuario_id: i64, anio: &str) -> Result<PagosSummaryDto> {
        self.repository.get_summary(usuario_id, anio).await
    }

    pub async fn subir_evidencia(&self, usuario_id: i64, pago_id: i32, file_data: Vec<u8>, file_name: &str, tipo: &str) -> Result<PagoExistente> {
        let extension = file_name.split('.').last().unwrap_or("bin");
        let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S");
        let prefix = if tipo == "cliente" { "evidencia-pagos" } else { "evidencia-constructora" };
        let storage_name = format!("{}/pago-{}-{}.{}", prefix, pago_id, timestamp, extension);
        
        let file_url = self.storage_service.upload_file(file_data, &storage_name, &self.bucket).await?;
        
        self.repository.actualizar_evidencia(usuario_id, pago_id, &file_url, tipo).await?
            .ok_or_else(|| anyhow!("Pago no encontrado"))
    }

    pub async fn marcar_pagado(&self, usuario_id: i64, id: i32) -> Result<PagoExistente> {
        self.repository.marcar_pagado(usuario_id, id).await?
            .ok_or_else(|| anyhow!("Pago no encontrado"))
    }

    pub async fn eliminar_pago(&self, usuario_id: i64, id: i32) -> Result<PagoExistente> {
        let pago = self.repository.find_by_id(usuario_id, id).await?
            .ok_or_else(|| anyhow!("Pago no encontrado"))?;
        
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
        
        self.repository.delete(usuario_id, id).await?
            .ok_or_else(|| anyhow!("Error eliminando pago"))
    }

    pub async fn editar_pago(&self, usuario_id: i64, id: i32, descripcion: &str, valor: Decimal, mes: &str, anio: &str) -> Result<PagoExistente> {
        self.repository.update(usuario_id, id, descripcion, valor, mes, anio).await?
            .ok_or_else(|| anyhow!("Pago no encontrado"))
    }

    pub async fn editar_pago_con_evidencias(
        &self, usuario_id: i64, id: i32, descripcion: &str, valor: Decimal, mes: &str, anio: &str,
        evidencia_cliente: Option<(Vec<u8>, String)>,
        evidencia_constructora: Option<(Vec<u8>, String)>,
    ) -> Result<PagoExistente> {
        let pago = self.repository.update(usuario_id, id, descripcion, valor, mes, anio).await?
            .ok_or_else(|| anyhow!("Pago no encontrado"))?;
        
        let pago = if let Some((data, name)) = evidencia_cliente {
            self.subir_evidencia(usuario_id, id, data, &name, "cliente").await?
        } else {
            pago
        };
        
        let pago = if let Some((data, name)) = evidencia_constructora {
            self.subir_evidencia(usuario_id, id, data, &name, "constructora").await?
        } else {
            pago
        };
        
        Ok(pago)
    }
}
