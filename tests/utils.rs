
// helper to clean db when testing
pub async fn clean_db(pool: &sqlx::PgPool) {
    sqlx::query!("TRUNCATE TABLE employees CASCADE").execute(pool).await.unwrap();
    sqlx::query!("TRUNCATE TABLE offices CASCADE").execute(pool).await.unwrap();
}