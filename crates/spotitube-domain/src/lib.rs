use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod users;

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiError {
    pub errors: HashMap<String, Vec<String>>,
}

impl ApiError {
    pub fn from_str(error: &str) -> Self {
        let mut error_map: HashMap<String, Vec<String>> = HashMap::new();
        error_map.insert(String::from("message"), vec![String::from(error)]);
        Self { errors: error_map }
    }

    pub fn from_map(errors: HashMap<String, Vec<String>>) -> Self {
        Self { errors }
    }
}
