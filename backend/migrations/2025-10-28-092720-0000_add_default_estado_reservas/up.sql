-- Your SQL goes here
-- 🔧 Agregar valor por defecto al campo "estado"
ALTER TABLE reservas
ALTER COLUMN estado SET DEFAULT 'pendiente';

-- 🔍 Asegurarse de que las filas existentes tengan valor
UPDATE reservas
SET estado = 'pendiente'
WHERE estado IS NULL;
