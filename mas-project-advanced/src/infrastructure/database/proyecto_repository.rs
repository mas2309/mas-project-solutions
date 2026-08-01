use crate::domain::entities::{PagoExistente, Proyecto};
use crate::application::dto::{CreateProyectoDto, UpdateProyectoDto, ProyectoSummaryDto};
use crate::application::repositories::proyecto_repository::IProyectoRepository;
use sqlx::PgPool;
use anyhow::Result;
use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::types::BigDecimal;
use std::str::FromStr;
use async_trait::async_trait;

pub struct ProyectoRepository {
    pool: PgPool,
}

impl ProyectoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn bigdecimal_to_decimal(bd: BigDecimal) -> Decimal {
        Decimal::from_str(&bd.to_string()).unwrap_or(Decimal::ZERO)
    }

    fn bigdecimal_opt_to_decimal_opt(bd_opt: Option<BigDecimal>) -> Option<Decimal> {
        bd_opt.map(|bd| Self::bigdecimal_to_decimal(bd))
    }

    pub async fn create_impl(&self, usuario_id: i64, dto: CreateProyectoDto) -> Result<Proyecto> {
        let now = Utc::now().naive_utc();
        
        let row = sqlx::query_as::<_, (i32, String, Option<String>, Option<BigDecimal>, Option<BigDecimal>, String, Option<chrono::NaiveDateTime>, Option<chrono::NaiveDateTime>, Option<chrono::NaiveDateTime>, Option<String>, Option<String>, chrono::NaiveDateTime, Option<chrono::NaiveDateTime>)>(
            r#"
            INSERT INTO personal.proyectos (nombre, descripcion, presupuesto, costo_actual, estado, fecha_fin_estimada, cliente, responsable, fecha_creacion, usuario_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id, nombre, descripcion, presupuesto, costo_actual, estado, fecha_inicio, fecha_fin_estimada, fecha_fin_real, cliente, responsable, fecha_creacion, fecha_actualizacion
            "#
        )
        .bind(&dto.nombre)
        .bind(&dto.descripcion)
        .bind(dto.presupuesto.map(|p| p.to_string().parse::<BigDecimal>().unwrap()))
        .bind(BigDecimal::from(0))
        .bind("Planificacion")
        .bind(dto.fecha_fin_estimada.as_ref().and_then(|d| chrono::NaiveDateTime::parse_from_str(d, "%Y-%m-%d %H:%M:%S").ok()))
        .bind(&dto.cliente)
        .bind(&dto.responsable)
        .bind(now)
        .bind(usuario_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(Proyecto {
            id: row.0,
            nombre: row.1,
            descripcion: row.2,
            presupuesto: Self::bigdecimal_opt_to_decimal_opt(row.3),
            costo_actual: Self::bigdecimal_opt_to_decimal_opt(row.4),
            estado: row.5.into(),
            fecha_inicio: row.6.map(|dt| dt.to_string()),
            fecha_fin_estimada: row.7.map(|dt| dt.to_string()),
            fecha_fin_real: row.8.map(|dt| dt.to_string()),
            cliente: row.9,
            responsable: row.10,
            fecha_creacion: row.11.to_string(),
            fecha_actualizacion: row.12.map(|dt| dt.to_string()),
        })
    }

    pub async fn update_impl(&self, usuario_id: i64, id: i32, dto: UpdateProyectoDto) -> Result<Option<Proyecto>> {
        let now = Utc::now().naive_utc();
        
        let row = sqlx::query_as::<_, (i32, String, Option<String>, Option<BigDecimal>, Option<BigDecimal>, String, Option<chrono::NaiveDateTime>, Option<chrono::NaiveDateTime>, Option<chrono::NaiveDateTime>, Option<String>, Option<String>, chrono::NaiveDateTime, Option<chrono::NaiveDateTime>)>(
            r#"
            UPDATE personal.proyectos 
            SET nombre = COALESCE($2, nombre),
                descripcion = COALESCE($3, descripcion),
                presupuesto = COALESCE($4, presupuesto),
                fecha_fin_estimada = COALESCE($5, fecha_fin_estimada),
                cliente = COALESCE($6, cliente),
                responsable = COALESCE($7, responsable),
                fecha_actualizacion = $8
            WHERE id = $1 AND usuario_id = $9
            RETURNING id, nombre, descripcion, presupuesto, costo_actual, estado, fecha_inicio, fecha_fin_estimada, fecha_fin_real, cliente, responsable, fecha_creacion, fecha_actualizacion
            "#
        )
        .bind(id)
        .bind(&dto.nombre)
        .bind(&dto.descripcion)
        .bind(dto.presupuesto.map(|p| p.to_string().parse::<BigDecimal>().unwrap()))
        .bind(dto.fecha_fin_estimada.as_ref().and_then(|d| chrono::NaiveDateTime::parse_from_str(d, "%Y-%m-%d %H:%M:%S").ok()))
        .bind(&dto.cliente)
        .bind(&dto.responsable)
        .bind(now)
        .bind(usuario_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Proyecto {
            id: r.0,
            nombre: r.1,
            descripcion: r.2,
            presupuesto: Self::bigdecimal_opt_to_decimal_opt(r.3),
            costo_actual: Self::bigdecimal_opt_to_decimal_opt(r.4),
            estado: r.5.into(),
            fecha_inicio: r.6.map(|dt| dt.to_string()),
            fecha_fin_estimada: r.7.map(|dt| dt.to_string()),
            fecha_fin_real: r.8.map(|dt| dt.to_string()),
            cliente: r.9,
            responsable: r.10,
            fecha_creacion: r.11.to_string(),
            fecha_actualizacion: r.12.map(|dt| dt.to_string()),
        }))
    }

    pub async fn cambiar_estado_impl(&self, usuario_id: i64, id: i32, nuevo_estado: &str) -> Result<Option<Proyecto>> {
        let now = Utc::now().naive_utc();
        
        let row = sqlx::query_as::<_, (i32, String, Option<String>, Option<BigDecimal>, Option<BigDecimal>, String, Option<chrono::NaiveDateTime>, Option<chrono::NaiveDateTime>, Option<chrono::NaiveDateTime>, Option<String>, Option<String>, chrono::NaiveDateTime, Option<chrono::NaiveDateTime>)>(
            r#"
            UPDATE personal.proyectos 
            SET estado = $2,
                fecha_inicio = CASE WHEN $2 = 'En_Progreso' AND fecha_inicio IS NULL THEN $3 ELSE fecha_inicio END,
                fecha_fin_real = CASE WHEN $2 = 'Completado' THEN $3 ELSE fecha_fin_real END,
                fecha_actualizacion = $3
            WHERE id = $1 AND usuario_id = $4
            RETURNING id, nombre, descripcion, presupuesto, costo_actual, estado, fecha_inicio, fecha_fin_estimada, fecha_fin_real, cliente, responsable, fecha_creacion, fecha_actualizacion
            "#
        )
        .bind(id)
        .bind(nuevo_estado)
        .bind(now)
        .bind(usuario_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Proyecto {
            id: r.0,
            nombre: r.1,
            descripcion: r.2,
            presupuesto: Self::bigdecimal_opt_to_decimal_opt(r.3),
            costo_actual: Self::bigdecimal_opt_to_decimal_opt(r.4),
            estado: r.5.into(),
            fecha_inicio: r.6.map(|dt| dt.to_string()),
            fecha_fin_estimada: r.7.map(|dt| dt.to_string()),
            fecha_fin_real: r.8.map(|dt| dt.to_string()),
            cliente: r.9,
            responsable: r.10,
            fecha_creacion: r.11.to_string(),
            fecha_actualizacion: r.12.map(|dt| dt.to_string()),
        }))
    }
}

