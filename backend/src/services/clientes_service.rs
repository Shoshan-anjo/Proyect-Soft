use diesel::prelude::*;
use crate::{
    models::{Cliente, NewCliente},
    schema::clientes::dsl::*,
};

pub fn listar_clientes(conn: &mut PgConnection) -> QueryResult<Vec<Cliente>> {
    clientes.order(id.desc()).load::<Cliente>(conn)
}

pub fn crear_cliente(conn: &mut PgConnection, nuevo: NewCliente) -> QueryResult<Cliente> {
    diesel::insert_into(clientes)
        .values(&nuevo)
        .get_result::<Cliente>(conn)
}
