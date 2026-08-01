use std::sync::Arc;
use anyhow::{Result, anyhow};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use chrono::Utc;

use crate::infrastructure::database::usuario_repository::UsuarioRepository;
use crate::application::dto::{CreateUsuarioDto, UpdateUsuarioDto};
use crate::domain::entities::Usuario;

// ─── DTOs ────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct RegisterDto {
    pub username: String,
    pub email: String,
    pub nombre_completo: String,
    pub password: String,
}

// ─── JWT Claims ──────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,       // user_id as string
    pub username: String,
    pub rol: String,
    pub exp: usize,        // expiration timestamp
}

// ─── Login Response ──────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub usuario: UsuarioInfo,
}

#[derive(Debug, Serialize)]
pub struct UsuarioInfo {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub nombre_completo: String,
    pub rol: String,
}

// ─── Auth Service ────────────────────────────────────────────────────────────

pub struct AuthService {
    usuario_repository: Arc<UsuarioRepository>,
    jwt_secret: String,
    jwt_expiration_hours: i64,
}

impl AuthService {
    pub fn new(usuario_repository: Arc<UsuarioRepository>) -> Self {
        let jwt_secret = std::env::var("JWT_SECRET")
            .expect("⚠️ JWT_SECRET debe estar configurado. No se puede iniciar sin esta variable de entorno.");
        let jwt_expiration_hours = std::env::var("JWT_EXPIRATION_HOURS")
            .unwrap_or_else(|_| "8".to_string())
            .parse::<i64>()
            .unwrap_or(8);

        Self {
            usuario_repository,
            jwt_secret,
            jwt_expiration_hours,
        }
    }

    /// Asegura que exista un usuario admin.
    /// La contraseña se lee de ADMIN_DEFAULT_PASSWORD o usa un valor por defecto seguro.
    pub async fn ensure_admin_exists(&self) -> Result<()> {
        let admin = self.usuario_repository.find_by_username("admin").await?;
        if admin.is_none() {
            let admin_password = std::env::var("ADMIN_DEFAULT_PASSWORD")
                .unwrap_or_else(|_| "Admin123!".to_string());
            
            let hashed_password = hash(&admin_password, DEFAULT_COST)
                .map_err(|e| anyhow!("Error al hashear la contraseña del admin: {}", e))?;
            
            let create_dto = CreateUsuarioDto {
                username: "admin".to_string(),
                email: "admin@masfinance.com".to_string(),
                nombre_completo: "Administrador del Sistema".to_string(),
                password: hashed_password,
                rol: "admin".to_string(),
            };
            
            self.usuario_repository.create(create_dto).await?;
            println!("👤 Usuario admin creado. Cambie la contraseña por defecto lo antes posible.");
        }
        Ok(())
    }

    /// Registra un nuevo usuario con contraseña hasheada.
    /// El usuario se crea con activo=false (requiere activación por admin).
    pub async fn register(&self, dto: RegisterDto) -> Result<Usuario> {
        // Validaciones de input
        let username = dto.username.trim().to_string();
        let email = dto.email.trim().to_lowercase();
        let nombre_completo = dto.nombre_completo.trim().to_string();

        if username.len() < 3 || username.len() > 50 {
            return Err(anyhow!("El nombre de usuario debe tener entre 3 y 50 caracteres"));
        }

        if !username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            return Err(anyhow!("El nombre de usuario solo puede contener letras, números, guiones y guiones bajos"));
        }

        if dto.password.len() < 8 {
            return Err(anyhow!("La contraseña debe tener al menos 8 caracteres"));
        }

        if !email.contains('@') || !email.contains('.') || email.len() < 5 {
            return Err(anyhow!("El formato del correo electrónico no es válido"));
        }

        if nombre_completo.is_empty() || nombre_completo.len() > 100 {
            return Err(anyhow!("El nombre completo es requerido y no puede exceder 100 caracteres"));
        }

        // Verificar que el username no esté en uso
        let existing = self.usuario_repository.find_by_username(&username).await?;
        if existing.is_some() {
            return Err(anyhow!("El nombre de usuario '{}' ya está en uso", username));
        }

