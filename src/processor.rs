use crate::database::Trade;
use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::CommitmentConfig;
use solana_sdk::signature::Signature;
use std::str::FromStr;

const WSOL: &str = "So11111111111111111111111111111111111111112";

pub async fn parse_trade(rpc_client: &RpcClient, signature: &str) -> Result<Option<Trade>> {
    let sig = match Signature::from_str(signature) {
        Ok(s) => s,
        Err(_) => return Ok(None),
    };

    let tx = rpc_client
        .get_transaction_with_config(
            &sig,
            solana_client::rpc_config::RpcTransactionConfig {
                encoding: Some(solana_transaction_status::UiTransactionEncoding::JsonParsed),
                commitment: Some(CommitmentConfig::confirmed()),
                max_supported_transaction_version: Some(0),
            },
        )
        .await?;

    let meta = match tx.transaction.meta {
        Some(m) => m,
        None => return Ok(None),
    };

    let pre_balances = meta.pre_token_balances.unwrap_or_else(|| Vec::new());
    let post_balances = meta.post_token_balances.unwrap_or_else(|| Vec::new());

    let mut sol_change = 0.0;
    let mut token_change = 0.0;
    let mut token_address = String::new();

    for post in post_balances.iter() {
        if let Some(pre) = pre_balances
            .iter()
            .find(|p| p.account_index == post.account_index)
        {
            let pre_amount = pre.ui_token_amount.ui_amount.unwrap_or(0.0);
            let post_amount = post.ui_token_amount.ui_amount.unwrap_or(0.0);

            let diff = post_amount - pre_amount;

            if diff == 0.0 {
                continue;
            }

            if post.mint == WSOL {
                sol_change += diff;
            } else {
                token_change += diff;
                token_address = post.mint.clone();
            }
        }
    }

    if token_address.is_empty() || sol_change == 0.0 || token_change == 0.0 {
        return Ok(None);
    }

    let is_buy = sol_change < 0.0;

    let trade = Trade {
        signature: signature.to_string(),
        token_address,
        is_buy,
        amount_sol: sol_change.abs(),
        amount_token: token_change.abs(),
    };

    println!(
        "  🚀 Parsed: {} {:.2} SOL for {:.2} Token",
        if is_buy { "BUY" } else { "SELL" },
        trade.amount_sol,
        trade.amount_token
    );

    Ok(Some(trade))
}
