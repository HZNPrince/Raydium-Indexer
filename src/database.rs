use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::env;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trade {
    pub signature: String,
    pub mint_in: String,
    pub amount_in: f64,
    pub mint_out: String,
    pub amount_out: f64,
    pub block_time: i64,
}

pub async fn connect() -> Result<Pool<Postgres>> {
    dotenv::dotenv().ok();
    let url = env::var("DATABASE_URL").expect("Database url must be set");

    println!("Connecting to DB (Postgres)...");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;
    println!("Connected to DB (Postgres) !");

    sqlx::query("DROP TABLE IF EXISTS trades")
        .execute(&pool)
        .await;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS trades (
        id SERIAL PRIMARY KEY,
        signature TEXT NOT NULL,
        mint_in TEXT NOT NULL,
        amount_in DOUBLE PRECISION,
        mint_out TEXT NOT NULL,
        amount_out DOUBLE PRECISION,
        block_time BIGINT
        )
        "#,
    )
    .execute(&pool)
    .await?;

    println!("Schema Setup Complete :)");
    Ok(pool)
}
