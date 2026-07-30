use sqlx::{PgPool, postgres::PgPoolOptions};
use sqlx::migrate::Migrator;
use crate::shared::config::AppConfig;
use anyhow::Result;
use std::path::Path;

pub struct DatabaseManager;

impl DatabaseManager {
    pub async fn create_pool(config: &AppConfig) -> Result<PgPool> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&config.database_url())
            .await?;

        // Ejecutar migraciones automáticamente al iniciar
        Self::run_migrations(&pool).await?;

        Ok(pool)
    }

    /// Ejecuta las migraciones pendientes automáticamente.
    /// Similar a Flyway: solo aplica las que no se han ejecutado.
    async fn run_migrations(pool: &PgPool) -> Result<()> {
        let migrations_path = Path::new("migrations");

        if migrations_path.exists() {
            let migrator = Migrator::new(migrations_path).await?;
            migrator.run(pool).await?;
            println!("✅ Migraciones ejecutadas correctamente");
        } else {
            println!("⚠️  No se encontró carpeta 'migrations/', omitiendo migraciones");
        }

        Ok(())
    }

    pub async fn test_connection(config: &AppConfig) -> Result<bool> {
        let pool = Self::create_pool(config).await?;
        let _row = sqlx::query("SELECT 1").fetch_one(&pool).await?;
        Ok(true)
    }
}