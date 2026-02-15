use chrono::NaiveDateTime;
use diesel::{Queryable, Selectable, Insertable, AsChangeset, Identifiable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::schema::users;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    #[serde(skip_serializing)] // Never serialize password hash
    pub password_hash: String,
    pub is_active: bool,
    pub is_verified: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub last_login_at: Option<NaiveDateTime>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub password_hash: String,
}
