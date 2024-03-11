use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod requests;
pub mod responses;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserDto {
    pub id: Uuid,
    pub username: String,
    pub token: String,
}
