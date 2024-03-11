use std::sync::Arc;

use axum::async_trait;
use spotitube_domain::users::{
    requests::{LoginUserDto, RegisterUserDto},
    UserDto,
};
use uuid::Uuid;

use crate::errors::SpotitubeResult;

pub type DynUsersService = Arc<dyn UsersService + Send + Sync>;

#[async_trait]
pub trait UsersService {
    async fn register_user(&self, request: RegisterUserDto) -> SpotitubeResult<UserDto>;
    async fn login_user(&self, request: LoginUserDto) -> SpotitubeResult<UserDto>;
    async fn get_user(&self, user_id: &Uuid) -> SpotitubeResult<UserDto>;
}
