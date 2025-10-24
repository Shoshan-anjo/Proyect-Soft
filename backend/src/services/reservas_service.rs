use diesel::prelude::*;
use crate::models::{Reserva, NewReserva};
use crate::schema::{reservas, cabanas};
use crate::services::validaciones_service;

// =============================
// 📋 Listar reservas
// =============================
pub fn listar_reservas(conn: &mut PgConnection) -> QueryResult<Vec<Reserva>> {
    reservas::table.load::<Reserva>(conn)
}

// =============================
// ➕ Crear nueva reserva
// =============================
pub fn crear_reserva(conn: &mut PgConnection, nueva_reserva: NewReserva) -> QueryResult<Reserva> {
    conn.transaction(|conn| {
        // ✅ 1️⃣ Verificar conflicto antes de insertar
        let hay_conflicto = validaciones_service::existe_conflicto(
            conn,
            nueva_reserva.cabana_id,
            nueva_reserva.fecha_reserva,
            nueva_reserva.hora_inicio,
            nueva_reserva.hora_fin,
        )?;

        if hay_conflicto {
            return Err(diesel::result::Error::RollbackTransaction);
        }

        // ✅ 2️⃣ Crear la reserva
        let reserva = diesel::insert_into(reservas::table)
            .values(&nueva_reserva)
            .get_result::<Reserva>(conn)?;

        // ✅ 3️⃣ Actualizar estado de la cabaña a "ocupada"
        diesel::update(cabanas::table.find(reserva.cabana_id))
            .set(cabanas::estado.eq("ocupada"))
            .execute(conn)?;

        Ok(reserva)
    })
}

// =============================
// ❌ Eliminar reserva
// =============================
pub fn eliminar_reserva(conn: &mut PgConnection, reserva_id: i32) -> QueryResult<usize> {
    conn.transaction(|conn| {
        // 1️⃣ Buscar la reserva antes de eliminar
        let reserva_eliminada: Reserva = reservas::table.find(reserva_id).first(conn)?;

        // 2️⃣ Eliminar la reserva
        let deleted = diesel::delete(reservas::table.find(reserva_id)).execute(conn)?;

        // 3️⃣ Marcar la cabaña como "disponible"
        diesel::update(cabanas::table.find(reserva_eliminada.cabana_id))
            .set(cabanas::estado.eq("disponible"))
            .execute(conn)?;

        Ok(deleted)
    })
}

// =============================
// 🔄 Actualizar estado reserva
// =============================
pub fn actualizar_estado_reserva(
    conn: &mut PgConnection,
    reserva_id: i32,
    nuevo_estado: &str,
) -> QueryResult<Reserva> {
    conn.transaction(|conn| {
        // 1️⃣ Actualizamos el estado de la reserva
        let reserva_actualizada = diesel::update(reservas::table.find(reserva_id))
            .set(reservas::estado.eq(nuevo_estado))
            .get_result::<Reserva>(conn)?;

        // 2️⃣ Si se completó o canceló, liberar la cabaña
        if nuevo_estado == "completada" || nuevo_estado == "cancelada" {
            diesel::update(cabanas::table.find(reserva_actualizada.cabana_id))
                .set(cabanas::estado.eq("disponible"))
                .execute(conn)?;
        }

        Ok(reserva_actualizada)
    })
}
