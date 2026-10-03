use axum::{
    extract::{State, Path, Query},
    response::{Html, Redirect, IntoResponse, Response},
    Form,
};
use askama::Template;
use tower_cookies::{Cookies, Cookie};
use serde::Deserialize;

use crate::presentation::web::server::AppState;
use crate::application::services::auth_service::RegisterDto;
use crate::domain::entities::Usuario;

const AUTH_COOKIE_NAME: &str = "auth_token";

// ─── Templates ───────────────────────────────────────────────────────────────

#[derive(Template)]
#[template(path = "auth/login.html")]
pub struct LoginTemplate {
    pub login_error: Option<String>,
}

#[derive(Template)]
#[template(path = "auth/register.html")]
pub struct RegisterTemplate {
    pub register_error: Option<String>,
    pub register_success: Option<String>,
}

#[derive(Template)]
#[template(path = "auth/admin_users.html")]
pub struct AdminUsersTemplate {
    pub title: String,
    pub users: Vec<Usuario>,
}

// ─── Form DTOs ───────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterForm {
    pub username: String,
    pub email: String,
    pub nombre_completo: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginQuery {
    pub redirect: Option<String>,
}

// ─── Handlers ────────────────────────────────────────────────────────────────

/// GET /login - Muestra el formulario de login
pub async fn login_page(
    Query(_query): Query<LoginQuery>,
) -> impl IntoResponse {
    let template = LoginTemplate {
        login_error: None,
    };
    Html(template.render().unwrap_or_default())
}

/// POST /login - Procesa el login
pub async fn login_submit(
    State(state): State<AppState>,
    cookies: Cookies,
    Form(form): Form<LoginForm>,
) -> Response {
    match state.auth_service.login(&form.username, &form.password).await {
        Ok(login_response) => {
            // Establecer cookie segura con el token JWT
            let mut cookie = Cookie::new(AUTH_COOKIE_NAME, login_response.token);
            cookie.set_path("/");
            cookie.set_http_only(true);
            cookie.set_same_site(tower_cookies::cookie::SameSite::Lax);
            cookie.set_secure(true);
            cookies.add(cookie);
            
            // Redirigir al dashboard
            Redirect::to("/dashboard").into_response()
        }
        Err(e) => {
            let template = LoginTemplate {
                login_error: Some(e.to_string()),
            };
            Html(template.render().unwrap_or_default()).into_response()
        }
    }
}

/// GET /auth/register - Muestra el formulario de registro
pub async fn register_page() -> impl IntoResponse {
    let template = RegisterTemplate {
        register_error: None,
        register_success: None,
    };
    Html(template.render().unwrap_or_default())
}

/// POST /auth/register - Procesa el registro
pub async fn register_submit(
    State(state): State<AppState>,
    Form(form): Form<RegisterForm>,
) -> impl IntoResponse {
    let dto = RegisterDto {
        username: form.username,
        email: form.email,
        nombre_completo: form.nombre_completo,
        password: form.password,
    };

    match state.auth_service.register(dto).await {
        Ok(_) => {
            let template = RegisterTemplate {
                register_error: None,
                register_success: Some(
                    "¡Registro exitoso! Un administrador debe activar tu cuenta antes de poder iniciar sesión.".to_string()
                ),
            };
            Html(template.render().unwrap_or_default())
        }
        Err(e) => {
            let template = RegisterTemplate {
                register_error: Some(e.to_string()),
                register_success: None,
            };
            Html(template.render().unwrap_or_default())
        }
    }
}

/// POST /auth/logout - Cierra sesión
pub async fn logout(cookies: Cookies) -> impl IntoResponse {
    let mut cookie = Cookie::new(AUTH_COOKIE_NAME, "");
    cookie.set_path("/");
    cookie.set_http_only(true);
    cookies.remove(cookie);
    
    Redirect::to("/login")
}

/// GET /auth/admin/users - Panel de administración de usuarios
pub async fn admin_users_page(
    State(state): State<AppState>,
) -> impl IntoResponse {
    match state.auth_service.list_users().await {
        Ok(users) => {
            let template = AdminUsersTemplate {
                title: "Administración de Usuarios".to_string(),
                users,
            };
            Html(template.render().unwrap_or_default()).into_response()
        }
        Err(_) => {
            Redirect::to("/dashboard").into_response()
        }
    }
}

/// POST /auth/admin/users/:id/activate - Activar usuario
pub async fn activate_user(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let _ = state.auth_service.activate_user(id).await;
    Redirect::to("/auth/admin/users")
}

/// POST /auth/admin/users/:id/deactivate - Desactivar usuario
pub async fn deactivate_user(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let _ = state.auth_service.deactivate_user(id).await;
    Redirect::to("/auth/admin/users")
}
