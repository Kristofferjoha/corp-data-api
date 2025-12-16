use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Settings {
    pub database_url: String,
}

impl Settings {
    pub fn connect_from_env() -> Self {
        dotenv().ok();
        let database_url =
            env::var("DATABASE_URL").expect("DATABASE_URL not found in environment vars");
        Self { database_url }
    }
}
