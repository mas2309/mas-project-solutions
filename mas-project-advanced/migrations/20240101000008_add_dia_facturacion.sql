-- Agregar día de facturación a gastos recurrentes
-- Indica qué día del mes se genera automáticamente (para los Fijos)
ALTER TABLE personal.gastos_recurrentes ADD COLUMN IF NOT EXISTS dia_facturacion INTEGER NOT NULL DEFAULT 1;
