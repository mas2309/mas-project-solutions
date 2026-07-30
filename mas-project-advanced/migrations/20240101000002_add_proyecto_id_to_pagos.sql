-- Agregar relación proyecto_id a la tabla pagos (idempotente)
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns 
                   WHERE table_schema = 'personal' AND table_name = 'pagos' AND column_name = 'proyecto_id') THEN
        ALTER TABLE personal.pagos ADD COLUMN proyecto_id INTEGER REFERENCES personal.proyectos(id);
    END IF;
END $$;

-- Crear índice para mejorar consultas
CREATE INDEX IF NOT EXISTS idx_pagos_proyecto_id ON personal.pagos(proyecto_id);

-- Comentario para documentación
COMMENT ON COLUMN personal.pagos.proyecto_id IS 'Referencia al proyecto al que pertenece este pago';