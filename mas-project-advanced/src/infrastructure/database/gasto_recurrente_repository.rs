use crate::domain::entities::GastoRecurrente;
use crate::application::dto::CreateGastoRecurrenteDto;
use crate::application::repositories::gasto_recurrente_repository::IGastoRecurrenteRepository;
use async_trait::async_trait;
use sqlx::PgPool;
use anyhow::Result;
use chrono::Utc;
use sqlx::types::BigDecimal;
use rust_decimal::Decimal;
use std::str::FromStr;

pub struct GastoRecurrenteRepository {
    pool: PgPool,
}

impl GastoRecurrenteRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn bd_to_decimal(bd: BigDecimal) -> Decimal {
        Decimal::from_str(&bd.to_string()).unwrap_or(Decimal::ZERO)
    }

    fn map_row(r: (i32, String, BigDecimal, String, String, Option<String>, bool, i32, chrono::NaiveDateTime)) -> GastoRecurrente {
        GastoRecurrente {
            id: r.0,
            descripcion: r.1,
            monto_referencia: Self::bd_to_decimal(r.2),
            categoria: r.3,
            tipo: r.4.into(),
            responsable: r.5,
            activo: r.6,
            dia_facturacion: r.7,
            fecha_creacion: r.8.to_string(),
        }
    }
}

const SELECT_FIELDS: &str = "id, descripcion, monto_referencia, categoria, tipo, responsable, activo, dia_facturacion, fecha_creacion";

#[async_trait]
impl IGastoRecurrenteRepository for GastoRecurrenteRepository {
    async fn create(&self, dto: CreateGastoRecurrenteDto) -> Result<GastoRecurrente> {
        let now = Utc::now().naive_utc();

        let row = sqlx::query_as::<_, (i32, String, BigDecimal, String, String, Option<String>, bool, i32, chrono::NaiveDateTime)>(
            &format!(
                "INSERT INTO personal.gastos_recurrentes (descripcion, monto_referencia, categoria, tipo, responsable, dia_facturacion, fecha_creacion) VALUES ($1,$2,$3,$4,$5,$6,$7) RETURNING {}",
                SELECT_FIELDS
            )
        )
        .bind(&dto.descripcion)
        .bind(dto.monto_referencia.to_string().parse::<BigDecimal>().unwrap())
        .bind(&dto.categoria)
        .bind(&dto.tipo)
        .bind(&dto.responsable)
        .bind(dto.dia_facturacion)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(Self::map_row(row))
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<GastoRecurrente>> {
        let row = sqlx::query_as::<_, (i32, String, BigDecimal, String, String, Option<String>, bool, i32, chrono::NaiveDateTime)>(
            &format!("SELECT {} FROM personal.gastos_recurrentes WHERE id = $1", SELECT_FIELDS)
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Self::map_row))
    }

    async fn list_activos(&self) -> Result<Vec<GastoRecurrente>> {
        let rows = sqlx::query_as::<_, (i32, String, BigDecimal, String, String, Option<String>, bool, i32, chrono::NaiveDateTime)>(
            &format!("SELECT {} FROM personal.gastos_recurrentes WHERE activo = true ORDER BY dia_facturacion, descripcion", SELECT_FIELDS)
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Self::map_row).collect())
    }

    async fn list_all(&self) -> Result<Vec<GastoRecurrente>> {
        let rows = sqlx::query_as::<_, (i32, String, BigDecimal, String, String, Option<String>, bool, i32, chrono::NaiveDateTime)>(
            &format!("SELECT {} FROM personal.gastos_recurrentes ORDER BY activo DESC, dia_facturacion, descripcion", SELECT_FIELDS)
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Self::map_row).collect())
    }

    async fn update(&self, id: i32, dto: CreateGastoRecurrenteDto) -> Result<Option<GastoRecurrente>> {
        let row = sqlx::query_as::<_, (i32, String, BigDecimal, String, String, Option<String>, bool, i32, chrono::NaiveDateTime)>(
            &format!(
                "UPDATE personal.gastos_recurrentes SET descripcion=$2, monto_referencia=$3, categoria=$4, tipo=$5, responsable=$6, dia_facturacion=$7 WHERE id=$1 RETURNING {}",
                SELECT_FIELDS
            )
        )
        .bind(id)
        .bind(&dto.descripcion)
        .bind(dto.monto_referencia.to_string().parse::<BigDecimal>().unwrap())
        .bind(&dto.categoria)
        .bind(&dto.tipo)
        .bind(&dto.responsable)
        .bind(dto.dia_facturacion)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Self::map_row))
    }

    async fn toggle_activo(&self, id: i32) -> Result<Option<GastoRecurrente>> {
        let row = sqlx::query_as::<_, (i32, String, BigDecimal, String, String, Option<String>, bool, i32, chrono::NaiveDateTime)>(
            &format!(
                "UPDATE personal.gastos_recurrentes SET activo = NOT activo WHERE id=$1 RETURNING {}",
                SELECT_FIELDS
            )
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Self::map_row))
    }

    async fn delete(&self, id: i32) -> Result<Option<GastoRecurrente>> {
        let row = sqlx::query_as::<_, (i32, String, BigDecimal, String, String, Option<String>, bool, i32, chrono::NaiveDateTime)>(
            &format!(
                "DELETE FROM personal.gastos_recurrentes WHERE id=$1 RETURNING {}",
                SELECT_FIELDS
            )
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Self::map_row))
    }

    /// Retorna los IDs de gastos_recurrentes que ya fueron generados para el mes dado
    async fn ya_generados_en_mes(&self, anio: i32, mes: u32) -> Result<Vec<i32>> {
        let rows = sqlx::query_as::<_, (i32,)>(
            r#"
            SELECT DISTINCT g.gasto_recurrente_id 
            FROM personal.gastos g
            WHERE g.gasto_recurrente_id IS NOT NULL
              AND EXTRACT(YEAR FROM g.fecha) = $1
              AND EXTRACT(MONTH FROM g.fecha) = $2
            "#
        )
        .bind(anio)
        .bind(mes as i32)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.0).collect())
    }
}
