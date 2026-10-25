# CryptoArbitrage

A Rust prototype exploring inventory-based arbitrage between Binance Spot and Jupiter on Solana: hold assets on both venues, buy cheaper, and sell higher without transferring funds for every trade.

Binance account/depth access and Spot Testnet order submission are implemented. Jupiter has a quote-only prototype, disabled in the runtime. Cross-venue coordination and risk controls are not implemented.

## Quick start

Install stable Rust (edition 2024), then run:

```bash
cargo test --all-targets
./scripts/verify.sh
```

The script checks formatting, strict Clippy, tests, and a release build without running the trading workflow.

Inspect `src/binance.rs` and the architecture notes before running. The current code has credential placeholders; testnet credentials must be configured locally and kept out of Git:

```bash
cargo run
```

**Running can submit a Binance Spot Testnet LIMIT FOK order.** There is no execution opt-in or dry-run switch yet; trading parameters are hard-coded. Keep credentials outside committed source. Building and testing require no exchange credentials.

## Documentation

| Read | Content |
| --- | --- |
| [Architecture](docs/architecture/README.md) | Current flow, integration diagrams, and limitations |
| [Arbitrage domain](docs/domain/arbitrage-domain.md) | Strategy, boundaries, and integration domain links |
| [Research](docs/research/) | Dated investigations and sources |
| [Inbox](INBOX.md) | Unfinished work and open decisions |
| [Agent briefing](AGENTS.md) | Lightweight agent workflow and context routing |
