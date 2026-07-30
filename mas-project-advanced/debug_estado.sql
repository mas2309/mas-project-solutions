-- Verificar el estado exacto de los pagos
SELECT 
    id, 
    descripcion, 
    estado,
    LENGTH(estado) as longitud_estado,
    saldo,
    valor
FROM personal.pagos 
ORDER BY id DESC
LIMIT 10;
