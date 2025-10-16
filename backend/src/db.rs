use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_pool() -> DbPool {
    let database_url = std::env::var("DATABASE_URL")
        .expect("❌ No se encontró la variable DATABASE_URL en el archivo .env");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("❌ Error al crear el pool de conexiones a la base de datos")
}
