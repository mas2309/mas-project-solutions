use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Archivo {
    pub id: i32,
    pub nombre_archivo: String,
    pub nombre_original: String,
    pub ruta: String,
    pub tipo_archivo: Option<String>,
    pub tipo_contenido: String,
    pub tamanio: i64,
    pub fecha_subida: String,
    pub pago_id: Option<i32>,
    pub usuario_id: i32,
}

impl Archivo {
    pub fn new(
        nombre_archivo: String,
        nombre_original: String,
        ruta: String,
        tipo_contenido: String,
        tamanio: i64,
        usuario_id: i32,
        pago_id: Option<i32>,
    ) -> Self {
        let tipo_archivo = Self::determinar_tipo_archivo(&tipo_contenido);
        
        Self {
            id: 0, // Will be set by database
            nombre_archivo,
            nombre_original,
            ruta,
            tipo_archivo: Some(tipo_archivo),
            tipo_contenido,
            tamanio,
            fecha_subida: "2024-01-01T00:00:00Z".to_string(),
            pago_id,
            usuario_id,
        }
    }

    fn determinar_tipo_archivo(tipo_contenido: &str) -> String {
        match tipo_contenido {
            ct if ct.starts_with("image/") => "imagen".to_string(),
            "application/pdf" => "pdf".to_string(),
            ct if ct.starts_with("video/") => "video".to_string(),
            ct if ct.starts_with("audio/") => "audio".to_string(),
            ct if ct.contains("document") || ct.contains("word") => "documento".to_string(),
            ct if ct.contains("spreadsheet") || ct.contains("excel") => "hoja_calculo".to_string(),
            _ => "otro".to_string(),
        }
    }

    pub fn is_imagen(&self) -> bool {
        self.tipo_contenido.starts_with("image/")
    }

    pub fn is_pdf(&self) -> bool {
        self.tipo_contenido == "application/pdf"
    }

    pub fn is_documento(&self) -> bool {
        self.tipo_contenido.contains("document") || 
        self.tipo_contenido.contains("word") ||
        self.tipo_contenido.contains("text")
    }

    pub fn tamanio_mb(&self) -> f64 {
        self.tamanio as f64 / (1024.0 * 1024.0)
    }

    pub fn extension(&self) -> Option<String> {
        self.nombre_original
            .split('.')
            .last()
            .map(|ext| ext.to_lowercase())
    }

    pub fn url_completa(&self, base_url: &str) -> String {
        format!("{}/{}", base_url.trim_end_matches('/'), self.ruta)
    }

    pub fn esta_asociado_a_pago(&self) -> bool {
        self.pago_id.is_some()
    }
}