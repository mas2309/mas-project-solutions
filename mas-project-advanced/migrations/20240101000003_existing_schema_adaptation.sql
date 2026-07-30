-- Adaptación al esquema existente de la base de datos
-- Schema: personal
-- Database: acueducto_hato

-- Las tablas ya existen, solo documentamos la estructura para referencia

/*
TABLAS EXISTENTES:

1. usuarios
   - id: bigint (PK)
   - username: varchar
   - email: varchar
   - nombre_completo: varchar
   - password: varchar
   - rol: varchar
   - activo: boolean
   - fecha_creacion: timestamp
   - fecha_actualizacion: timestamp
   - ultimo_acceso: timestamp
   - failed_login_attempts: integer
   - lockout_end_time: timestamp

2. pagos
   - id: bigint (PK)
   - descripcion: varchar
   - valor: numeric
   - saldo: numeric
   - estado: varchar
   - mes: varchar
   - anio: varchar
   - evidencia: varchar
   - evidencia_constructora: varchar
   - fecha_creacion: timestamp
   - fecha_actualizacion: timestamp

3. pago (tabla legacy)
   - Similar a pagos pero con algunos campos como varchar

4. archivos
   - id: bigint (PK)
   - nombre_archivo: varchar
   - nombre_original: varchar
   - ruta: varchar
   - tipo_archivo: varchar
   - tipo_contenido: varchar
   - tamanio: bigint
   - fecha_subida: timestamp
   - pago_id: bigint (FK a pagos)
   - usuario_id: bigint (FK a usuarios)
*/

-- Crear índices adicionales si no existen
CREATE INDEX IF NOT EXISTS idx_pagos_estado ON personal.pagos(estado);
CREATE INDEX IF NOT EXISTS idx_pagos_fecha_creacion ON personal.pagos(fecha_creacion);
CREATE INDEX IF NOT EXISTS idx_archivos_pago_id ON personal.archivos(pago_id);
CREATE INDEX IF NOT EXISTS idx_archivos_usuario_id ON personal.archivos(usuario_id);
CREATE INDEX IF NOT EXISTS idx_usuarios_email ON personal.usuarios(email);
CREATE INDEX IF NOT EXISTS idx_usuarios_username ON personal.usuarios(username);