use std::sync::Arc;
use crate::application::repositories::credito_repository::ICreditoRepository;
use crate::application::repositories::gasto_repository::IGastoRepository;
use crate::domain::entities::{Credito, EstadoCredito, Gasto};
use crate::application::dto::{CreateCreditoDto, CreateGastoDto};
use anyhow::{Result, anyhow};
use chrono::{Datelike, NaiveDate};
use rust_decimal::Decimal;
use rust_decimal::prelude::*;

pub struct CreditoService {
    repository: Arc<dyn ICreditoRepository>,
    gasto_repository: Arc<dyn IGastoRepository>,
}

impl CreditoService {
    pub fn new(repository: Arc<dyn ICreditoRepository>, gasto_repository: Arc<dyn IGastoRepository>) -> Self {
        Self { repository, gasto_repository }
    }

    /// Si no se indica el día de pago se toma el día de la fecha de inicio
    fn normalizar_dia_pago(mut dto: CreateCreditoDto) -> Result<CreateCreditoDto> {
        let dia = match dto.dia_pago {
            Some(dia) => dia,
            None => NaiveDate::parse_from_str(&dto.fecha_inicio, "%Y-%m-%d")
                .map(|f| f.day() as i32)
                .unwrap_or(1),
        };
        if !(1..=31).contains(&dia) {
            return Err(anyhow!("El día de pago debe estar entre 1 y 31"));
        }
        dto.dia_pago = Some(dia);
        Ok(dto)
    }

    fn descripcion_cuota(credito: &Credito, numero: i32) -> String {
        format!("Cuota {}/{} - {} - {}", numero, credito.cuotas_totales, credito.entidad, credito.descripcion)
    }

    /// Último día del mes (para créditos con día de pago 29, 30 o 31)
    fn ultimo_dia_mes(anio: i32, mes: u32) -> u32 {
        let (sig_anio, sig_mes) = if mes == 12 { (anio + 1, 1) } else { (anio, mes + 1) };
        NaiveDate::from_ymd_opt(sig_anio, sig_mes, 1)
            .and_then(|d| d.pred_opt())
            .map(|d| d.day())
            .unwrap_or(28)
    }

    pub async fn crear_credito(&self, usuario_id: i64, dto: CreateCreditoDto) -> Result<Credito> {
        let dto = Self::normalizar_dia_pago(dto)?;
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

    /// Registra el pago de una cuota desde el módulo de créditos.
    /// Marca como pagado el gasto pendiente más antiguo de la cuota; si no existe, lo crea ya pagado.
    pub async fn registrar_cuota(&self, usuario_id: i64, id: i32) -> Result<Credito> {
        let credito = self.obtener_credito(usuario_id, id).await?;
        if credito.estado == EstadoCredito::Pagado {
            return Err(anyhow!("El crédito ya está pagado"));
        }

        match self.gasto_repository.find_pendiente_por_credito(usuario_id, id).await? {
            Some(gasto) => {
                self.gasto_repository.marcar_pagado(usuario_id, gasto.id).await?;
            }
            None => {
                let gasto_dto = CreateGastoDto {
                    descripcion: Self::descripcion_cuota(&credito, credito.cuotas_pagadas + 1),
                    monto: credito.valor_cuota,
                    categoria: "Credito".to_string(),
                    responsable: None,
                    fecha: chrono::Local::now().format("%Y-%m-%d").to_string(),
                };
                self.gasto_repository.create_from_credito(usuario_id, gasto_dto, id, "Pagado").await?;
            }
        }

        self.repository.registrar_cuota(usuario_id, id).await?
            .ok_or_else(|| anyhow!("Crédito no encontrado"))
    }

    /// Genera como gasto Pendiente la cuota del mes actual de cada crédito activo
    /// cuyo día de pago ya llegó. No genera meses anteriores ni duplica cuotas.
    /// Se llama automáticamente al entrar a la sección de gastos.
    pub async fn auto_generar_cuotas(&self, usuario_id: i64) -> Result<Vec<Gasto>> {
        let hoy = chrono::Local::now().date_naive();
        let (anio, mes) = (hoy.year(), hoy.month());
        let ultimo_dia = Self::ultimo_dia_mes(anio, mes);

        let creditos = self.repository.list_activos(usuario_id).await?;
        let ya_generados = self.gasto_repository.creditos_con_cuota_en_mes(usuario_id, anio, mes).await?;

        let mut gastos_generados = Vec::new();

        for credito in creditos {
            if ya_generados.contains(&credito.id) {
                continue;
            }

            let dia = (credito.dia_pago.max(1) as u32).min(ultimo_dia);
            let fecha_pago = match NaiveDate::from_ymd_opt(anio, mes, dia) {
                Some(f) => f,
                None => continue,
            };
            if fecha_pago > hoy {
                continue;
            }

            // La primera cuota se paga después del inicio del crédito
            let inicio_posterior = NaiveDate::parse_from_str(&credito.fecha_inicio, "%Y-%m-%d")
                .map(|inicio| fecha_pago <= inicio)
                .unwrap_or(false);
            if inicio_posterior {
                continue;
            }

            let pendientes = self.gasto_repository.contar_pendientes_por_credito(usuario_id, credito.id).await?;
            let numero = credito.cuotas_pagadas + pendientes as i32 + 1;
            if numero > credito.cuotas_totales {
                continue;
            }

            let gasto_dto = CreateGastoDto {
                descripcion: Self::descripcion_cuota(&credito, numero),
                monto: credito.valor_cuota,
                categoria: "Credito".to_string(),
                responsable: None,
                fecha: fecha_pago.format("%Y-%m-%d").to_string(),
            };

            let gasto = self.gasto_repository.create_from_credito(usuario_id, gasto_dto, credito.id, "Pendiente").await?;
            gastos_generados.push(gasto);
        }

        if !gastos_generados.is_empty() {
            println!("📌 Auto-generadas {} cuotas de créditos del mes", gastos_generados.len());
        }

        Ok(gastos_generados)
    }

    /// Da por finalizado (cancelado) un crédito: queda en estado Pagado y sin saldo pendiente.
    /// Las cuotas pendientes generadas en gastos se eliminan.
    pub async fn finalizar_credito(&self, usuario_id: i64, id: i32) -> Result<Credito> {
        let credito = self.obtener_credito(usuario_id, id).await?;
        if credito.estado == EstadoCredito::Pagado {
            return Err(anyhow!("El crédito ya está finalizado"));
        }
        self.gasto_repository.eliminar_pendientes_por_credito(usuario_id, id).await?;
        self.repository.finalizar(usuario_id, id).await?
            .ok_or_else(|| anyhow!("Crédito no encontrado"))
    }

    pub async fn eliminar_credito(&self, usuario_id: i64, id: i32) -> Result<Credito> {
        self.gasto_repository.eliminar_pendientes_por_credito(usuario_id, id).await?;
        self.repository.delete(usuario_id, id).await?
            .ok_or_else(|| anyhow!("Crédito no encontrado"))
    }

    pub async fn editar_credito(&self, usuario_id: i64, id: i32, dto: CreateCreditoDto) -> Result<Credito> {
        let dto = Self::normalizar_dia_pago(dto)?;
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
