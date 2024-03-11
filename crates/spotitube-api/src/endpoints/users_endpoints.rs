use axum::{
    debug_handler,
    routing::{get, post},
    Extension, Form, Json, Router,
};
use spotitube_core::{errors::SpotitubeResult, users::service::DynUsersService};
use spotitube_domain::users::{
    requests::{LoginUserRequest, RegisterUserRequest},
    responses::UserAuthResponse,
};
use spotitube_infrastructure::service_register::ServiceRegister;
use tracing::info;

use crate::extractors::validation_extractor::ValidationExtractor;

pub struct UsersRouter;

impl UsersRouter {
    pub fn new_router(service_register: ServiceRegister) -> Router {
        Router::new()
            .route("/auth/register", get(UsersRouter::register_user_endpoint))
            .route("/auth/login", post(UsersRouter::login_user_endpoint))
            .layer(Extension(service_register.users_service))
    }

    pub async fn register_user_endpoint(
        Extension(users_service): Extension<DynUsersService>,
        ValidationExtractor(request): ValidationExtractor<RegisterUserRequest>,
    ) -> SpotitubeResult<Json<UserAuthResponse>> {
        info!(
            "received request to register user {:?}",
            request.user.username
        );
        let created_user = users_service.register_user(request.user).await?;
        Ok(Json(UserAuthResponse { user: created_user }))
    }

    pub async fn login_user_endpoint(
        Extension(users_service): Extension<DynUsersService>,
        ValidationExtractor(request): ValidationExtractor<LoginUserRequest>,
    ) -> SpotitubeResult<Json<UserAuthResponse>> {
        info!("received request to login user {:?}", request.user.username);
        let user = users_service.login_user(request.user).await?;
        Ok(Json(UserAuthResponse { user }))
    }
}
