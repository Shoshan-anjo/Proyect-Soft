#[macro_use]
extern crate rocket;

use rocket::{serde::json::Json, State};
use rocket::fairing::AdHoc;
use dotenvy::dotenv;

// =========================
// 📦 Módulos internos
// =========================
mod schema;
mod models;
mod db;
mod services;

use db::DbPool;
use models::{Reserva, NewReserva, Cliente, NewCliente, Cabana, NewCabana};
use services::{reservas_service, clientes_service, cabanas_service};

// =========================
// 🚀 ENDPOINTS BASE
// =========================
#[get("/")]
fn index() -> &'static str {
    "🚀 Bienvenido a la API de Reservas del Dubai Resto Bar!"
}

// =========================
// 🧍 CLIENTES
// =========================
#[get("/clientes")]
fn listar_clientes(pool: &State<DbPool>) -> Json<Vec<Cliente>> {
    let mut conn = pool.get().expect("❌ No se pudo obtener conexión del pool");
    let results = clientes_service::listar_clientes(&mut conn)
        .expect("❌ Error al listar clientes");
    Json(results)
}

#[post("/clientes", format = "json", data = "<nuevo_cliente>")]
fn crear_cliente(pool: &State<DbPool>, nuevo_cliente: Json<NewCliente>) -> Json<Cliente> {
    let mut conn = pool.get().expect("❌ No se pudo obtener conexión del pool");
    let cliente = clientes_service::crear_cliente(&mut conn, nuevo_cliente.into_inner())
        .expect("❌ Error al crear cliente");
    Json(cliente)
}

// =========================
// 🏠 CABAÑAS
// =========================
#[get("/cabanas")]
fn listar_cabanas(pool: &State<DbPool>) -> Json<Vec<Cabana>> {
    let mut conn = pool.get().expect("❌ No se pudo obtener conexión del pool");
    let result = cabanas_service::listar_cabanas(&mut conn)
        .expect("❌ Error al listar cabañas");
    Json(result)
}

#[post("/cabanas", format = "json", data = "<nueva_cabana>")]
fn crear_cabana(pool: &State<DbPool>, nueva_cabana: Json<NewCabana>) -> Json<Cabana> {
    let mut conn = pool.get().expect("❌ No se pudo obtener conexión del pool");
    let cab = cabanas_service::crear_cabana(&mut conn, nueva_cabana.into_inner())
        .expect("❌ Error al crear cabaña");
    Json(cab)
}

#[put("/cabanas/<cabana_id>/<nuevo_estado>")]
fn actualizar_estado_cabana(
    pool: &State<DbPool>,
    cabana_id: i32,
    nuevo_estado: &str,
) -> Json<Cabana> {
    let mut conn = pool.get().expect("❌ No se pudo obtener conexión del pool");
    let cab = cabanas_service::actualizar_estado(&mut conn, cabana_id, nuevo_estado)
        .expect("❌ Error al actualizar estado de cabaña");
    Json(cab)
}

// =========================
// 📅 RESERVAS
// =========================
#[get("/reservas")]
fn listar_reservas(pool: &State<DbPool>) -> Json<Vec<Reserva>> {
    let mut conn = pool.get().expect("❌ No se pudo obtener conexión del pool");
    let results = reservas_service::listar_reservas(&mut conn)
        .expect("❌ Error al listar reservas");
    Json(results)
}

#[post("/reservas", format = "json", data = "<nueva_reserva>")]
fn crear_reserva(pool: &State<DbPool>, nueva_reserva: Json<NewReserva>) -> Json<Reserva> {
    let mut conn = pool.get().expect("❌ No se pudo obtener conexión del pool");
    let reserva = reservas_service::crear_reserva(&mut conn, nueva_reserva.into_inner())
        .expect("❌ Error al crear la reserva");
    Json(reserva)
}

#[delete("/reservas/<id>")]
fn eliminar_reserva(pool: &State<DbPool>, id: i32) -> Json<String> {
    let mut conn = pool.get().expect("❌ No se pudo obtener conexión del pool");
    reservas_service::eliminar_reserva(&mut conn, id)
        .expect("❌ Error al eliminar reserva");
    Json(format!("🗑️ Reserva {} eliminada correctamente", id))
}

#[put("/reservas/<id>/<nuevo_estado>")]
fn actualizar_estado_reserva(
    pool: &State<DbPool>,
    id: i32,
    nuevo_estado: &str,
) -> Json<String> {
    let mut conn = pool.get().expect("❌ No se pudo obtener conexión del pool");
    reservas_service::actualizar_estado_reserva(&mut conn, id, nuevo_estado)
        .expect("❌ Error al actualizar estado de reserva");
    Json(format!("✅ Reserva {} marcada como {}", id, nuevo_estado))
}

// =========================
// 🌍 CONFIGURACIÓN CORS
// =========================
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use rocket::http::Method;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let pool = db::establish_pool();

    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![
            Method::Get,
            Method::Post,
            Method::Put,
            Method::Delete,
            Method::Options
        ]
        .into_iter()
        .map(From::from)
        .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("❌ Error al crear configuración CORS");

    rocket::build()
        .attach(cors)
        .manage(pool)
        .mount(
            "/",
            routes![
                index,
                // CLIENTES
                listar_clientes,
                crear_cliente,
                // CABAÑAS
                listar_cabanas,
                crear_cabana,
                actualizar_estado_cabana,
                // RESERVAS
                listar_reservas,
                crear_reserva,
                eliminar_reserva,
                actualizar_estado_reserva
            ],
        )
        .attach(AdHoc::on_ignite("Database Migrations", |rocket| async {
            println!("✅ Conectado a la base de datos PostgreSQL con éxito!");
            rocket
        }))
}
