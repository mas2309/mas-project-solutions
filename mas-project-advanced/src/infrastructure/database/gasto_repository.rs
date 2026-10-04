use crate::domain::entities::Gasto;
use crate::application::dto::CreateGastoDto;
use crate::application::repositories::gasto_repository::IGastoRepository;
use async_trait::async_trait;
use sqlx::PgPool;
use anyhow::Result;
use chrono::Utc;
use sqlx::types::BigDecimal;
use rust_decimal::Decimal;
use std::str::FromStr;

pub struct GastoRepository {
    pool: PgPool,
}

type GastoRow = (i32, String, BigDecimal, String, String, Option<String>, Option<String>, chrono::NaiveDate, chrono::NaiveDateTime, Option<i32>);

const SELECT_FIELDS: &str = "id, descripcion, monto, categoria, estado, responsable, soporte, fecha, fecha_creacion, credito_id";

impl GastoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn bd_to_decimal(bd: BigDecimal) -> Decimal {
        Decimal::from_str(&bd.to_string()).unwrap_or(Decimal::ZERO)
    }

    fn map_row(r: GastoRow) -> Gasto {
        Gasto {
            id: r.0,
            descripcion: r.1,
            monto: Self::bd_to_decimal(r.2),
            categoria: r.3.into(),
            estado: r.4.into(),
            responsable: r.5,
            soporte: r.6,
            fecha: r.7.to_string(),
            fecha_creacion: r.8.to_string(),
            credito_id: r.9,
        }
    }
}

#[async_trait]
impl IGastoRepository for GastoRepository {
    async fn create(&self, usuario_id: i64, dto: CreateGastoDto) -> Result<Gasto> {
        let now = Utc::now().naive_utc();
        let fecha = chrono::NaiveDate::parse_from_str(&dto.fecha, "%Y-%m-%d")?;

        let row = sqlx::query_as::<_, GastoRow>(
            &format!(r#"
            INSERT INTO personal.gastos (usuario_id, descripcion, monto, categoria, estado, responsable, fecha, fecha_creacion)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING {}
            "#, SELECT_FIELDS)
        )
        .bind(usuario_id)
        .bind(&dto.descripcion)
        .bind(dto.monto.to_string().parse::<BigDecimal>().unwrap())
        .bind(&dto.categoria)
        .bind("Pendiente")
        .bind(&dto.responsable)
        .bind(fecha)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(Self::map_row(row))
    }

    async fn create_from_recurrente(&self, usuario_id: i64, dto: CreateGastoDto, gasto_recurrente_id: i32) -> Result<Gasto> {
        let now = Utc::now().naive_utc();
        let fecha = chrono::NaiveDate::parse_from_str(&dto.fecha, "%Y-%m-%d")?;

        let row = sqlx::query_as::<_, GastoRow>(
            &format!(r#"
            INSERT INTO personal.gastos (usuario_id, descripcion, monto, categoria, estado, responsable, fecha, fecha_creacion, gasto_recurrente_id)
            VALUES ($1, $2, $3, $4, 'Pendiente', $5, $6, $7, $8)
            RETURNING {}
            "#, SELECT_FIELDS)
        )
        .bind(usuario_id)
        .bind(&dto.descripcion)
        .bind(dto.monto.to_string().parse::<BigDecimal>().unwrap())
        .bind(&dto.categoria)
        .bind(&dto.responsable)
        .bind(fecha)
        .bind(now)
        .bind(gasto_recurrente_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(Self::map_row(row))
    }

    async fn find_by_id(&self, usuario_id: i64, id: i32) -> Result<Option<Gasto>> {
        let row = sqlx::query_as::<_, GastoRow>(
            &format!("SELECT {} FROM personal.gastos WHERE id = $1 AND usuario_id = $2", SELECT_FIELDS)
        )
        .bind(id)
        .bind(usuario_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Self::map_row))
    }

    async fn list_by_month(&self, usuario_id: i64, anio: &str, mes: &str) -> Result<Vec<Gasto>> {
        let rows = sqlx::query_as::<_, GastoRow>(
            &format!("SELECT {} FROM personal.gastos WHERE usuario_id = $1 AND EXTRACT(YEAR FROM fecha)::text = $2 AND EXTRACT(MONTH FROM fecha)::text = $3 ORDER BY fecha DESC", SELECT_FIELDS)
        )
        .bind(usuario_id)
        .bind(anio)
        .bind(mes)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Self::map_row).collect())
    }

    async fn list_all(&self, usuario_id: i64, page: u32, page_size: u32) -> Result<(Vec<Gasto>, i64)> {
        let offset = (page - 1) * page_size;

        let rows = sqlx::query_as::<_, GastoRow>(
            &format!("SELECT {} FROM personal.gastos WHERE usuario_id = $1 ORDER BY fecha DESC LIMIT $2 OFFSET $3", SELECT_FIELDS)
        )
        .bind(usuario_id)
        .bind(page_size as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await?;

        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM personal.gastos WHERE usuario_id = $1")
            .bind(usuario_id)
            .fetch_one(&self.pool)
            .await?;

        Ok((rows.into_iter().map(Self::map_row).collect(), count.0))
    }

    async fn marcar_pagado(&self, usuario_id: i64, id: i32) -> Result<Option<Gasto>> {
        let row = sqlx::query_as::<_, GastoRow>(
            &format!("UPDATE personal.gastos SET estado = 'Pagado' WHERE id = $1 AND usuario_id = $2 RETURNING {}", SELECT_FIELDS)
        )
        .bind(id)
        .bind(usuario_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Self::map_row))
    }

