# Fase 1: Gestión de Proyectos

## 🎯 Objetivos

Implementar el módulo completo de gestión de proyectos que incluye:
- Entidades de dominio para proyectos, clientes y pagos
- Casos de uso para CRUD y lógica de negocio
- API REST para todas las operaciones
- Integración con PostgreSQL
- Upload de archivos a AWS S3

## 📋 Entidades del Dominio

### 1. Project (Proyecto)
```rust
pub struct Project {
    pub id: ProjectId,
    pub name: String,
    pub description: Option<String>,
    pub client_id: ClientId,
    pub total_value: Money,
    pub status: ProjectStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### 2. Client (Cliente)
```rust
pub struct Client {
    pub id: ClientId,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub created_at: DateTime<Utc>,
}
```

### 3. PaymentPlan (Plan de Pagos)
```rust
pub struct PaymentPlan {
    pub id: PaymentPlanId,
    pub project_id: ProjectId,
    pub installments: Vec<Installment>,
    pub created_at: DateTime<Utc>,
}
```

### 4. Payment (Pago)
```rust
pub struct Payment {
    pub id: PaymentId,
    pub installment_id: InstallmentId,
    pub amount: Money,
    pub payment_date: DateTime<Utc>,
    pub evidence_files: Vec<FileReference>,
    pub notes: Option<String>,
}
```

## 🔧 Casos de Uso

### Proyectos
- `CreateProject`: Crear nuevo proyecto
- `UpdateProject`: Actualizar proyecto existente
- `GetProject`: Obtener proyecto por ID
- `ListProjects`: Listar proyectos con filtros
- `DeleteProject`: Eliminar proyecto

### Clientes
- `CreateClient`: Registrar nuevo cliente
- `UpdateClient`: Actualizar información del cliente
- `GetClient`: Obtener cliente por ID
- `ListClients`: Listar clientes

### Planes de Pago
- `CreatePaymentPlan`: Crear plan de pagos para proyecto
- `UpdatePaymentPlan`: Modificar plan existente
- `GetPaymentPlan`: Obtener plan por proyecto

### Pagos
- `RegisterPayment`: Registrar nuevo pago
- `UploadPaymentEvidence`: Subir evidencia a S3
- `GetPaymentHistory`: Historial de pagos
- `GetProjectPaymentStatus`: Estado de pagos del proyecto

## 🗄️ Modelo de Base de Datos

### Tablas Principales

```sql
-- Clientes
CREATE TABLE clients (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR NOT NULL,
    email VARCHAR UNIQUE NOT NULL,
    phone VARCHAR,
    address TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Proyectos
CREATE TABLE projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR NOT NULL,
    description TEXT,
    client_id UUID NOT NULL REFERENCES clients(id),
    total_value DECIMAL(15,2) NOT NULL,
    status VARCHAR NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Planes de pago
CREATE TABLE payment_plans (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Cuotas del plan
CREATE TABLE installments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    payment_plan_id UUID NOT NULL REFERENCES payment_plans(id),
    installment_number INTEGER NOT NULL,
    amount DECIMAL(15,2) NOT NULL,
    due_date DATE NOT NULL,
    status VARCHAR NOT NULL DEFAULT 'pending'
);

-- Pagos realizados
CREATE TABLE payments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    installment_id UUID NOT NULL REFERENCES installments(id),
    amount DECIMAL(15,2) NOT NULL,
    payment_date TIMESTAMPTZ NOT NULL,
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Referencias a archivos en S3
CREATE TABLE file_references (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    payment_id UUID NOT NULL REFERENCES payments(id),
    file_name VARCHAR NOT NULL,
    s3_key VARCHAR NOT NULL,
    file_size BIGINT,
    content_type VARCHAR,
    uploaded_at TIMESTAMPTZ DEFAULT NOW()
);
```

## 🌐 API Endpoints

### Proyectos
- `POST /api/projects` - Crear proyecto
- `GET /api/projects` - Listar proyectos
- `GET /api/projects/{id}` - Obtener proyecto
- `PUT /api/projects/{id}` - Actualizar proyecto
- `DELETE /api/projects/{id}` - Eliminar proyecto

### Clientes
- `POST /api/clients` - Crear cliente
- `GET /api/clients` - Listar clientes
- `GET /api/clients/{id}` - Obtener cliente
- `PUT /api/clients/{id}` - Actualizar cliente

### Pagos
- `POST /api/projects/{id}/payments` - Registrar pago
- `POST /api/payments/{id}/evidence` - Subir evidencia
- `GET /api/projects/{id}/payment-status` - Estado de pagos

## 📊 Dashboard Inicial

### Métricas Clave
- Total de proyectos activos
- Valor total en proyectos
- Pagos pendientes
- Pagos del mes actual
- Proyectos por estado

### Visualizaciones
- Gráfico de pagos por mes
- Estado de proyectos (pie chart)
- Timeline de pagos pendientes
- Top clientes por valor

## ✅ Checklist de Implementación

### Dominio
- [ ] Entidades principales
- [ ] Value Objects (Money, ProjectId, etc.)
- [ ] Enums (ProjectStatus, PaymentStatus)
- [ ] Servicios de dominio

### Aplicación
- [ ] DTOs para requests/responses
- [ ] Casos de uso principales
- [ ] Validaciones de negocio

### Infraestructura
- [ ] Repositorios con SQLx
- [ ] Servicio de S3
- [ ] Migraciones de base de datos

### Presentación
- [ ] Controladores REST
- [ ] Middleware de autenticación
- [ ] Manejo de errores

### Testing
- [ ] Tests unitarios de dominio
- [ ] Tests de integración
- [ ] Tests de API

## 🔄 Próximos Pasos

1. Implementar entidades de dominio
2. Crear migraciones de base de datos
3. Desarrollar repositorios
4. Implementar casos de uso
5. Crear API REST
6. Integrar S3 para archivos
7. Desarrollar dashboard básico

---

**Estado Actual**: 🟡 En Desarrollo
**Fecha de Inicio**: [Fecha actual]
**Estimación**: 2-3 semanas