use crate::domain::entities::Credito;
use crate::application::dto::CreateCreditoDto;
use crate::application::repositories::credito_repository::ICreditoRepository;
use async_trait::async_trait;
use sqlx::PgPool;
use anyhow::Result;
use chrono::Utc;
use sqlx::types::BigDecimal;
use rust_decimal::Decimal;
use std::str::FromStr;

pub struct CreditoRepository {
    pool: PgPool,
}

impl CreditoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn bd_to_decimal(bd: BigDecimal) -> Decimal {
        Decimal::from_str(&bd.to_string()).unwrap_or(Decimal::ZERO)
    }

    fn map_row(r: (i32, String, String, BigDecimal, BigDecimal, BigDecimal, String, i32, i32, BigDecimal, String, chrono::NaiveDate, Option<chrono::NaiveDate>, chrono::NaiveDateTime)) -> Credito {
        Credito {
            id: r.0,
            entidad: r.1,
            descripcion: r.2,
            monto_total: Self::bd_to_decimal(r.3),
            saldo_pendiente: Self::bd_to_decimal(r.4),
            tasa_interes: Self::bd_to_decimal(r.5),
            tipo_tasa: r.6.into(),
            cuotas_totales: r.7,
            cuotas_pagadas: r.8,
            valor_cuota: Self::bd_to_decimal(r.9),
            estado: r.10.into(),
            fecha_inicio: r.11.to_string(),
            fecha_fin_estimada: r.12.map(|d| d.to_string()),
            fecha_creacion: r.13.to_string(),
        }
    }
}

const SELECT_FIELDS: &str = "id, entidad, descripcion, monto_total, saldo_pendiente, tasa_interes, tipo_tasa, cuotas_totales, cuotas_pagadas, valor_cuota, estado, fecha_inicio, fecha_fin_estimada, fecha_creacion";

#[async_trait]
impl ICreditoRepository for CreditoRepository {
    async fn create(&self, usuario_id: i64, dto: CreateCreditoDto) -> Result<Credito> {
        let now = Utc::now().naive_utc();
        let fecha_inicio = chrono::NaiveDate::parse_from_str(&dto.fecha_inicio, "%Y-%m-%d")?;
        let fecha_fin = dto.fecha_fin_estimada.as_ref().and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());

