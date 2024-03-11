use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::UserDto;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserAuthResponse {
    pub user: UserDto,
}

impl UserAuthResponse {
    pub fn new(id: Uuid, username: String, token: String) -> Self {
        Self {
            user: UserDto {
                id: id,
                username: username,
                token: token,
            },
        }
    }
}
