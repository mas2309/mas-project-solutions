-- Agregar constraint unique a username si no existe
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'uq_usuarios_username') THEN
        ALTER TABLE personal.usuarios ADD CONSTRAINT uq_usuarios_username UNIQUE (username);
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'uq_usuarios_email') THEN
        ALTER TABLE personal.usuarios ADD CONSTRAINT uq_usuarios_email UNIQUE (email);
    END IF;
END $$;
