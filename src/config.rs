//! Server configuration

use std::env;

/// Struct to store server configuration
///
/// Currently only stores the port
pub struct Config {
    pub port: String,
}

impl Config {
    /// Load the config from environment variables
    pub fn load() -> Self {
        let port = env::var("PORT").unwrap_or("8080".to_string());

        Self { port }
    }
}
