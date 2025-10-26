use diesel::prelude::*;
use crate::models::{Reserva, NewReserva};
use crate::schema::{reservas, cabanas};

pub fn listar_reservas(conn: &mut PgConnection) -> QueryResult<Vec<Reserva>> {
    reservas::table.load::<Reserva>(conn)
}

pub fn crear_reserva(conn: &mut PgConnection, nueva_reserva: NewReserva) -> QueryResult<Reserva> {
    conn.transaction(|conn| {
        // Validación de solapamiento
        if crate::services::validaciones_service::existe_conflicto(
            conn,
            nueva_reserva.cabana_id,
            nueva_reserva.fecha_reserva,
            nueva_reserva.hora_inicio,
            nueva_reserva.hora_fin,
        )? {
            return Err(diesel::result::Error::RollbackTransaction);
        }

        // Insertar reserva
        let reserva = diesel::insert_into(reservas::table)
            .values(&nueva_reserva)
            .get_result::<Reserva>(conn)?;

        // Marcar cabaña como ocupada (estado lógico)
        diesel::update(cabanas::table.find(reserva.cabana_id))
            .set(cabanas::estado.eq("ocupada"))
            .execute(conn)?;

        Ok(reserva)
    })
}

pub fn eliminar_reserva(conn: &mut PgConnection, id: i32) -> QueryResult<usize> {
    use crate::schema::cabanas::dsl::{cabanas, estado as estado_cabana};

    conn.transaction(|conn| {
        let reserva: Reserva = reservas::table.find(id).first(conn)?;

        // Eliminar reserva
        let deleted = diesel::delete(reservas::table.find(id)).execute(conn)?;

        // Liberar cabaña
        diesel::update(cabanas.find(reserva.cabana_id))
            .set(estado_cabana.eq("disponible"))
            .execute(conn)?;

        Ok(deleted)
    })
}

pub fn actualizar_estado_reserva(
    conn: &mut PgConnection,
    reserva_id: i32,
    nuevo_estado: &str,
) -> QueryResult<Reserva> {
    use crate::schema::reservas::dsl::{reservas, estado as estado_reserva};
    use crate::schema::cabanas::dsl::{cabanas, estado as estado_cabana};

    conn.transaction(|conn| {
        let reserva_actualizada = diesel::update(reservas.find(reserva_id))
            .set(estado_reserva.eq(nuevo_estado))
            .get_result::<Reserva>(conn)?;

        // Si se cancela o completa, liberar cabaña
        if nuevo_estado == "cancelada" || nuevo_estado == "completada" {
            diesel::update(cabanas.find(reserva_actualizada.cabana_id))
                .set(estado_cabana.eq("disponible"))
                .execute(conn)?;
        }

        Ok(reserva_actualizada)
    })
}