#[async_trait]
impl IProyectoRepository for ProyectoRepository {
    async fn list_all(&self, usuario_id: i64) -> Result<Vec<Proyecto>> {
        let rows = sqlx::query_as::<_, (i32, String, Option<String>, Option<BigDecimal>, Option<BigDecimal>, String, Option<chrono::NaiveDateTime>, Option<chrono::NaiveDateTime>, Option<chrono::NaiveDateTime>, Option<String>, Option<String>, chrono::NaiveDateTime, Option<chrono::NaiveDateTime>)>(
            "SELECT id, nombre, descripcion, presupuesto, costo_actual, estado, fecha_inicio, fecha_fin_estimada, fecha_fin_real, cliente, responsable, fecha_creacion, fecha_actualizacion FROM personal.proyectos WHERE usuario_id = $1 ORDER BY fecha_creacion DESC"
        )
        .bind(usuario_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| Proyecto {
            id: r.0,
            nombre: r.1,
            descripcion: r.2,
            presupuesto: Self::bigdecimal_opt_to_decimal_opt(r.3),
            costo_actual: Self::bigdecimal_opt_to_decimal_opt(r.4),
            estado: r.5.into(),
            fecha_inicio: r.6.map(|dt| dt.to_string()),
            fecha_fin_estimada: r.7.map(|dt| dt.to_string()),
            fecha_fin_real: r.8.map(|dt| dt.to_string()),
            cliente: r.9,
            responsable: r.10,
            fecha_creacion: r.11.to_string(),
            fecha_actualizacion: r.12.map(|dt| dt.to_string()),
        }).collect())
    }

    async fn get_summary(&self, usuario_id: i64) -> Result<ProyectoSummaryDto> {
        let row = sqlx::query_as::<_, (Option<i64>, Option<i64>, Option<i64>, Option<BigDecimal>, Option<BigDecimal>, Option<i64>)>(
            r#"
            SELECT 
                COUNT(*) as total_proyectos,
                COUNT(CASE WHEN estado IN ('Planificacion', 'En_Progreso') THEN 1 END) as proyectos_activos,
                COUNT(CASE WHEN estado = 'Completado' THEN 1 END) as proyectos_completados,
                COALESCE(SUM(presupuesto), 0) as presupuesto_total,
                COALESCE(SUM(costo_actual), 0) as costo_total,
                COUNT(CASE WHEN costo_actual > presupuesto THEN 1 END) as proyectos_sobre_presupuesto
            FROM personal.proyectos
            WHERE usuario_id = $1
            "#
        )
        .bind(usuario_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(ProyectoSummaryDto {
            total_proyectos: row.0.unwrap_or(0),
            proyectos_activos: row.1.unwrap_or(0),
            proyectos_completados: row.2.unwrap_or(0),
            presupuesto_total: Self::bigdecimal_to_decimal(row.3.unwrap_or(BigDecimal::from(0))),
            costo_total: Self::bigdecimal_to_decimal(row.4.unwrap_or(BigDecimal::from(0))),
            proyectos_sobre_presupuesto: row.5.unwrap_or(0),
        })
    }

    async fn find_by_id(&self, usuario_id: i64, id: i32) -> Result<Option<Proyecto>> {
        let row = sqlx::query_as::<_, (i32, String, Option<String>, Option<BigDecimal>, Option<BigDecimal>, String, Option<chrono::NaiveDateTime>, Option<chrono::NaiveDateTime>, Option<chrono::NaiveDateTime>, Option<String>, Option<String>, chrono::NaiveDateTime, Option<chrono::NaiveDateTime>)>(
            "SELECT id, nombre, descripcion, presupuesto, costo_actual, estado, fecha_inicio, fecha_fin_estimada, fecha_fin_real, cliente, responsable, fecha_creacion, fecha_actualizacion FROM personal.proyectos WHERE id = $1 AND usuario_id = $2"
        )
        .bind(id)
        .bind(usuario_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Proyecto {
            id: r.0,
            nombre: r.1,
            descripcion: r.2,
            presupuesto: Self::bigdecimal_opt_to_decimal_opt(r.3),
            costo_actual: Self::bigdecimal_opt_to_decimal_opt(r.4),
            estado: r.5.into(),
            fecha_inicio: r.6.map(|dt| dt.to_string()),
            fecha_fin_estimada: r.7.map(|dt| dt.to_string()),
            fecha_fin_real: r.8.map(|dt| dt.to_string()),
            cliente: r.9,
            responsable: r.10,
            fecha_creacion: r.11.to_string(),
            fecha_actualizacion: r.12.map(|dt| dt.to_string()),
        }))
    }

    async fn create(&self, usuario_id: i64, dto: CreateProyectoDto) -> Result<Proyecto> {
        ProyectoRepository::create_impl(self, usuario_id, dto).await
    }

    async fn update(&self, usuario_id: i64, id: i32, dto: UpdateProyectoDto) -> Result<Option<Proyecto>> {
        ProyectoRepository::update_impl(self, usuario_id, id, dto).await
    }

    async fn get_pagos_by_proyecto(&self, usuario_id: i64, proyecto_id: i32, page: u32, page_size: u32) -> Result<(Vec<PagoExistente>, i64)> {
        let offset = (page - 1) * page_size;

        let rows = sqlx::query_as::<_, (i64, String, BigDecimal, Option<BigDecimal>, String, String, String, Option<i32>, Option<String>, Option<String>, chrono::NaiveDateTime, Option<chrono::NaiveDateTime>)>(
            "SELECT p.id, p.descripcion, p.valor, p.saldo, p.estado, p.mes, p.anio, p.proyecto_id, p.evidencia, p.evidencia_constructora, p.fecha_creacion, p.fecha_actualizacion 
             FROM personal.pagos p
             INNER JOIN personal.proyectos pr ON pr.id = p.proyecto_id
             WHERE p.proyecto_id = $1 AND pr.usuario_id = $2
             ORDER BY p.anio DESC, p.fecha_creacion DESC
             LIMIT $3 OFFSET $4"
        )
        .bind(proyecto_id)
        .bind(usuario_id)
        .bind(page_size as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await?;

        let total_count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM personal.pagos p INNER JOIN personal.proyectos pr ON pr.id = p.proyecto_id WHERE p.proyecto_id = $1 AND pr.usuario_id = $2"
        )
            .bind(proyecto_id)
            .bind(usuario_id)
            .fetch_one(&self.pool)
            .await?;

        let pagos = rows.into_iter().map(|r| PagoExistente {
            id: r.0 as i32,
            descripcion: r.1,
            valor: Self::bigdecimal_to_decimal(r.2),
            saldo: Self::bigdecimal_opt_to_decimal_opt(r.3),
            estado: r.4.into(),
            mes: r.5,
            anio: r.6,
            proyecto_id: r.7,
            evidencia: r.8,
            evidencia_constructora: r.9,
            fecha_creacion: r.10.to_string(),
            fecha_actualizacion: r.11.map(|dt| dt.to_string()),
        }).collect();

        Ok((pagos, total_count.0))
    }

    async fn cambiar_estado(&self, usuario_id: i64, id: i32, nuevo_estado: &str) -> Result<Option<Proyecto>> {
        ProyectoRepository::cambiar_estado_impl(self, usuario_id, id, nuevo_estado).await
    }
}
