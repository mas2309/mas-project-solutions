-- Agregar campo tipo_tasa a la tabla de créditos
-- Valores posibles: 'Fija', 'Variable'
ALTER TABLE personal.creditos ADD COLUMN IF NOT EXISTS tipo_tasa VARCHAR(20) NOT NULL DEFAULT 'Fija';
