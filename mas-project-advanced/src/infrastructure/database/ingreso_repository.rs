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
    async fn create(&self, usuario_id: i64, dto: CreateIngresoDto) -> Result<Ingreso> {
        let now = Utc::now().naive_utc();
        let fecha = chrono::NaiveDate::parse_from_str(&dto.fecha, "%Y-%m-%d")?;

        let row = sqlx::query_as::<_, (i32, String, BigDecimal, String, Option<String>, chrono::NaiveDate, bool, chrono::NaiveDateTime)>(
            r#"
            INSERT INTO personal.ingresos (descripcion, monto, categoria, fuente, fecha, recurrente, fecha_creacion, usuario_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
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
        .bind(usuario_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(Self::map_row(row))
    }

    async fn find_by_id(&self, usuario_id: i64, id: i32) -> Result<Option<Ingreso>> {
        let row = sqlx::query_as::<_, (i32, String, BigDecimal, String, Option<String>, chrono::NaiveDate, bool, chrono::NaiveDateTime)>(
            "SELECT id, descripcion, monto, categoria, fuente, fecha, recurrente, fecha_creacion FROM personal.ingresos WHERE id = $1 AND usuario_id = $2"
        )
        .bind(id)
        .bind(usuario_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Self::map_row))
    }

    async fn list_by_month(&self, usuario_id: i64, anio: &str, mes: &str) -> Result<Vec<Ingreso>> {
        let rows = sqlx::query_as::<_, (i32, String, BigDecimal, String, Option<String>, chrono::NaiveDate, bool, chrono::NaiveDateTime)>(
            "SELECT id, descripcion, monto, categoria, fuente, fecha, recurrente, fecha_creacion FROM personal.ingresos WHERE usuario_id = $1 AND EXTRACT(YEAR FROM fecha)::text = $2 AND EXTRACT(MONTH FROM fecha)::text = $3 ORDER BY fecha DESC"
        )
        .bind(usuario_id)
        .bind(anio)
        .bind(mes)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Self::map_row).collect())
    }

    async fn list_all(&self, usuario_id: i64, page: u32, page_size: u32) -> Result<(Vec<Ingreso>, i64)> {
        let offset = (page - 1) * page_size;

        let rows = sqlx::query_as::<_, (i32, String, BigDecimal, String, Option<String>, chrono::NaiveDate, bool, chrono::NaiveDateTime)>(
            "SELECT id, descripcion, monto, categoria, fuente, fecha, recurrente, fecha_creacion FROM personal.ingresos WHERE usuario_id = $1 ORDER BY fecha DESC LIMIT $2 OFFSET $3"
        )
        .bind(usuario_id)
        .bind(page_size as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await?;

        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM personal.ingresos WHERE usuario_id = $1")
            .bind(usuario_id)
            .fetch_one(&self.pool)
            .await?;

        Ok((rows.into_iter().map(Self::map_row).collect(), count.0))
    }

    async fn delete(&self, usuario_id: i64, id: i32) -> Result<Option<Ingreso>> {
        let row = sqlx::query_as::<_, (i32, String, BigDecimal, String, Option<String>, chrono::NaiveDate, bool, chrono::NaiveDateTime)>(
            "DELETE FROM personal.ingresos WHERE id = $1 AND usuario_id = $2 RETURNING id, descripcion, monto, categoria, fuente, fecha, recurrente, fecha_creacion"
        )
        .bind(id)
        .bind(usuario_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Self::map_row))
    }

    async fn update(&self, usuario_id: i64, id: i32, dto: CreateIngresoDto) -> Result<Option<Ingreso>> {
        let fecha = chrono::NaiveDate::parse_from_str(&dto.fecha, "%Y-%m-%d")?;

        let row = sqlx::query_as::<_, (i32, String, BigDecimal, String, Option<String>, chrono::NaiveDate, bool, chrono::NaiveDateTime)>(
            r#"
            UPDATE personal.ingresos 
            SET descripcion = $3, monto = $4, categoria = $5, fuente = $6, fecha = $7, recurrente = $8
            WHERE id = $1 AND usuario_id = $2
            RETURNING id, descripcion, monto, categoria, fuente, fecha, recurrente, fecha_creacion
            "#
        )
        .bind(id)
        .bind(usuario_id)
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

    async fn get_total_monto(&self, usuario_id: i64) -> Result<Decimal> {
        let row: (Option<BigDecimal>,) = sqlx::query_as(
            "SELECT COALESCE(SUM(monto), 0) FROM personal.ingresos WHERE usuario_id = $1"
        )
        .bind(usuario_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(Self::bd_to_decimal(row.0.unwrap_or(BigDecimal::from(0))))
    }
}
