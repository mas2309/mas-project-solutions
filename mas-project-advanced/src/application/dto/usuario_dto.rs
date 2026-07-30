use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUsuarioDto {
    pub username: String,
    pub email: String,
    pub nombre_completo: String,
    pub password: String,
    pub rol: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUsuarioDto {
    pub username: Option<String>,
    pub email: Option<String>,
    pub nombre_completo: Option<String>,
    pub rol: Option<String>,
    pub activo: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsuarioResponseDto {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub nombre_completo: String,
    pub rol: String,
    pub activo: bool,
    pub fecha_creacion: String,
    pub ultimo_acceso: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginDto {
    pub username: String,
    pub password: String,
}