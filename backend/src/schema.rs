// @generated automatically by Diesel CLI.

diesel::table! {
    cabanas (id) {
        id -> Int4,
        #[max_length = 50]
        nombre -> Varchar,
        capacidad -> Int4,
        #[max_length = 100]
        ubicacion -> Nullable<Varchar>,
        #[max_length = 20]
        estado -> Varchar,
        descripcion -> Nullable<Text>,
        precio_hora -> Nullable<Numeric>,
    }
}

diesel::table! {
    clientes (id) {
        id -> Int4,
        #[max_length = 100]
        nombre -> Varchar,
        #[max_length = 20]
        telefono -> Nullable<Varchar>,
        #[max_length = 100]
        email -> Nullable<Varchar>,
        #[max_length = 30]
        dni -> Nullable<Varchar>,
        fecha_registro -> Nullable<Timestamp>,
    }
}

diesel::table! {
    reservas (id) {
        id -> Int4,
        cliente_id -> Int4,
        cabana_id -> Int4,
        fecha_reserva -> Date,
        hora_inicio -> Time,
        hora_fin -> Time,
        #[max_length = 20]
        estado -> Varchar,
        observaciones -> Nullable<Text>,
        fecha_creacion -> Nullable<Timestamp>,
    }
}

diesel::joinable!(reservas -> cabanas (cabana_id));
diesel::joinable!(reservas -> clientes (cliente_id));

diesel::allow_tables_to_appear_in_same_query!(cabanas, clientes, reservas,);
