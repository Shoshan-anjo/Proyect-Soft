use diesel::prelude::*;
use crate::models::Reserva;
use crate::schema::reservas::dsl::*;

/// ✅ Verifica si una cabaña ya tiene reservas que se solapan en horario
pub fn existe_conflicto(
    conn: &mut PgConnection,
    cabana: i32,
    fecha: chrono::NaiveDate,
    inicio: chrono::NaiveTime,
    fin: chrono::NaiveTime,
) -> QueryResult<bool> {
    let conflicto = reservas
        .filter(cabana_id.eq(cabana))
        .filter(fecha_reserva.eq(fecha))
        .filter(
            hora_inicio.lt(fin)
                .and(hora_fin.gt(inicio)),
        )
        .filter(estado.ne("cancelada"))
        .first::<Reserva>(conn)
        .optional()?; // devuelve Ok(None) si no hay conflicto

    Ok(conflicto.is_some())
}
