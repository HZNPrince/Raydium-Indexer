use crate::database::Trade;
use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::{CommitmentConfig, RpcTransactionConfig};
use solana_client::rpc_response::OptionSerializer;
use solana_sdk::signature::Signature;
use std::collections::HashMap;
use std::str::FromStr;

const WSOL: &str = "So11111111111111111111111111111111111111112";

pub async fn parse_trade(rpc_client: &RpcClient, signature: &str) -> Result<Option<Trade>> {
    // Validate Signature
    let sig = match Signature::from_str(signature) {
        Ok(s) => s,
        Err(_) => return Ok(None),
    };

    // Fetch Transaction
    let tx = rpc_client
        .get_transaction_with_config(
            &sig,
            RpcTransactionConfig {
                encoding: Some(solana_transaction_status::UiTransactionEncoding::JsonParsed),
                commitment: Some(CommitmentConfig::confirmed()),
                max_supported_transaction_version: Some(0),
            },
        )
        .await?;

    // Find the Signer (User)
    // situated transaction -> message -> account_keys to first key (payer)
    let transaction = match tx.transaction.transaction {
        solana_transaction_status::EncodedTransaction::Json(t) => t,
        _ => return Ok(None),
    };

    let message = match transaction.message {
        solana_transaction_status::UiMessage::Parsed(m) => m,
        _ => return Ok(None),
    };
    let signer = match message.account_keys.first() {
        Some(acc) => acc,
        None => return Ok(None),
    };
    // Calculate Net Changes
    // We use a Map because a user might have multiple accounts for the same token
    let mut net_changes: HashMap<String, f64> = HashMap::new();

    let meta = match tx.transaction.meta {
        Some(m) => m,
        None => return Ok(None),
    };

    // Native SOL changes
    let pre_sol = meta.pre_balances.get(0).copied().unwrap_or(0);
    let post_sol = meta.post_balances.get(0).copied().unwrap_or(0);
    let fee = meta.fee;

    let sol_diff_lamports = (post_sol as i64 - pre_sol as i64) + fee as i64;
    let sol_diff = sol_diff_lamports as f64 / 1_000_000_000.0;

    if sol_diff.abs() > 0.000001 {
        net_changes.insert(WSOL.to_string(), sol_diff);
    }

    // For Token Changes
    let pre_balance = meta.pre_token_balances.unwrap_or_else(|| Vec::new());
    let post_balance = meta.post_token_balances.unwrap_or_else(|| Vec::new());

    for post in post_balance.iter() {
        // filter accounts owned by signer
        let owner = match &post.owner {
            OptionSerializer::Some(o) => o,
            _ => continue,
        };
        if owner != &signer.pubkey {
            continue;
        }

        // Find the matching Before state
        let pre = pre_balance
            .iter()
            .find(|p| p.account_index == post.account_index);
        let pre_amount = pre.and_then(|p| p.ui_token_amount.ui_amount).unwrap_or(0.0);
        let post_amount = post.ui_token_amount.ui_amount.unwrap_or(0.0);

        let diff = post_amount - pre_amount;
        if diff == 0.0 {
            continue;
        }

        // Add to our map
        let entry = net_changes.entry(post.mint.clone()).or_insert(0.0);
        *entry += diff;
    }

    // Sort into Input (Given) vs Output (Received)
    let mut mint_in = String::new();
    let mut amount_in = 0.0;

    let mut mint_out = String::new();
    let mut amount_out = 0.0;

    for (mint, change) in net_changes {
        if change < 0.0 {
            if change.abs() > amount_in {
                amount_in = change.abs();
                mint_in = mint;
            }
        } else {
            if change > amount_out {
                amount_out = change;
                mint_out = mint;
            }
        }
    }

    if mint_in.is_empty() && mint_out.is_empty() {
        println!("Not valid trade");
        return Ok(None);
    }

    if mint_in.is_empty() {
        mint_in = "UNKNOWN".to_string();
    }
    if mint_out.is_empty() {
        mint_out = "UNKNOWN".to_string();
    }

    let trade = Trade {
        signature: signature.to_string(),
        mint_in: mint_in.clone(),
        amount_in,
        mint_out: mint_out.to_string(),
        amount_out,
        block_time: tx.block_time.unwrap_or(0),
    };
    println!(
        " 🔄 Swap: -{:.4} {}  -->  +{:.4} {}",
        trade.amount_in,
        // Helper to show "SOL" instead of the long address
        if trade.mint_in == WSOL {
            "SOL"
        } else {
            &trade.mint_in[..4]
        },
        trade.amount_out,
        if trade.mint_out == WSOL {
            "SOL"
        } else {
            &trade.mint_out[..4]
        }
    );

    Ok(Some(trade))
}
