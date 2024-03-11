use std::{str::FromStr, sync::Arc};

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use spotitube_core::{
    config::AppConfig, errors::SpotitubeResult, utils::token_service::TokenService,
};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    user_id: String,
}

pub struct JwtService {
    config: Arc<AppConfig>,
}

impl JwtService {
    pub fn new(config: Arc<AppConfig>) -> Self {
        Self { config }
    }
}

impl TokenService for JwtService {
    fn new_token(&self, user_id: &Uuid, username: &str) -> SpotitubeResult<String> {
        let claims = Claims {
            sub: String::from(username),
            user_id: user_id.to_string(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.token_secret.as_bytes()),
        )?;

        Ok(token)
    }

    fn get_user_id_from_token(&self, token: String) -> SpotitubeResult<Uuid> {
        let decoded_token = decode::<Claims>(
            token.as_str(),
            &DecodingKey::from_secret(self.config.token_secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )?;

        let user_id = Uuid::from_str(&decoded_token.claims.user_id)?;
        Ok(user_id)
    }
}
