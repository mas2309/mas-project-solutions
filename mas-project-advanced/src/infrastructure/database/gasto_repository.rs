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

impl GastoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn bd_to_decimal(bd: BigDecimal) -> Decimal {
        Decimal::from_str(&bd.to_string()).unwrap_or(Decimal::ZERO)
    }

    fn map_row(r: (i32, String, BigDecimal, String, String, Option<String>, Option<String>, chrono::NaiveDate, chrono::NaiveDateTime)) -> Gasto {
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
        }
    }
}

#[async_trait]
impl IGastoRepository for GastoRepository {
    async fn create(&self, dto: CreateGastoDto) -> Result<Gasto> {
        let now = Utc::now().naive_utc();
        let fecha = chrono::NaiveDate::parse_from_str(&dto.fecha, "%Y-%m-%d")?;

        let row = sqlx::query_as::<_, (i32, String, BigDecimal, String, String, Option<String>, Option<String>, chrono::NaiveDate, chrono::NaiveDateTime)>(
            r#"
            INSERT INTO personal.gastos (descripcion, monto, categoria, estado, responsable, fecha, fecha_creacion)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, descripcion, monto, categoria, estado, responsable, soporte, fecha, fecha_creacion
            "#
        )
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

    async fn find_by_id(&self, id: i32) -> Result<Option<Gasto>> {
        let row = sqlx::query_as::<_, (i32, String, BigDecimal, String, String, Option<String>, Option<String>, chrono::NaiveDate, chrono::NaiveDateTime)>(
            "SELECT id, descripcion, monto, categoria, estado, responsable, soporte, fecha, fecha_creacion FROM personal.gastos WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Self::map_row))
    }

    async fn list_by_month(&self, anio: &str, mes: &str) -> Result<Vec<Gasto>> {
        let rows = sqlx::query_as::<_, (i32, String, BigDecimal, String, String, Option<String>, Option<String>, chrono::NaiveDate, chrono::NaiveDateTime)>(
            "SELECT id, descripcion, monto, categoria, estado, responsable, soporte, fecha, fecha_creacion FROM personal.gastos WHERE EXTRACT(YEAR FROM fecha)::text = $1 AND EXTRACT(MONTH FROM fecha)::text = $2 ORDER BY fecha DESC"
        )
        .bind(anio)
        .bind(mes)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Self::map_row).collect())
    }

    async fn list_all(&self, page: u32, page_size: u32) -> Result<(Vec<Gasto>, i64)> {
        let offset = (page - 1) * page_size;

        let rows = sqlx::query_as::<_, (i32, String, BigDecimal, String, String, Option<String>, Option<String>, chrono::NaiveDate, chrono::NaiveDateTime)>(
            "SELECT id, descripcion, monto, categoria, estado, responsable, soporte, fecha, fecha_creacion FROM personal.gastos ORDER BY fecha DESC LIMIT $1 OFFSET $2"
        )
        .bind(page_size as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await?;

        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM personal.gastos")
            .fetch_one(&self.pool)
            .await?;

        Ok((rows.into_iter().map(Self::map_row).collect(), count.0))
    }

    async fn marcar_pagado(&self, id: i32) -> Result<Option<Gasto>> {
        let row = sqlx::query_as::<_, (i32, String, BigDecimal, String, String, Option<String>, Option<String>, chrono::NaiveDate, chrono::NaiveDateTime)>(
            "UPDATE personal.gastos SET estado = 'Pagado' WHERE id = $1 RETURNING id, descripcion, monto, categoria, estado, responsable, soporte, fecha, fecha_creacion"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Self::map_row))
    }

    async fn delete(&self, id: i32) -> Result<Option<Gasto>> {
        let row = sqlx::query_as::<_, (i32, String, BigDecimal, String, String, Option<String>, Option<String>, chrono::NaiveDate, chrono::NaiveDateTime)>(
            "DELETE FROM personal.gastos WHERE id = $1 RETURNING id, descripcion, monto, categoria, estado, responsable, soporte, fecha, fecha_creacion"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Self::map_row))
    }

    async fn actualizar_soporte(&self, id: i32, url: &str) -> Result<Option<Gasto>> {
        let row = sqlx::query_as::<_, (i32, String, BigDecimal, String, String, Option<String>, Option<String>, chrono::NaiveDate, chrono::NaiveDateTime)>(
            "UPDATE personal.gastos SET soporte = $2 WHERE id = $1 RETURNING id, descripcion, monto, categoria, estado, responsable, soporte, fecha, fecha_creacion"
        )
        .bind(id)
        .bind(url)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Self::map_row))
    }

    async fn update(&self, id: i32, dto: CreateGastoDto) -> Result<Option<Gasto>> {
        let fecha = chrono::NaiveDate::parse_from_str(&dto.fecha, "%Y-%m-%d")?;

        let row = sqlx::query_as::<_, (i32, String, BigDecimal, String, String, Option<String>, Option<String>, chrono::NaiveDate, chrono::NaiveDateTime)>(
            r#"
            UPDATE personal.gastos 
            SET descripcion = $2, monto = $3, categoria = $4, responsable = $5, fecha = $6
            WHERE id = $1
            RETURNING id, descripcion, monto, categoria, estado, responsable, soporte, fecha, fecha_creacion
            "#
        )
        .bind(id)
        .bind(&dto.descripcion)
        .bind(dto.monto.to_string().parse::<BigDecimal>().unwrap())
        .bind(&dto.categoria)
        .bind(&dto.responsable)
        .bind(fecha)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Self::map_row))
    }

    async fn create_from_recurrente(&self, dto: CreateGastoDto, gasto_recurrente_id: i32) -> Result<Gasto> {
        let now = Utc::now().naive_utc();
        let fecha = chrono::NaiveDate::parse_from_str(&dto.fecha, "%Y-%m-%d")?;

        let row = sqlx::query_as::<_, (i32, String, BigDecimal, String, String, Option<String>, Option<String>, chrono::NaiveDate, chrono::NaiveDateTime)>(
            r#"
            INSERT INTO personal.gastos (descripcion, monto, categoria, estado, responsable, fecha, fecha_creacion, gasto_recurrente_id)
            VALUES ($1, $2, $3, 'Pendiente', $4, $5, $6, $7)
            RETURNING id, descripcion, monto, categoria, estado, responsable, soporte, fecha, fecha_creacion
            "#
        )
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
}