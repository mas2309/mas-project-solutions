use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TipoGastoRecurrente {
    Fijo,
    FijoVariable,
}

impl From<String> for TipoGastoRecurrente {
    fn from(tipo: String) -> Self {
        match tipo.to_lowercase().as_str() {
            "fijovariable" | "fijo_variable" => TipoGastoRecurrente::FijoVariable,
            _ => TipoGastoRecurrente::Fijo,
        }
    }
}

impl ToString for TipoGastoRecurrente {
    fn to_string(&self) -> String {
        match self {
            TipoGastoRecurrente::Fijo => "Fijo".to_string(),
            TipoGastoRecurrente::FijoVariable => "FijoVariable".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GastoRecurrente {
    pub id: i32,
    pub descripcion: String,
    pub monto_referencia: Decimal,
    pub categoria: String,
    pub tipo: TipoGastoRecurrente,
    pub responsable: Option<String>,
    pub activo: bool,
    pub dia_facturacion: i32,
    pub fecha_creacion: String,
}
