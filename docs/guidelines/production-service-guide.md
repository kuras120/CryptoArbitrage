# Production Service Guide

Use this checklist when designing, implementing, or reviewing production trading behavior.

## Boundaries

- Exchange adapters own authentication, transport, rate-limit interpretation, and mapping into internal types.
- The arbitrage coordinator owns quote comparison, opportunity policy, inventory checks, and two-leg execution state.
- Configuration owns endpoints, symbols/mints, limits, risk thresholds, and execution mode.
- Observability reports decisions and outcomes without exposing secrets.

## Trading And Precision

- Normalize pair direction and token decimals before comparing prices.
- Include taker/maker fees, Jupiter route fees, network fees, slippage, and quantity-dependent depth.
- Use integer base units at chain boundaries and `Decimal` for human/exchange decimal values.
- Validate Binance tick size, lot size, minimum quantity, and minimum notional before submitting.
- Reject stale quotes and order-book snapshots using explicit timestamps and maximum-age policy.

## Execution Safety

- Default to read-only or dry-run behavior.
- Require explicit configuration to enable order or transaction execution.
- Define idempotency keys or client order identifiers before retrying submissions.
- Track each leg as a state machine and handle partial fills, rejected legs, unknown status, and timeouts.
- Set position, notional, loss, slippage, and frequency limits before enabling production endpoints.
- Verify inventory before each leg and reconcile balances after execution.

## Security

- Load API keys, secrets, keypairs, and wallet material from external secret storage or environment configuration.
- Never commit or log credentials, private keys, seed phrases, signed transactions, or full sensitive payloads.
- Keep testnet and production endpoints/configurations visibly distinct.
- Use least-privilege exchange keys with withdrawals disabled when possible.
- Treat signing code and dependency changes as security-sensitive review areas.

## Reliability And Observability

- Use bounded timeouts and retries with jitter for safe read operations.
- Do not blindly retry state-changing requests.
- Return contextual typed errors instead of panicking.
- Log exchange, market, side, normalized quantity, decision reason, request correlation ID, and outcome.
- Add metrics for quote age, spread after costs, API latency, rejected opportunities, orders, fills, and inventory drift.

## Required Verification

- Unit tests for arithmetic, normalization, thresholds, and state transitions.
- Fixture tests for every exchange response schema that drives decisions.
- Mock tests for timeout, rate limit, malformed response, rejection, and unknown-order-state cases.
- Testnet or simulation verification for execution workflows.
- Formatting, linting, automated tests, and a release build before implementation review.
