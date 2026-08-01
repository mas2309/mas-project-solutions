use mas_project_advanced::shared::config::*;
use mas_project_advanced::presentation::web::*;

#[tokio::main]
async fn main() {
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
