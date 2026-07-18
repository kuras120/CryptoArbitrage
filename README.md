# CryptoArbitrage

CryptoArbitrage is a Rust project for researching and building inventory-based cryptocurrency arbitrage between Binance Spot and Jupiter on Solana. The intended strategy keeps assets funded on both venues, buys where the asset is cheaper, and sells where it is more expensive without transferring funds for every trade.

The current code is an early testnet prototype. It can query exchange data and submit Binance Spot Testnet orders, but the cross-venue arbitrage coordinator is not implemented yet.

## What Is Inside

The application is divided into Binance Spot and Jupiter integrations connected through a small asynchronous runtime. The exchange modules currently cover account data, market quotes, order-book handling, and testnet order execution; opportunity evaluation, inventory management, risk controls, and coordinated two-leg execution remain planned work.

Supporting documentation separates the cross-venue arbitrage domain from the Binance and Jupiter integration domains. It also captures application design rules, operational workflows, architecture diagrams, active project proposals, and reusable research. A shared verification workflow checks formatting, linting, tests, and release builds.

## Documentation

- Repository guide: [`docs/guidelines/repository-guide.md`](docs/guidelines/repository-guide.md)
- Arbitrage domain: [`docs/domain/arbitrage-domain.md`](docs/domain/arbitrage-domain.md)
- Binance integration domain: [`docs/domain/binance-domain.md`](docs/domain/binance-domain.md)
- Jupiter integration domain: [`docs/domain/jupiter-domain.md`](docs/domain/jupiter-domain.md)

## Quick Start

```bash
cargo test --all-targets
```

Use `./scripts/verify.sh` for the full quality gate. The executable currently targets Binance Spot Testnet and may submit an order; read the repository guide and inspect the runtime configuration before using `cargo run`.
