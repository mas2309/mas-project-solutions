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
    /// Día del mes en que se paga la cuota. Si no se envía, se usa el día de `fecha_inicio`.
    #[serde(default)]
    pub dia_pago: Option<i32>,
}