use std::fs;

use sqlx::{Pool, Postgres};

pub async fn get_db_url(
    db_user: &str,
    db_password: &str,
    db_host: &str,
    db_port: &str,
    db_name: &str,
) -> String {
    format!(
        "postgres://{}:{}@{}:{}/{}",
        db_user,
        db_password,
        db_host,
        db_port,
        db_name,
    )
}

pub async fn init_db(db_url: &str) -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
}

pub async fn run_migrations(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let mut migrator = sqlx::migrate!();
    migrator
        .run(pool)
        .await
        .expect("Failed to run migrations");
    Ok(())
}
