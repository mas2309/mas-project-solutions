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

    pub async fn crear_credito(&self, usuario_id: i64, dto: CreateCreditoDto) -> Result<Credito> {
        self.repository.create(usuario_id, dto).await
    }

    pub async fn listar_creditos(&self, usuario_id: i64, page: u32, page_size: u32) -> Result<(Vec<Credito>, i64)> {
        self.repository.list_all(usuario_id, page, page_size).await
    }

    pub async fn obtener_deuda_total(&self, usuario_id: i64) -> Result<Decimal> {
        self.repository.get_deuda_total(usuario_id).await
    }

    pub async fn obtener_credito(&self, usuario_id: i64, id: i32) -> Result<Credito> {
        self.repository.find_by_id(usuario_id, id).await?
            .ok_or_else(|| anyhow!("Crédito no encontrado"))
    }

    pub async fn registrar_cuota(&self, usuario_id: i64, id: i32) -> Result<Credito> {
        self.repository.registrar_cuota(usuario_id, id).await?
            .ok_or_else(|| anyhow!("Crédito no encontrado"))
    }

    pub async fn eliminar_credito(&self, usuario_id: i64, id: i32) -> Result<Credito> {
        self.repository.delete(usuario_id, id).await?
            .ok_or_else(|| anyhow!("Crédito no encontrado"))
    }

    pub async fn editar_credito(&self, usuario_id: i64, id: i32, dto: CreateCreditoDto) -> Result<Credito> {
        self.repository.update(usuario_id, id, dto).await?
            .ok_or_else(|| anyhow!("Crédito no encontrado"))
    }

    /// Calcula la cuota mensual fija usando la fórmula de amortización francesa
    pub fn calcular_cuota_fija(monto: Decimal, tasa_anual: Decimal, cuotas: i32) -> Decimal {
        if tasa_anual == Decimal::ZERO || cuotas == 0 {
            if cuotas > 0 {
                return monto / Decimal::from(cuotas);
            }
            return Decimal::ZERO;
        }

        let tasa_mensual = tasa_anual.to_f64().unwrap_or(0.0) / 12.0 / 100.0;
        let n = cuotas as f64;

        let factor = (1.0 + tasa_mensual).powf(n);
        let cuota = monto.to_f64().unwrap_or(0.0) * (tasa_mensual * factor) / (factor - 1.0);

        Decimal::from_f64_retain(cuota)
            .unwrap_or(Decimal::ZERO)
            .round_dp(2)
    }
}
