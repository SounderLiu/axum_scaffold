use sqlx::postgres::PgPoolOptions;

pub async fn establish_connection(database_url: &str) -> Result<sqlx::PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    Ok(pool)
}