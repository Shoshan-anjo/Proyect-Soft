-- This file should undo anything in `up.sql`
ALTER TABLE clientes DROP COLUMN IF EXISTS activo;
ALTER TABLE cabanas DROP COLUMN IF EXISTS precio_hora;
ALTER TABLE reservas DROP COLUMN IF EXISTS precio_total;
DROP INDEX IF EXISTS idx_reservas_cliente_fecha;
DROP TABLE IF EXISTS empleados;
