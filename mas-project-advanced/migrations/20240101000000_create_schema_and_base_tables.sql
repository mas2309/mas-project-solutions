-- Crear schema personal si no existe
CREATE SCHEMA IF NOT EXISTS personal;

-- Crear tablas base que se asumen existentes (del sistema legacy)
CREATE TABLE IF NOT EXISTS personal.usuarios (
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(255),
    email VARCHAR(255),
    nombre_completo VARCHAR(255),
    password VARCHAR(255),
    rol VARCHAR(50) DEFAULT 'user',
    activo BOOLEAN DEFAULT true,
    fecha_creacion TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    fecha_actualizacion TIMESTAMP,
    ultimo_acceso TIMESTAMP,
    failed_login_attempts INTEGER DEFAULT 0,
    lockout_end_time TIMESTAMP
);

CREATE TABLE IF NOT EXISTS personal.pagos (
    id BIGSERIAL PRIMARY KEY,
    descripcion VARCHAR(255),
    valor NUMERIC(15,2),
    saldo NUMERIC(15,2),
    estado VARCHAR(50) DEFAULT 'Pendiente',
    mes VARCHAR(20),
    anio VARCHAR(10),
    evidencia VARCHAR(500),
    evidencia_constructora VARCHAR(500),
    fecha_creacion TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    fecha_actualizacion TIMESTAMP
);

CREATE TABLE IF NOT EXISTS personal.archivos (
    id BIGSERIAL PRIMARY KEY,
    nombre_archivo VARCHAR(255),
    nombre_original VARCHAR(255),
    ruta VARCHAR(500),
    tipo_archivo VARCHAR(100),
    tipo_contenido VARCHAR(100),
    tamanio BIGINT,
    fecha_subida TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    pago_id BIGINT REFERENCES personal.pagos(id),
    usuario_id BIGINT REFERENCES personal.usuarios(id)
);
