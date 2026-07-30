-- Corregir inconsistencias entre estado y saldo
UPDATE personal.pagos
SET estado = CASE 
    WHEN saldo = 0 THEN 'Pagado'
    WHEN saldo < valor THEN 'Parcial'
    WHEN saldo = valor THEN 'Pendiente'
    ELSE estado
END
WHERE (estado = 'Pagado' AND saldo > 0) 
   OR (estado = 'Pendiente' AND saldo = 0)
   OR (estado = 'Pendiente' AND saldo < valor);

-- Verificar los cambios
SELECT id, descripcion, estado, saldo, valor,
    CASE 
        WHEN saldo = 0 THEN 'Debería ser Pagado'
        WHEN saldo < valor THEN 'Debería ser Parcial'
        WHEN saldo = valor THEN 'Debería ser Pendiente'
    END as estado_correcto
FROM personal.pagos
ORDER BY id DESC
LIMIT 10;
