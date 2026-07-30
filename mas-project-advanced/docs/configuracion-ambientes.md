# 🌍 Configuración por Ambientes

## 📋 **Ambientes Disponibles**

### 🔧 **Development (Desarrollo)**
- Base de datos: `acueducto_hato`
- Puerto: `8080`
- Host: `127.0.0.1`
- Log level: `debug`
- Max conexiones BD: `10`

### 🧪 **Testing (Pruebas)**
- Base de datos: `acueducto_hato_test`
- Puerto: `8081`
- Host: `127.0.0.1`
- Log level: `info`
- Max conexiones BD: `5`

### 🚀 **Production (Producción)**
- Base de datos: Variables de entorno
- Puerto: Variables de entorno
- Host: `0.0.0.0`
- Log level: `info`
- Max conexiones BD: `20`

## 🔧 **Configuración**

### **Cambiar Ambiente**
```bash
# Windows
scripts\set-env.bat development
scripts\set-env.bat testing
scripts\set-env.bat production

# O manualmente
copy .env.development .env
```

### **Variables de Entorno**

#### **Desarrollo**
```env
ENVIRONMENT=development
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
DB_HOST=localhost
DB_PORT=5432
DB_NAME=acueducto_hato
DB_USER=postgres
DB_PASSWORD=Mas23
DB_SCHEMA=personal
```

#### **Producción**
```env
ENVIRONMENT=production
DB_HOST=your-production-host
DB_PASSWORD=your-secure-password
JWT_SECRET=your-super-secure-secret
```

## 🏗️ **Estructura de Configuración**

```rust
AppConfig {
    environment: Environment,
    server: ServerConfig {
        host: String,
        port: u16,
    },
    database: DatabaseConfig {
        host: String,
        port: u16,
        database: String,
        username: String,
        password: String,
        schema: String,
        max_connections: u32,
    },
    log_level: String,
}
```

## 🚀 **Uso en Código**

```rust
// Cargar configuración automáticamente
let config = AppConfig::load();

// Crear pool de conexiones
let pool = DatabaseManager::create_pool(&config).await?;

// Test de conexión
let is_connected = DatabaseManager::test_connection(&config).await?;
```

## 📊 **Características**

### ✅ **Implementado**
- Configuración automática por ambiente
- Variables de entorno por ambiente
- Pool de conexiones configurables
- Test de conexión automático
- Scripts de cambio de ambiente

### 🔄 **Beneficios**
- **Seguridad**: Credenciales separadas por ambiente
- **Flexibilidad**: Configuración específica por ambiente
- **Mantenibilidad**: Cambio fácil entre ambientes
- **Escalabilidad**: Configuración optimizada por ambiente

## 🎯 **Comandos Útiles**

```bash
# Configurar desarrollo
scripts\set-env.bat development
cargo run

# Configurar testing
scripts\set-env.bat testing
cargo test

# Ver configuración actual
cargo run
```