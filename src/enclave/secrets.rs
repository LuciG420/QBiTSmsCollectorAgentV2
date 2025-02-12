use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct SecretsManager {
    secrets: Arc<Mutex<HashMap<String, String>>>,
}

impl SecretsManager {
    pub fn new() -> Self {
        SecretsManager {
            secrets: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn store_secret(&self, key: &str, value: &str) {
        let mut secrets = self.secrets.lock().unwrap();
        secrets.insert(key.to_string(), value.to_string());
    }

    pub fn retrieve_secret(&self, key: &str) -> Option<String> {
        let secrets = self.secrets.lock().unwrap();
        secrets.get(key).cloned()
    }
}
