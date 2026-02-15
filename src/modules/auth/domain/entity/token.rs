use chrono::NaiveDateTime;
use diesel::{Queryable, Selectable, Insertable, Identifiable, Associations};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::schema::{email_verification_tokens, password_reset_tokens};
use crate::modules::users::domain::entity::User;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(table_name = email_verification_tokens)]
pub struct EmailVerificationToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: NaiveDateTime,
    pub used: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = email_verification_tokens)]
pub struct NewEmailVerificationToken {
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(table_name = password_reset_tokens)]
pub struct PasswordResetToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: NaiveDateTime,
    pub used: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = password_reset_tokens)]
pub struct NewPasswordResetToken {
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: NaiveDateTime,
}
