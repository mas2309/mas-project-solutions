//! Descarga de archivos (documentos, soportes de gastos y evidencias de pagos)
//! a través del backend. El cliente (web o app móvil) nunca se conecta directo
//! al storage: el servidor lee el archivo y lo entrega con la sesión del usuario.
//! Las mismas rutas se exponen en la web y en `/api/v1` (token Bearer).
//!
//! Query `?descargar=1` fuerza la descarga; sin él, PDF e imágenes se muestran en el navegador.

use axum::{
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Deserialize;

use crate::application::services::storage_service::ArchivoDescarga;
use crate::presentation::middleware::AuthUser;
use super::server::AppState;

#[derive(Deserialize, Default)]
pub struct DescargaQuery {
    #[serde(default)]
    descargar: Option<String>,
}

impl DescargaQuery {
    fn forzar_descarga(&self) -> bool {
        matches!(self.descargar.as_deref(), Some("1") | Some("true") | Some("si"))
    }
}

/// GET /documentos/:id/descargar
pub async fn descargar_documento(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
    Query(q): Query<DescargaQuery>,
) -> Response {
    responder(state.documento_service.descargar_documento(user.id, id).await, q.forzar_descarga())
}

/// GET /gastos/:id/soporte
pub async fn descargar_soporte_gasto(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<i32>,
    Query(q): Query<DescargaQuery>,
) -> Response {
    responder(state.gasto_service.descargar_soporte(user.id, id).await, q.forzar_descarga())
}

/// GET /pagos/:id/evidencia/:tipo  (tipo: cliente | constructora)
pub async fn descargar_evidencia_pago(
    State(state): State<AppState>,
    user: AuthUser,
    Path((id, tipo)): Path<(i32, String)>,
    Query(q): Query<DescargaQuery>,
) -> Response {
    responder(state.pago_service.descargar_evidencia(user.id, id, &tipo).await, q.forzar_descarga())
}

fn responder(resultado: anyhow::Result<ArchivoDescarga>, forzar_descarga: bool) -> Response {
    let archivo = match resultado {
        Ok(a) => a,
        Err(e) => {
            let msg = e.to_string();
            let status = if msg.contains("no encontrado") || msg.contains("no tiene") {
                StatusCode::NOT_FOUND
            } else if msg.contains("no válido") {
                StatusCode::BAD_REQUEST
            } else {
                println!("❌ Error descargando archivo: {:#}", e);
                StatusCode::BAD_GATEWAY
            };
            return (status, msg).into_response();
        }
    };

    let mostrar = !forzar_descarga
        && (archivo.content_type.starts_with("image/") || archivo.content_type == "application/pdf");
    let disposicion = format!(
        "{}; filename=\"{}\"; filename*=UTF-8''{}",
        if mostrar { "inline" } else { "attachment" },
        nombre_ascii(&archivo.nombre),
        codificar_rfc5987(&archivo.nombre),
    );

    (
        [
            (header::CONTENT_TYPE, archivo.content_type),
            (header::CONTENT_DISPOSITION, disposicion),
            // Datos financieros privados: que no queden en cachés compartidas
            (header::CACHE_CONTROL, "private, no-store".to_string()),
        ],
        archivo.contenido,
    )
        .into_response()
}

/// Versión ASCII del nombre para navegadores que no soportan `filename*`.
fn nombre_ascii(nombre: &str) -> String {
    nombre
        .chars()
        .map(|c| if c.is_ascii_graphic() && c != '"' && c != '\\' || c == ' ' { c } else { '_' })
        .collect()
}

/// Codificación RFC 5987 para `filename*` (permite tildes y ñ en el nombre).
fn codificar_rfc5987(nombre: &str) -> String {
    let mut out = String::new();
    for b in nombre.bytes() {
        if b.is_ascii_alphanumeric() || b"-._~".contains(&b) {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{:02X}", b));
        }
    }
    out
}
