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

    let office = domain::office::Office::new(1, "Silkeborg".to_string(), 1);
    match office {
        Ok(o) => println!("Created office: {:?}", o),
        Err(e) => eprintln!("Failed to create office: {}", e),
    }

    Ok(())
}