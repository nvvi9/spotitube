use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterUserRequest {
    #[validate]
    pub user: RegisterUserDto,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginUserRequest {
    #[validate]
    pub user: LoginUserDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RegisterUserDto {
    #[validate(required, length(min = 1))]
    pub username: Option<String>,
    #[validate(required, length(min = 8))]
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LoginUserDto {
    #[validate(required, length(min = 1))]
    pub username: Option<String>,
    #[validate(required, length(min = 8))]
    pub password: Option<String>,
}
