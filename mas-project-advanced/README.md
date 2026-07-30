# 🚀 MAS Finance - Sistema de Gestión Financiera Personal

## 📋 ¿Qué es?

**MAS Finance** es un sistema monolítico desarrollado en **Rust** que permite gestionar las finanzas personales y proyectos de manera integral. Combina una aplicación web tradicional con una API REST que puede ser consumida por aplicaciones móviles.

El sistema centraliza el control de ingresos, gastos, créditos, documentos importantes y proyectos con sus respectivos planes de pago, todo respaldado por almacenamiento de archivos en la nube (AWS S3 / Huawei OBS).

---

## 🌐 Acceso

| Desde | URL |
|-------|-----|
| Navegador (PC) | `http://localhost:8082` |
| App móvil (misma red WiFi) | `http://[IP_LOCAL_PC]:8082/api/v1/` |

---

## 📦 Módulos Implementados

| Módulo | Descripción |
|--------|-------------|
| 📋 **Proyectos** | Gestión de proyectos con plan de pagos mensual, evidencias y control de presupuesto |
| 💵 **Ingresos** | Registro de fuentes de ingreso por categoría y recurrencia |
| 💸 **Gastos** | Control de egresos con soporte de archivos y estados |
| 🏦 **Créditos** | Seguimiento de deudas, cuotas y amortización automática |
| 📁 **Documentos** | Repositorio de archivos importantes (contratos, pólizas, escrituras, etc.) |
| 🔌 **API REST** | 22 endpoints JSON bajo `/api/v1/` para consumo desde app móvil |

---

## 🏗️ Arquitectura

El proyecto sigue una **arquitectura limpia (Clean Architecture)** con separación estricta de responsabilidades por capas:

```
src/
├── domain/                     # Núcleo del negocio (sin dependencias externas)
│   └── entities/
│       ├── proyecto.rs         # Proyecto con estados y presupuesto
│       ├── pago_existente.rs   # Pago con estados y saldos
│       ├── ingreso.rs          # Ingreso con categorías
│       ├── gasto.rs            # Gasto con estados y soporte
│       ├── credito.rs          # Crédito con amortización
│       ├── documento.rs        # Documento con categorías
│       ├── usuario.rs          # Usuario con autenticación
│       └── archivo.rs          # Archivo de evidencia
│
├── application/                # Casos de uso y contratos
│   ├── dto/                    # Objetos de transferencia de datos
│   ├── repositories/           # Traits (contratos) de repositorios
│   └── services/               # Lógica de negocio
│       ├── proyecto_service.rs
│       ├── pago_service.rs
│       ├── ingreso_service.rs
│       ├── gasto_service.rs
│       ├── credito_service.rs
│       ├── documento_service.rs
│       └── storage_service.rs  # Contrato de almacenamiento
│
├── infrastructure/             # Implementaciones externas
│   ├── database/               # Repositorios PostgreSQL con SQLx
│   │   ├── proyecto_repository.rs
│   │   ├── pago_repository.rs
│   │   ├── ingreso_repository.rs
│   │   ├── gasto_repository.rs
│   │   ├── credito_repository.rs
│   │   └── documento_repository.rs
│   └── storage/
│       └── s3_storage_service.rs  # Implementación S3 / OBS Huawei
│
├── presentation/               # Capa de presentación
│   ├── web/                    # Aplicación web
│   │   ├── handlers.rs         # Controladores HTTP (vistas HTML)
│   │   ├── templates.rs        # Structs de templates Askama
│   │   ├── server.rs           # Router y AppState
│   │   └── start_server.rs     # Inicialización del servidor
│   └── api/                    # API REST
│       ├── handlers.rs         # Controladores HTTP (JSON)
│       └── routes.rs           # Rutas API bajo /api/v1/
│
└── shared/
    └── config/
        └── app_config.rs       # Configuración por ambiente
```

### Principios Aplicados

- **Inversión de dependencias**: Los servicios dependen de traits, no de implementaciones concretas
- **Separación de responsabilidades**: Cada capa tiene una responsabilidad única
- **Inyección de dependencias**: Vía `Arc<dyn Trait>` en Rust
- **Arquitectura monolítica**: Apropiada para el tamaño y alcance del proyecto

