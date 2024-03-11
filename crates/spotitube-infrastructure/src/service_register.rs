use std::sync::Arc;

use spotitube_core::{
    config::AppConfig, users::service::DynUsersService, utils::token_service::DynTokenService,
};

use crate::{
    connection_pool::SpotitubeConnectionPool,
    repositories::users_repository::PostgresUsersRepository,
    services::{
        users_service::SpotitubeUsersService,
        utils::{argon_security_service::ArgonSecurityService, jwt_service::JwtService},
    },
};

pub struct ServiceRegister {
    pub users_service: DynUsersService,
    pub token_service: DynTokenService,
}

impl ServiceRegister {
    pub fn new(pool: SpotitubeConnectionPool, config: Arc<AppConfig>) -> Self {
        let security_service = Arc::new(ArgonSecurityService::new(config.clone()));
        let token_service = Arc::new(JwtService::new(config));

        let users_repository = Arc::new(PostgresUsersRepository::new(pool));
        let users_service = Arc::new(SpotitubeUsersService::new(
            users_repository,
            security_service,
            token_service.clone(),
        )) as DynUsersService;

        Self {
            users_service: users_service,
            token_service,
        }
    }
}
