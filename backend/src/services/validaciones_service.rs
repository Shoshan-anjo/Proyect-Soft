use diesel::prelude::*;

/// Retorna true si existe solapamiento con otra reserva en la misma cabaña y fecha
pub fn existe_conflicto(
    conn: &mut PgConnection,
    cabana_id_value: i32,
    fecha: chrono::NaiveDate,
    inicio: chrono::NaiveTime,
    fin: chrono::NaiveTime,
) -> QueryResult<bool> {
    use crate::schema::reservas::dsl::*;

    let count: i64 = reservas
        .filter(cabana_id.eq(cabana_id_value))
        .filter(fecha_reserva.eq(fecha))
        // solapa si (inicio_a < fin_b) y (fin_a > inicio_b)
        .filter(hora_inicio.lt(fin))
        .filter(hora_fin.gt(inicio))
        .filter(estado.ne("cancelada"))
        .count()
        .get_result(conn)?;

    Ok(count > 0)
}
