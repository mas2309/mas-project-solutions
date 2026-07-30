use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EstadoProyecto {
    Planificacion,
    EnProgreso,
    Pausado,
    Completado,
    Cancelado,
}

impl From<String> for EstadoProyecto {
    fn from(estado: String) -> Self {
        match estado.to_lowercase().as_str() {
            "en_progreso" => EstadoProyecto::EnProgreso,
            "pausado" => EstadoProyecto::Pausado,
            "completado" => EstadoProyecto::Completado,
            "cancelado" => EstadoProyecto::Cancelado,
            _ => EstadoProyecto::Planificacion,
        }
    }
}

impl ToString for EstadoProyecto {
    fn to_string(&self) -> String {
        match self {
            EstadoProyecto::Planificacion => "Planificacion".to_string(),
            EstadoProyecto::EnProgreso => "En_Progreso".to_string(),
            EstadoProyecto::Pausado => "Pausado".to_string(),
            EstadoProyecto::Completado => "Completado".to_string(),
            EstadoProyecto::Cancelado => "Cancelado".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proyecto {
    pub id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub presupuesto: Option<Decimal>,
    pub costo_actual: Option<Decimal>,
    pub estado: EstadoProyecto,
    pub fecha_inicio: Option<String>,
    pub fecha_fin_estimada: Option<String>,
    pub fecha_fin_real: Option<String>,
    pub cliente: Option<String>,
    pub responsable: Option<String>,
    pub fecha_creacion: String,
    pub fecha_actualizacion: Option<String>,
}

impl Proyecto {
    pub fn new(
        nombre: String,
        descripcion: Option<String>,
        presupuesto: Option<Decimal>,
        cliente: Option<String>,
    ) -> Self {
        Self {
            id: 0,
            nombre,
            descripcion,
            presupuesto,
            costo_actual: Some(Decimal::ZERO),
            estado: EstadoProyecto::Planificacion,
            fecha_inicio: None,
            fecha_fin_estimada: None,
            fecha_fin_real: None,
            cliente,
            responsable: None,
            fecha_creacion: chrono::Utc::now().naive_utc().to_string(),
            fecha_actualizacion: None,
        }
    }

    pub fn iniciar(&mut self) {
        self.estado = EstadoProyecto::EnProgreso;
        self.fecha_inicio = Some(chrono::Utc::now().naive_utc().to_string());
        self.fecha_actualizacion = Some(chrono::Utc::now().naive_utc().to_string());
    }

    pub fn completar(&mut self) {
        self.estado = EstadoProyecto::Completado;
        self.fecha_fin_real = Some(chrono::Utc::now().naive_utc().to_string());
        self.fecha_actualizacion = Some(chrono::Utc::now().naive_utc().to_string());
    }

    pub fn pausar(&mut self) {
        self.estado = EstadoProyecto::Pausado;
        self.fecha_actualizacion = Some(chrono::Utc::now().naive_utc().to_string());
    }

    pub fn porcentaje_presupuesto_usado(&self) -> f64 {
        if let (Some(presupuesto), Some(costo)) = (&self.presupuesto, &self.costo_actual) {
            if *presupuesto > Decimal::ZERO {
                (*costo / *presupuesto * Decimal::from(100)).try_into().unwrap_or(0.0)
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    pub fn esta_sobre_presupuesto(&self) -> bool {
        if let (Some(presupuesto), Some(costo)) = (&self.presupuesto, &self.costo_actual) {
            *costo > *presupuesto
        } else {
            false
        }
    }

    pub fn is_activo(&self) -> bool {
        matches!(self.estado, EstadoProyecto::Planificacion | EstadoProyecto::EnProgreso)
    }
}