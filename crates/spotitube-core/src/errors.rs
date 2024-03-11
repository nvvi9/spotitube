use std::collections::HashMap;

use axum::{http::StatusCode, response::IntoResponse, Json};
use spotitube_domain::ApiError;
use validator::ValidationErrorsKind;

pub type SpotitubeResult<T> = Result<T, SpotitubeError>;

#[derive(Debug)]
pub enum SpotitubeError {
    Unauthorized,
    InvalidUsername,
    InvalidPassword,
    Forbidden,
    AppStartup,
    NotFound(String),
    BadRequest(String),
    Conflict(String),
    InternalServerError,
    SqlxError(sqlx::error::Error),
    SqlxMigrateError(sqlx::migrate::MigrateError),
    ArgonError(argon2::Error),
    JwtError(jsonwebtoken::errors::Error),
    UuidError(uuid::Error),
    ValidationError(validator::ValidationErrors),
    FormRejection(axum::extract::rejection::FormRejection),
}

impl IntoResponse for SpotitubeError {
    fn into_response(self) -> axum::response::Response {
        let (status, api_error) = match self {
            SpotitubeError::Unauthorized => {
                (StatusCode::UNAUTHORIZED, ApiError::from_str("unauthorized"))
            }
            SpotitubeError::InvalidUsername => (
                StatusCode::BAD_REQUEST,
                ApiError::from_str("invalid username"),
            ),
            SpotitubeError::InvalidPassword => (
                StatusCode::BAD_REQUEST,
                ApiError::from_str("invalid password"),
            ),
            SpotitubeError::Conflict(err) => (StatusCode::CONFLICT, ApiError::from_str(&err)),
            SpotitubeError::NotFound(err) => (StatusCode::NOT_FOUND, ApiError::from_str(&err)),
            SpotitubeError::ValidationError(errors) => {
                let mut validation_errors = HashMap::new();
                for (_, error_kind) in errors.into_errors() {
                    if let ValidationErrorsKind::Struct(meta) = error_kind {
                        for (struct_property, struct_error_kind) in meta.into_errors() {
                            if let ValidationErrorsKind::Field(field_meta) = struct_error_kind {
                                for error in field_meta.into_iter() {
                                    let message = error
                                        .message
                                        .map(|c| c.into_owned())
                                        .unwrap_or(format!("{} is required", struct_property));

                                    validation_errors
                                        .entry(String::from(struct_property))
                                        .or_insert_with(Vec::new)
                                        .push(message);
                                }
                            }
                        }
                    }
                }

                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    ApiError::from_map(validation_errors),
                )
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiError::from_str("internal server error"),
            ),
        };

        let body = Json(api_error);
        (status, body).into_response()
    }
}

impl From<sqlx::error::Error> for SpotitubeError {
    fn from(value: sqlx::error::Error) -> Self {
        Self::SqlxError(value)
    }
}

impl From<sqlx::migrate::MigrateError> for SpotitubeError {
    fn from(value: sqlx::migrate::MigrateError) -> Self {
        Self::SqlxMigrateError(value)
    }
}

impl From<argon2::Error> for SpotitubeError {
    fn from(value: argon2::Error) -> Self {
        Self::ArgonError(value)
    }
}

impl From<jsonwebtoken::errors::Error> for SpotitubeError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Self::JwtError(value)
    }
}

impl From<uuid::Error> for SpotitubeError {
    fn from(value: uuid::Error) -> Self {
        Self::UuidError(value)
    }
}

impl From<validator::ValidationErrors> for SpotitubeError {
    fn from(value: validator::ValidationErrors) -> Self {
        Self::ValidationError(value)
    }
}

impl From<axum::extract::rejection::FormRejection> for SpotitubeError {
    fn from(value: axum::extract::rejection::FormRejection) -> Self {
        Self::FormRejection(value)
    }
}
