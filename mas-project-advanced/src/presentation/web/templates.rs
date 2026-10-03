use askama::Template;
use crate::domain::entities::{Proyecto, PagoExistente, Ingreso, Gasto, Credito, Documento, GastoRecurrente};
use crate::application::dto::ProyectoSummaryDto;
use rust_decimal::Decimal;

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub title: String,
    pub balance: Decimal,
    pub total_ingresos: Decimal,
    pub total_gastos: Decimal,
    pub deuda_total: Decimal,
    pub num_ingresos: usize,
    pub num_gastos: usize,
    pub num_creditos: usize,
    pub proyectos_activos: i64,
    pub ultimos_ingresos: Vec<Ingreso>,
    pub ultimos_gastos: Vec<Gasto>,
    pub creditos_activos: Vec<Credito>,
    // Chart data (JSON strings for Chart.js)
    pub chart_labels_json: String,
    pub chart_ingresos_json: String,
    pub chart_gastos_json: String,
    pub chart_categorias_gastos_labels_json: String,
    pub chart_categorias_gastos_data_json: String,
    // Document alerts
    pub documentos_por_vencer: Vec<Documento>,
}

mod filters {
    use rust_decimal::Decimal;
    
    pub fn format_money(value: &Decimal) -> askama::Result<String> {
        let s = format!("{:.0}", value);
        let formatted = add_thousands_separator(&s);
        Ok(format!("${}", formatted))
    }

    fn add_thousands_separator(s: &str) -> String {
        let (sign, digits) = if s.starts_with('-') {
            ("-", &s[1..])
        } else {
            ("", s.as_ref())
        };
        
        let chars: Vec<char> = digits.chars().collect();
        let mut result = String::new();
        for (i, ch) in chars.iter().enumerate() {
            if i > 0 && (chars.len() - i) % 3 == 0 {
                result.push('.');
            }
            result.push(*ch);
        }
        format!("{}{}", sign, result)
    }
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    pub title: String,
}

#[derive(Template)]
#[template(path = "proyectos/list.html")]
pub struct ProyectosListTemplate {
    pub title: String,
    pub proyectos: Vec<Proyecto>,
    pub summary: ProyectoSummaryDto,
}

#[derive(Template)]
#[template(path = "proyectos/new.html")]
pub struct NewProyectoTemplate {
    pub title: String,
}

#[derive(Template)]
#[template(path = "proyectos/show.html")]
pub struct ShowProyectoTemplate {
    pub title: String,
    pub proyecto: Proyecto,
}

#[derive(Template)]
#[template(path = "proyectos/edit.html")]
pub struct EditProyectoTemplate {
    pub title: String,
    pub proyecto: Proyecto,
}

#[derive(Template)]
#[template(path = "proyectos/pagos.html")]
pub struct PagosProyectoTemplate {
    pub title: String,
    pub proyecto: Proyecto,
    pub pagos: Vec<PagoExistente>,
    pub total_valor: Decimal,
    pub total_abonado: Decimal,
    pub saldo_pendiente_proyecto: Decimal,
    pub total_pagos: usize,
    pub pagos_completados: usize,
    pub current_page: u32,
    pub total_pages: u32,
}

#[derive(Template)]
#[template(path = "proyectos/new_pago.html")]
pub struct NewPagoProyectoTemplate {
    pub title: String,
    pub proyecto: Proyecto,
    pub error: Option<String>,
    pub saldo_disponible: Decimal,
}

#[derive(Template)]
#[template(path = "proyectos/edit_pago.html")]
pub struct EditPagoTemplate {
    pub title: String,
    pub pago: PagoExistente,
}

#[derive(Template)]
#[template(path = "ingresos/list.html")]
pub struct IngresosListTemplate {
    pub title: String,
    pub ingresos: Vec<Ingreso>,
    pub total_ingresos: usize,
    pub monto_total: Decimal,
    pub current_page: u32,
    pub total_pages: u32,
}

#[derive(Template)]
#[template(path = "ingresos/new.html")]
pub struct NewIngresoTemplate {
    pub title: String,
}

#[derive(Template)]
#[template(path = "gastos/list.html")]
pub struct GastosListTemplate {
    pub title: String,
    pub gastos: Vec<Gasto>,
    pub total_gastos: usize,
    pub monto_total: Decimal,
    pub current_page: u32,
    pub total_pages: u32,
}

#[derive(Template)]
#[template(path = "gastos/new.html")]
pub struct NewGastoTemplate {
    pub title: String,
}

#[derive(Template)]
#[template(path = "ingresos/edit.html")]
pub struct EditIngresoTemplate {
    pub title: String,
    pub ingreso: Ingreso,
}

#[derive(Template)]
#[template(path = "gastos/edit.html")]
pub struct EditGastoTemplate {
    pub title: String,
    pub gasto: Gasto,
}

#[derive(Template)]
#[template(path = "creditos/list.html")]
pub struct CreditosListTemplate {
    pub title: String,
    pub creditos: Vec<Credito>,
    pub total_creditos: usize,
    pub deuda_total: Decimal,
    pub current_page: u32,
    pub total_pages: u32,
}

#[derive(Template)]
#[template(path = "creditos/new.html")]
pub struct NewCreditoTemplate {
    pub title: String,
}

#[derive(Template)]
#[template(path = "creditos/edit.html")]
pub struct EditCreditoTemplate {
    pub title: String,
    pub credito: Credito,
}

#[derive(Template)]
#[template(path = "documentos/list.html")]
pub struct DocumentosListTemplate {
    pub title: String,
    pub documentos: Vec<Documento>,
    pub total_documentos: usize,
    pub current_page: u32,
    pub total_pages: u32,
}

#[derive(Template)]
#[template(path = "documentos/new.html")]
pub struct NewDocumentoTemplate {
    pub title: String,
}

#[derive(Template)]
#[template(path = "gastos_recurrentes/list.html")]
pub struct GastosRecurrentesListTemplate {
    pub title: String,
    pub gastos_recurrentes: Vec<GastoRecurrente>,
    pub total_recurrentes: usize,
    pub monto_mensual_estimado: Decimal,
    pub pendientes_generar: u32,
    pub mes_actual: String,
}

#[derive(Template)]
#[template(path = "gastos_recurrentes/new.html")]
pub struct NewGastoRecurrenteTemplate {
    pub title: String,
}

#[derive(Template)]
#[template(path = "gastos_recurrentes/edit.html")]
pub struct EditGastoRecurrenteTemplate {
    pub title: String,
    pub gasto_recurrente: GastoRecurrente,
}