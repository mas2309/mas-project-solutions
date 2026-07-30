-- Crear tabla proyectos en el schema personal
CREATE TABLE IF NOT EXISTS personal.proyectos (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR(255) NOT NULL,
    descripcion TEXT,
    presupuesto DECIMAL(15,2),
    costo_actual DECIMAL(15,2) DEFAULT 0.00,
    estado VARCHAR(50) DEFAULT 'Planificacion',
    fecha_inicio TIMESTAMP,
    fecha_fin_estimada TIMESTAMP,
    fecha_fin_real TIMESTAMP,
    cliente VARCHAR(255),
    responsable VARCHAR(255),
    fecha_creacion TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    fecha_actualizacion TIMESTAMP
);

-- Crear índices para mejorar rendimiento
CREATE INDEX IF NOT EXISTS idx_proyectos_escletado ON personal.proyectos(estado);
CREATE INDEX IF NOT EXISTS idx_proyectos_cliente ON personal.proyectos(cliente);
CREATE INDEX IF NOT EXISTS idx_proyectos_fecha_creacion ON personal.proyectos(fecha_creacion);

-- Agregar restricciones (idempotente)
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'chk_estado') THEN
        ALTER TABLE personal.proyectos 
        ADD CONSTRAINT chk_estado CHECK (estado IN ('Planificacion', 'En_Progreso', 'Pausado', 'Completado', 'Cancelado'));
    END IF;
END $$;

-- Comentarios para documentación
COMMENT ON TABLE personal.proyectos IS 'Tabla para gestionar proyectos del sistema';
COMMENT ON COLUMN personal.proyectos.estado IS 'Estados: Planificacion, En_Progreso, Pausado, Completado, Cancelado';