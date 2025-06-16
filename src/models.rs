use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub id: Uuid,
    pub username: String,
}

#[derive(Queryable, Serialize)]
pub struct Message {
    pub id: Uuid,
    pub sender_id: Uuid,
    pub content: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::messages)]
pub struct NewMessage {
    pub id: Uuid,
    pub sender_id: Uuid,
    pub content: String,
}
