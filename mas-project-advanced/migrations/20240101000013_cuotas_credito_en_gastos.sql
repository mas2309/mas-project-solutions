-- Día del mes en que se paga la cuota del crédito.
-- Para los créditos existentes se toma el día de la fecha de inicio.
ALTER TABLE personal.creditos ADD COLUMN IF NOT EXISTS dia_pago INTEGER;
UPDATE personal.creditos SET dia_pago = EXTRACT(DAY FROM fecha_inicio)::int WHERE dia_pago IS NULL;
ALTER TABLE personal.creditos ALTER COLUMN dia_pago SET NOT NULL;
ALTER TABLE personal.creditos ALTER COLUMN dia_pago SET DEFAULT 1;

-- Enlace del gasto con el crédito cuya cuota representa.
-- Si se elimina el crédito, los gastos se conservan como histórico.
ALTER TABLE personal.gastos ADD COLUMN IF NOT EXISTS credito_id INTEGER REFERENCES personal.creditos(id) ON DELETE SET NULL;

CREATE INDEX IF NOT EXISTS idx_gastos_credito_id ON personal.gastos(credito_id);
