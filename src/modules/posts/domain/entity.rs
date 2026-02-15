use diesel::{Queryable, Selectable, Insertable, Identifiable, Associations};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::schema::posts;
use crate::modules::users::domain::entity::User;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(User, foreign_key = author_id))]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub is_published: bool,
    pub author_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
    pub content: String,
    pub author_id: Uuid,
}
