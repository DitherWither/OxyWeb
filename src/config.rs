use std::env;

pub struct Config {
    pub port: String
}

impl Config {
    pub fn load() -> Self {
        let port = env::var("PORT").unwrap_or("8080".to_string());

        Self {
            port
        }
    }
}