    async fn delete(&self, usuario_id: i64, id: i32) -> Result<Option<Gasto>> {
        let row = sqlx::query_as::<_, GastoRow>(
            &format!("DELETE FROM personal.gastos WHERE id = $1 AND usuario_id = $2 RETURNING {}", SELECT_FIELDS)
        )
        .bind(id)
        .bind(usuario_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Self::map_row))
    }

    async fn actualizar_soporte(&self, usuario_id: i64, id: i32, url: &str) -> Result<Option<Gasto>> {
        let row = sqlx::query_as::<_, GastoRow>(
            &format!("UPDATE personal.gastos SET soporte = $3 WHERE id = $1 AND usuario_id = $2 RETURNING {}", SELECT_FIELDS)
        )
        .bind(id)
        .bind(usuario_id)
        .bind(url)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Self::map_row))
    }

    async fn update(&self, usuario_id: i64, id: i32, dto: CreateGastoDto) -> Result<Option<Gasto>> {
        let fecha = chrono::NaiveDate::parse_from_str(&dto.fecha, "%Y-%m-%d")?;

        let row = sqlx::query_as::<_, GastoRow>(
            &format!(r#"
            UPDATE personal.gastos
            SET descripcion = $3, monto = $4, categoria = $5, responsable = $6, fecha = $7
            WHERE id = $1 AND usuario_id = $2
            RETURNING {}
            "#, SELECT_FIELDS)
        )
        .bind(id)
        .bind(usuario_id)
        .bind(&dto.descripcion)
        .bind(dto.monto.to_string().parse::<BigDecimal>().unwrap())
        .bind(&dto.categoria)
        .bind(&dto.responsable)
        .bind(fecha)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Self::map_row))
    }

    async fn get_total_monto(&self, usuario_id: i64) -> Result<Decimal> {
        let row: (Option<BigDecimal>,) = sqlx::query_as(
            "SELECT COALESCE(SUM(monto), 0) FROM personal.gastos WHERE usuario_id = $1"
        )
        .bind(usuario_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(Self::bd_to_decimal(row.0.unwrap_or(BigDecimal::from(0))))
    }

    async fn create_from_credito(&self, usuario_id: i64, dto: CreateGastoDto, credito_id: i32, estado: &str) -> Result<Gasto> {
        let now = Utc::now().naive_utc();
        let fecha = chrono::NaiveDate::parse_from_str(&dto.fecha, "%Y-%m-%d")?;

        let row = sqlx::query_as::<_, GastoRow>(
            &format!(r#"
            INSERT INTO personal.gastos (usuario_id, descripcion, monto, categoria, estado, responsable, fecha, fecha_creacion, credito_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING {}
            "#, SELECT_FIELDS)
        )
        .bind(usuario_id)
        .bind(&dto.descripcion)
        .bind(dto.monto.to_string().parse::<BigDecimal>().unwrap())
        .bind(&dto.categoria)
        .bind(estado)
        .bind(&dto.responsable)
        .bind(fecha)
        .bind(now)
        .bind(credito_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(Self::map_row(row))
    }

    async fn creditos_con_cuota_en_mes(&self, usuario_id: i64, anio: i32, mes: u32) -> Result<Vec<i32>> {
        let rows = sqlx::query_as::<_, (i32,)>(
            r#"
            SELECT DISTINCT credito_id
            FROM personal.gastos
            WHERE credito_id IS NOT NULL
              AND usuario_id = $1
              AND EXTRACT(YEAR FROM fecha) = $2
              AND EXTRACT(MONTH FROM fecha) = $3
            "#
        )
        .bind(usuario_id)
        .bind(anio)
        .bind(mes as i32)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.0).collect())
    }

    async fn contar_pendientes_por_credito(&self, usuario_id: i64, credito_id: i32) -> Result<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM personal.gastos WHERE usuario_id = $1 AND credito_id = $2 AND estado = 'Pendiente'"
        )
        .bind(usuario_id)
        .bind(credito_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count.0)
    }

    async fn find_pendiente_por_credito(&self, usuario_id: i64, credito_id: i32) -> Result<Option<Gasto>> {
        let row = sqlx::query_as::<_, GastoRow>(
            &format!("SELECT {} FROM personal.gastos WHERE usuario_id = $1 AND credito_id = $2 AND estado = 'Pendiente' ORDER BY fecha ASC, id ASC LIMIT 1", SELECT_FIELDS)
        )
        .bind(usuario_id)
        .bind(credito_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Self::map_row))
    }

    async fn eliminar_pendientes_por_credito(&self, usuario_id: i64, credito_id: i32) -> Result<u64> {
        let result = sqlx::query(
            "DELETE FROM personal.gastos WHERE usuario_id = $1 AND credito_id = $2 AND estado = 'Pendiente' AND soporte IS NULL"
        )
        .bind(usuario_id)
        .bind(credito_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }
}
