use std::sync::Arc;

use crate::errors::SpotitubeResult;

pub type DynSecurityService = Arc<dyn SecurityService + Send + Sync>;

pub trait SecurityService {
    fn hash_password(&self, raw_password: &str) -> SpotitubeResult<String>;
    fn verify_password(
        &self,
        stored_password: &str,
        attempted_password: &str,
    ) -> SpotitubeResult<bool>;
}
