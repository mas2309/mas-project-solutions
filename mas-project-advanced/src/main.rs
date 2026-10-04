use mas_project_advanced::shared::config::*;
use mas_project_advanced::presentation::web::*;

#[tokio::main]
async fn main() {
    // Cargar variables de entorno: primero el archivo del ambiente, luego .env como respaldo.
    // dotenvy no sobrescribe variables ya definidas, así que las del sistema/IDE tienen prioridad.
    let environment = std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
    dotenvy::from_filename(format!(".env.{}", environment)).ok();
    dotenvy::dotenv().ok();

    let config = AppConfig::load();
    
    println!("🚀 MAS Finance - Sistema de Gestión Financiera Personal");
    println!("🌍 Ambiente: {:?}", config.environment);
    println!("🌐 Servidor: {}", config.server_address());

    // Iniciar servidor web (incluye conexión a BD, migraciones y creación de admin)
    if let Err(e) = start_web_server(&config).await {
        eprintln!("❌ Error fatal al iniciar el servidor: {}", e);
        std::process::exit(1);
    }
}
