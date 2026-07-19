# Engineering Guide

This guide defines how CryptoArbitrage code should be designed, implemented, and tested.

## Application Structure

- Keep exchange adapters responsible for authentication, transport, rate-limit interpretation, and conversion into internal types.
- Keep arbitrage policy independent of Binance, Jupiter, HTTP clients, and serialization formats.
- Put opportunity evaluation, inventory policy, risk checks, and execution coordination behind explicit service boundaries.
- Separate read-only market-data operations from state-changing order and transaction operations.
- Inject configuration and external clients instead of constructing them inside business logic.
- Prefer small modules with one reason to change over broad exchange-specific orchestration functions.

## Domain Types And Precision

- Model venue, market, asset, side, quantity, price, fee, quote timestamp, and execution state with explicit types.
- Use `rust_decimal::Decimal` for exchange decimal values and integer base units at blockchain boundaries.
- Never use `f32` or `f64` for prices, balances, fees, quantities, thresholds, or trading decisions.
- Make pair direction and base/quote asset roles explicit; do not infer them from string position in calculation code.
- Convert token decimals at a single boundary and test rounding behavior.
- Represent normalized and venue-specific values as different types when mixing them could produce an invalid trade.

## Error Handling

- Return contextual errors from transport, parsing, validation, signing, persistence, and execution paths.
- Do not use `unwrap()` or `expect()` in code reachable from the runtime workflow.
- Distinguish retryable transport failures from rejected requests, invalid data, stale quotes, and unknown execution state.
- Preserve external error codes and request identifiers needed for reconciliation without exposing secrets.
- Treat an unknown result from a state-changing request as a distinct state, not as a failed request safe to repeat.

## Async And External I/O

- Reuse configured HTTP clients and set explicit connection and request timeouts.
- Bound concurrency to respect exchange limits and protect local resources.
- Apply retries only to operations proven safe to repeat; use bounded exponential backoff with jitter.
- Do not perform blocking network or filesystem work on Tokio executor threads.
- Attach timestamps at the data-ingestion boundary and validate quote age before use.
- Keep serialization models separate from internal business models when the external schema is unstable or incomplete.

## Trading Workflow Design

- Calculate opportunities from quantity-aware executable prices, not top-of-book prices alone.
- Include exchange fees, route fees, network costs, slippage, rounding, and safety margin before approving a trade.
- Validate balances, venue filters, risk limits, and data freshness immediately before execution.
- Coordinate both legs through an explicit state machine that handles partial fills, rejection, timeout, and unknown status.
- Use stable client identifiers or idempotency mechanisms for every state-changing request.
- Reconcile exchange balances and order states after every execution attempt.
- Make dry-run and live execution separate explicit modes; default to the non-executing mode.

## Security

- Load credentials and wallet material through configuration boundaries backed by environment or external secret storage.
- Never embed or log API secrets, private keys, seed phrases, signed transactions, or complete sensitive payloads.
- Keep testnet and production configuration types or profiles visibly distinct.
- Minimize credential permissions and disable withdrawals where supported.
- Redact sensitive fields before structured logging or error reporting.

## Testing

- Unit-test arithmetic, rounding, normalization, thresholds, state transitions, and risk decisions.
- Use fixtures for external response schemas and cover missing, malformed, and unexpected fields.
- Mock time, transport, and execution ports instead of calling live exchanges in unit tests.
- Test timeout, rate-limit, rejection, partial-fill, duplicate-request, and unknown-status behavior.
- Keep testnet checks explicit and opt-in; they are not a substitute for deterministic tests.
- Add a regression test for every corrected financial or execution bug.

## Code Review Standard

- Prioritize financial loss, leaked secrets, precision errors, stale data, unsafe retries, partial execution, and behavioral regressions.
- Require clear ownership of side effects and state transitions.
- Verify that new abstractions reduce coupling without hiding trading semantics.
- Require tests for changed parsing, arithmetic, policy, and failure handling.
- Avoid cosmetic feedback unless readability obscures safety or correctness.
