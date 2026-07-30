-- Tabla de gastos recurrentes (plantillas)
-- Tipos: 'Fijo' (valor exacto cada mes), 'FijoVariable' (valor referencia, ajustable)
CREATE TABLE IF NOT EXISTS personal.gastos_recurrentes (
    id SERIAL PRIMARY KEY,
    descripcion VARCHAR(255) NOT NULL,
    monto_referencia DECIMAL(15,2) NOT NULL,
    categoria VARCHAR(50) NOT NULL DEFAULT 'Servicios',
    tipo VARCHAR(20) NOT NULL DEFAULT 'Fijo',
    responsable VARCHAR(255),
    activo BOOLEAN NOT NULL DEFAULT true,
    fecha_creacion TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_gastos_recurrentes_activo ON personal.gastos_recurrentes(activo);
CREATE INDEX IF NOT EXISTS idx_gastos_recurrentes_tipo ON personal.gastos_recurrentes(tipo);

-- Campo en gastos para rastrear si fue generado desde una plantilla
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns 
                   WHERE table_schema = 'personal' AND table_name = 'gastos' AND column_name = 'gasto_recurrente_id') THEN
        ALTER TABLE personal.gastos ADD COLUMN gasto_recurrente_id INTEGER REFERENCES personal.gastos_recurrentes(id);
    END IF;
END $$;

CREATE INDEX IF NOT EXISTS idx_gastos_recurrente_id ON personal.gastos(gasto_recurrente_id);
