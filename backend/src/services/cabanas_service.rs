use diesel::prelude::*;
use crate::{
    models::{Cabana, NewCabana},
    schema::cabanas::dsl::*,
};

// ==============================
// 📋 Listar cabañas
// ==============================
pub fn listar_cabanas(conn: &mut PgConnection) -> QueryResult<Vec<Cabana>> {
    cabanas.order(id.desc()).load::<Cabana>(conn)
}

// ==============================
// ➕ Crear nueva cabaña
// ==============================
pub fn crear_cabana(conn: &mut PgConnection, nueva: NewCabana) -> QueryResult<Cabana> {
    diesel::insert_into(cabanas)
        .values(&nueva)
        .get_result::<Cabana>(conn)
}

// ==============================
// ♻️ Cambiar estado (disponible / ocupada / mantenimiento)
// ==============================
pub fn actualizar_estado(
    conn: &mut PgConnection,
    cabana_id: i32,
    nuevo_estado: &str,
) -> QueryResult<Cabana> {
    diesel::update(cabanas.find(cabana_id))
        .set(estado.eq(nuevo_estado))
        .get_result::<Cabana>(conn)
}
