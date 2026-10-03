use axum::{
    extract::State,
    http::{Request, StatusCode, Uri},
    middleware::Next,
    response::{Response, Redirect, IntoResponse},
    body::Body,
    Json,
    extract::FromRequestParts,
    http::request::Parts,
};
use tower_cookies::Cookies;
use serde_json::json;

use crate::application::services::auth_service::Claims;
use crate::presentation::web::server::AppState;

const AUTH_COOKIE_NAME: &str = "auth_token";

// ─── AuthUser Extractor ──────────────────────────────────────────────────────
// Extractor que permite obtener el usuario autenticado en handlers
// Uso: pub async fn handler(user: AuthUser, ...) -> ... { user.id }

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: i64,
    pub username: String,
    pub rol: String,
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let claims = parts.extensions.get::<Claims>()
            .ok_or(StatusCode::UNAUTHORIZED)?;
        
        let id = claims.sub.parse::<i64>()
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        Ok(AuthUser {
            id,
            username: claims.username.clone(),
            rol: claims.rol.clone(),
        })
    }
}

// ─── Auth Guard ──────────────────────────────────────────────────────────────
// Middleware para rutas protegidas.
// - Web routes: lee JWT de cookie 'auth_token', redirige a /login si falla.
// - API routes (/api/): lee JWT de cookie o header Authorization, retorna 401 si falla.

pub async fn auth_guard(
    State(state): State<AppState>,
    cookies: Cookies,
    mut request: Request<Body>,
    next: Next,
) -> Response {
    let is_api = request.uri().path().starts_with("/api/");

    // 1. Intentar obtener el token
    let token = extract_token(&cookies, &request);

    let token = match token {
        Some(t) => t,
        None => {
            return unauthorized_response(is_api, request.uri());
        }
    };

    // 2. Validar el token con AuthService
    let claims = match state.auth_service.validate_token(&token) {
        Ok(claims) => claims,
        Err(_) => {
            // Token inválido o expirado: limpiar cookie
            return unauthorized_response(is_api, request.uri());
        }
    };

    // 3. Inyectar Claims en las extensiones del request
    request.extensions_mut().insert(claims);

    // 4. Continuar con el siguiente handler
    next.run(request).await
}

// ─── Admin Guard ─────────────────────────────────────────────────────────────
// Middleware que verifica que el usuario autenticado tenga rol 'admin'.
// DEBE usarse DESPUÉS de auth_guard en la cadena de middleware.

pub async fn admin_guard(
    State(_state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let is_api = request.uri().path().starts_with("/api/");

    // Obtener Claims del request (insertados por auth_guard)
    let claims = request.extensions().get::<Claims>().cloned();

    match claims {
        Some(ref c) if c.rol == "admin" => {
            // Usuario es admin, continuar
            next.run(request).await
        }
        Some(_) => {
            // Usuario autenticado pero no es admin
            forbidden_response(is_api)
        }
        None => {
            // No hay claims (auth_guard no se ejecutó antes)
            unauthorized_response(is_api, request.uri())
        }
    }
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

/// Extrae el token JWT de la cookie o del header Authorization (Bearer).
fn extract_token(cookies: &Cookies, request: &Request<Body>) -> Option<String> {
    // Primero intentar desde la cookie
    if let Some(cookie) = cookies.get(AUTH_COOKIE_NAME) {
        let value = cookie.value().to_string();
        if !value.is_empty() {
            return Some(value);
        }
    }

    // Fallback: header Authorization: Bearer <token> (útil para API)
    if let Some(auth_header) = request.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                return Some(token.to_string());
            }
        }
    }

    None
}

/// Respuesta para usuario no autenticado.
fn unauthorized_response(is_api: bool, uri: &Uri) -> Response {
    if is_api {
        // API: retornar 401 JSON
        let body = json!({
            "success": false,
            "data": null,
            "message": "No autenticado. Debe iniciar sesión."
        });
        (StatusCode::UNAUTHORIZED, Json(body)).into_response()
    } else {
        // Web: redirigir a /login con la URL original como parámetro
        let redirect_url = format!("/login?redirect={}", uri.path());
        Redirect::to(&redirect_url).into_response()
    }
}

/// Respuesta para usuario sin permisos suficientes.
fn forbidden_response(is_api: bool) -> Response {
    if is_api {
        let body = json!({
            "success": false,
            "data": null,
            "message": "Acceso denegado. Se requiere rol de administrador."
        });
        (StatusCode::FORBIDDEN, Json(body)).into_response()
    } else {
        // Web: redirigir al dashboard con mensaje (o mostrar página de error)
        Redirect::to("/dashboard?error=acceso_denegado").into_response()
    }
}
