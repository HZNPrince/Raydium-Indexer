use anyhow::Result;
use futures::StreamExt;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::pubsub_client::PubsubClient;
use solana_client::rpc_config::{
    CommitmentConfig, RpcTransactionLogsConfig, RpcTransactionLogsFilter,
};
use sqlx::{Pool, Postgres};

use crate::{database, processor};

const RAYDIUM_V4: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";

pub async fn start_listening(pool: Pool<Postgres>) -> Result<()> {
    // Setup - HTTP Client (For fetching full Tx)
    let http_url = "https://api.mainnet-beta.solana.com/";
    let rpc_client = RpcClient::new(http_url.to_string());

    // Setup - WebSocket Client (For listening to logs)
    let wss_url = "wss://api.mainnet-beta.solana.com";
    println!("Connection via WSS to {}", wss_url);

    let (_subscription, receiver) = PubsubClient::logs_subscribe(
        wss_url,
        RpcTransactionLogsFilter::Mentions(vec![RAYDIUM_V4.to_string()]),
        RpcTransactionLogsConfig {
            commitment: Some(CommitmentConfig::confirmed()),
        },
    )?;

    println!("Listening for Trades...");

    loop {
        match receiver.recv() {
            Ok(response) => {
                let mut is_swap = false;
                for log in &response.value.logs {
                    if log.contains("Instruction: Swap") {
                        is_swap = true;
                        break;
                    }
                }

                if is_swap {
                    println!(" 🔎 Swap detected : {}", response.value.signature);

                    // Trigger the processor
                    match processor::parse_trade(&rpc_client, &response.value.signature).await {
                        Ok(Some(trade)) => {
                            // Call database
                            match database::add_trade(&pool, trade).await {
                                Ok(id) => println!("   ✅ Saved to DB: ID {}", id),
                                Err(e) => println!("   ❌ DB Error: {}", e),
                            }
                        }
                        Ok(None) => {}

                        Err(e) => println!("Error parsing trade {}", e),
                    }
                }
            }
            Err(e) => {
                println!("Disconnected: {}", e);
                break;
            }
        }
    }

    Ok(())
}
