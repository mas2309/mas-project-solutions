# Sistema de Gestión Financiera Personal y Proyectos

## 📋 Descripción del Proyecto

Sistema completo de gestión financiera personal desarrollado en Rust, que incluye:
- Gestión de proyectos con planes de pago
- Control financiero personal
- Dashboard con indicadores y métricas
- Integración con AWS S3 para documentos
- Arquitectura moderna en capas

## 🏗️ Arquitectura

### Capas del Sistema
```
┌─────────────────────────────────────────┐
│           Presentation Layer            │
│         (API REST + Web UI)            │
├─────────────────────────────────────────┤
│           Application Layer             │
│     (Use Cases & Business Logic)       │
├─────────────────────────────────────────┤
│            Domain Layer                 │
│    (Entities, Value Objects, Rules)    │
├─────────────────────────────────────────┤
│         Infrastructure Layer            │
│  (Database, S3, External Services)     │
└─────────────────────────────────────────┘
```

## 📦 Módulos Principales

### 1. Gestión de Proyectos
- **Proyectos**: Creación y gestión con valor total
- **Clientes**: Información y contacto
- **Planes de Pago**: Cronogramas personalizables
- **Pagos**: Registro con evidencias en S3
- **Dashboard**: Seguimiento de pagos y proyectos

### 2. Gestión Financiera Personal
- **Cuentas**: Bancos, tarjetas, efectivo
- **Transacciones**: Ingresos, gastos, transferencias
- **Categorías**: Clasificación automática y manual
- **Presupuestos**: Planificación y control
- **Reportes**: Análisis y tendencias

## 🚀 Plan de Desarrollo

### Fase 1: Gestión de Proyectos (Actual)
- [x] Configuración base del proyecto
- [x] Estructura de directorios
- [ ] Entidades de dominio
- [ ] Casos de uso básicos
- [ ] API REST
- [ ] Integración con base de datos

### Fase 2: Core Financiero
- [ ] Entidades financieras
- [ ] Sistema de transacciones
- [ ] Categorización

### Fase 3: Dashboard e Indicadores
- [ ] Métricas y KPIs
- [ ] Visualizaciones
- [ ] Reportes

## 🛠️ Stack Tecnológico

- **Backend**: Rust + Axum
- **Base de Datos**: PostgreSQL + SQLx
- **Storage**: AWS S3
- **Autenticación**: JWT
- **Validación**: Validator
- **Cálculos**: rust_decimal

## 📁 Estructura del Proyecto

```
src/
├── main.rs                 # Punto de entrada
├── lib.rs                  # Biblioteca principal
├── domain/                 # Lógica de negocio
│   ├── entities/          # Entidades del dominio
│   ├── value_objects/     # Objetos de valor
│   └── services/          # Servicios de dominio
├── application/           # Casos de uso
│   ├── use_cases/        # Lógica de aplicación
│   └── dto/              # Data Transfer Objects
├── infrastructure/       # Implementaciones técnicas
│   ├── database/         # Acceso a datos
│   └── storage/          # Almacenamiento S3
├── presentation/         # Capa de presentación
│   └── api/              # Endpoints REST
└── shared/               # Utilidades compartidas
    └── config/           # Configuración
```

## 🔧 Configuración

1. Instalar Rust: https://rustup.rs/
2. Configurar PostgreSQL
3. Configurar AWS S3
4. Copiar `.env.example` a `.env`
5. Ejecutar migraciones: `sqlx migrate run`
6. Iniciar servidor: `cargo run`

## 📝 Documentación por Fases

- [Fase 1: Gestión de Proyectos](./fase-1-proyectos.md)
- [Fase 2: Core Financiero](./fase-2-financiero.md)
- [Fase 3: Dashboard](./fase-3-dashboard.md)