use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EstadoCredito {
    Activo,
    Pagado,
    Mora,
    Refinanciado,
}

impl From<String> for EstadoCredito {
    fn from(estado: String) -> Self {
        match estado.to_lowercase().as_str() {
            "pagado" => EstadoCredito::Pagado,
            "mora" => EstadoCredito::Mora,
            "refinanciado" => EstadoCredito::Refinanciado,
            _ => EstadoCredito::Activo,
        }
    }
}

impl ToString for EstadoCredito {
    fn to_string(&self) -> String {
        match self {
            EstadoCredito::Activo => "Activo".to_string(),
            EstadoCredito::Pagado => "Pagado".to_string(),
            EstadoCredito::Mora => "Mora".to_string(),
            EstadoCredito::Refinanciado => "Refinanciado".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TipoTasa {
    Fija,
    Variable,
}

impl From<String> for TipoTasa {
    fn from(tipo: String) -> Self {
        match tipo.to_lowercase().as_str() {
            "variable" => TipoTasa::Variable,
            _ => TipoTasa::Fija,
        }
    }
}

impl ToString for TipoTasa {
    fn to_string(&self) -> String {
        match self {
            TipoTasa::Fija => "Fija".to_string(),
            TipoTasa::Variable => "Variable".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credito {
    pub id: i32,
    pub entidad: String,
    pub descripcion: String,
    pub monto_total: Decimal,
    pub saldo_pendiente: Decimal,
    pub tasa_interes: Decimal,
    pub tipo_tasa: TipoTasa,
    pub cuotas_totales: i32,
    pub cuotas_pagadas: i32,
    pub valor_cuota: Decimal,
    pub estado: EstadoCredito,
    pub fecha_inicio: String,
    pub fecha_fin_estimada: Option<String>,
    pub fecha_creacion: String,
}

impl Credito {
    pub fn cuotas_restantes(&self) -> i32 {
        self.cuotas_totales - self.cuotas_pagadas
    }

    pub fn porcentaje_pagado(&self) -> f64 {
        if self.cuotas_totales > 0 {
            (self.cuotas_pagadas as f64 / self.cuotas_totales as f64) * 100.0
        } else {
            0.0
        }
    }
}