---

## 🛠️ Stack Tecnológico

| Componente | Tecnología | Versión |
|------------|------------|---------|
| Lenguaje | Rust | 2021 edition |
| Web Framework | Axum | 0.7 |
| Templates HTML | Askama | 0.12 |
| Base de datos | PostgreSQL | - |
| ORM / Query | SQLx | 0.7 |
| Storage | AWS S3 / Huawei OBS | - |
| Serialización | Serde JSON | 1.0 |
| Decimales financieros | rust_decimal | 1.0 |
| CORS | tower-http | 0.5 |
| Slugify | slug | 0.1 |
| Async runtime | Tokio | 1.0 |

---

## 🗄️ Base de Datos

**Conexión**: `postgresql://postgres:****@localhost:5432/acueducto_hato`
**Schema**: `personal`

### Tablas

| Tabla | Descripción |
|-------|-------------|
| `personal.proyectos` | Proyectos con presupuesto y estados |
| `personal.pagos` | Pagos asociados a proyectos con evidencias |
| `personal.ingresos` | Registro de ingresos |
| `personal.gastos` | Registro de gastos con soporte |
| `personal.creditos` | Créditos con amortización |
| `personal.documentos` | Documentos importantes |
| `personal.usuarios` | Usuarios del sistema |

### Migraciones

```
migrations/
├── 001_create_proyectos_table.sql
├── 002_add_proyecto_id_to_pagos.sql
├── 003_create_ingresos_gastos.sql
└── 004_create_creditos_documentos.sql
```

---

## 🔌 API REST

Base URL: `http://[HOST]:8082/api/v1`

### Formato de Respuesta

```json
{
  "success": true,
  "data": { ... },
  "message": null
}
```

### Endpoints Disponibles

#### 📊 Dashboard
| Método | Endpoint | Descripción |
|--------|----------|-------------|
| GET | `/dashboard` | Resumen financiero general |

#### 💵 Ingresos
| Método | Endpoint | Descripción |
|--------|----------|-------------|
| GET | `/ingresos` | Listar con paginación |
| GET | `/ingresos/:id` | Obtener por ID |
| POST | `/ingresos` | Crear |
| PUT | `/ingresos/:id` | Actualizar |
| DELETE | `/ingresos/:id` | Eliminar |

#### 💸 Gastos
| Método | Endpoint | Descripción |
|--------|----------|-------------|
| GET | `/gastos` | Listar con paginación |
| GET | `/gastos/:id` | Obtener por ID |
| POST | `/gastos` | Crear |
| PUT | `/gastos/:id` | Actualizar |
| POST | `/gastos/:id/pagado` | Marcar como pagado |
| DELETE | `/gastos/:id` | Eliminar |

#### 🏦 Créditos
| Método | Endpoint | Descripción |
|--------|----------|-------------|
| GET | `/creditos` | Listar con paginación |
| GET | `/creditos/:id` | Obtener por ID |
| POST | `/creditos` | Crear |
| PUT | `/creditos/:id` | Actualizar |
| POST | `/creditos/:id/cuota` | Registrar cuota pagada |
| DELETE | `/creditos/:id` | Eliminar |

#### 📁 Documentos
| Método | Endpoint | Descripción |
|--------|----------|-------------|
| GET | `/documentos` | Listar con paginación |
| DELETE | `/documentos/:id` | Eliminar |

#### 📋 Proyectos
| Método | Endpoint | Descripción |
|--------|----------|-------------|
| GET | `/proyectos` | Listar todos |
| GET | `/proyectos/:id` | Obtener por ID |
| GET | `/proyectos/:id/pagos` | Pagos del proyecto |

> 📄 La colección Postman está disponible en `docs/MAS_Finance_API_v1.postman_collection.json`

---

## 🚀 Ejecutar el Proyecto

### Requisitos
- Rust (2021 edition)
- PostgreSQL corriendo en `localhost:5432`
- Credenciales AWS S3 o Huawei OBS configuradas

