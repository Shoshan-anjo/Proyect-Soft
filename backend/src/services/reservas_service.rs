use diesel::prelude::*;
use crate::models::{Reserva, NewReserva};
use crate::schema::{reservas, cabanas};

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
        // 1️⃣ Crear la reserva
        let reserva = diesel::insert_into(reservas::table)
            .values(&nueva_reserva)
            .get_result::<Reserva>(conn)?;

        // 2️⃣ Actualizar estado de la cabaña a "ocupada"
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
    use crate::schema::reservas::dsl::reservas;
    use crate::schema::cabanas::dsl::{cabanas, estado as estado_cabana};

    conn.transaction(|conn| {
        // 1️⃣ Buscar la reserva antes de eliminar
        let reserva_eliminada: Reserva = reservas.find(reserva_id).first(conn)?;

        // 2️⃣ Eliminar la reserva
        let deleted = diesel::delete(reservas.find(reserva_id)).execute(conn)?;

        // 3️⃣ Marcar la cabaña como "disponible"
        diesel::update(cabanas.find(reserva_eliminada.cabana_id))
            .set(estado_cabana.eq("disponible"))
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
    use crate::schema::reservas::dsl::{reservas, estado as estado_reserva};
    use crate::schema::cabanas::dsl::{cabanas, estado as estado_cabana};

    conn.transaction(|conn| {
        // 1️⃣ Actualizamos el estado de la reserva
        let reserva_actualizada = diesel::update(reservas.find(reserva_id))
            .set(estado_reserva.eq(nuevo_estado))
            .get_result::<Reserva>(conn)?;

        // 2️⃣ Si se completó o canceló, liberar la cabaña
        if nuevo_estado == "completada" || nuevo_estado == "cancelada" {
            diesel::update(cabanas.find(reserva_actualizada.cabana_id))
                .set(estado_cabana.eq("disponible"))
                .execute(conn)?;
        }

        Ok(reserva_actualizada)
    })
}
