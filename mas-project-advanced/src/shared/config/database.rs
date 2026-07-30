use crate::shared::config::app_config::AppConfig;
use sqlx::PgPool;
use anyhow::Result;

pub struct DatabaseManager;

impl DatabaseManager {
    pub async fn create_pool(config: &AppConfig) -> Result<PgPool> {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(config.database.max_connections)
            .connect(&config.database_url())
            .await?;
        
        // Set schema
        sqlx::query(&format!("SET search_path TO {}", config.database.schema))
            .execute(&pool)
            .await?;
            
        println!("✅ Conexión a BD establecida: {}@{}/{}", 
                 config.database.username, 
                 config.database.host, 
                 config.database.database);
        println!("📁 Schema: {}", config.database.schema);
        println!("🔗 Max conexiones: {}", config.database.max_connections);
            
        Ok(pool)
    }
    
    pub async fn test_connection(config: &AppConfig) -> Result<bool> {
        let pool = Self::create_pool(config).await?;
        
        // Test query
        let row: (i64,) = sqlx::query_as("SELECT 1")
            .fetch_one(&pool)
            .await?;
            
        Ok(row.0 == 1)
    }
}