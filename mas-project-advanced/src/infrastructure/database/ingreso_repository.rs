use crate::domain::entities::Ingreso;
use crate::application::dto::CreateIngresoDto;
use crate::application::repositories::ingreso_repository::IIngresoRepository;
use async_trait::async_trait;
use sqlx::PgPool;
use anyhow::Result;
use chrono::Utc;
use sqlx::types::BigDecimal;
use rust_decimal::Decimal;
use std::str::FromStr;

pub struct IngresoRepository {
    pool: PgPool,
}

impl IngresoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn bd_to_decimal(bd: BigDecimal) -> Decimal {
        Decimal::from_str(&bd.to_string()).unwrap_or(Decimal::ZERO)
    }

    fn map_row(r: (i32, String, BigDecimal, String, Option<String>, chrono::NaiveDate, bool, chrono::NaiveDateTime)) -> Ingreso {
        Ingreso {
            id: r.0,
            descripcion: r.1,
            monto: Self::bd_to_decimal(r.2),
            categoria: r.3.into(),
            fuente: r.4,
            fecha: r.5.to_string(),
            recurrente: r.6,
            fecha_creacion: r.7.to_string(),
        }
    }
}

#[async_trait]
impl IIngresoRepository for IngresoRepository {
    async fn create(&self, dto: CreateIngresoDto) -> Result<Ingreso> {
        let now = Utc::now().naive_utc();
        let fecha = chrono::NaiveDate::parse_from_str(&dto.fecha, "%Y-%m-%d")?;

        let row = sqlx::query_as::<_, (i32, String, BigDecimal, String, Option<String>, chrono::NaiveDate, bool, chrono::NaiveDateTime)>(
            r#"
            INSERT INTO personal.ingresos (descripcion, monto, categoria, fuente, fecha, recurrente, fecha_creacion)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, descripcion, monto, categoria, fuente, fecha, recurrente, fecha_creacion
            "#
        )
        .bind(&dto.descripcion)
        .bind(dto.monto.to_string().parse::<BigDecimal>().unwrap())
        .bind(&dto.categoria)
        .bind(&dto.fuente)
        .bind(fecha)
        .bind(dto.recurrente)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(Self::map_row(row))
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Ingreso>> {
        let row = sqlx::query_as::<_, (i32, String, BigDecimal, String, Option<String>, chrono::NaiveDate, bool, chrono::NaiveDateTime)>(
            "SELECT id, descripcion, monto, categoria, fuente, fecha, recurrente, fecha_creacion FROM personal.ingresos WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Self::map_row))
    }

    async fn list_by_month(&self, anio: &str, mes: &str) -> Result<Vec<Ingreso>> {
        let rows = sqlx::query_as::<_, (i32, String, BigDecimal, String, Option<String>, chrono::NaiveDate, bool, chrono::NaiveDateTime)>(
            "SELECT id, descripcion, monto, categoria, fuente, fecha, recurrente, fecha_creacion FROM personal.ingresos WHERE EXTRACT(YEAR FROM fecha)::text = $1 AND EXTRACT(MONTH FROM fecha)::text = $2 ORDER BY fecha DESC"
        )
        .bind(anio)
        .bind(mes)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Self::map_row).collect())
    }

    async fn list_all(&self, page: u32, page_size: u32) -> Result<(Vec<Ingreso>, i64)> {
        let offset = (page - 1) * page_size;

        let rows = sqlx::query_as::<_, (i32, String, BigDecimal, String, Option<String>, chrono::NaiveDate, bool, chrono::NaiveDateTime)>(
            "SELECT id, descripcion, monto, categoria, fuente, fecha, recurrente, fecha_creacion FROM personal.ingresos ORDER BY fecha DESC LIMIT $1 OFFSET $2"
        )
        .bind(page_size as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await?;

        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM personal.ingresos")
            .fetch_one(&self.pool)
            .await?;

        Ok((rows.into_iter().map(Self::map_row).collect(), count.0))
    }

    async fn delete(&self, id: i32) -> Result<Option<Ingreso>> {
        let row = sqlx::query_as::<_, (i32, String, BigDecimal, String, Option<String>, chrono::NaiveDate, bool, chrono::NaiveDateTime)>(
            "DELETE FROM personal.ingresos WHERE id = $1 RETURNING id, descripcion, monto, categoria, fuente, fecha, recurrente, fecha_creacion"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Self::map_row))
    }

    async fn update(&self, id: i32, dto: CreateIngresoDto) -> Result<Option<Ingreso>> {
        let fecha = chrono::NaiveDate::parse_from_str(&dto.fecha, "%Y-%m-%d")?;

        let row = sqlx::query_as::<_, (i32, String, BigDecimal, String, Option<String>, chrono::NaiveDate, bool, chrono::NaiveDateTime)>(
            r#"
            UPDATE personal.ingresos 
            SET descripcion = $2, monto = $3, categoria = $4, fuente = $5, fecha = $6, recurrente = $7
            WHERE id = $1
            RETURNING id, descripcion, monto, categoria, fuente, fecha, recurrente, fecha_creacion
            "#
        )
        .bind(id)
        .bind(&dto.descripcion)
        .bind(dto.monto.to_string().parse::<BigDecimal>().unwrap())
        .bind(&dto.categoria)
        .bind(&dto.fuente)
        .bind(fecha)
        .bind(dto.recurrente)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Self::map_row))
    }
}