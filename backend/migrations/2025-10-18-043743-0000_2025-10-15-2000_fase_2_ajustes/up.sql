-- Your SQL goes here
-- ================================
-- 🚀 Fase 2 - Ajustes estructurales
-- ================================

-- CLIENTES: permitir activar/desactivar
ALTER TABLE clientes
ADD COLUMN IF NOT EXISTS activo BOOLEAN DEFAULT TRUE;

-- CABAÑAS: agregar precio por hora
ALTER TABLE cabanas
ADD COLUMN IF NOT EXISTS precio_hora DECIMAL(10,2) DEFAULT 0;

-- RESERVAS: agregar precio total
ALTER TABLE reservas
ADD COLUMN IF NOT EXISTS precio_total DECIMAL(10,2) DEFAULT 0;

-- RESERVAS: índice para búsquedas por cliente y fecha
CREATE INDEX IF NOT EXISTS idx_reservas_cliente_fecha
ON reservas (cliente_id, fecha_reserva);

-- EMPLEADOS: base para control interno
CREATE TABLE IF NOT EXISTS empleados (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    cargo VARCHAR(50) NOT NULL DEFAULT 'mesero'
        CHECK (cargo IN ('mesero', 'admin', 'recepcion', 'otro')),
    telefono VARCHAR(20),
    email VARCHAR(100),
    fecha_registro TIMESTAMP DEFAULT NOW(),
    activo BOOLEAN DEFAULT TRUE
);
