use crate::domain::entities::PagoExistente;
use crate::application::dto::{CreatePagoDto, PagosSummaryDto};
use crate::application::repositories::pago_repository::IPagoRepository;
use async_trait::async_trait;
use sqlx::PgPool;
use anyhow::Result;
use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::types::BigDecimal;
use std::str::FromStr;

pub struct PagoRepository {
    pool: PgPool,
}

impl PagoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn bigdecimal_to_decimal(bd: BigDecimal) -> Decimal {
        Decimal::from_str(&bd.to_string()).unwrap_or(Decimal::ZERO)
    }

    fn bigdecimal_opt_to_decimal_opt(bd_opt: Option<BigDecimal>) -> Option<Decimal> {
        bd_opt.map(|bd| Self::bigdecimal_to_decimal(bd))
    }

    fn map_row_to_pago(r: (i64, String, BigDecimal, Option<BigDecimal>, String, String, String, Option<i32>, Option<String>, Option<String>, chrono::NaiveDateTime, Option<chrono::NaiveDateTime>)) -> PagoExistente {
        println!("DEBUG - Estado desde DB: '{}' (len: {})", r.4, r.4.len());
        let estado: crate::domain::entities::pago_existente::EstadoPago = r.4.clone().into();
        println!("DEBUG - Estado convertido: {:?}", estado);
        
        PagoExistente {
            id: r.0 as i32,
            descripcion: r.1,
            valor: Self::bigdecimal_to_decimal(r.2),
            saldo: Self::bigdecimal_opt_to_decimal_opt(r.3),
            estado,
            mes: r.5,
            anio: r.6,
            proyecto_id: r.7,
            evidencia: r.8,
            evidencia_constructora: r.9,
            fecha_creacion: r.10.to_string(),
            fecha_actualizacion: r.11.map(|dt| dt.to_string()),
        }
    }
}

#[async_trait]
impl IPagoRepository for PagoRepository {
    async fn create(&self, usuario_id: i64, dto: CreatePagoDto) -> Result<PagoExistente> {
        let now = Utc::now().naive_utc();
        
        let row = sqlx::query_as::<_, (i64, String, BigDecimal, Option<BigDecimal>, String, String, String, Option<i32>, Option<String>, Option<String>, chrono::NaiveDateTime, Option<chrono::NaiveDateTime>)>(
            r#"
            INSERT INTO personal.pagos (descripcion, valor, saldo, estado, mes, anio, proyecto_id, fecha_creacion, usuario_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, descripcion, valor, saldo, estado, mes, anio, proyecto_id, evidencia, evidencia_constructora, fecha_creacion, fecha_actualizacion
            "#
        )
        .bind(&dto.descripcion)
        .bind(dto.valor.to_string().parse::<BigDecimal>().unwrap())
        .bind(dto.valor.to_string().parse::<BigDecimal>().unwrap())
        .bind("Pendiente")
        .bind(&dto.mes)
        .bind(&dto.anio)
        .bind(dto.proyecto_id)
        .bind(now)
        .bind(usuario_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(Self::map_row_to_pago(row))
    }

    async fn find_by_id(&self, usuario_id: i64, id: i32) -> Result<Option<PagoExistente>> {
        let row = sqlx::query_as::<_, (i64, String, BigDecimal, Option<BigDecimal>, String, String, String, Option<i32>, Option<String>, Option<String>, chrono::NaiveDateTime, Option<chrono::NaiveDateTime>)>(
            "SELECT id, descripcion, valor, saldo, estado, mes, anio, proyecto_id, evidencia, evidencia_constructora, fecha_creacion, fecha_actualizacion FROM personal.pagos WHERE id = $1 AND usuario_id = $2"
        )
        .bind(id)
        .bind(usuario_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Self::map_row_to_pago))
    }

    async fn registrar_pago(&self, usuario_id: i64, id: i32, monto: Decimal) -> Result<Option<PagoExistente>> {
        let now = Utc::now().naive_utc();
        
        let row = sqlx::query_as::<_, (i64, String, BigDecimal, Option<BigDecimal>, String, String, String, Option<i32>, Option<String>, Option<String>, chrono::NaiveDateTime, Option<chrono::NaiveDateTime>)>(
            r#"
            UPDATE personal.pagos 
            SET saldo = GREATEST(0, saldo - $2),
                estado = CASE 
                    WHEN (saldo - $2) <= 0 THEN 'Pagado'
                    WHEN (saldo - $2) < valor THEN 'Parcial'
                    ELSE estado
                END,
                fecha_actualizacion = $3
            WHERE id = $1 AND usuario_id = $4
            RETURNING id, descripcion, valor, saldo, estado, mes, anio, proyecto_id, evidencia, evidencia_constructora, fecha_creacion, fecha_actualizacion
            "#
        )
        .bind(id)
        .bind(monto.to_string().parse::<BigDecimal>().unwrap())
        .bind(now)
        .bind(usuario_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Self::map_row_to_pago))
    }

