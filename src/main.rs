mod database;
mod listener;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to DB
    let pool = database::connect().await?;

    println!("Main Database pool established: {:#?}", pool);

    // Start the listener
    println!("Starting the Spy...");
    listener::start_listening().await?;

    Ok(())
}