        // Hashear la contraseña
        let hashed_password = hash(&dto.password, DEFAULT_COST)
            .map_err(|e| anyhow!("Error al hashear la contraseña: {}", e))?;

        // Crear el DTO para el repositorio
        let create_dto = CreateUsuarioDto {
            username,
            email,
            nombre_completo,
            password: hashed_password,
            rol: "usuario".to_string(), // Rol por defecto
        };

        let usuario = self.usuario_repository.create(create_dto).await?;

        // Desactivar el usuario (requiere activación por admin)
        let update_dto = UpdateUsuarioDto {
            username: None,
            email: None,
            nombre_completo: None,
            rol: None,
            activo: Some(false),
        };

        let usuario = self.usuario_repository.update(usuario.id, update_dto).await?
            .ok_or_else(|| anyhow!("Error al desactivar el usuario recién creado"))?;

        Ok(usuario)
    }

    /// Autentica un usuario y retorna un JWT si las credenciales son válidas.
    pub async fn login(&self, username: &str, password: &str) -> Result<LoginResponse> {
        // Buscar usuario por username
        let usuario = self.usuario_repository.find_by_username(username).await?
            .ok_or_else(|| anyhow!("Credenciales inválidas"))?;

        // Verificar que el usuario esté activo
        if !usuario.activo {
            return Err(anyhow!("La cuenta no está activa. Contacte al administrador para activarla."));
        }

        // Verificar que no esté bloqueado
        if usuario.is_locked() {
            return Err(anyhow!("La cuenta está bloqueada temporalmente por intentos fallidos."));
        }

        // Verificar contraseña
        let password_valid = verify(password, &usuario.password)
            .map_err(|e| anyhow!("Error al verificar la contraseña: {}", e))?;

        if !password_valid {
            return Err(anyhow!("Credenciales inválidas"));
        }

        // Generar JWT
        let token = self.generate_token(&usuario)?;

        Ok(LoginResponse {
            token,
            usuario: UsuarioInfo {
                id: usuario.id,
                username: usuario.username,
                email: usuario.email,
                nombre_completo: usuario.nombre_completo,
                rol: usuario.rol,
            },
        })
    }

    /// Valida un token JWT y retorna los claims si es válido.
    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        let validation = Validation::new(Algorithm::HS256);
        
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &validation,
        )
        .map_err(|e| anyhow!("Token inválido: {}", e))?;

        Ok(token_data.claims)
    }

    /// Activa un usuario (solo admin).
    pub async fn activate_user(&self, user_id: i64) -> Result<Usuario> {
        let update_dto = UpdateUsuarioDto {
            username: None,
            email: None,
            nombre_completo: None,
            rol: None,
            activo: Some(true),
        };

        self.usuario_repository.update(user_id, update_dto).await?
            .ok_or_else(|| anyhow!("Usuario con id {} no encontrado", user_id))
    }

    /// Lista todos los usuarios (para panel admin).
    pub async fn list_users(&self) -> Result<Vec<Usuario>> {
        self.usuario_repository.list_all().await
    }

    /// Desactiva un usuario (solo admin).
    pub async fn deactivate_user(&self, user_id: i64) -> Result<Usuario> {
        let update_dto = UpdateUsuarioDto {
            username: None,
            email: None,
            nombre_completo: None,
            rol: None,
            activo: Some(false),
        };

        self.usuario_repository.update(user_id, update_dto).await?
            .ok_or_else(|| anyhow!("Usuario con id {} no encontrado", user_id))
    }

    // ─── Métodos Privados ────────────────────────────────────────────────────

    fn generate_token(&self, usuario: &Usuario) -> Result<String> {
        let expiration = Utc::now()
            .checked_add_signed(chrono::Duration::hours(self.jwt_expiration_hours))
            .ok_or_else(|| anyhow!("Error al calcular la expiración del token"))?
            .timestamp() as usize;

        let claims = Claims {
            sub: usuario.id.to_string(),
            username: usuario.username.clone(),
            rol: usuario.rol.clone(),
            exp: expiration,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| anyhow!("Error al generar el token JWT: {}", e))?;

        Ok(token)
    }
}
