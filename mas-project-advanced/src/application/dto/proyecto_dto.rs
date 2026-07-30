use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProyectoDto {
    pub nombre: String,
    pub descripcion: Option<String>,
    pub presupuesto: Option<Decimal>,
    pub fecha_fin_estimada: Option<String>,
    pub cliente: Option<String>,
    pub responsable: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProyectoDto {
    pub nombre: Option<String>,
    pub descripcion: Option<String>,
    pub presupuesto: Option<Decimal>,
    pub fecha_fin_estimada: Option<String>,
    pub cliente: Option<String>,
    pub responsable: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProyectoSummaryDto {
    pub total_proyectos: i64,
    pub proyectos_activos: i64,
    pub proyectos_completados: i64,
    pub presupuesto_total: Decimal,
    pub costo_total: Decimal,
    pub proyectos_sobre_presupuesto: i64,
}