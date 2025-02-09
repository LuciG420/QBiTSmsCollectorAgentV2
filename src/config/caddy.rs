use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CaddyConfig {
    // Define the structure of your Caddy configuration
}

pub async fn load_caddy_config(client: &Client, config: &CaddyConfig) -> Result<(), Box<dyn std::error::Error>> {
    let response = client.post("http://localhost:2019/load")
        .json(&config)
        .send()
        .await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to load config")))
    }
}
