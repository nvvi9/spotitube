use std::sync::Arc;

use uuid::Uuid;

use crate::errors::SpotitubeResult;

pub type DynTokenService = Arc<dyn TokenService + Send + Sync>;

pub trait TokenService {
    fn new_token(&self, user_id: &Uuid, username: &str) -> SpotitubeResult<String>;
    fn get_user_id_from_token(&self, token: String) -> SpotitubeResult<Uuid>;
}
