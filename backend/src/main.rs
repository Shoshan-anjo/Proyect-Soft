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
use models::{Reserva, NewReserva, Cliente, NewCliente};
use services::{reservas_service, clientes_service};

// =========================
// 🚀 ENDPOINTS BASE
// =========================
#[get("/")]
fn index() -> &'static str {
    "🚀 Bienvenido a la API de Reservas del Dubai Resto Bar!"
}

// =========================
// 🗓️ RESERVAS
// =========================
#[get("/reservas")]
fn listar_reservas(pool: &State<DbPool>) -> Json<Vec<Reserva>> {
    let mut conn = pool.get().expect("No se pudo obtener conexión del pool");
    let results = reservas_service::listar_reservas(&mut conn)
        .expect("Error al listar reservas");
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
// 🧍 CLIENTES
// =========================
#[get("/clientes")]
fn listar_clientes(pool: &State<DbPool>) -> Json<Vec<Cliente>> {
    let mut conn = pool.get().expect("No se pudo obtener conexión del pool");
    let results = clientes_service::listar_clientes(&mut conn)
        .expect("Error al listar clientes");
    Json(results)
}

#[post("/clientes", format = "json", data = "<nuevo_cliente>")]
fn crear_cliente(pool: &State<DbPool>, nuevo_cliente: Json<NewCliente>) -> Json<Cliente> {
    let mut conn = pool.get().expect("No se pudo obtener conexión del pool");
    let cliente = clientes_service::crear_cliente(&mut conn, nuevo_cliente.into_inner())
        .expect("Error al crear cliente");
    Json(cliente)
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
        .mount(
            "/",
            routes![
                index,
                listar_reservas,
                crear_reserva,
                listar_clientes,
                crear_cliente
            ],
        )
        .attach(AdHoc::on_ignite("Database Migrations", |rocket| async {
            println!("✅ Conectado a la base de datos PostgreSQL con éxito!");
            rocket
        }))
}
