-- Asignar todos los datos existentes al primer usuario admin
-- Esto permite que los registros creados antes del multitenancy no se pierdan

UPDATE personal.proyectos SET usuario_id = (SELECT id FROM personal.usuarios WHERE username = 'admin' LIMIT 1) WHERE usuario_id IS NULL;
UPDATE personal.pagos SET usuario_id = (SELECT id FROM personal.usuarios WHERE username = 'admin' LIMIT 1) WHERE usuario_id IS NULL;
UPDATE personal.ingresos SET usuario_id = (SELECT id FROM personal.usuarios WHERE username = 'admin' LIMIT 1) WHERE usuario_id IS NULL;
UPDATE personal.gastos SET usuario_id = (SELECT id FROM personal.usuarios WHERE username = 'admin' LIMIT 1) WHERE usuario_id IS NULL;
UPDATE personal.creditos SET usuario_id = (SELECT id FROM personal.usuarios WHERE username = 'admin' LIMIT 1) WHERE usuario_id IS NULL;
UPDATE personal.documentos SET usuario_id = (SELECT id FROM personal.usuarios WHERE username = 'admin' LIMIT 1) WHERE usuario_id IS NULL;
UPDATE personal.gastos_recurrentes SET usuario_id = (SELECT id FROM personal.usuarios WHERE username = 'admin' LIMIT 1) WHERE usuario_id IS NULL;
