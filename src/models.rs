use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Evento {
    pub id: i32,
    pub nombre: String,
    pub fecha: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usuario {
    pub id: i32,
    pub nombre: String,
    pub correo: Option<String>,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asiento {
    pub id: i32,
    pub evento_id: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reserva {
    pub id: i32,
    pub usuario_id: i32,
    pub evento_id: i32,
    pub asiento_id: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}
