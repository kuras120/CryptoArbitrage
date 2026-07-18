# Repository Guide

This guide contains the operational information needed to build, run, test, and release CryptoArbitrage.

## Environment And Setup

1. Install the stable Rust toolchain. The manifest uses Rust edition 2024.
2. Fetch dependencies:

   ```bash
   cargo fetch
   ```

3. Run repository verification:

   ```bash
   ./scripts/verify.sh
   ```

4. Build without executing exchange calls:

   ```bash
   cargo build
   ```

5. Run the executable only after reviewing the selected integration and execution configuration:

   ```bash
   RUST_LOG=info cargo run
   ```

The current executable selects the Binance runner and points it at Binance Spot Testnet. It may submit a LIMIT FOK order. There is no safe runtime CLI or dry-run switch yet.

### Current Verification Baseline

- `cargo test --all-targets` passes, but the repository currently contains no automated tests.
- `cargo build --release` passes with warnings about unused prototype code.
- `./scripts/verify.sh` currently stops at `cargo fmt -- --check` because the existing Rust source is not rustfmt-clean.
- Strict Clippy also reports unused prototype surfaces and two style lints. Keep the gate strict; resolve this baseline as a focused code-quality change instead of weakening verification.

## Production Module: Runtime Entry

### Responsibilities

- Start the Tokio runtime.
- Configure process logging.
- Select the exchange workflow to execute.

### Current Interface

- Binary name: `CryptoArbitrage`.
- Local command: `cargo run`.
- The current binary starts the Binance workflow; the Jupiter workflow is not enabled.

### Outstanding Work

- Add an explicit CLI with exchange, market, quantity, and dry-run options.
- Move runtime configuration out of source code.
- Prevent any order submission unless an execution mode is explicitly selected.

## Production Module: Binance Spot

### Responsibilities

- Authenticate against Binance Spot Testnet.
- Load account state and available balances.
- Fetch order-book depth and select eligible bids or asks.
- Submit LIMIT FOK orders.
- Query or cancel open orders through order-management helpers.

### Runtime Flow

1. Build an authenticated Binance client.
2. Fetch account information and check `can_trade`.
3. Request 100 levels of depth for the configured symbol.
4. Parse prices and quantities into `Decimal` values.
5. Select the best eligible opposing quote and clamp the requested quantity.
6. Submit a LIMIT FOK order at the selected price.

See the [Binance Spot flow](../architecture/binance_spot_flow.puml).

### Configuration And Safety

- The testnet URL, symbol, thresholds, quantity, side, and credentials are currently hard-coded.
- `api_keys` is ignored, but credentials must be supplied through environment variables or an external secret manager before further development.
- New code must distinguish quote/read-only operations from execution operations.

### Testing Surface

- Account and order-book response deserialization.
- Best bid/ask and threshold-filtering behavior.
- Quantity clipping and symbol filter validation.
- Error mapping for transport failures, invalid responses, rate limits, and rejected orders.
- Execution tests through mocks or Binance Spot Testnet only.

### Outstanding Work

- Remove hard-coded credentials and trading parameters.
- Replace `unwrap()` calls with contextual error propagation.
- Validate exchange filters such as lot size, tick size, and minimum notional.
- Add fill tracking, retries with bounded backoff, and stale-order handling.
- Add unit and integration tests.

## Production Module: Jupiter

### Responsibilities

- Request Jupiter quotes for a token pair and amount.
- Deserialize route, output amount, price impact, and fee information.
- Supply normalized quote data to the future arbitrage coordinator.

### Runtime Flow

1. Build a quote URL from input mint, output mint, and amount.
2. Send the HTTP request.
3. Validate and deserialize the response.
4. Expose the quote for comparison with the CEX order book.

See the [Jupiter quote flow](../architecture/jupiter_swap_flow.puml).

### Configuration And Safety

- The host, SOL and USDC mint addresses, and amount are currently constants.
- The current module quotes but does not sign or execute a Solana transaction.
- Wallet material must remain outside the repository.

### Testing Surface

- Fixture-based quote deserialization.
- Token base-unit conversion.
- HTTP status, malformed payload, timeout, and rate-limit handling.
- Route fee and price-impact normalization.

### Outstanding Work

- Return typed results instead of printing responses.
- Standardize logging with `tracing`.
- Add a reusable HTTP client, timeouts, and bounded retries.
- Implement transaction building, signing, simulation, and execution only after the quote boundary is tested.

## Planned Production Module: Arbitrage Coordinator

The coordinator does not exist yet. Its intended responsibility is to compare executable prices after fees and slippage, validate inventory on both venues, and coordinate the buy and sell legs without transferring assets for every opportunity.

Before implementation, approve a project proposal covering partial execution, stale quotes, inventory drift, loss limits, idempotency, and recovery.

## Experimental Work

- Keep exploratory code isolated from the production binary and production credentials.
- Experiments may optimize for learning speed but must not execute real trades.
- Promotion into production requires an approved project plan, production error handling, deterministic tests, and updated technical documentation.

## Tooling And Scripts

- `scripts/verify.sh`: formatting, lint, test, and build verification.
- `cargo fmt -- --check`: formatting check without rewriting files.
- `cargo clippy --all-targets -- -D warnings`: lint all targets and reject warnings.
- `cargo test --all-targets`: run automated tests.
- `cargo build --release`: build the release binary without running it.
- `plantuml docs/architecture/*.puml`: render architecture diagrams.

Verification and maintenance scripts must be deterministic, shell-safe, and documented with their user-facing commands.

## Release Workflow

There is no automated deployment or package publication yet. Releases are manual build artifacts and Git tags. Follow the [release workflow](release-workflow.md); do not describe the binary as production-ready until secrets, dry-run controls, tests, and execution safety are implemented.

## Quality Requirements

- Follow the project lifecycle for non-trivial changes.
- Treat all exchange schemas, money calculations, signing, balances, and order execution as production-sensitive.
- Finish changes with `./scripts/verify.sh` or document the exact baseline failure.
- Keep tests, runtime behavior, and operational documentation consistent.
