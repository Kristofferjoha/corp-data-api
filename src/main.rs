use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

mod config {
    pub mod db_settings;
}

mod domain {
    pub mod office;
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let settings = config::db_settings::Settings::connect_from_env();
    println!("DB URL: {}", settings.database_url);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&settings.database_url)
        .await?;
    
    println!("Successfully connected to DB.");

    Ok(())
}