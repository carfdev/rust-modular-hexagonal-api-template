use chrono::NaiveDateTime;
use diesel::{Queryable, Selectable, Insertable, Identifiable, Associations};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::schema::user_sessions;
use crate::modules::users::domain::entity::User;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(table_name = user_sessions)]
pub struct UserSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub refresh_token_hash: String,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub device_name: Option<String>,
    pub is_revoked: bool,
    pub expires_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub last_used_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = user_sessions)]
pub struct NewUserSession {
    pub user_id: Uuid,
    pub refresh_token_hash: String,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub device_name: Option<String>,
    pub expires_at: NaiveDateTime,
}
pub mod token;
