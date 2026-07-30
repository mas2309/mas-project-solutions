use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EstadoPago {
    Pendiente,
    Pagado,
    Vencido,
    Parcial,
}

impl From<String> for EstadoPago {
    fn from(estado: String) -> Self {
        match estado.trim().to_lowercase().as_str() {
            "pagado" => EstadoPago::Pagado,
            "vencido" => EstadoPago::Vencido,
            "parcial" => EstadoPago::Parcial,
            _ => EstadoPago::Pendiente,
        }
    }
}

impl ToString for EstadoPago {
    fn to_string(&self) -> String {
        match self {
            EstadoPago::Pendiente => "Pendiente".to_string(),
            EstadoPago::Pagado => "Pagado".to_string(),
            EstadoPago::Vencido => "Vencido".to_string(),
            EstadoPago::Parcial => "Parcial".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagoExistente {
    pub id: i32,
    pub descripcion: String,
    pub valor: Decimal,
    pub saldo: Option<Decimal>,
    pub estado: EstadoPago,
    pub mes: String,
    pub anio: String,
    pub proyecto_id: Option<i32>,
    pub evidencia: Option<String>,
    pub evidencia_constructora: Option<String>,
    pub fecha_creacion: String,
    pub fecha_actualizacion: Option<String>,
}

impl PagoExistente {
    pub fn new(
        descripcion: String,
        valor: Decimal,
        mes: String,
        anio: String,
        proyecto_id: Option<i32>,
    ) -> Self {
        Self {
            id: 0, // Will be set by database
            descripcion,
            valor,
            saldo: Some(valor), // Initially, saldo equals valor
            estado: EstadoPago::Pendiente,
            mes,
            anio,
            proyecto_id,
            evidencia: None,
            evidencia_constructora: None,
            fecha_creacion: "2024-01-01T00:00:00Z".to_string(),
            fecha_actualizacion: None,
        }
    }

    pub fn registrar_pago(&mut self, monto_pagado: Decimal) {
        if let Some(saldo_actual) = self.saldo {
            let nuevo_saldo = saldo_actual - monto_pagado;
            self.saldo = Some(nuevo_saldo);
            
            if nuevo_saldo <= Decimal::ZERO {
                self.estado = EstadoPago::Pagado;
                self.saldo = Some(Decimal::ZERO);
            } else if nuevo_saldo < self.valor {
                self.estado = EstadoPago::Parcial;
            }
        }
        self.fecha_actualizacion = Some("2024-01-01T00:00:00Z".to_string());
    }

    pub fn agregar_evidencia(&mut self, evidencia: String) {
        self.evidencia = Some(evidencia);
        self.fecha_actualizacion = Some("2024-01-01T00:00:00Z".to_string());
    }

    pub fn agregar_evidencia_constructora(&mut self, evidencia: String) {
        self.evidencia_constructora = Some(evidencia);
        self.fecha_actualizacion = Some("2024-01-01T00:00:00Z".to_string());
    }

    pub fn is_pagado_completo(&self) -> bool {
        matches!(self.estado, EstadoPago::Pagado)
    }

    pub fn porcentaje_pagado(&self) -> f64 {
        if let Some(saldo) = self.saldo {
            let pagado = self.valor - saldo;
            if self.valor > Decimal::ZERO {
                (pagado / self.valor * Decimal::from(100)).to_f64().unwrap_or(0.0)
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    pub fn monto_pagado(&self) -> Decimal {
        if let Some(saldo) = self.saldo {
            self.valor - saldo
        } else {
            Decimal::ZERO
        }
    }
}