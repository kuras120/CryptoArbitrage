# Jupiter Swap API & Binance Spot Connector (Rust)

This folder serves two complementary purposes:
1. Visualize how our trading service communicates with Jupiter and Binance (see the PlantUML diagrams `jupiter_swap_flow.puml` and `binance_spot_flow.puml`).
2. Capture concise examples that show how to submit SOL buy/sell orders through both platforms using the crates listed in `Cargo.toml` (`reqwest` + Solana SDK for Jupiter, `binance_spot_connector_rust` 1.3.0 for Binance).

## Primary References

- **Jupiter Swap API (Swap v2):**
  - Entry point `https://api.jup.ag/swap/v2` with key paths `GET /order`, `GET /build`, `POST /execute` (all require the `x-api-key` header).
  - `GET /order` already returns a base64-encoded transaction plus `requestId`; pass both to `POST /execute` after signing.
  - The Order & Execute flow gives access to routers like Metis, JupiterZ/RFQ, Dflow, and OKX while handling landing, priority fees, and confirmations for you.
- **Binance Spot Connector Rust 1.3.0:**
  - Wraps all `/api/*` and `/sapi/*` REST endpoints with blocking (`ureq`) and async (`hyper`) clients.
  - Version 1.3.0 added trade endpoints such as `GET /api/v3/myAllocations` and refreshed the OCO path to `POST /api/v3/orderList/oco`.
  - `trade::new_order` builds the canonical `POST /api/v3/order` call, exposing setters for `quantity`, `quote_order_qty`, `time_in_force`, `new_order_resp_type`, etc., and attaches credentials for HMAC signing.
  - Signed endpoints need an API key, secret, `timestamp`, and optional `recvWindow` (defaults to 5 000 ms, max 60 000 ms).

## Example: Swap SOL → USDC on Jupiter (sell SOL)

```rust
use anyhow::Context;
use reqwest::Client;
use serde::Deserialize;
use solana_sdk::{signature::{Keypair, Signer}, transaction::VersionedTransaction};

const JUP_BASE_URL: &str = "https://api.jup.ag/swap/v2";
const SOL_MINT: &str = "So11111111111111111111111111111111111111112";
const USDC_MINT: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";

#[derive(Deserialize)]
struct OrderResponse {
    transaction: String,
    requestId: String,
}

#[derive(Deserialize)]
struct ExecuteResponse {
    status: String,
    signature: Option<String>,
}

pub async fn jupiter_sell_sol_for_usdc(
    http: &Client,
    api_key: &str,
    taker_pubkey: &str,
    lamports: u64,
    signer: &Keypair,
) -> anyhow::Result<String> {
    let order_url = format!(
        "{JUP_BASE_URL}/order?inputMint={}&outputMint={}&amount={}&taker={}",
        SOL_MINT,
        USDC_MINT,
        lamports,
        taker_pubkey
    );

    let order: OrderResponse = http
        .get(order_url)
        .header("x-api-key", api_key)
        .send()
        .await?
        .error_for_status()? 
        .json()
        .await?;

    let tx_bytes = base64::decode(order.transaction)?;
    let mut tx: VersionedTransaction = bincode::deserialize(&tx_bytes)?;
    tx.sign(&[signer], solana_sdk::hash::Hash::default()); // replace with a fresh blockhash from your RPC
    let signed_tx = base64::encode(bincode::serialize(&tx)?);

    let exec: ExecuteResponse = http
        .post(format!("{JUP_BASE_URL}/execute"))
        .header("x-api-key", api_key)
        .json(&serde_json::json!({
            "signedTransaction": signed_tx,
            "requestId": order.requestId,
        }))
        .send()
        .await?
        .error_for_status()? 
        .json()
        .await?;

    anyhow::ensure!(exec.status == "Success", "Swap failed: {:?}", exec);
    Ok(exec.signature.unwrap_or_default())
}
```

> Tip: load the `Keypair` from a BS58 secret or CLI keyfile and always refresh the blockhash before signing, otherwise Solana validators will reject the transaction.

## Example: Spot orders on Binance (buy & sell SOL/USDT)

```rust
use anyhow::Context;
use binance_spot_connector_rust::{
    http::Credentials,
    trade::{self, order::{Side, TimeInForce, NewOrderResponseType}},
    ureq::BinanceHttpClient,
};
use rust_decimal::Decimal;

pub fn binance_buy_sol_with_usdt(
    api_key: &str,
    secret: &str,
    quote_usdt: Decimal,
) -> anyhow::Result<String> {
    let creds = Credentials::from_hmac(api_key.to_owned(), secret.to_owned());
    let request = trade::new_order("SOLUSDT", Side::Buy, "MARKET")
        .quote_order_qty(quote_usdt)
        .new_order_resp_type(NewOrderResponseType::Full)
        .credentials(&creds);

    let response = BinanceHttpClient::default()
        .send(request)
        .context("HTTP error")?;
    Ok(response.into_body_str().context("Failed to read body")?)
}

pub fn binance_sell_sol_for_usdt(
    api_key: &str,
    secret: &str,
    quantity_sol: Decimal,
    limit_price: Decimal,
) -> anyhow::Result<String> {
    let creds = Credentials::from_hmac(api_key.to_owned(), secret.to_owned());
    let request = trade::new_order("SOLUSDT", Side::Sell, "LIMIT")
        .time_in_force(TimeInForce::Gtc)
        .price(limit_price)
        .quantity(quantity_sol)
        .new_order_resp_type(NewOrderResponseType::Full)
        .credentials(&creds);

    let response = BinanceHttpClient::default()
        .send(request)
        .context("HTTP error")?;
    Ok(response.into_body_str().context("Failed to read body")?)
}
```

Key takeaways:

- `Credentials::from_hmac` signs the request payload exactly as required by `POST /api/v3/order`.
- For market buys it is often simpler to specify the quote amount via `quote_order_qty`; for sells you typically pass `quantity` (base asset).
- Limit orders must obey the filters returned by `GET /api/v3/exchangeInfo` (price tick, notional, and lot size).
- To hit the Spot Testnet, construct the client with `BinanceHttpClient::with_url("https://testnet.binance.vision")`.

## Next steps

1. Store API keys and Solana secrets outside the repo (dotenv, key vault, etc.) and inject them via runtime configuration.
2. Add retries/backoff plus custom `recvWindow` handling for Binance and real-time monitoring of `requestId` / transaction signatures for Jupiter.
3. Extend the diagrams or examples once you add WebSocket user-data streams or custom Solana CPI logic.

Render the PUML diagrams locally with `plantuml architecture/<diagram>.puml` whenever you need fresh PNG/SVG assets.
