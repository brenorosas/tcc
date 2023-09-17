use tracing::{event, Level};

pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub dbname: String,
}

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

impl PostgresConfig {
    pub fn new() -> Self {
        Self {
            host: get_var("DB_HOST").unwrap_or_else(|| "localhost".to_string()),
            port: get_var("DB_PORT")
                .unwrap_or_else(|| "5432".to_string())
                .parse::<u16>()
                .unwrap(),
            user: get_var("DB_USER").unwrap_or_else(|| "tcc".to_string()),
            password: get_var("DB_PASS").unwrap_or_else(|| "tcc".to_string()),
            dbname: get_var("DB_NAME").unwrap_or_else(|| "tcc".to_string()),
        }
    }
}
