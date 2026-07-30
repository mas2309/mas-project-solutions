use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDocumentoDto {
    pub nombre: String,
    pub descripcion: Option<String>,
    pub categoria: String,
    pub fecha_vencimiento: Option<String>,
}