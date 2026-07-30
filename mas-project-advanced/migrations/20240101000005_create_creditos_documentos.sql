-- Tabla de Créditos
CREATE TABLE IF NOT EXISTS personal.creditos (
    id SERIAL PRIMARY KEY,
    entidad VARCHAR(255) NOT NULL,
    descripcion VARCHAR(255) NOT NULL,
    monto_total DECIMAL(15,2) NOT NULL,
    saldo_pendiente DECIMAL(15,2) NOT NULL,
    tasa_interes DECIMAL(5,2) NOT NULL DEFAULT 0,
    cuotas_totales INTEGER NOT NULL,
    cuotas_pagadas INTEGER NOT NULL DEFAULT 0,
    valor_cuota DECIMAL(15,2) NOT NULL,
    estado VARCHAR(50) DEFAULT 'Activo',
    fecha_inicio DATE NOT NULL,
    fecha_fin_estimada DATE,
    fecha_creacion TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_creditos_estado ON personal.creditos(estado);
CREATE INDEX IF NOT EXISTS idx_creditos_entidad ON personal.creditos(entidad);

-- Tabla de Documentos
CREATE TABLE IF NOT EXISTS personal.documentos (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR(255) NOT NULL,
    descripcion TEXT,
    categoria VARCHAR(50) NOT NULL DEFAULT 'Otro',
    archivo_url TEXT NOT NULL,
    nombre_archivo VARCHAR(255) NOT NULL,
    fecha_vencimiento DATE,
    fecha_creacion TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_documentos_categoria ON personal.documentos(categoria);
CREATE INDEX IF NOT EXISTS idx_documentos_vencimiento ON personal.documentos(fecha_vencimiento);