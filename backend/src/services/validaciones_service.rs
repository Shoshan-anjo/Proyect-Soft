use diesel::prelude::*;
use chrono::{NaiveDate, NaiveTime};
//use crate::schema::reservas;

/// Retorna `true` si existe solapamiento para (cabana_id, fecha, [inicio, fin))
pub fn existe_conflicto(
    conn: &mut PgConnection,
    cabana: i32,
    fecha: NaiveDate,
    inicio_nuevo: NaiveTime,
    fin_nuevo: NaiveTime,
) -> QueryResult<bool> {
    use crate::schema::reservas::dsl::{
        reservas as t_reservas, cabana_id, fecha_reserva, hora_inicio, hora_fin, estado,
    };

    // Regla de solape: (inicio < fin_nuevo) AND (fin > inicio_nuevo)
    // Además, ignoramos reservas canceladas
    let count = t_reservas
        .filter(cabana_id.eq(cabana))
        .filter(fecha_reserva.eq(fecha))
        .filter(estado.ne("cancelada"))
        .filter(hora_inicio.lt(fin_nuevo))
        .filter(hora_fin.gt(inicio_nuevo))
        .count()
        .get_result::<i64>(conn)?;

    Ok(count > 0)
}
