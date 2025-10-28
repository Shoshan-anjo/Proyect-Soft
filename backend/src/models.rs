use diesel::prelude::*;
use crate::schema::*;
use serde::{Serialize, Deserialize};

// =============================
// 🧍 CLIENTES
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
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = clientes)]
pub struct NewCliente<'a> {
    pub nombre: &'a str,
    pub telefono: Option<&'a str>,
    pub email: Option<&'a str>,
    pub dni: Option<&'a str>,
}

// =============================
// 🏠 CABAÑAS
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
    pub precio_hora: Option<bigdecimal::BigDecimal>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = cabanas)]
pub struct NewCabana<'a> {
    pub nombre: &'a str,
    pub capacidad: i32,
    pub ubicacion: Option<&'a str>,
    pub estado: &'a str,
    pub descripcion: Option<&'a str>,
    pub precio_hora: Option<bigdecimal::BigDecimal>,
}

// =============================
// 📅 RESERVAS
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
}

// ⚙️ Valor por defecto para el campo "estado"
fn estado_por_defecto() -> String {
    "pendiente".to_string()
}


#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = reservas)]
pub struct NewReserva {
    pub cliente_id: i32,
    pub cabana_id: i32,
    pub fecha_reserva: chrono::NaiveDate,
    pub hora_inicio: chrono::NaiveTime,
    pub hora_fin: chrono::NaiveTime,
    #[serde(default = "estado_por_defecto")] // 👈 Valor por defecto si no se envía
    pub estado: String,
    pub observaciones: Option<String>,
}

