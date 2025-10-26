use diesel::prelude::*;
use crate::models::{Cabana, NewCabana};
use crate::schema::cabanas;

pub fn listar_cabanas(conn: &mut PgConnection) -> QueryResult<Vec<Cabana>> {
    cabanas::table.load::<Cabana>(conn)
}

pub fn crear_cabana(conn: &mut PgConnection, nueva: NewCabana) -> QueryResult<Cabana> {
    diesel::insert_into(cabanas::table)
        .values(&nueva)
        .get_result(conn)
}

pub fn actualizar_estado(conn: &mut PgConnection, cabana_id: i32, nuevo_estado: &str) -> QueryResult<Cabana> {
    diesel::update(cabanas::table.find(cabana_id))
        .set(cabanas::estado.eq(nuevo_estado))
        .get_result(conn)
}
