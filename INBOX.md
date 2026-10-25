# Inbox

Put loose notes, links, observations, questions, constraints, and unverified claims below. They are input, not requirements or implementation approval.

## Untriaged

### Binance safety follow-up (from PR 1; not implemented)

| Work | Expected behavior / checks |
| --- | --- |
| Safe startup | Default to dry-run; require explicit testnet execution; reject unknown arguments before any requests. Test that dry-run cannot reach order submission. Production endpoints stay unsupported. |
| Endpoint-specific order models | Separate open, canceled, and cancel-all responses. Cover `isWorking`, `time`, `updateTime`, cancellation `transactTime`, and mixed order-list reports with fixtures; retain reconciliation fields. |
| Depth errors | Return contextual request/body/JSON errors and stop before order construction. Reject the entire snapshot on any malformed consumed bid/ask price or quantity; remove silent skipping and zero fallbacks. Test both sides and fetch failures. |
| Submission outcomes | Request a status-bearing FOK response. Confirm `status` and `executedQty` before reporting a fill; retain optional `expiryReason`. Distinguish filled, unfilled/expired, proven rejection, and unknown outcomes. |
| Reconciliation | Assign a caller-controlled client order ID before sending. Transport, body, or success-payload errors leave the result unknown; retain the ID and reconcile before retry/replacement. Never blindly retry. |
| Review follow-up | Revisit the six PR 1 review threads after fixes and verification. Posting replies or resolving threads needs owner instruction. |

Previous recommendation (still unimplemented): a small typed dry-run mode with `--execute-testnet`. It keeps useful reads while isolating submission; disabling the default workflow is safer but less useful, and a full CLI expands the safety-fix scope. A UUID-based ID was suggested to avoid timestamp collisions, not selected as a dependency.

Unresolved choices: dry-run with reads versus no default exchange workflow; a small typed `--execute-testnet` flag versus a full CLI; client ID format/dependency (previous suggestion: `ca-<UUIDv4 simple hex>`). Manual testnet execution remains separate from automated checks; previously suggested deferring it until credentials are externalized.

### Exchange contract validation (not implemented)

