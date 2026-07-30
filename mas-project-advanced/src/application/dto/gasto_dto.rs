use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGastoDto {
    pub descripcion: String,
    pub monto: Decimal,
    pub categoria: String,
    pub responsable: Option<String>,
    pub fecha: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GastoSummaryDto {
    pub total_gastos: i64,
    pub monto_total: Decimal,
    pub por_categoria: Vec<(String, Decimal)>,
}