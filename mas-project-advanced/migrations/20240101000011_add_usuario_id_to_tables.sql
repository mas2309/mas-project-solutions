-- Agregar columna usuario_id a todas las tablas de datos financieros
-- para implementar aislamiento de datos por usuario (multitenancy)

-- Proyectos
ALTER TABLE personal.proyectos ADD COLUMN IF NOT EXISTS usuario_id BIGINT REFERENCES personal.usuarios(id);

-- Pagos
ALTER TABLE personal.pagos ADD COLUMN IF NOT EXISTS usuario_id BIGINT REFERENCES personal.usuarios(id);

-- Ingresos
ALTER TABLE personal.ingresos ADD COLUMN IF NOT EXISTS usuario_id BIGINT REFERENCES personal.usuarios(id);

-- Gastos
ALTER TABLE personal.gastos ADD COLUMN IF NOT EXISTS usuario_id BIGINT REFERENCES personal.usuarios(id);

-- Créditos
ALTER TABLE personal.creditos ADD COLUMN IF NOT EXISTS usuario_id BIGINT REFERENCES personal.usuarios(id);

-- Documentos
ALTER TABLE personal.documentos ADD COLUMN IF NOT EXISTS usuario_id BIGINT REFERENCES personal.usuarios(id);

-- Gastos Recurrentes
ALTER TABLE personal.gastos_recurrentes ADD COLUMN IF NOT EXISTS usuario_id BIGINT REFERENCES personal.usuarios(id);

-- Índices para performance
CREATE INDEX IF NOT EXISTS idx_proyectos_usuario_id ON personal.proyectos(usuario_id);
CREATE INDEX IF NOT EXISTS idx_pagos_usuario_id ON personal.pagos(usuario_id);
CREATE INDEX IF NOT EXISTS idx_ingresos_usuario_id ON personal.ingresos(usuario_id);
CREATE INDEX IF NOT EXISTS idx_gastos_usuario_id ON personal.gastos(usuario_id);
CREATE INDEX IF NOT EXISTS idx_creditos_usuario_id ON personal.creditos(usuario_id);
CREATE INDEX IF NOT EXISTS idx_documentos_usuario_id ON personal.documentos(usuario_id);
CREATE INDEX IF NOT EXISTS idx_gastos_recurrentes_usuario_id ON personal.gastos_recurrentes(usuario_id);
