use diesel::prelude::*;
use diesel::result::Error;
//use chrono::{Local, NaiveDate, NaiveTime};
use crate::models::{Reserva, NewReserva};
use crate::schema::{reservas, cabanas};

// =============================
// 📋 Listar reservas
// =============================
pub fn listar_reservas(conn: &mut PgConnection) -> QueryResult<Vec<Reserva>> {
    reservas::table
        .order((reservas::fecha_reserva.asc(), reservas::hora_inicio.asc()))
        .load::<Reserva>(conn)
}

// =============================
// ➕ Crear nueva reserva
// =============================
pub fn crear_reserva(conn: &mut PgConnection, nueva_reserva: NewReserva) -> QueryResult<Reserva> {
    use crate::services::validaciones_service;

    conn.transaction::<Reserva, Error, _>(|conn| {
        // Validar conflictos (solapamientos)
        let hay_conflicto = validaciones_service::existe_conflicto(
            conn,
            nueva_reserva.cabana_id,
            nueva_reserva.fecha_reserva,
            nueva_reserva.hora_inicio,
            nueva_reserva.hora_fin,
        )?;

        if hay_conflicto {
            return Err(Error::RollbackTransaction);
        }

        // Insertar reserva
        let reserva = diesel::insert_into(reservas::table)
            .values(&nueva_reserva)
            .get_result::<Reserva>(conn)?;

        // Marcar cabaña como "ocupada" (estado lógico)
        diesel::update(cabanas::table.find(reserva.cabana_id))
            .set(cabanas::estado.eq("ocupada"))
            .execute(conn)?;

        Ok(reserva)
    })
}

// =============================
// ❌ Eliminar reserva
// =============================
pub fn eliminar_reserva(conn: &mut PgConnection, id: i32) -> QueryResult<usize> {
    use crate::schema::cabanas::dsl::{cabanas, estado as estado_cabana};

    conn.transaction::<usize, Error, _>(|conn| {
        // Obtener la reserva para liberar la cabaña luego
        let reserva: Reserva = reservas::table.find(id).first(conn)?;

        // Eliminar
        let deleted = diesel::delete(reservas::table.find(id)).execute(conn)?;

        // Liberar cabaña
        diesel::update(cabanas.find(reserva.cabana_id))
            .set(estado_cabana.eq("disponible"))
            .execute(conn)?;

        Ok(deleted)
    })
}

// =============================
// 🔄 Actualizar estado de reserva
// =============================
pub fn actualizar_estado_reserva(
    conn: &mut PgConnection,
    reserva_id: i32,
    nuevo_estado: &str,
) -> QueryResult<Reserva> {
    use crate::schema::reservas::dsl::{reservas as t_reservas, estado as estado_reserva};
    use crate::schema::cabanas::dsl::{cabanas as t_cabanas, estado as estado_cabana};

    conn.transaction::<Reserva, Error, _>(|conn| {
        let reserva_actualizada = diesel::update(t_reservas.find(reserva_id))
            .set(estado_reserva.eq(nuevo_estado))
            .get_result::<Reserva>(conn)?;

        // Si se cancela o completa → liberar cabaña
        if nuevo_estado == "cancelada" || nuevo_estado == "completada" {
            diesel::update(t_cabanas.find(reserva_actualizada.cabana_id))
                .set(estado_cabana.eq("disponible"))
                .execute(conn)?;
        }

        Ok(reserva_actualizada)
    })
}

// =============================
// 🕒 Actualizar estados automáticos (opcional)
//  - Marca "en curso" si ahora ∈ [inicio, fin)
//  - Marca "completada" si ahora ≥ fin
//  - Libera cabaña cuando corresponda
// =============================
pub fn actualizar_estados_automaticos(conn: &mut PgConnection) -> QueryResult<()> {
    use crate::schema::reservas::dsl::*;
    use crate::schema::cabanas::dsl::{cabanas, estado as estado_cabana};

    use chrono::{Local, NaiveDate, NaiveTime};
    let ahora = Local::now();
    let fecha_actual: NaiveDate = ahora.date_naive();
    let hora_actual: NaiveTime = ahora.time();

    // Obtenemos TODAS las reservas del día actual (o incluso futuras si quieres ampliar)
    let reservas_hoy = reservas
        .filter(fecha_reserva.eq(fecha_actual))
        .order((cabana_id.asc(), hora_inicio.asc()))
        .load::<crate::models::Reserva>(conn)?;

    // Vamos a llevar un registro de la última cabaña procesada
    let mut ultima_cabana_id: Option<i32> = None;
    let mut estado_cabana_actual = String::from("disponible");

    for r in reservas_hoy {
        // Si cambiamos de cabaña, reseteamos el estado
        if Some(r.cabana_id) != ultima_cabana_id {
            ultima_cabana_id = Some(r.cabana_id);
            estado_cabana_actual = String::from("disponible");
        }

        if r.estado != "cancelada" {
            if hora_actual >= r.hora_inicio && hora_actual < r.hora_fin {
                // En curso
                diesel::update(reservas.find(r.id))
                    .set(estado.eq("en curso"))
                    .execute(conn)?;

                estado_cabana_actual = "ocupada".to_string();
            } else if hora_actual >= r.hora_fin {
                // Completada
                diesel::update(reservas.find(r.id))
                    .set(estado.eq("completada"))
                    .execute(conn)?;
            } else if hora_actual < r.hora_inicio {
                // Próxima reserva
                diesel::update(reservas.find(r.id))
                    .set(estado.eq("pendiente"))
                    .execute(conn)?;
            }
        }

        // Al final de cada grupo de cabaña, reflejamos el estado actual
        diesel::update(cabanas.find(r.cabana_id))
            .set(estado_cabana.eq(&estado_cabana_actual))
            .execute(conn)?;
    }

    Ok(())
}

