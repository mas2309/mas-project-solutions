use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateArchivoDto {
    pub nombre_original: String,
    pub tipo_contenido: String,
    pub tamanio: i64,
    pub pago_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArchivoResponseDto {
    pub id: i64,
    pub nombre_archivo: String,
    pub nombre_original: String,
    pub ruta: String,
    pub tipo_archivo: Option<String>,
    pub tipo_contenido: String,
    pub tamanio: i64,
    pub tamanio_mb: f64,
    pub fecha_subida: String,
    pub pago_id: Option<i64>,
    pub usuario_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadFileDto {
    pub file_data: Vec<u8>,
    pub nombre_original: String,
    pub tipo_contenido: String,
    pub pago_id: Option<i64>,
}