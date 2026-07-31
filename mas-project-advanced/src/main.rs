use mas_project_advanced::domain::entities::*;
use mas_project_advanced::shared::config::*;
use mas_project_advanced::infrastructure::database::*;
use mas_project_advanced::application::dto::*;
use mas_project_advanced::presentation::web::*;
use mas_project_advanced::application::repositories::proyecto_repository::IProyectoRepository;
use mas_project_advanced::application::repositories::pago_repository::IPagoRepository;
use rust_decimal::Decimal;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    println!("🚀 Sistema de Gestión Financiera Personal y Proyectos");
    println!("📋 Fase 1B: Configuración por Ambiente y Base de Datos");
    
    // Cargar configuración
    let config = AppConfig::load();
    println!("\n🌍 Ambiente: {:?}", config.environment);
    println!("🌐 Servidor: {}", config.server_address());
    println!("📊 Base de datos: {}", config.database_url());
    
    // Demo sin base de datos
    demo_existing_entities();
    
    // Test de conexión a base de datos
    println!("\n🔌 Probando conexión a PostgreSQL...");
    match DatabaseManager::test_connection(&config).await {
        Ok(true) => {
            println!("✅ ¡Conexión exitosa a PostgreSQL!");
            
            // Demo con base de datos real
            if let Err(e) = demo_database_connection(&config).await {
                println!("⚠️ Error en demo de BD: {}", e);
            }
            
            // Iniciar servidor web
            println!("\n🌐 Iniciando servidor web en {}", config.server_address());
            if let Err(e) = start_web_server(&config).await {
                println!("⚠️ Error iniciando servidor: {}", e);
            }
        }
        Ok(false) => println!("⚠️ Test de conexión falló"),
        Err(e) => {
            println!("⚠️ Error de conexión: {}", e);
            println!("📝 Verifica que PostgreSQL esté ejecutándose");
            println!("📝 Credenciales: postgres/Mas23@localhost:5432");
        }
    }
}

fn demo_existing_entities() {
    // Configuración de base de datos
    let config = AppConfig::load();
    println!("\n📊 Configuración de BD: {}", config.database_url());
    
    // Crear un usuario
    let usuario = Usuario::new(
        "juan_perez".to_string(),
        "juan@email.com".to_string(),
        "Juan Pérez".to_string(),
        "password_hash".to_string(),
        "cliente".to_string(),
    );
    
    println!("\n✅ Usuario creado: {} ({})", usuario.nombre_completo, usuario.email);
    println!("   🔒 Activo: {}, Intentos fallidos: {}", usuario.is_active(), usuario.failed_login_attempts);
    
    // Crear un pago
    let mut pago = PagoExistente::new(
        "Pago cuota enero - Proyecto Web".to_string(),
        Decimal::from(10000), // $10,000
        "enero".to_string(),
        "2024".to_string(),
        None, // Sin proyecto asociado en el demo
    );
    
    println!("\n✅ Pago creado: {}", pago.descripcion);
    println!("   💰 Valor: ${}, Saldo: ${}", pago.valor, pago.saldo.unwrap_or(Decimal::ZERO));
    println!("   📅 Estado: {}", pago.estado.to_string());
    
    // Registrar un pago parcial
    pago.registrar_pago(Decimal::from(6000)); // Pagar $6,000
    println!("\n💳 Pago parcial registrado:");
    println!("   💰 Nuevo saldo: ${}", pago.saldo.unwrap_or(Decimal::ZERO));
    println!("   📈 Progreso: {:.1}%", pago.porcentaje_pagado());
    println!("   📅 Estado: {}", pago.estado.to_string());
    
    // Crear un archivo de evidencia
    let archivo = Archivo::new(
        "evidencia_pago_enero_2024.pdf".to_string(),
        "Comprobante Pago Enero.pdf".to_string(),
        "/uploads/evidencias/evidencia_pago_enero_2024.pdf".to_string(),
        "application/pdf".to_string(),
        1024000, // 1MB
        usuario.id as i32,
        Some(pago.id),
    );
    
    println!("\n📄 Archivo de evidencia creado:");
    println!("   📝 Nombre: {}", archivo.nombre_original);
    println!("   💾 Tamaño: {:.2} MB", archivo.tamanio_mb());
    println!("   📁 Tipo: {}", archivo.tipo_archivo.as_ref().unwrap_or(&"desconocido".to_string()));
    println!("   🔗 Asociado a pago: {}", archivo.esta_asociado_a_pago());
    
    // Completar el pago
    pago.registrar_pago(Decimal::from(4000)); // Pagar los $4,000 restantes
    pago.agregar_evidencia(archivo.ruta.clone());
    
    println!("\n✅ Pago completado:");
    println!("   💰 Saldo final: ${}", pago.saldo.unwrap_or(Decimal::ZERO));
    println!("   📈 Progreso: {:.1}%", pago.porcentaje_pagado());
    println!("   📅 Estado: {}", pago.estado.to_string());
    println!("   📄 Evidencia: {}", pago.evidencia.as_ref().unwrap_or(&"Sin evidencia".to_string()));
    
    println!("\n🎯 Entidades adaptadas al esquema existente correctamente!");
    println!("📊 Listo para conectar con PostgreSQL: {}", config.database.database);
}

