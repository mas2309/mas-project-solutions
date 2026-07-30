use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCreditoDto {
    pub entidad: String,
    pub descripcion: String,
    pub monto_total: Decimal,
    pub tasa_interes: Decimal,
    pub tipo_tasa: String,
    pub cuotas_totales: i32,
    pub valor_cuota: Decimal,
    pub fecha_inicio: String,
    pub fecha_fin_estimada: Option<String>,
}