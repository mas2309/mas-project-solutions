use std::sync::Arc;
use crate::application::repositories::credito_repository::ICreditoRepository;
use crate::domain::entities::Credito;
use crate::application::dto::CreateCreditoDto;
use anyhow::{Result, anyhow};
use rust_decimal::Decimal;
use rust_decimal::prelude::*;

pub struct CreditoService {
    repository: Arc<dyn ICreditoRepository>,
}

impl CreditoService {
    pub fn new(repository: Arc<dyn ICreditoRepository>) -> Self {
        Self { repository }
    }

    pub async fn crear_credito(&self, dto: CreateCreditoDto) -> Result<Credito> {
        self.repository.create(dto).await
    }

    pub async fn listar_creditos(&self, page: u32, page_size: u32) -> Result<(Vec<Credito>, i64)> {
        self.repository.list_all(page, page_size).await
    }

    pub async fn obtener_credito(&self, id: i32) -> Result<Credito> {
        self.repository.find_by_id(id).await?
            .ok_or_else(|| anyhow!("Crédito no encontrado"))
    }

    pub async fn registrar_cuota(&self, id: i32) -> Result<Credito> {
        self.repository.registrar_cuota(id).await?
            .ok_or_else(|| anyhow!("Crédito no encontrado"))
    }

    pub async fn eliminar_credito(&self, id: i32) -> Result<Credito> {
        self.repository.delete(id).await?
            .ok_or_else(|| anyhow!("Crédito no encontrado"))
    }

    pub async fn editar_credito(&self, id: i32, dto: CreateCreditoDto) -> Result<Credito> {
        self.repository.update(id, dto).await?
            .ok_or_else(|| anyhow!("Crédito no encontrado"))
    }

    /// Calcula la cuota mensual fija usando la fórmula de amortización francesa
    /// tasa_anual: tasa de interés anual en porcentaje (ej: 12.5 = 12.5%)
    /// monto: monto total del crédito
    /// cuotas: número total de cuotas mensuales
    pub fn calcular_cuota_fija(monto: Decimal, tasa_anual: Decimal, cuotas: i32) -> Decimal {
        // Si la tasa es 0, cuota = monto / cuotas
        if tasa_anual == Decimal::ZERO || cuotas == 0 {
            if cuotas > 0 {
                return monto / Decimal::from(cuotas);
            }
            return Decimal::ZERO;
        }

        // Convertir tasa anual a mensual: tasa_mensual = tasa_anual / 12 / 100
        let tasa_mensual = tasa_anual.to_f64().unwrap_or(0.0) / 12.0 / 100.0;
        let n = cuotas as f64;

        // Fórmula: cuota = monto * (i * (1+i)^n) / ((1+i)^n - 1)
        let factor = (1.0 + tasa_mensual).powf(n);
        let cuota = monto.to_f64().unwrap_or(0.0) * (tasa_mensual * factor) / (factor - 1.0);

        // Redondear a 2 decimales
        Decimal::from_f64_retain(cuota)
            .unwrap_or(Decimal::ZERO)
            .round_dp(2)
    }
}