    async fn get_summary(&self, usuario_id: i64, anio: &str) -> Result<PagosSummaryDto> {
        let row = sqlx::query_as::<_, (Option<i64>, Option<BigDecimal>, Option<BigDecimal>, Option<i64>, Option<i64>)>(
            r#"
            SELECT 
                COUNT(*) as total_pagos,
                COALESCE(SUM(valor), 0) as total_valor,
                COALESCE(SUM(saldo), 0) as total_saldo,
                COUNT(CASE WHEN estado = 'Pagado' THEN 1 END) as pagos_completados,
                COUNT(CASE WHEN estado != 'Pagado' THEN 1 END) as pagos_pendientes
            FROM personal.pagos 
            WHERE usuario_id = $1 AND anio = $2
            "#
        )
        .bind(usuario_id)
        .bind(anio)
        .fetch_one(&self.pool)
        .await?;

        Ok(PagosSummaryDto {
            total_pagos: row.0.unwrap_or(0),
            total_valor: Self::bigdecimal_to_decimal(row.1.unwrap_or(BigDecimal::from(0))),
            total_saldo: Self::bigdecimal_to_decimal(row.2.unwrap_or(BigDecimal::from(0))),
            pagos_completados: row.3.unwrap_or(0),
            pagos_pendientes: row.4.unwrap_or(0),
        })
    }

    async fn actualizar_evidencia(&self, usuario_id: i64, id: i32, url: &str, tipo: &str) -> Result<Option<PagoExistente>> {
        let now = Utc::now().naive_utc();
        
        let query = match tipo {
            "cliente" => r#"
                UPDATE personal.pagos
                SET evidencia = $2, fecha_actualizacion = $3
                WHERE id = $1 AND usuario_id = $4
                RETURNING id, descripcion, valor, saldo, estado, mes, anio, proyecto_id, evidencia, evidencia_constructora, fecha_creacion, fecha_actualizacion
            "#,
            "constructora" => r#"
                UPDATE personal.pagos
                SET evidencia_constructora = $2, fecha_actualizacion = $3
                WHERE id = $1 AND usuario_id = $4
                RETURNING id, descripcion, valor, saldo, estado, mes, anio, proyecto_id, evidencia, evidencia_constructora, fecha_creacion, fecha_actualizacion
            "#,
            _ => return Err(anyhow::anyhow!("Tipo de evidencia inválido")),
        };
        
        let row = sqlx::query_as::<_, (i64, String, BigDecimal, Option<BigDecimal>, String, String, String, Option<i32>, Option<String>, Option<String>, chrono::NaiveDateTime, Option<chrono::NaiveDateTime>)>(query)
            .bind(id)
            .bind(url)
            .bind(now)
            .bind(usuario_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(Self::map_row_to_pago))
    }

