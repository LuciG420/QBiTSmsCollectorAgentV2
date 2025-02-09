pub mod caddy;

pub struct Config {
    pub caddy_url: String,
    pub db_path: String,
}

impl Config {
    pub fn new() -> Self {
        // Load configuration from environment or default values
        Config {
            caddy_url: std::env::var("CADDY_URL").unwrap_or("http://localhost:2019".to_string()),
            db_path: std::env::var("DB_PATH").unwrap_or("events.db".to_string()),
        }
    }
}
