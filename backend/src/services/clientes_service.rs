use diesel::prelude::*;
use crate::models::{Cliente, NewCliente};
use crate::schema::clientes;

pub fn listar_clientes(conn: &mut PgConnection) -> QueryResult<Vec<Cliente>> {
    clientes::table.load::<Cliente>(conn)
}

pub fn crear_cliente(conn: &mut PgConnection, nuevo: NewCliente) -> QueryResult<Cliente> {
    diesel::insert_into(clientes::table)
        .values(&nuevo)
        .get_result(conn)
}
