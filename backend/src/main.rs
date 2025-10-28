#[macro_use]
extern crate rocket;

use rocket::{serde::json::Json, State};
use rocket::fairing::AdHoc;
use dotenvy::dotenv;
use std::time::Duration;

mod schema;
mod models;
mod db;
mod services;
mod websocket;

use db::DbPool;
use models::{Reserva, NewReserva, Cliente, NewCliente, Cabana, NewCabana};
use services::{reservas_service, clientes_service, cabanas_service};
use websocket::{Broadcaster, ws};

// =========================
// 🚀 BASE
// =========================
#[get("/")]
fn index() -> &'static str {
    "🚀 API de Reservas Dubai Resto Bar — en tiempo real con SSE y actualización automática"
}

// =========================
// 🧍 CLIENTES
// =========================
#[get("/clientes")]
fn listar_clientes(pool: &State<DbPool>) -> Json<Vec<Cliente>> {
    let mut conn = pool.get().expect("No se pudo obtener conexión del pool");
    let results = clientes_service::listar_clientes(&mut conn).expect("Error al listar clientes");
    Json(results)
}

#[post("/clientes", format = "json", data = "<nuevo_cliente>")]
fn crear_cliente(pool: &State<DbPool>, nuevo_cliente: Json<NewCliente>) -> Json<Cliente> {
    let mut conn = pool.get().expect("No se pudo obtener conexión del pool");
    let cliente =
        clientes_service::crear_cliente(&mut conn, nuevo_cliente.into_inner()).expect("Error al crear cliente");
    Json(cliente)
}

// =========================
// 🏠 CABAÑAS
// =========================
#[get("/cabanas")]
fn listar_cabanas(pool: &State<DbPool>) -> Json<Vec<Cabana>> {
    let mut conn = pool.get().expect("No se pudo obtener conexión del pool");
    let result = cabanas_service::listar_cabanas(&mut conn).expect("Error al listar cabañas");
    Json(result)
}

#[post("/cabanas", format = "json", data = "<nueva_cabana>")]
fn crear_cabana(
    pool: &State<DbPool>,
    nueva_cabana: Json<NewCabana>,
    broadcaster: &State<Broadcaster>,
) -> Json<Cabana> {
    let mut conn = pool.get().expect("No se pudo obtener conexión del pool");
    let cab = cabanas_service::crear_cabana(&mut conn, nueva_cabana.into_inner())
        .expect("Error al crear cabaña");
    broadcaster.send("actualizar");
    Json(cab)
}

#[put("/cabanas/<cabana_id>/<nuevo_estado>")]
fn actualizar_estado_cabana(
    pool: &State<DbPool>,
    cabana_id: i32,
    nuevo_estado: &str,
    broadcaster: &State<Broadcaster>,
) -> Json<Cabana> {
    let mut conn = pool.get().expect("No se pudo obtener conexión del pool");
    let cab =
        cabanas_service::actualizar_estado(&mut conn, cabana_id, nuevo_estado).expect("Error al actualizar estado");
    broadcaster.send("actualizar");
    Json(cab)
}

// =========================
// 📅 RESERVAS
// =========================
#[get("/reservas")]
fn listar_reservas(pool: &State<DbPool>) -> Json<Vec<Reserva>> {
    let mut conn = pool.get().expect("No se pudo obtener conexión del pool");
    let results = reservas_service::listar_reservas(&mut conn).expect("Error al listar reservas");
    Json(results)
}

#[post("/reservas", format = "json", data = "<nueva_reserva>")]
fn crear_reserva(
    pool: &State<DbPool>,
    nueva_reserva: Json<NewReserva>,
    broadcaster: &State<Broadcaster>,
) -> Result<Json<Reserva>, Json<serde_json::Value>> {
    let mut conn = pool.get().expect("No se pudo obtener conexión del pool");

    match reservas_service::crear_reserva(&mut conn, nueva_reserva.into_inner()) {
        Ok(reserva) => {
            broadcaster.send("actualizar");
            Ok(Json(reserva))
        }
        Err(diesel::result::Error::RollbackTransaction) => Err(Json(serde_json::json!({
            "error": "⚠️ Conflicto de horario: ya existe una reserva en ese rango."
        }))),
        Err(e) => Err(Json(serde_json::json!({
            "error": format!("❌ Error inesperado: {:?}", e)
        }))),
    }
}

#[delete("/reservas/<id>")]
fn eliminar_reserva(
    pool: &State<DbPool>,
    id: i32,
    broadcaster: &State<Broadcaster>,
) -> Json<String> {
    let mut conn = pool.get().expect("No se pudo obtener conexión del pool");
    reservas_service::eliminar_reserva(&mut conn, id).expect("Error al eliminar reserva");
    broadcaster.send("actualizar");
    Json(format!("🗑️ Reserva {} eliminada correctamente", id))
}

#[put("/reservas/<id>/<nuevo_estado>")]
fn actualizar_estado_reserva(
    pool: &State<DbPool>,
    id: i32,
    nuevo_estado: &str,
    broadcaster: &State<Broadcaster>,
) -> Json<String> {
    let mut conn = pool.get().expect("No se pudo obtener conexión del pool");
    reservas_service::actualizar_estado_reserva(&mut conn, id, nuevo_estado)
        .expect("Error al actualizar estado de reserva");
    broadcaster.send("actualizar");
    Json(format!("✅ Reserva {} marcada como {}", id, nuevo_estado))
}

// =========================
// 🌍 CORS + ACTUALIZACIÓN AUTOMÁTICA
// =========================
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use rocket::http::Method;
use rocket::tokio;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let pool = db::establish_pool();
    let broadcaster = websocket::Broadcaster::new();

    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete, Method::Options]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Error al crear configuración CORS");

    rocket::build()
        .attach(cors)
        .manage(pool.clone())
        .manage(broadcaster.clone())
        .mount(
            "/",
            routes![
                index,
                listar_clientes,
                crear_cliente,
                listar_cabanas,
                crear_cabana,
                actualizar_estado_cabana,
                listar_reservas,
                crear_reserva,
                eliminar_reserva,
                actualizar_estado_reserva
            ],
        )
        .mount("/ws", routes![ws::ws])
        .attach(AdHoc::on_liftoff("Background Updater", move |_| {
            let pool_clone = pool.clone();
            let bc_clone = broadcaster.clone();
            Box::pin(async move {
                tokio::spawn(async move {
                    loop {
                        //ESPERA DE TIEMPO PARA LA ACTUALIZACIÓN AUTOMÁTICA
                        tokio::time::sleep(Duration::from_secs(5)).await;
                        if let Ok(mut conn) = pool_clone.get() {
                            match reservas_service::actualizar_estados_automaticos(&mut conn) {
                                Ok(_) => {
                                    bc_clone.send("actualizar");
                                    println!("⏱️ Estados actualizados automáticamente.");
                                }
                                Err(e) => eprintln!("⚠️ Error al actualizar estados automáticos: {:?}", e),
                            }
                        }
                    }
                });
            })
        }))
}
