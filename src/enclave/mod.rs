pub mod secrets;

use secrets::SecretsManager;

pub struct EnclaveSecrets {
    manager: SecretsManager,
}

impl EnclaveSecrets {
    pub fn new() -> Self {
        EnclaveSecrets {
            manager: SecretsManager::new(),
        }
    }

    pub fn store_secret(&self, key: &str, value: &str) {
        self.manager.store_secret(key, value);
    }

    pub fn retrieve_secret(&self, key: &str) -> Option<String> {
        self.manager.retrieve_secret(key)
    }
}
