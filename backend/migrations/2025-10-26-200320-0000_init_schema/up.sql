-- Your SQL goes here
-- ===============================
-- RESET TOTAL DEL ESQUEMA
-- ===============================

DROP TABLE IF EXISTS reservas CASCADE;
DROP TABLE IF EXISTS cabanas CASCADE;
DROP TABLE IF EXISTS clientes CASCADE;

-- ===============================
-- 🧍 CLIENTES
-- ===============================
CREATE TABLE clientes (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    telefono VARCHAR(20),
    email VARCHAR(100),
    dni VARCHAR(30),
    fecha_registro TIMESTAMP DEFAULT NOW()
);

-- ===============================
-- 🏠 CABAÑAS
-- ===============================
CREATE TABLE cabanas (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR(50) NOT NULL,
    capacidad INT NOT NULL DEFAULT 4,
    ubicacion VARCHAR(100),
    estado VARCHAR(20) NOT NULL DEFAULT 'disponible'
        CHECK (estado IN ('disponible', 'ocupada', 'mantenimiento')),
    descripcion TEXT,
    precio_hora NUMERIC(10,2)
);

-- ===============================
-- 📅 RESERVAS
-- ===============================
CREATE TABLE reservas (
    id SERIAL PRIMARY KEY,
    cliente_id INT NOT NULL REFERENCES clientes(id) ON DELETE CASCADE,
    cabana_id INT NOT NULL REFERENCES cabanas(id) ON DELETE CASCADE,
    fecha_reserva DATE NOT NULL,
    hora_inicio TIME NOT NULL,
    hora_fin TIME NOT NULL,
    estado VARCHAR(20) NOT NULL DEFAULT 'pendiente'
        CHECK (estado IN ('pendiente', 'confirmada', 'cancelada', 'completada')),
    observaciones TEXT,
    fecha_creacion TIMESTAMP DEFAULT NOW(),
    CONSTRAINT chk_horas CHECK (hora_fin > hora_inicio)
);

-- ===============================
-- 📊 ÍNDICES ÚTILES
-- ===============================
CREATE INDEX idx_reservas_fecha ON reservas (fecha_reserva);
CREATE INDEX idx_reservas_cabana_fecha ON reservas (cabana_id, fecha_reserva);
CREATE INDEX idx_clientes_nombre ON clientes (nombre);

-- ===============================
-- 🌱 DATOS DE PRUEBA
-- ===============================
INSERT INTO clientes (nombre, telefono, email)
VALUES ('Cliente Demo', '70000000', 'demo@correo.com');

INSERT INTO cabanas (nombre, capacidad, ubicacion, estado, descripcion, precio_hora)
VALUES
('Cabaña VIP 1', 6, 'Zona A - Frente al escenario', 'disponible', 'Cabaña privada con sistema de sonido y luces', 50.00),
('Cabaña VIP 2', 8, 'Zona B - Cerca del bar', 'disponible', 'Cabaña para grupos grandes con pantalla LED', 70.00),
('Cabaña Familiar 1', 10, 'Zona C - Patio exterior', 'disponible', 'Cabaña amplia con karaoke y ventilación natural', 90.00),
('Cabaña Parejas', 2, 'Zona D - Área reservada', 'disponible', 'Cabaña romántica con luz tenue y privacidad', 40.00),
('Cabaña Terraza', 4, 'Zona E - Azotea', 'disponible', 'Vista panorámica y ambiente chill', 60.00);

-- 🧹 Eliminar todas las cabañas y reservas asociadas
--DELETE FROM reservas;
--DELETE FROM cabanas;

-- 🧹 Reiniciar los contadores de las secuencias
--ALTER SEQUENCE cabanas_id_seq RESTART WITH 1;

---- 🌱 Insertar tus nuevas cabañas personalizadas
--INSERT INTO cabanas (nombre, capacidad, ubicacion, estado, descripcion, precio_hora)
--VALUES
--('Cabaña Oro', 8, 'Sector VIP', 'disponible', 'Amplia cabaña con vista panorámica', 150.00),
--('Cabaña Plata', 6, 'Zona Media', 'disponible', 'Ideal para grupos pequeños o familias', 120.00),
--('Cabaña Bronze', 4, 'Zona Clásica', 'disponible', 'Ambiente acogedor con decoración rústica', 90.00),
--('Cabaña Deluxe', 10, 'Vista Lago', 'disponible', 'Cabaña premium con jacuzzi privado', 200.00);
