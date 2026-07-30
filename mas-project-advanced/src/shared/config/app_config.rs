use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Environment {
    Development,
    Testing,
    Production,
}

impl From<String> for Environment {
    fn from(env: String) -> Self {
        match env.to_lowercase().as_str() {
            "production" | "prod" => Environment::Production,
            "testing" | "test" => Environment::Testing,
            _ => Environment::Development,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub schema: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub endpoint: String,
    pub tenant_id: String,
    pub access_key: String,
    pub secret_key: String,
    pub region: String,
    pub bucket_proyectos: String,
    pub bucket_documentos: String,
    pub bucket_gastos: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub environment: Environment,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub storage: StorageConfig,
    pub log_level: String,
}

impl AppConfig {
    pub fn load() -> Self {
        let environment = env::var("ENVIRONMENT")
            .unwrap_or_else(|_| "development".to_string())
            .into();

        match environment {
            Environment::Development => Self::development(),
            Environment::Testing => Self::testing(),
            Environment::Production => Self::production(),
        }
    }

    fn development() -> Self {
        Self {
            environment: Environment::Development,
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8082,
            },
            database: DatabaseConfig {
                host: "localhost".to_string(),
                port: 5432,
                database: "acueducto_hato".to_string(),
                username: "postgres".to_string(),
                password: "Mas23".to_string(),
                schema: "personal".to_string(),
                max_connections: 10,
            },
            storage: StorageConfig {
                endpoint: env::var("STORAGE_ENDPOINT").unwrap_or_else(|_| "https://usc1.contabostorage.com".to_string()),
                tenant_id: env::var("STORAGE_TENANT_ID").unwrap_or_else(|_| "e920c78ff1e84217ab7612302c49cbc8".to_string()),
                access_key: env::var("CONTABO_ACCESS_KEY").unwrap_or_else(|_| "".to_string()),
                secret_key: env::var("CONTABO_SECRET_KEY").unwrap_or_else(|_| "".to_string()),
                region: "default".to_string(),
                bucket_proyectos: env::var("STORAGE_BUCKET_PROYECTOS").unwrap_or_else(|_| "apartamento-paipa-sochagota-25".to_string()),
                bucket_documentos: env::var("STORAGE_BUCKET_DOCUMENTOS").unwrap_or_else(|_| "important-document".to_string()),
                bucket_gastos: env::var("STORAGE_BUCKET_GASTOS").unwrap_or_else(|_| "mas-finance-gastos".to_string()),
            },
            log_level: "debug".to_string(),
        }
    }

    fn testing() -> Self {
        Self {
            environment: Environment::Testing,
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8081,
            },
            database: DatabaseConfig {
                host: "localhost".to_string(),
                port: 5432,
                database: "acueducto_hato_test".to_string(),
                username: "postgres".to_string(),
                password: "Mas23".to_string(),
                schema: "personal".to_string(),
                max_connections: 5,
            },
            storage: StorageConfig {
                endpoint: env::var("STORAGE_ENDPOINT").unwrap_or_else(|_| "https://usc1.contabostorage.com".to_string()),
                tenant_id: env::var("STORAGE_TENANT_ID").unwrap_or_else(|_| "e920c78ff1e84217ab7612302c49cbc8".to_string()),
                access_key: env::var("CONTABO_ACCESS_KEY").unwrap_or_else(|_| "".to_string()),
                secret_key: env::var("CONTABO_SECRET_KEY").unwrap_or_else(|_| "".to_string()),
                region: "default".to_string(),
                // En testing, un solo bucket para todo
                bucket_proyectos: "mas-finance-test".to_string(),
                bucket_documentos: "mas-finance-test".to_string(),
                bucket_gastos: "mas-finance-test".to_string(),
            },
            log_level: "info".to_string(),
        }
    }

    fn production() -> Self {
        Self {
            environment: Environment::Production,
            server: ServerConfig {
                host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()
                    .unwrap_or(8080),
            },
            database: DatabaseConfig {
                host: env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
                port: env::var("DB_PORT")
                    .unwrap_or_else(|_| "5432".to_string())
                    .parse()
                    .unwrap_or(5432),
                database: env::var("DB_NAME").unwrap_or_else(|_| "acueducto_hato".to_string()),
                username: env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string()),
                password: env::var("DB_PASSWORD").expect("DB_PASSWORD must be set in production"),
                schema: env::var("DB_SCHEMA").unwrap_or_else(|_| "personal".to_string()),
                max_connections: env::var("DB_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "20".to_string())
                    .parse()
                    .unwrap_or(20),
            },
            storage: StorageConfig {
                endpoint: env::var("STORAGE_ENDPOINT").expect("STORAGE_ENDPOINT must be set in production"),
                tenant_id: env::var("STORAGE_TENANT_ID").expect("STORAGE_TENANT_ID must be set in production"),
                access_key: env::var("CONTABO_ACCESS_KEY").expect("CONTABO_ACCESS_KEY must be set in production"),
                secret_key: env::var("CONTABO_SECRET_KEY").expect("CONTABO_SECRET_KEY must be set in production"),
                region: "default".to_string(),
                bucket_proyectos: env::var("STORAGE_BUCKET_PROYECTOS").unwrap_or_else(|_| "apartamento-paipa-sochagota-25".to_string()),
                bucket_documentos: env::var("STORAGE_BUCKET_DOCUMENTOS").unwrap_or_else(|_| "important-document".to_string()),
                bucket_gastos: env::var("STORAGE_BUCKET_GASTOS").unwrap_or_else(|_| "mas-finance-gastos".to_string()),
            },
            log_level: "info".to_string(),
        }
    }

    pub fn database_url(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.database.username,
            self.database.password,
            self.database.host,
            self.database.port,
            self.database.database
        )
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
}