- [ ] Inventory the used contracts: Binance account/depth/new order/open orders/cancel/cancel-all and Jupiter quote; evaluate Swap V1 maintenance versus Swap V2 migration separately.
- [ ] Check exact upstream files, licenses, and endpoint coverage in the official [Binance OpenAPI repository](https://github.com/binance/binance-api-swagger), [Spot API docs/changelog](https://github.com/binance/binance-spot-api-docs), and [Jupiter specifications](https://github.com/jup-ag/docs/tree/main/openapi-spec).
- [ ] Spike selected-schema Rust DTO generation versus schema-plus-fixture validation. Pin tool versions and source revisions; decide whether generated DTOs belong in Git. Full generated clients were considered too broad for this prototype.
- [ ] Start with Binance account/depth and Jupiter quote fixtures, validating both schemas and Rust deserialization. Keep transport DTOs separate from validated decimal/base-unit domain values.
- [ ] Add deterministic offline checks under `scripts/`; unchanged inputs must reproduce output, and an incompatible fixture/model must fail. Retain fixtures for semantics missing from OpenAPI.
- [ ] Choose CI and drift-check cadence. Report source/operation/schema changes without automatically modifying or merging code; review regeneration explicitly.
- [ ] Evaluate the Binance connector upgrade independently, with changelog, dependency, signing, request-construction, and fixture checks. Keep live orders and transactions out of contract verification.
- [ ] Extend to execution contracts after read-only coverage; document source updates and failure triage. Generated artifacts must contain no credentials or environment configuration.

The earlier contract proposal retained these trade-offs; no tool or approach has been selected:

| Option | Argument for | Argument against / constraint |
| --- | --- | --- |
| Selected-schema DTO generation + adapters | Compiler detects many mismatches; reduces transcription; limits generated surface | Generator changes can create noisy diffs; signing, precision, custom serialization, and missing semantics still need handwritten code/fixtures |
| Schema + fixture validation | Smaller diff; preserves current connector/models; fallback if generation is unstable | Fixtures can miss variants; does not prove every Rust field/schema branch agrees; needs reference-resolution tooling |
| Full generated clients | Broad mechanical endpoint coverage | Large migration/review surface; schema limitations affect runtime, signing, authentication, and precision |

Pin checksums as well as tool/spec revisions and review licenses. Source pinning provides reproducibility, not freshness; scheduled drift checks and changelog monitoring are separate. Contract checks do not prove fee, precision, freshness, or unknown-state semantics, and a connector upgrade does not establish API compatibility. Review upstream endpoint/authentication/error-semantics changes explicitly. Use fixtures and read-only smoke checks for specification gaps; do not add network-dependent fetching to ordinary builds.

### Runtime and integration work

| Area | Follow-up |
| --- | --- |
| Configuration | Externalize credentials and trading parameters; add exchange/market/side/quantity/mode options; keep testnet and production visibly separate. |
| Binance | Validate tick/step size, minimum quantity/notional, and available balances. Add contextual errors throughout order management, bounded safe-read retries, fill tracking, and stale-order handling. |
| Jupiter | Return typed results, use `tracing`, reuse a client with timeouts and bounded read retries, and test status/malformed/timeout/rate-limit responses, base-unit conversion, and route-cost normalization. |
| Jupiter execution | Select API/authentication and RPC provider; define trusted mint metadata, commitment, priority fees, and confirmation timeout before implementing build/simulate/sign/submit/confirm and signature/balance reconciliation. |
| Arbitrage | Choose common asset and quote-value normalization (including USDT/USDC risk), fee sources, quote age, minimum net spread, executable quantity, and route/slippage/price-impact limits. |
| Coordination and inventory | Define leg ordering, partial-fill/unknown-state recovery, persistence/restart recovery, account/data refresh cadence, inventory targets, reservation, reconciliation, and separate rebalancing. |
| Risk and operations | Define position/notional/loss/slippage/frequency limits, kill switch and ownership, health checks, decision/outcome metrics, alerts, and a stop/recovery runbook before production use. |
| Releases | If releases become useful, define version selection, release notes, artifacts/checksums, annotated tags, rollback, and automation. Review dependencies and secrets and keep execution checks explicitly testnet-only. No publication or deployment automation exists today. |

### Retained engineering follow-ups

- [ ] Inject configuration and clients through explicit boundaries; model venue, asset, side, quantities, timestamps, and execution states explicitly. Keep transport DTOs separate from normalized domain types.
- [ ] Replace runtime panics with contextual typed errors; preserve exchange error codes and request IDs. Separate rejected, retryable read, stale/invalid-data, and unknown execution outcomes.
- [ ] Remove blocking I/O from Tokio workers; bound concurrency, set connection/request timeouts, reuse clients, and apply bounded backoff with jitter only where safe.
- [ ] Add regression tests for financial/execution fixes and fixtures for missing/malformed/variant fields. Cover arithmetic/rounding, thresholds, quantity clipping, state/risk decisions, timeout/rate-limit/rejection, duplicate requests, partial fills, and unknown outcomes with mocked time/transport/execution.
- [ ] Redact sensitive payloads; report venue/market/side/quantity, decision reason, correlation ID, and outcome. Measure quote age, net spread, latency, rejections, orders/fills, and inventory drift.

### GPW feed feasibility (not implemented)

Use the dated [provider research](docs/research/gpw-market-monitoring.md) as the starting point. Recheck provider terms and capabilities before implementation. Run a read-only session experiment for 5 → 20 → 60 symbols over WebSocket; fall back to a 10-symbol one-minute polling tier with bounded backoff. Store source/receipt timestamps idempotently; measure gaps, duplicate rate, rate limits, and stale periods, and cross-check samples manually. After five sessions, choose a sustainable universe/cadence under the zero-cost constraint before adding signals or alerts.