    async fn marcar_pagado(&self, usuario_id: i64, id: i32) -> Result<Option<PagoExistente>> {
        let now = Utc::now().naive_utc();
        
        let row = sqlx::query_as::<_, (i64, String, BigDecimal, Option<BigDecimal>, String, String, String, Option<i32>, Option<String>, Option<String>, chrono::NaiveDateTime, Option<chrono::NaiveDateTime>)>(
            r#"
            UPDATE personal.pagos 
            SET saldo = 0, estado = 'Pagado', fecha_actualizacion = $2
            WHERE id = $1 AND usuario_id = $3
            RETURNING id, descripcion, valor, saldo, estado, mes, anio, proyecto_id, evidencia, evidencia_constructora, fecha_creacion, fecha_actualizacion
            "#
        )
        .bind(id)
        .bind(now)
        .bind(usuario_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Self::map_row_to_pago))
    }

    async fn delete(&self, usuario_id: i64, id: i32) -> Result<Option<PagoExistente>> {
        let row = sqlx::query_as::<_, (i64, String, BigDecimal, Option<BigDecimal>, String, String, String, Option<i32>, Option<String>, Option<String>, chrono::NaiveDateTime, Option<chrono::NaiveDateTime>)>(
            r#"
            DELETE FROM personal.pagos 
            WHERE id = $1 AND usuario_id = $2
            RETURNING id, descripcion, valor, saldo, estado, mes, anio, proyecto_id, evidencia, evidencia_constructora, fecha_creacion, fecha_actualizacion
            "#
        )
        .bind(id)
        .bind(usuario_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Self::map_row_to_pago))
    }

    async fn update(&self, usuario_id: i64, id: i32, descripcion: &str, valor: Decimal, mes: &str, anio: &str) -> Result<Option<PagoExistente>> {
        let now = Utc::now().naive_utc();
        
        let row = sqlx::query_as::<_, (i64, String, BigDecimal, Option<BigDecimal>, String, String, String, Option<i32>, Option<String>, Option<String>, chrono::NaiveDateTime, Option<chrono::NaiveDateTime>)>(
            r#"
            UPDATE personal.pagos 
            SET descripcion = $2, 
                valor = $3, 
                saldo = saldo + ($3 - valor),
                mes = $4, 
                anio = $5, 
                fecha_actualizacion = $6
            WHERE id = $1 AND usuario_id = $7
            RETURNING id, descripcion, valor, saldo, estado, mes, anio, proyecto_id, evidencia, evidencia_constructora, fecha_creacion, fecha_actualizacion
            "#
        )
        .bind(id)
        .bind(descripcion)
        .bind(valor.to_string().parse::<BigDecimal>().unwrap())
        .bind(mes)
        .bind(anio)
        .bind(now)
        .bind(usuario_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Self::map_row_to_pago))
    }

    /// Obtiene los totales globales de un proyecto (sin paginación)
    /// Retorna: (total_valor, total_saldo, pagos_completados, total_pagos)
    async fn get_totals_by_proyecto(&self, usuario_id: i64, proyecto_id: i32) -> Result<(Decimal, Decimal, i64, i64)> {
        let row = sqlx::query_as::<_, (Option<BigDecimal>, Option<BigDecimal>, Option<i64>, Option<i64>)>(
            r#"
            SELECT 
                COALESCE(SUM(p.valor), 0) as total_valor,
                COALESCE(SUM(p.saldo), 0) as total_saldo,
                COUNT(CASE WHEN p.estado = 'Pagado' THEN 1 END) as pagos_completados,
                COUNT(*) as total_pagos
            FROM personal.pagos p
            INNER JOIN personal.proyectos pr ON pr.id = p.proyecto_id
            WHERE p.proyecto_id = $1 AND pr.usuario_id = $2
            "#
        )
        .bind(proyecto_id)
        .bind(usuario_id)
        .fetch_one(&self.pool)
        .await?;

        let total_valor = Self::bigdecimal_to_decimal(row.0.unwrap_or(BigDecimal::from(0)));
        let total_saldo = Self::bigdecimal_to_decimal(row.1.unwrap_or(BigDecimal::from(0)));
        let pagos_completados = row.2.unwrap_or(0);
        let total_pagos = row.3.unwrap_or(0);

        Ok((total_valor, total_saldo, pagos_completados, total_pagos))
    }
}
