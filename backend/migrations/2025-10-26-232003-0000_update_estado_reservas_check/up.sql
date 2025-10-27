-- Your SQL goes here
-- =========================================
-- 🧩 Actualizar restricción CHECK de reservas.estado
-- =========================================

ALTER TABLE reservas
DROP CONSTRAINT IF EXISTS reservas_estado_check;

ALTER TABLE reservas
ADD CONSTRAINT reservas_estado_check
CHECK (estado IN ('pendiente', 'en curso', 'completada', 'cancelada'));

-- =========================================
-- 🧠 Nota:
-- Esta actualización permite los nuevos estados manejados por el backend.
-- - pendiente: creada pero aún no iniciada
-- - en curso: hora actual dentro del rango
-- - completada: hora actual superó el fin
-- - cancelada: cancelada manualmente
-- =========================================
