use std::sync::Arc;

use argon2::Config;
use spotitube_core::{
    config::AppConfig, errors::SpotitubeResult, utils::security_service::SecurityService,
};

pub struct ArgonSecurityService {
    config: Arc<AppConfig>,
}

impl ArgonSecurityService {
    pub fn new(config: Arc<AppConfig>) -> Self {
        Self { config }
    }
}

impl SecurityService for ArgonSecurityService {
    fn hash_password(&self, raw_password: &str) -> SpotitubeResult<String> {
        let hashed_password = argon2::hash_encoded(
            raw_password.as_bytes(),
            self.config.argon_salt.as_bytes(),
            &Config::default(),
        )?;
        Ok(hashed_password)
    }

    fn verify_password(
        &self,
        stored_password: &str,
        attempted_password: &str,
    ) -> SpotitubeResult<bool> {
        let hashes_match = argon2::verify_encoded(stored_password, attempted_password.as_bytes())?;
        Ok(hashes_match)
    }
}
