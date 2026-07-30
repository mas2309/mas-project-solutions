-- Tabla de Ingresos
CREATE TABLE IF NOT EXISTS personal.ingresos (
    id SERIAL PRIMARY KEY,
    descripcion VARCHAR(255) NOT NULL,
    monto DECIMAL(15,2) NOT NULL,
    categoria VARCHAR(50) NOT NULL DEFAULT 'Otro',
    fuente VARCHAR(255),
    fecha DATE NOT NULL,
    recurrente BOOLEAN DEFAULT false,
    fecha_creacion TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_ingresos_fecha ON personal.ingresos(fecha);
CREATE INDEX IF NOT EXISTS idx_ingresos_categoria ON personal.ingresos(categoria);

-- Tabla de Gastos
CREATE TABLE IF NOT EXISTS personal.gastos (
    id SERIAL PRIMARY KEY,
    descripcion VARCHAR(255) NOT NULL,
    monto DECIMAL(15,2) NOT NULL,
    categoria VARCHAR(50) NOT NULL DEFAULT 'Otro',
    estado VARCHAR(50) DEFAULT 'Pendiente',
    responsable VARCHAR(255),
    soporte TEXT,
    fecha DATE NOT NULL,
    fecha_creacion TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_gastos_fecha ON personal.gastos(fecha);
CREATE INDEX IF NOT EXISTS idx_gastos_categoria ON personal.gastos(categoria);
CREATE INDEX IF NOT EXISTS idx_gastos_estado ON personal.gastos(estado);