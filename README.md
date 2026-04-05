# CryptoArbitrage

## Overview
CryptoArbitrage is a software that orchestrates automated arbitrage between centralized (Binance Spot testnet) and decentralized (Jupiter Lite API on Solana) venues. It compares live quotes from both markets, keeps liquidity on each side to avoid transfer fees, and attempts to capture spreads by buying on the cheaper venue and selling on the more expensive one. The project currently focuses on BTC/USDT on Binance and SOL/USDC on Jupiter, but the structure is ready for multi-market extension.

Key third-party crates:
- `tokio`, `reqwest`, `serde`, `serde_json` for async HTTP workflows and JSON parsing.
- `tracing` / `tracing-subscriber` for structured logs.
- `binance_spot_connector_rust` for authenticated REST access to Binance Spot.
- `rust_decimal` / `rust_decimal_macros` for deterministic price/qty math.

## Runtime Entry Point
`src/main.rs` initializes a Tokio runtime, configures INFO-level tracing, and currently calls only `binance::run()`. Enabling the commented `jupiter::run()` line will also hit the DEX quote path.

## Binance (CEX) Module
Location: `src/binance.rs` plus the `binance/` submodules.

Workflow:
1. Builds HMAC credentials (currently stored both in `api_keys` and inline) and instantiates a `BinanceHttpClient` pointed at `https://testnet.binance.vision`.
2. Fetches account metadata via `account_info::get_account_info` to ensure `can_trade` is true and logs non-zero balances.
3. Pulls a 100-level order book for `BTCUSDT`, parses it into `model::order_book::OrderBook`, and logs depth.
4. Filters bids or asks with helper methods (`find_bids_better_than`, `find_asks_better_than`) against configurable thresholds (`target_price_min`, `target_price_max`).
5. Selects the best quote, clips quantity to the configured `desired_quantity`, then places a LIMIT FOK order via `trade::new_order` to immediately match or cancel.
6. `order_management.rs` provides helpers to inspect open orders and cancel individual or all orders for follow-up workflows.

Supporting data models:
- `model/account.rs` mirrors Binance account schema (commissions, permissions, balances).
- `model/order_book.rs` captures bids/asks and exposes `Decimal`-based helpers for best levels and threshold filtering.
- `model/avg_price.rs` stores moving average snapshots with millisecond timestamps for future spread analysis.

## Jupiter (DEX) Module
Location: `src/jupiter.rs` and `jupiter/model`.

Workflow:
1. Defines constants for Jupiter Lite API host plus quote/order endpoints and canonical mint addresses (`So111…` for SOL, `EPjF…` for USDC).
2. Builds a quote URL (`/swap/v1/quote`) with `reqwest::Url::parse_with_params` for a notional 1 SOL trade (`amount = 1,000,000,000` lamports).
3. Sends an unauthenticated `reqwest::get`, logs HTTP status/headers, and attempts to deserialize into `model::order::SwapOrder`.
4. Pretty-prints the parsed payload or logs raw text/errors for debugging.

Data models provide:
- `SwapOrder` for quote responses (slippage, route plan, USD value, etc.).
- `Order`, `RoutePlan`, `SwapInfo` for full swap construction, including router metadata, fee mints, and AMM hops—useful when expanding beyond quoting.

## Configuration & Secrets
- API keys currently live in plain text at `api_keys` and are duplicated inside `binance::run()`. Before production, move them to environment variables or a secrets manager.
- Symbols (`BTCUSDT`, SOL/USDC) and trade parameters (`target_price_*`, `desired_quantity`, `trade_side`) are hard-coded. Parameterize them to evaluate multiple pairs and directions.

## Next Steps
1. **Orchestrator:** Implement a coordinator that queries both Binance and Jupiter, normalizes quotes, and decides where to buy/sell based on spread and latency.
2. **Config Surface:** Externalize credentials and market parameters (symbols, quantities, thresholds) via config files or env vars.
3. **Unified Logging:** Switch Jupiter prints to `tracing` so both venues emit consistent structured telemetry.
4. **Testing:** Add unit/integration tests—e.g., verify order book filtering logic and mock Jupiter responses—to prevent regressions as the arbitrage loop grows.

Use this README as the authoritative entry point for future tasks; keep sections in sync when new modules, tools, or release processes are added.
