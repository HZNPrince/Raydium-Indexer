mod database;
mod listener;
mod processor;
mod tgbot;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to DB
    let pool = database::connect().await?;

    // Initialize Bot
    let bot = tgbot::initialize().await?;

    println!("Main Database pool established: {:#?}", pool);

    println!("-------- STARTING THE SPY --------");
    listener::start_listening(pool, bot).await?;

    Ok(())
}
