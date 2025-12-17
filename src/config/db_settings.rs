use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

/// Database configuration
/// Loads settings from environment variables and creates a connection pool.
/// Expected environment variables:
/// - POSTGRES_USER
/// - POSTGRES_PASSWORD
/// - POSTGRES_DB


#[derive(Debug, Clone)]
pub struct Settings {
    pub database_url: String,
}

impl Settings {
    // Loads environment varaibles and formats DB URL
    // Panics if any required lvalue variable is missing
    pub fn connect_from_env() -> anyhow::Result<Self> {
        tracing::info!("Loading database configuration: user, password, db");
        
        let user = std::env::var("POSTGRES_USER").expect("POSTGRES_USER must be set in env");
        let password = std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set in env");
        let db = std::env::var("POSTGRES_DB").expect("POSTGRES_DB must be set in env");

        let database_url = format!("postgres://{}:{}@127.0.0.1:5432/{}", user, password, db);

        Ok(Self { database_url })
    }

    // Creates a Postgres connection pool with a maxiumum of 5 connections using the DB url
    pub async fn create_pool(&self) -> anyhow::Result<PgPool> {
        tracing::info!("Creating Postgres connection pool"); 

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&self.database_url)
            .await?;

        tracing::info!("Database connection pool established.");
        Ok(pool)
    }
}

