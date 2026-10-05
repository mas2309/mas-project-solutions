use async_trait::async_trait;
use anyhow::Result;

/// Archivo leído del storage, listo para entregarlo al cliente a través del backend.
pub struct ArchivoDescarga {
    pub contenido: Vec<u8>,
    pub content_type: String,
    pub nombre: String,
}

#[async_trait]
pub trait IStorageService: Send + Sync {
    async fn upload_file(&self, file_data: Vec<u8>, file_name: &str, bucket: &str) -> Result<String>;
    async fn delete_file(&self, file_url: &str) -> Result<()>;
    /// Lee el archivo desde el storage. Retorna (contenido, content_type).
    async fn download_file(&self, file_url: &str) -> Result<(Vec<u8>, String)>;
}

/// Nombre del archivo a partir de su URL en el storage (último segmento).
pub fn nombre_desde_url(file_url: &str) -> String {
    file_url.rsplit('/').next().filter(|s| !s.is_empty()).unwrap_or("archivo").to_string()
}
