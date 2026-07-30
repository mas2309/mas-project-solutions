use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CategoriaIngreso {
    Salario,
    Freelance,
    Inversiones,
    Arriendo,
    Venta,
    Otro,
}

impl From<String> for CategoriaIngreso {
    fn from(cat: String) -> Self {
        match cat.to_lowercase().as_str() {
            "salario" => CategoriaIngreso::Salario,
            "freelance" => CategoriaIngreso::Freelance,
            "inversiones" => CategoriaIngreso::Inversiones,
            "arriendo" => CategoriaIngreso::Arriendo,
            "venta" => CategoriaIngreso::Venta,
            _ => CategoriaIngreso::Otro,
        }
    }
}

impl ToString for CategoriaIngreso {
    fn to_string(&self) -> String {
        match self {
            CategoriaIngreso::Salario => "Salario".to_string(),
            CategoriaIngreso::Freelance => "Freelance".to_string(),
            CategoriaIngreso::Inversiones => "Inversiones".to_string(),
            CategoriaIngreso::Arriendo => "Arriendo".to_string(),
            CategoriaIngreso::Venta => "Venta".to_string(),
            CategoriaIngreso::Otro => "Otro".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingreso {
    pub id: i32,
    pub descripcion: String,
    pub monto: Decimal,
    pub categoria: CategoriaIngreso,
    pub fuente: Option<String>,
    pub fecha: String,
    pub recurrente: bool,
    pub fecha_creacion: String,
}