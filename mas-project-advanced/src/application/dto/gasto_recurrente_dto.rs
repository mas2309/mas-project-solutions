use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGastoRecurrenteDto {
    pub descripcion: String,
    pub monto_referencia: Decimal,
    pub categoria: String,
    pub tipo: String,
    pub responsable: Option<String>,
    pub dia_facturacion: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerarGastosDto {
    pub mes: u32,
    pub anio: i32,
}
