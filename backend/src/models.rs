use diesel::prelude::*;
use crate::schema::*;
use serde::{Serialize, Deserialize};
use bigdecimal::BigDecimal;

// =============================
// 🧍 Tabla: clientes
// =============================
#[derive(Debug, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = clientes)]
pub struct Cliente {
    pub id: i32,
    pub nombre: String,
    pub telefono: Option<String>,
    pub email: Option<String>,
    pub dni: Option<String>,
    pub fecha_registro: Option<chrono::NaiveDateTime>,
    pub activo: Option<bool>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = clientes)]
pub struct NewCliente<'a> {
    pub nombre: &'a str,
    pub telefono: Option<&'a str>,
    pub email: Option<&'a str>,
    pub dni: Option<&'a str>,
    pub activo: Option<bool>,
}

// =============================
// 🏠 Tabla: cabanas
// =============================
#[derive(Debug, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = cabanas)]
pub struct Cabana {
    pub id: i32,
    pub nombre: String,
    pub capacidad: i32,
    pub ubicacion: Option<String>,
    pub estado: String,
    pub descripcion: Option<String>,
    pub precio_hora: Option<BigDecimal>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = cabanas)]
pub struct NewCabana<'a> {
    pub nombre: &'a str,
    pub capacidad: i32,
    pub ubicacion: Option<&'a str>,
    pub estado: &'a str,
    pub descripcion: Option<&'a str>,
    pub precio_hora: Option<BigDecimal>,
}

// =============================
// 📅 Tabla: reservas
// =============================
#[derive(Debug, Queryable, Identifiable, Associations, Serialize, Deserialize)]
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
    pub precio_total: Option<BigDecimal>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = reservas)]
pub struct NewReserva {
    pub cliente_id: i32,
    pub cabana_id: i32,
    pub fecha_reserva: chrono::NaiveDate,
    pub hora_inicio: chrono::NaiveTime,
    pub hora_fin: chrono::NaiveTime,
    pub estado: String,
    pub observaciones: Option<String>,
    pub precio_total: Option<BigDecimal>,
}

// =============================
// 💳 Tabla: pagos
// =============================
#[derive(Debug, Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(Reserva))]
#[diesel(table_name = pagos)]
pub struct Pago {
    pub id: i32,
    pub reserva_id: i32,
    pub monto: BigDecimal,
    pub metodo: String,
    pub estado: String,
    pub fecha_pago: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = pagos)]
pub struct NewPago {
    pub reserva_id: i32,
    pub monto: BigDecimal,
    pub metodo: String,
    pub estado: String,
}

// =============================
// 👨‍🍳 Tabla: empleados
// =============================
#[derive(Debug, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = empleados)]
pub struct Empleado {
    pub id: i32,
    pub nombre: String,
    pub cargo: String,
    pub telefono: Option<String>,
    pub email: Option<String>,
    pub fecha_registro: Option<chrono::NaiveDateTime>,
    pub activo: Option<bool>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = empleados)]
pub struct NewEmpleado<'a> {
    pub nombre: &'a str,
    pub cargo: &'a str,
    pub telefono: Option<&'a str>,
    pub email: Option<&'a str>,
    pub activo: Option<bool>,
}
