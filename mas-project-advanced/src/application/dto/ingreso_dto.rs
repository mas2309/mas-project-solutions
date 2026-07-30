use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateIngresoDto {
    pub descripcion: String,
    pub monto: Decimal,
    pub categoria: String,
    pub fuente: Option<String>,
    pub fecha: String,
    pub recurrente: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IngresoSummaryDto {
    pub total_ingresos: i64,
    pub monto_total: Decimal,
    pub por_categoria: Vec<(String, Decimal)>,
}