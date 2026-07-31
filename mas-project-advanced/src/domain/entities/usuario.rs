use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usuario {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub nombre_completo: String,
    pub password: String,
    pub rol: String,
    pub activo: bool,
    pub fecha_creacion: String, // Simplified to string for now
    pub fecha_actualizacion: Option<String>,
    pub ultimo_acceso: Option<String>,
    pub failed_login_attempts: i32,
    pub lockout_end_time: Option<String>,
}

impl Usuario {
    pub fn new(
        username: String,
        email: String,
        nombre_completo: String,
        password: String,
        rol: String,
    ) -> Self {
        Self {
            id: 0, // Will be set by database
            username,
            email,
            nombre_completo,
            password,
            rol,
            activo: true,
            fecha_creacion: "2024-01-01T00:00:00Z".to_string(),
            fecha_actualizacion: None,
            ultimo_acceso: None,
            failed_login_attempts: 0,
            lockout_end_time: None,
        }
    }

    pub fn is_active(&self) -> bool {
        self.activo
    }

    pub fn is_locked(&self) -> bool {
        self.lockout_end_time.is_some()
    }

    pub fn increment_failed_attempts(&mut self) {
        self.failed_login_attempts += 1;
        if self.failed_login_attempts >= 5 {
            self.lockout_end_time = Some("2024-01-01T01:00:00Z".to_string()); // 1 hour lockout
        }
    }

    pub fn reset_failed_attempts(&mut self) {
        self.failed_login_attempts = 0;
        self.lockout_end_time = None;
    }
}