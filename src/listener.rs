use anyhow::Result;
use futures::StreamExt;
use solana_client::pubsub_client::PubsubClient;
use solana_client::rpc_config::{
    CommitmentConfig, RpcTransactionLogsConfig, RpcTransactionLogsFilter,
};

const RAYDIUM_V4: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";

pub async fn start_listening() -> Result<()> {
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
                println!("New Tx: {}", response.value.signature);
                for log in response.value.logs {
                    println!("    {}", log);
                }
                println!("---------------------------------------------------");
            }
            Err(e) => {
                println!("Disconnected: {}", e);
                break;
            }
        }
    }

    Ok(())
}