### Pasos

```bash
# 1. Clonar el repositorio
git clone [repo-url]
cd mas-project-advanced

# 2. Ejecutar migraciones SQL
psql -h localhost -U postgres -d acueducto_hato -f migrations/001_create_proyectos_table.sql
psql -h localhost -U postgres -d acueducto_hato -f migrations/002_add_proyecto_id_to_pagos.sql
psql -h localhost -U postgres -d acueducto_hato -f migrations/003_create_ingresos_gastos.sql
psql -h localhost -U postgres -d acueducto_hato -f migrations/004_create_creditos_documentos.sql

# 3. Configurar variables de entorno (opcional)
DATABASE_URL=postgresql://postgres:****@localhost:5432/acueducto_hato
AWS_S3_BUCKET=nombre-del-bucket
AWS_REGION=us-east-1

# 4. Ejecutar
cargo run
```

El servidor inicia en `http://0.0.0.0:8082`

---

## 📅 Plan de Desarrollo

### ✅ Fase 1 - Base del Proyecto (Completada)
- [x] Entidades de dominio
- [x] Conexión a PostgreSQL con SQLx
- [x] Repositorios base con tipos correctos (i32)
- [x] Configuración por ambiente (dev/test/prod)

### ✅ Fase 2 - Módulos Financieros (Completada)
- [x] Proyectos con plan de pagos mensual
- [x] Control de presupuesto en pagos
- [x] Ingresos con categorías y recurrencia
- [x] Gastos con soporte de archivos
- [x] Créditos con amortización automática
- [x] Documentos importantes con upload
- [x] Integración S3 / Huawei OBS
- [x] Nombres legibles en bucket (slug + timestamp)
- [x] Formato de miles en valores monetarios
- [x] Modal de confirmación personalizado

### ✅ Fase 3 - API REST (Completada)
- [x] 22 endpoints JSON bajo `/api/v1/`
- [x] CORS habilitado para app móvil
- [x] Servidor en `0.0.0.0` para acceso desde red local
- [x] Colección Postman documentada
- [x] Respuestas estandarizadas `{ success, data, message }`

### 🔄 Fase 4 - Dashboard (Pendiente)
- [ ] Vista consolidada de finanzas
- [ ] Balance general (ingresos - gastos)
- [ ] Gráficos de ingresos vs gastos por mes
- [ ] Resumen de deuda total (créditos)
- [ ] Alertas de documentos próximos a vencer
- [ ] Endpoint API `/dashboard` con métricas completas

### 🔄 Fase 5 - Autenticación (Pendiente)
- [ ] Login / Logout en la web
- [ ] JWT para proteger la API REST
- [ ] Roles de usuario (admin, viewer)
- [ ] Protección de rutas web y API
- [ ] Refresh tokens

### 🔄 Fase 6 - Mejoras UX (Pendiente)
- [ ] Filtros por fecha, categoría y estado en tablas
- [ ] Búsqueda en tiempo real
- [ ] Paginación visual en tablas
- [ ] Exportar a Excel / PDF
- [ ] Notificaciones de vencimientos de créditos y documentos
- [ ] Presupuestos mensuales por categoría
- [ ] Metas de ahorro con seguimiento de progreso

### 🔄 Fase 7 - App Móvil (Pendiente)
- [ ] Consumo de API REST desde app móvil
- [ ] Autenticación JWT en app
- [ ] Vistas de dashboard, ingresos, gastos y créditos
- [ ] Upload de archivos desde móvil

---

## 📁 Estructura de Archivos en Bucket

```
bucket/
├── documentos/          # nombre-documento-YYYYMMDDHHMMSS.ext
├── evidencia-pagos/     # pago-{id}-YYYYMMDDHHMMSS.ext
├── evidencia-constructora/ # pago-{id}-YYYYMMDDHHMMSS.ext
└── soportes-gastos/     # gasto-{id}-YYYYMMDDHHMMSS.ext
```

---

## 🤝 Contribución

Proyecto en desarrollo activo. Las fases pendientes están priorizadas en el orden listado arriba.

---

*Desarrollado con ❤️ en Rust*
