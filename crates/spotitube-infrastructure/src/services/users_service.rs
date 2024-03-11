use async_trait::async_trait;
use spotitube_core::{
    errors::{SpotitubeError, SpotitubeResult},
    users::{repository::DynUsersRepository, service::UsersService},
    utils::{security_service::DynSecurityService, token_service::DynTokenService},
};
use spotitube_domain::users::{
    requests::{LoginUserDto, RegisterUserDto},
    UserDto,
};
use tracing::error;
use uuid::Uuid;

pub struct SpotitubeUsersService {
    repository: DynUsersRepository,
    security_service: DynSecurityService,
    token_service: DynTokenService,
}

impl SpotitubeUsersService {
    pub fn new(
        repository: DynUsersRepository,
        security_service: DynSecurityService,
        token_service: DynTokenService,
    ) -> Self {
        Self {
            repository,
            security_service,
            token_service,
        }
    }
}

#[async_trait]
impl UsersService for SpotitubeUsersService {
    async fn register_user(&self, request: RegisterUserDto) -> SpotitubeResult<UserDto> {
        let username = request.username.unwrap();
        let password = request.password.unwrap();

        if let Some(existing_user) = self.repository.get_user_by_username(&username).await? {
            error!(
                "user with username {:?} already exists",
                existing_user.username
            );
            return Err(SpotitubeError::Conflict(String::from("username is taken")));
        };

        let hashed_password = self.security_service.hash_password(&password)?;
        let created_user = self
            .repository
            .create_user(&username, &hashed_password)
            .await?;

        let token = self
            .token_service
            .new_token(&created_user.id, &created_user.username)?;

        Ok(created_user.into_dto(token))
    }
    async fn login_user(&self, request: LoginUserDto) -> SpotitubeResult<UserDto> {
        let username = request.username.unwrap();
        let attempted_password = request.password.unwrap();

        let user = self
            .repository
            .get_user_by_username(&username)
            .await?
            .ok_or(SpotitubeError::NotFound(String::from(
                "username does not exist",
            )))?;

        let is_valid_password = self
            .security_service
            .verify_password(&user.password, &attempted_password)?;

        if is_valid_password {
            let token = self.token_service.new_token(&user.id, &user.username)?;
            Ok(user.into_dto(token))
        } else {
            Err(SpotitubeError::InvalidPassword)
        }
    }

    async fn get_user(&self, user_id: &Uuid) -> SpotitubeResult<UserDto> {
        let user = self.repository.get_user_by_id(user_id).await?;
        let token = self.token_service.new_token(&user.id, &user.username)?;

        Ok(user.into_dto(token))
    }
}