#[allow(dead_code)]
async fn demo_database_connection(config: &AppConfig) -> anyhow::Result<()> {
    println!("\n🔌 Ejecutando demo con base de datos real...");
    
    let pool = DatabaseManager::create_pool(config).await?;
    
    // Test de repositorios
    let usuario_repo = UsuarioRepository::new(pool.clone());
    let pago_repo = PagoRepository::new(pool.clone());
    let proyecto_repo = ProyectoRepository::new(pool.clone());
    
    // Crear usuario de prueba
    let create_usuario_dto = CreateUsuarioDto {
        username: format!("test_user_{}", chrono::Utc::now().timestamp()),
        email: format!("test{}@example.com", chrono::Utc::now().timestamp()),
        nombre_completo: "Usuario de Prueba Rust".to_string(),
        password: "hashed_password_123".to_string(),
        rol: "cliente".to_string(),
    };
    
    match usuario_repo.create(create_usuario_dto).await {
        Ok(usuario) => {
            println!("✅ Usuario creado en BD: {} (ID: {})", usuario.nombre_completo, usuario.id);
            
            // Crear pago de prueba
            let create_pago_dto = CreatePagoDto {
                descripcion: "Pago de prueba desde Rust".to_string(),
                valor: rust_decimal::Decimal::from(25000),
                mes: "noviembre".to_string(),
                anio: "2024".to_string(),
                proyecto_id: None,
            };
            
            match pago_repo.create(1_i64, create_pago_dto).await {
                Ok(pago) => {
                    println!("✅ Pago creado en BD: {} (ID: {})", pago.descripcion, pago.id);
                    println!("   💰 Valor: ${}, Estado: {}", pago.valor, pago.estado.to_string());
                    
                    // Registrar pago parcial
                    if let Ok(Some(pago_actualizado)) = pago_repo.registrar_pago(1_i64, pago.id, rust_decimal::Decimal::from(15000)).await {
                        println!("✅ Pago parcial registrado:");
                        println!("   💰 Nuevo saldo: ${}", pago_actualizado.saldo.unwrap_or(rust_decimal::Decimal::ZERO));
                        println!("   📅 Estado: {}", pago_actualizado.estado.to_string());
                    }
                }
                Err(e) => println!("⚠️ Error creando pago: {}", e),
            }
        }
        Err(e) => println!("⚠️ Error creando usuario: {}", e),
    }
    
    // Obtener resumen de pagos
    match pago_repo.get_summary(1_i64, "2024").await {
        Ok(summary) => {
            println!("\n📊 Resumen de Pagos 2024:");
            println!("   📊 Total pagos: {}", summary.total_pagos);
            println!("   💰 Valor total: ${}", summary.total_valor);
            println!("   ⏳ Saldo pendiente: ${}", summary.total_saldo);
            println!("   ✅ Completados: {}", summary.pagos_completados);
            println!("   🔄 Pendientes: {}", summary.pagos_pendientes);
        }
        Err(e) => println!("⚠️ Error obteniendo resumen: {}", e),
    }
    
    // Demo de proyectos - COMENTADO para evitar creación automática
    /*
    let create_proyecto_dto = CreateProyectoDto {
        nombre: "Sistema Web Corporativo".to_string(),
        descripcion: Some("Desarrollo de plataforma web para gestión empresarial".to_string()),
        presupuesto: Some(rust_decimal::Decimal::from(50000)),
        fecha_fin_estimada: None,
        cliente: Some("Empresa ABC".to_string()),
        responsable: Some("Juan Pérez".to_string()),
    };
    
    match proyecto_repo.create(create_proyecto_dto).await {
        Ok(proyecto) => {
            println!("✅ Proyecto creado: {} (ID: {})", proyecto.nombre, proyecto.id);
            println!("   💰 Presupuesto: ${}", proyecto.presupuesto.unwrap_or(rust_decimal::Decimal::ZERO));
            println!("   📅 Estado: {}", proyecto.estado.to_string());
            
            // Cambiar estado a En Progreso
            if let Ok(Some(proyecto_actualizado)) = proyecto_repo.cambiar_estado(proyecto.id, "En_Progreso").await {
                println!("✅ Proyecto iniciado: {}", proyecto_actualizado.estado.to_string());
            }
        }
        Err(e) => println!("⚠️ Error creando proyecto: {}", e),
    }
    */
    
    // Obtener resumen de proyectos
    let proyecto_repo_trait: Arc<dyn IProyectoRepository> = Arc::new(proyecto_repo);
    match proyecto_repo_trait.get_summary(1_i64).await {
        Ok(summary) => {
            println!("\n📊 Resumen de Proyectos:");
            println!("   📊 Total proyectos: {}", summary.total_proyectos);
            println!("   🔄 Proyectos activos: {}", summary.proyectos_activos);
            println!("   ✅ Proyectos completados: {}", summary.proyectos_completados);
            println!("   💰 Presupuesto total: ${}", summary.presupuesto_total);
            println!("   💸 Costo total: ${}", summary.costo_total);
        }
        Err(e) => println!("⚠️ Error obteniendo resumen de proyectos: {}", e),
    }
    
    println!("\n🎯 ¡Demo de base de datos completado exitosamente!");
    Ok(())
}
