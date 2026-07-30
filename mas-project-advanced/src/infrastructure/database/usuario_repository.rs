use crate::domain::entities::Usuario;
use crate::application::dto::{CreateUsuarioDto, UpdateUsuarioDto};
use sqlx::PgPool;
use anyhow::Result;
use chrono::{Utc, NaiveDateTime};

pub struct UsuarioRepository {
    pool: PgPool,
}

impl UsuarioRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, dto: CreateUsuarioDto) -> Result<Usuario> {
        let now = Utc::now().naive_utc();
        
        let row = sqlx::query_as::<_, (i32, String, String, String, String, String, bool, NaiveDateTime, Option<NaiveDateTime>, Option<NaiveDateTime>, i32, Option<NaiveDateTime>)>(
            r#"
            INSERT INTO personal.usuarios (username, email, nombre_completo, password, rol, activo, fecha_creacion, failed_login_attempts)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, username, email, nombre_completo, password, rol, activo, 
                      fecha_creacion, fecha_actualizacion, ultimo_acceso, failed_login_attempts, lockout_end_time
            "#
        )
        .bind(&dto.username)
        .bind(&dto.email)
        .bind(&dto.nombre_completo)
        .bind(&dto.password)
        .bind(&dto.rol)
        .bind(true)
        .bind(now)
        .bind(0)
        .fetch_one(&self.pool)
        .await?;

        Ok(Usuario {
            id: row.0,
            username: row.1,
            email: row.2,
            nombre_completo: row.3,
            password: row.4,
            rol: row.5,
            activo: row.6,
            fecha_creacion: row.7.to_string(),
            fecha_actualizacion: row.8.map(|dt| dt.to_string()),
            ultimo_acceso: row.9.map(|dt| dt.to_string()),
            failed_login_attempts: row.10,
            lockout_end_time: row.11.map(|dt| dt.to_string()),
        })
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<Usuario>> {
        let row = sqlx::query_as::<_, (i32, String, String, String, String, String, bool, NaiveDateTime, Option<NaiveDateTime>, Option<NaiveDateTime>, i32, Option<NaiveDateTime>)>(
            "SELECT id, username, email, nombre_completo, password, rol, activo, fecha_creacion, fecha_actualizacion, ultimo_acceso, failed_login_attempts, lockout_end_time FROM personal.usuarios WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Usuario {
            id: r.0,
            username: r.1,
            email: r.2,
            nombre_completo: r.3,
            password: r.4,
            rol: r.5,
            activo: r.6,
            fecha_creacion: r.7.to_string(),
            fecha_actualizacion: r.8.map(|dt| dt.to_string()),
            ultimo_acceso: r.9.map(|dt| dt.to_string()),
            failed_login_attempts: r.10,
            lockout_end_time: r.11.map(|dt| dt.to_string()),
        }))
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<Usuario>> {
        let row = sqlx::query_as::<_, (i32, String, String, String, String, String, bool, NaiveDateTime, Option<NaiveDateTime>, Option<NaiveDateTime>, i32, Option<NaiveDateTime>)>(
            "SELECT id, username, email, nombre_completo, password, rol, activo, fecha_creacion, fecha_actualizacion, ultimo_acceso, failed_login_attempts, lockout_end_time FROM personal.usuarios WHERE username = $1"
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Usuario {
            id: r.0,
            username: r.1,
            email: r.2,
            nombre_completo: r.3,
            password: r.4,
            rol: r.5,
            activo: r.6,
            fecha_creacion: r.7.to_string(),
            fecha_actualizacion: r.8.map(|dt| dt.to_string()),
            ultimo_acceso: r.9.map(|dt| dt.to_string()),
            failed_login_attempts: r.10,
            lockout_end_time: r.11.map(|dt| dt.to_string()),
        }))
    }

    pub async fn update(&self, id: i32, dto: UpdateUsuarioDto) -> Result<Option<Usuario>> {
        let now = Utc::now().naive_utc();
        
        let row = sqlx::query_as::<_, (i32, String, String, String, String, String, bool, NaiveDateTime, Option<NaiveDateTime>, Option<NaiveDateTime>, i32, Option<NaiveDateTime>)>(
            r#"
            UPDATE personal.usuarios 
            SET username = COALESCE($2, username),
                email = COALESCE($3, email),
                nombre_completo = COALESCE($4, nombre_completo),
                rol = COALESCE($5, rol),
                activo = COALESCE($6, activo),
                fecha_actualizacion = $7
            WHERE id = $1
            RETURNING id, username, email, nombre_completo, password, rol, activo, 
                      fecha_creacion, fecha_actualizacion, ultimo_acceso, failed_login_attempts, lockout_end_time
            "#
        )
        .bind(id)
        .bind(dto.username)
        .bind(dto.email)
        .bind(dto.nombre_completo)
        .bind(dto.rol)
        .bind(dto.activo)
        .bind(now)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Usuario {
            id: r.0,
            username: r.1,
            email: r.2,
            nombre_completo: r.3,
            password: r.4,
            rol: r.5,
            activo: r.6,
            fecha_creacion: r.7.to_string(),
            fecha_actualizacion: r.8.map(|dt| dt.to_string()),
            ultimo_acceso: r.9.map(|dt| dt.to_string()),
            failed_login_attempts: r.10,
            lockout_end_time: r.11.map(|dt| dt.to_string()),
        }))
    }

    pub async fn list_all(&self) -> Result<Vec<Usuario>> {
        let rows = sqlx::query_as::<_, (i32, String, String, String, String, String, bool, NaiveDateTime, Option<NaiveDateTime>, Option<NaiveDateTime>, i32, Option<NaiveDateTime>)>(
            "SELECT id, username, email, nombre_completo, password, rol, activo, fecha_creacion, fecha_actualizacion, ultimo_acceso, failed_login_attempts, lockout_end_time FROM personal.usuarios ORDER BY fecha_creacion DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| Usuario {
            id: r.0,
            username: r.1,
            email: r.2,
            nombre_completo: r.3,
            password: r.4,
            rol: r.5,
            activo: r.6,
            fecha_creacion: r.7.to_string(),
            fecha_actualizacion: r.8.map(|dt| dt.to_string()),
            ultimo_acceso: r.9.map(|dt| dt.to_string()),
            failed_login_attempts: r.10,
            lockout_end_time: r.11.map(|dt| dt.to_string()),
        }).collect())
    }
}