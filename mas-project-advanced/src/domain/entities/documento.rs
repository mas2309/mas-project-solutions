use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CategoriaDocumento {
    Contrato,
    Poliza,
    Escritura,
    Certificado,
    Factura,
    Legal,
    Personal,
    Otro,
}

impl From<String> for CategoriaDocumento {
    fn from(cat: String) -> Self {
        match cat.to_lowercase().as_str() {
            "contrato" => CategoriaDocumento::Contrato,
            "poliza" => CategoriaDocumento::Poliza,
            "escritura" => CategoriaDocumento::Escritura,
            "certificado" => CategoriaDocumento::Certificado,
            "factura" => CategoriaDocumento::Factura,
            "legal" => CategoriaDocumento::Legal,
            "personal" => CategoriaDocumento::Personal,
            _ => CategoriaDocumento::Otro,
        }
    }
}

impl ToString for CategoriaDocumento {
    fn to_string(&self) -> String {
        match self {
            CategoriaDocumento::Contrato => "Contrato".to_string(),
            CategoriaDocumento::Poliza => "Poliza".to_string(),
            CategoriaDocumento::Escritura => "Escritura".to_string(),
            CategoriaDocumento::Certificado => "Certificado".to_string(),
            CategoriaDocumento::Factura => "Factura".to_string(),
            CategoriaDocumento::Legal => "Legal".to_string(),
            CategoriaDocumento::Personal => "Personal".to_string(),
            CategoriaDocumento::Otro => "Otro".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Documento {
    pub id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub categoria: CategoriaDocumento,
    pub archivo_url: String,
    pub nombre_archivo: String,
    pub fecha_vencimiento: Option<String>,
    pub fecha_creacion: String,
}