        let row = sqlx::query_as::<_, (i32, String, String, BigDecimal, BigDecimal, BigDecimal, String, i32, i32, BigDecimal, String, chrono::NaiveDate, Option<chrono::NaiveDate>, chrono::NaiveDateTime)>(
            &format!("INSERT INTO personal.creditos (usuario_id, entidad, descripcion, monto_total, saldo_pendiente, tasa_interes, tipo_tasa, cuotas_totales, cuotas_pagadas, valor_cuota, estado, fecha_inicio, fecha_fin_estimada, fecha_creacion) VALUES ($1,$2,$3,$4,$4,$5,$6,$7,0,$8,'Activo',$9,$10,$11) RETURNING {}", SELECT_FIELDS)
        )
        .bind(usuario_id)
        .bind(&dto.entidad)
        .bind(&dto.descripcion)
        .bind(dto.monto_total.to_string().parse::<BigDecimal>().unwrap())
        .bind(dto.tasa_interes.to_string().parse::<BigDecimal>().unwrap())
        .bind(&dto.tipo_tasa)
        .bind(dto.cuotas_totales)
        .bind(dto.valor_cuota.to_string().parse::<BigDecimal>().unwrap())
        .bind(fecha_inicio)
        .bind(fecha_fin)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(Self::map_row(row))
    }

    async fn find_by_id(&self, usuario_id: i64, id: i32) -> Result<Option<Credito>> {
        let row = sqlx::query_as::<_, (i32, String, String, BigDecimal, BigDecimal, BigDecimal, String, i32, i32, BigDecimal, String, chrono::NaiveDate, Option<chrono::NaiveDate>, chrono::NaiveDateTime)>(
            &format!("SELECT {} FROM personal.creditos WHERE id = $1 AND usuario_id = $2", SELECT_FIELDS)
        )
        .bind(id)
        .bind(usuario_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Self::map_row))
    }

    async fn list_all(&self, usuario_id: i64, page: u32, page_size: u32) -> Result<(Vec<Credito>, i64)> {
        let offset = (page - 1) * page_size;

        let rows = sqlx::query_as::<_, (i32, String, String, BigDecimal, BigDecimal, BigDecimal, String, i32, i32, BigDecimal, String, chrono::NaiveDate, Option<chrono::NaiveDate>, chrono::NaiveDateTime)>(
            &format!("SELECT {} FROM personal.creditos WHERE usuario_id = $1 ORDER BY fecha_creacion DESC LIMIT $2 OFFSET $3", SELECT_FIELDS)
        )
        .bind(usuario_id)
        .bind(page_size as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await?;

        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM personal.creditos WHERE usuario_id = $1")
            .bind(usuario_id)
            .fetch_one(&self.pool).await?;

        Ok((rows.into_iter().map(Self::map_row).collect(), count.0))
    }

    async fn registrar_cuota(&self, usuario_id: i64, id: i32) -> Result<Option<Credito>> {
        let row = sqlx::query_as::<_, (i32, String, String, BigDecimal, BigDecimal, BigDecimal, String, i32, i32, BigDecimal, String, chrono::NaiveDate, Option<chrono::NaiveDate>, chrono::NaiveDateTime)>(
            &format!(r#"
            UPDATE personal.creditos 
            SET cuotas_pagadas = cuotas_pagadas + 1,
                saldo_pendiente = GREATEST(0, saldo_pendiente - valor_cuota),
                estado = CASE WHEN cuotas_pagadas + 1 >= cuotas_totales THEN 'Pagado' ELSE estado END
            WHERE id = $1 AND usuario_id = $2
            RETURNING {}"#, SELECT_FIELDS)
        )
        .bind(id)
        .bind(usuario_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Self::map_row))
    }

    async fn delete(&self, usuario_id: i64, id: i32) -> Result<Option<Credito>> {
        let row = sqlx::query_as::<_, (i32, String, String, BigDecimal, BigDecimal, BigDecimal, String, i32, i32, BigDecimal, String, chrono::NaiveDate, Option<chrono::NaiveDate>, chrono::NaiveDateTime)>(
            &format!("DELETE FROM personal.creditos WHERE id = $1 AND usuario_id = $2 RETURNING {}", SELECT_FIELDS)
        )
        .bind(id)
        .bind(usuario_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Self::map_row))
    }

    async fn update(&self, usuario_id: i64, id: i32, dto: CreateCreditoDto) -> Result<Option<Credito>> {
        let fecha_inicio = chrono::NaiveDate::parse_from_str(&dto.fecha_inicio, "%Y-%m-%d")?;
        let fecha_fin = dto.fecha_fin_estimada.as_ref().and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());

        let row = sqlx::query_as::<_, (i32, String, String, BigDecimal, BigDecimal, BigDecimal, String, i32, i32, BigDecimal, String, chrono::NaiveDate, Option<chrono::NaiveDate>, chrono::NaiveDateTime)>(
            &format!("UPDATE personal.creditos SET entidad=$3, descripcion=$4, monto_total=$5, tasa_interes=$6, tipo_tasa=$7, cuotas_totales=$8, valor_cuota=$9, fecha_inicio=$10, fecha_fin_estimada=$11 WHERE id=$1 AND usuario_id=$2 RETURNING {}", SELECT_FIELDS)
        )
        .bind(id)
        .bind(usuario_id)
        .bind(&dto.entidad)
        .bind(&dto.descripcion)
        .bind(dto.monto_total.to_string().parse::<BigDecimal>().unwrap())
        .bind(dto.tasa_interes.to_string().parse::<BigDecimal>().unwrap())
        .bind(&dto.tipo_tasa)
        .bind(dto.cuotas_totales)
        .bind(dto.valor_cuota.to_string().parse::<BigDecimal>().unwrap())
        .bind(fecha_inicio)
        .bind(fecha_fin)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Self::map_row))
    }

    async fn get_deuda_total(&self, usuario_id: i64) -> Result<Decimal> {
        let row: (Option<BigDecimal>,) = sqlx::query_as(
            "SELECT COALESCE(SUM(saldo_pendiente), 0) FROM personal.creditos WHERE usuario_id = $1"
        )
        .bind(usuario_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(Self::bd_to_decimal(row.0.unwrap_or(BigDecimal::from(0))))
    }
}
