use std::sync::{Arc, Mutex};

pub struct AppConfig {
    // Define your configuration fields here
}

pub fn initialize_config() -> Arc<Mutex<AppConfig>> {
    // Initialize your configuration data and return it as a Mutex-wrapped Arc
    Arc::new(Mutex::new(AppConfig {
        // Initialize your configuration fields here
    }))
}
