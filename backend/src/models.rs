use diesel::prelude::*;
use diesel::sql_types::*;
use crate::schema::*;

// =============================
// 🧍 Tabla: clientes
// =============================
#[derive(Debug, Queryable, Insertable, Identifiable, Selectable)]
#[diesel(table_name = clientes)]
pub struct Cliente {
    pub id: i32,
    pub nombre: String,
    pub telefono: Option<String>,
    pub email: Option<String>,
    pub dni: Option<String>,
    pub fecha_registro: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = clientes)]
pub struct NewCliente<'a> {
    pub nombre: &'a str,
    pub telefono: Option<&'a str>,
    pub email: Option<&'a str>,
    pub dni: Option<&'a str>,
}

// =============================
// 🏠 Tabla: cabanas
// =============================
#[derive(Debug, Queryable, Insertable, Identifiable, Selectable)]
#[diesel(table_name = cabanas)]
pub struct Cabana {
    pub id: i32,
    pub nombre: String,
    pub capacidad: i32,
    pub ubicacion: Option<String>,
    pub estado: String,
    pub descripcion: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = cabanas)]
pub struct NewCabana<'a> {
    pub nombre: &'a str,
    pub capacidad: i32,
    pub ubicacion: Option<&'a str>,
    pub estado: &'a str,
    pub descripcion: Option<&'a str>,
}

// =============================
// 📅 Tabla: reservas
// =============================
#[derive(Debug, Queryable, Insertable, Identifiable, Associations, Selectable)]
#[diesel(belongs_to(Cliente))]
#[diesel(belongs_to(Cabana))]
#[diesel(table_name = reservas)]
pub struct Reserva {
    pub id: i32,
    pub cliente_id: i32,
    pub cabana_id: i32,
    pub fecha_reserva: chrono::NaiveDate,
    pub hora_inicio: chrono::NaiveTime,
    pub hora_fin: chrono::NaiveTime,
    pub estado: String,
    pub observaciones: Option<String>,
    pub fecha_creacion: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = reservas)]
pub struct NewReserva {
    pub cliente_id: i32,
    pub cabana_id: i32,
    pub fecha_reserva: chrono::NaiveDate,
    pub hora_inicio: chrono::NaiveTime,
    pub hora_fin: chrono::NaiveTime,
    pub estado: String,
    pub observaciones: Option<String>,
}

// =============================
// 💳 Tabla: pagos
// =============================
#[derive(Debug, Queryable, Insertable, Identifiable, Associations, Selectable)]
#[diesel(belongs_to(Reserva))]
#[diesel(table_name = pagos)]
pub struct Pago {
    pub id: i32,
    pub reserva_id: i32,
    pub monto: bigdecimal::BigDecimal,
    pub metodo: String,
    pub estado: String,
    pub fecha_pago: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = pagos)]
pub struct NewPago {
    pub reserva_id: i32,
    pub monto: bigdecimal::BigDecimal,
    pub metodo: String,
    pub estado: String,
}
