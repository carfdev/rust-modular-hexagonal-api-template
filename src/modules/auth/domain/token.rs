use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid, // user_id
    pub session_id: Uuid,
    pub exp: usize,
    pub iat: usize,
    pub roles: Vec<String>,
}
