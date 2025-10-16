#[macro_use]
extern crate rocket;

use rocket::{serde::json::Json, State};
use rocket::fairing::AdHoc;
use dotenvy::dotenv;

mod schema;
mod models;
mod db;
mod services;

use db::DbPool;
use models::{Reserva, NewReserva};
use services::reservas_service;

// =========================
// 🚀 ENDPOINTS
// =========================
#[get("/")]
fn index() -> &'static str {
    "🚀 Bienvenido a la API de Reservas del Dubai Resto Bar!"
}

#[get("/reservas")]
fn listar_reservas(pool: &State<DbPool>) -> Json<Vec<Reserva>> {
    let mut conn = pool.get().expect("No se pudo obtener conexión del pool");
    let results = reservas_service::listar_reservas(&mut conn).expect("Error al listar reservas");
    Json(results)
}

#[post("/reservas", format = "json", data = "<nueva_reserva>")]
fn crear_reserva(pool: &State<DbPool>, nueva_reserva: Json<NewReserva>) -> Json<Reserva> {
    let mut conn = pool.get().expect("No se pudo obtener conexión del pool");
    let reserva = reservas_service::crear_reserva(&mut conn, nueva_reserva.into_inner())
        .expect("Error al crear la reserva");
    Json(reserva)
}

// =========================
// ⚙️ ROCKET + DB INIT
// =========================
#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let pool = db::establish_pool();

    rocket::build()
        .manage(pool)
        .mount("/", routes![index, listar_reservas, crear_reserva])
        .attach(AdHoc::on_ignite("Database Migrations", |rocket| async {
            println!("✅ Conectado a la base de datos PostgreSQL con éxito!");
            rocket
        }))
}
