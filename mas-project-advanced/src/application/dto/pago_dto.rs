use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreatePagoDto {
    pub descripcion: String,
    pub valor: Decimal,
    pub mes: String,
    pub anio: String,
    pub proyecto_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePagoDto {
    pub descripcion: Option<String>,
    pub estado: Option<String>,
    pub evidencia: Option<String>,
    pub evidencia_constructora: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PagoResponseDto {
    pub id: i64,
    pub descripcion: String,
    pub valor: Decimal,
    pub saldo: Option<Decimal>,
    pub estado: String,
    pub mes: String,
    pub anio: String,
    pub evidencia: Option<String>,
    pub evidencia_constructora: Option<String>,
    pub fecha_creacion: String,
    pub fecha_actualizacion: Option<String>,
    pub porcentaje_pagado: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrarPagoDto {
    pub monto: Decimal,
    pub evidencia: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PagosSummaryDto {
    pub total_pagos: i64,
    pub total_valor: Decimal,
    pub total_saldo: Decimal,
    pub pagos_completados: i64,
    pub pagos_pendientes: i64,
}