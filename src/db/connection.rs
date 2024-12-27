use sqlx::{PgPool, Pool, Postgres};

pub async fn establish_connection() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    Pool::<Postgres>::connect(&database_url).await.unwrap()
}