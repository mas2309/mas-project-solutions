use crate::domain::entities::Documento;
use crate::application::dto::CreateDocumentoDto;
use crate::application::repositories::documento_repository::IDocumentoRepository;
use async_trait::async_trait;
use sqlx::PgPool;
use anyhow::Result;
use chrono::Utc;

pub struct DocumentoRepository {
    pool: PgPool,
}

impl DocumentoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn map_row(r: (i32, String, Option<String>, String, String, String, Option<chrono::NaiveDate>, chrono::NaiveDateTime)) -> Documento {
        Documento {
            id: r.0,
            nombre: r.1,
            descripcion: r.2,
            categoria: r.3.into(),
            archivo_url: r.4,
            nombre_archivo: r.5,
            fecha_vencimiento: r.6.map(|d| d.to_string()),
            fecha_creacion: r.7.to_string(),
        }
    }
}

#[async_trait]
impl IDocumentoRepository for DocumentoRepository {
    async fn create(&self, usuario_id: i64, dto: CreateDocumentoDto, archivo_url: &str, nombre_archivo: &str) -> Result<Documento> {
        let now = Utc::now().naive_utc();
        let fecha_venc = dto.fecha_vencimiento.as_ref().and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());

        let row = sqlx::query_as::<_, (i32, String, Option<String>, String, String, String, Option<chrono::NaiveDate>, chrono::NaiveDateTime)>(
            r#"
            INSERT INTO personal.documentos (usuario_id, nombre, descripcion, categoria, archivo_url, nombre_archivo, fecha_vencimiento, fecha_creacion)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, nombre, descripcion, categoria, archivo_url, nombre_archivo, fecha_vencimiento, fecha_creacion
            "#
        )
        .bind(usuario_id)
        .bind(&dto.nombre)
        .bind(&dto.descripcion)
        .bind(&dto.categoria)
        .bind(archivo_url)
        .bind(nombre_archivo)
        .bind(fecha_venc)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(Self::map_row(row))
    }

    async fn find_by_id(&self, usuario_id: i64, id: i32) -> Result<Option<Documento>> {
        let row = sqlx::query_as::<_, (i32, String, Option<String>, String, String, String, Option<chrono::NaiveDate>, chrono::NaiveDateTime)>(
            "SELECT id, nombre, descripcion, categoria, archivo_url, nombre_archivo, fecha_vencimiento, fecha_creacion FROM personal.documentos WHERE id = $1 AND usuario_id = $2"
        )
        .bind(id)
        .bind(usuario_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Self::map_row))
    }

    async fn list_all(&self, usuario_id: i64, page: u32, page_size: u32) -> Result<(Vec<Documento>, i64)> {
        let offset = (page - 1) * page_size;

        let rows = sqlx::query_as::<_, (i32, String, Option<String>, String, String, String, Option<chrono::NaiveDate>, chrono::NaiveDateTime)>(
            "SELECT id, nombre, descripcion, categoria, archivo_url, nombre_archivo, fecha_vencimiento, fecha_creacion FROM personal.documentos WHERE usuario_id = $1 ORDER BY fecha_creacion DESC LIMIT $2 OFFSET $3"
        )
        .bind(usuario_id)
        .bind(page_size as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await?;

        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM personal.documentos WHERE usuario_id = $1")
            .bind(usuario_id)
            .fetch_one(&self.pool).await?;

        Ok((rows.into_iter().map(Self::map_row).collect(), count.0))
    }

    async fn delete(&self, usuario_id: i64, id: i32) -> Result<Option<Documento>> {
        let row = sqlx::query_as::<_, (i32, String, Option<String>, String, String, String, Option<chrono::NaiveDate>, chrono::NaiveDateTime)>(
            "DELETE FROM personal.documentos WHERE id = $1 AND usuario_id = $2 RETURNING id, nombre, descripcion, categoria, archivo_url, nombre_archivo, fecha_vencimiento, fecha_creacion"
        )
        .bind(id)
        .bind(usuario_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Self::map_row))
    }
}
