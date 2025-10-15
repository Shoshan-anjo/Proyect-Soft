-- Your SQL goes here
-- ==========================================================
-- 🔹 Dubai Resto Bar - Fase 1: Gestión de Reservas
-- ==========================================================

-- ----------------------------
-- Tabla: clientes
-- ----------------------------
CREATE TABLE clientes (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    telefono VARCHAR(20),
    email VARCHAR(100),
    dni VARCHAR(30),
    fecha_registro TIMESTAMP DEFAULT NOW()
);

-- ----------------------------
-- Tabla: cabanas
-- ----------------------------
CREATE TABLE cabanas (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR(50) NOT NULL,
    capacidad INT NOT NULL DEFAULT 4,
    ubicacion VARCHAR(100),
    estado VARCHAR(20) NOT NULL DEFAULT 'disponible' 
        CHECK (estado IN ('disponible', 'ocupada', 'mantenimiento')),
    descripcion TEXT
);

-- ----------------------------
-- Tabla: reservas
-- ----------------------------
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

    CONSTRAINT chk_horas_validas CHECK (hora_fin > hora_inicio)
);

-- Evitar reservas duplicadas para la misma cabaña en mismo horario
CREATE UNIQUE INDEX idx_reserva_unica
ON reservas (cabana_id, fecha_reserva, hora_inicio, hora_fin);

-- ----------------------------
-- Tabla: pagos
-- ----------------------------
CREATE TABLE pagos (
    id SERIAL PRIMARY KEY,
    reserva_id INT NOT NULL REFERENCES reservas(id) ON DELETE CASCADE,
    monto DECIMAL(10,2) NOT NULL,
    metodo VARCHAR(30) NOT NULL DEFAULT 'efectivo'
        CHECK (metodo IN ('efectivo', 'tarjeta', 'qr', 'transferencia')),
    estado VARCHAR(20) NOT NULL DEFAULT 'pendiente'
        CHECK (estado IN ('pendiente', 'confirmado', 'anulado')),
    fecha_pago TIMESTAMP DEFAULT NOW()
);

-- Índices sugeridos
CREATE INDEX idx_cliente_nombre ON clientes (nombre);
CREATE INDEX idx_reservas_fecha ON reservas (fecha_reserva);
CREATE INDEX idx_pagos_reserva ON pagos (reserva_id);
