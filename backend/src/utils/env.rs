use tracing::{event, Level};

pub fn get_var(key: &str) -> Option<String> {
    match std::env::var(key) {
        Ok(value) => {
            event!(Level::DEBUG, "Found env var {}", key);
            Some(value)
        }
        Err(_) => {
            event!(Level::WARN, "Couldn't find env var {}", key);
            None
        }
    }
}
