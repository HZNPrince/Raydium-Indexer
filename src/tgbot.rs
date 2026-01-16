use crate::database::Trade;
use anyhow::{Context, Result};
use std::env;
use teloxide::prelude::*;
use teloxide::types::ParseMode;

// Initialize the bot
pub async fn initialize() -> Result<Bot> {
    let token = env::var("TELEGRAM_BOT_TOKEN").context("TELEGRAM_BOT_TOKEN not set")?;
    let bot = Bot::new(token);

    Ok(bot)
}

// The Alert Function
pub async fn send_trade_alert(bot: &Bot, trade: &Trade) -> Result<()> {
    let chat_id = env::var("TELEGRAM_CHAT_ID").context("TELEGRAM_CHAT_ID not set")?;

    // Emoji selection
    let (action, emoji) = if trade.mint_in == "So11111111111111111111111111111111111111112" {
        ("BUY", "🟢")
    } else if trade.mint_out == "So11111111111111111111111111111111111111112" {
        ("SELL", "🔴")
    } else {
        ("SWAP", "🔄")
    };

    let msg = format!(
        "{} <b>{}</b>\n\n
        🔻 <b>In:</b> {:.4} <code>{}</code>\n
        🟢 <b>Out:</b> {:.4} <code>{}</code>\n\n
        <a href=\"https://solscan.io/tx/{}\">View Transaction</a>",
        emoji,
        action,
        trade.amount_in,
        trade.mint_in,
        trade.amount_out,
        trade.mint_out,
        trade.signature
    );

    bot.send_message(chat_id, msg)
        .parse_mode(ParseMode::Html)
        .await?;

    Ok(())
}
