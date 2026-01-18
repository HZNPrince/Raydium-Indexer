#  🚀 DEX Tracker | Raydium Indexer

A high-performance Rust application that monitors **Raydium V4** swaps on the Solana blockchain in real-time. It detects trades, parses token balance changes, persists data to **PostgreSQL**, and sends instant alerts via **Telegram**.

## 🏗 Architecture

![System Architecture](assets/Architecture.png)

- **Listener**: Handles WebSocket stream and orchestrates the event loop.
- **Processor**: Parses raw Solana transactions to extract semantic trade data.
- **Database**: Persists trade records.
- **Telegram**: Broadcasting layer for user alerts.

## ✨ Features

- **Real-time Monitoring**: Subscribes to Solana WebSocket logs (`logsSubscribe`) for millisecond-latency swap detection.
- **Deep Analysis**: Fetches and parses full transaction details (RPC) to calculate net token changes and precise buy/sell amounts.
- **Data Persistence**: Automatically manages schema and stores structured trade history in PostgreSQL using `sqlx`.
- **Instant Alerts**: Sends formatted HTML notifications with trade direction (🟢 Buy / 🔴 Sell / 🔄 Swap) and Solscan links.

### Telegram Alert Example

<img src="assets/TelegramBot.png" alt="Telegram Bot Alert" width="400"/>

## 🛠 Tech Stack

- **Core**: Rust (Tokio Runtime)
- **Blockchain**: `solana-client`, `solana-sdk`
- **Database**: PostgreSQL, `sqlx` (Async)
- **Notifications**: `teloxide` (Telegram Bot API)

## 📋 Prerequisites

- Rust Toolchain (latest stable)
- PostgreSQL Server
- Solana RPC Endpoint (HTTP & WSS) created via Helius/Alchemy/QuickNode etc. (Mainnet-beta public RPCs may be rate-limited)

## 🚀 Getting Started

### 1. Configuration

Create a `.env` file in the project root:

```env
DATABASE_URL=postgres://user:password@localhost:5432/dex_tracker
TELEGRAM_BOT_TOKEN=your_bot_token_here
TELEGRAM_CHAT_ID=your_target_chat_id
```

### 2. Run

The application handles database schema migration automatically on startup.

```bash
cargo run
```

### Terminal Output

![Terminal Logs](assets/Logs.png)
