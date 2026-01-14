use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::env;

pub type DbPool = Pool<Postgres>;

pub async fn connect() -> Result<DbPool> {
    dotenv::dotenv().ok();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("Attempting to connect to database...");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;

    println!("Connected to Database!");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS trades (
        id SERIAL PRIMARY KEY,
        block_time BIGINT,
        signature TEXT NOT NULL,
        token_address TEXT NOT NULL,
        is_buy BOOLEAN,
        amount_sol DOUBLE PRECISION,
        amount_token DOUBLE PRECISION
        );
        "#,
    )
    .execute(&pool)
    .await?;

    println!("Schema ensured (Table 'trades' exists)");

    Ok(pool)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trade {
    pub signature: String,
    pub token_address: String,
    pub is_buy: bool,
    pub amount_sol: f64,
    pub amount_token: f64,
}
