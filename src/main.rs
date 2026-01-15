mod database;
mod listener;
mod processor;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to DB
    let pool = database::connect().await?;

    println!("Main Database pool established: {:#?}", pool);

    println!("-------- STARTING THE SPY --------");
    listener::start_listening().await?;

    Ok(())
}
