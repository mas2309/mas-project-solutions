use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CategoriaGasto {
    Alimentacion,
    Transporte,
    Servicios,
    Vivienda,
    Salud,
    Educacion,
    Entretenimiento,
    Mantenimiento,
    Impuestos,
    Otro,
}

impl From<String> for CategoriaGasto {
    fn from(cat: String) -> Self {
        match cat.to_lowercase().as_str() {
            "alimentacion" => CategoriaGasto::Alimentacion,
            "transporte" => CategoriaGasto::Transporte,
            "servicios" => CategoriaGasto::Servicios,
            "vivienda" => CategoriaGasto::Vivienda,
            "salud" => CategoriaGasto::Salud,
            "educacion" => CategoriaGasto::Educacion,
            "entretenimiento" => CategoriaGasto::Entretenimiento,
            "mantenimiento" => CategoriaGasto::Mantenimiento,
            "impuestos" => CategoriaGasto::Impuestos,
            _ => CategoriaGasto::Otro,
        }
    }
}

impl ToString for CategoriaGasto {
    fn to_string(&self) -> String {
        match self {
            CategoriaGasto::Alimentacion => "Alimentacion".to_string(),
            CategoriaGasto::Transporte => "Transporte".to_string(),
            CategoriaGasto::Servicios => "Servicios".to_string(),
            CategoriaGasto::Vivienda => "Vivienda".to_string(),
            CategoriaGasto::Salud => "Salud".to_string(),
            CategoriaGasto::Educacion => "Educacion".to_string(),
            CategoriaGasto::Entretenimiento => "Entretenimiento".to_string(),
            CategoriaGasto::Mantenimiento => "Mantenimiento".to_string(),
            CategoriaGasto::Impuestos => "Impuestos".to_string(),
            CategoriaGasto::Otro => "Otro".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EstadoGasto {
    Pendiente,
    Pagado,
    Anulado,
}

impl From<String> for EstadoGasto {
    fn from(estado: String) -> Self {
        match estado.to_lowercase().as_str() {
            "pagado" => EstadoGasto::Pagado,
            "anulado" => EstadoGasto::Anulado,
            _ => EstadoGasto::Pendiente,
        }
    }
}

impl ToString for EstadoGasto {
    fn to_string(&self) -> String {
        match self {
            EstadoGasto::Pendiente => "Pendiente".to_string(),
            EstadoGasto::Pagado => "Pagado".to_string(),
            EstadoGasto::Anulado => "Anulado".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gasto {
    pub id: i32,
    pub descripcion: String,
    pub monto: Decimal,
    pub categoria: CategoriaGasto,
    pub estado: EstadoGasto,
    pub responsable: Option<String>,
    pub soporte: Option<String>,
    pub fecha: String,
    pub fecha_creacion: String,
}