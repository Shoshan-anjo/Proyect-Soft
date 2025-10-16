use diesel::prelude::*;
use crate::models::{Reserva, NewReserva};
use crate::schema::reservas::dsl::*;

pub fn listar_reservas(conn: &mut PgConnection) -> QueryResult<Vec<Reserva>> {
    reservas.load::<Reserva>(conn)
}

pub fn crear_reserva(conn: &mut PgConnection, nueva: NewReserva) -> QueryResult<Reserva> {
    diesel::insert_into(reservas)
        .values(&nueva)
        .get_result(conn)
}
