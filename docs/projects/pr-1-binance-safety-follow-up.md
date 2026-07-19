# PR 1 Binance Safety Follow-Up

## Status

- Phase: PROPOSED

## Problem And Intended Outcome

A follow-up review of PR 1 identified six additional Binance risks after the first review-fix implementation was pushed:

- the default executable can reach Binance Spot Testnet order submission without an explicit execution-mode opt-in;
- one shared order model is used for open-order queries and cancellation responses even though those endpoints return different required fields;
- transport or response-body failure after order submission panics and discards the possibility that Binance accepted the request;
- malformed order-book levels are silently dropped, allowing trading decisions to continue with an incomplete snapshot;
- depth request and response-body failures panic instead of invalidating the snapshot and stopping execution;
- a successful FOK response is reported as executed without checking `status`, `executedQty`, or `expiryReason`.

The intended outcome is a fail-closed default runtime, contextual market-data failures, strict full-snapshot validation, status-aware FOK outcomes, explicit unknown order outcomes, and endpoint-specific Binance response models that preserve the state required for reconciliation.

## Scope

- Make the default executable read-only with respect to exchange orders.
- Require an explicit Binance Spot Testnet execution mode before constructing or submitting an order request.
- Reject unknown or malformed runtime mode arguments without entering an execution path.
- Keep production Binance endpoints unsupported.
- Replace the shared open/cancel order response model with endpoint-specific transport models.
- Correct the open-order `isWorking` mapping and cancellation `transactTime` handling.
- Cover open-order, cancel-order, and cancel-all response deserialization with local fixtures.
- Propagate depth request and response-body errors with context and stop before threshold evaluation or order submission.
- Reject an entire Binance depth snapshot when any consumed price or quantity cannot be parsed as `Decimal`.
- Request and deserialize a status-bearing Binance new-order response for LIMIT FOK submissions.
- Distinguish filled, unfilled or expired, rejected, and unknown submission outcomes without treating HTTP success as a fill.
- Assign a caller-controlled client order ID before submission so an unknown result can be reconciled without blind retry.
- Reply to and resolve the six follow-up PR review threads after accepted changes are pushed and verified.
- Keep arbitrage coordination, configurable trading parameters, production credentials, retries, ongoing fill monitoring, and Jupiter behavior out of scope.

## Confirmed Facts And Assumptions

- The current `cargo run` path calls the Binance runner directly.
- The Binance runner can construct and submit a LIMIT FOK order after account and order-book checks.
- Binance Spot Testnet is still a state-changing environment; testnet execution requires explicit intent.
- The documented current-open-orders response includes `time`, `updateTime`, and `isWorking`.
- The documented cancel-order response uses `transactTime` and does not require the open-order timing and working-state fields.
- Cancel-all responses can contain cancellation reports for individual orders and order lists; the implemented model must not assume every array item has the open-order shape.
- A transport or body-read failure after a state-changing request does not prove that Binance rejected the order.
- The current BUY and SELL helpers use `filter_map`, which silently removes malformed depth levels instead of invalidating the snapshot.
- The current depth request and body extraction use `unwrap()`, so a market-data failure panics before it can safely skip execution.
- A LIMIT FOK request can receive a successful HTTP response with a non-filled terminal status such as `EXPIRED`; success at the HTTP layer does not prove execution.

## Components And Boundaries

- Runtime mode selection in `src/main.rs`.
- Binance runner execution gating in `src/binance.rs`.
- Binance order transport models under `src/binance/model/**`.
- Binance depth validation under `src/binance/model/order_book.rs`.
- Binance market-data error propagation under `src/binance.rs` or a focused market-data module.
- Binance order-submission outcome and reconciliation identifiers under `src/binance.rs` or a focused execution module.
- Binance new-order result models under `src/binance/model/**`.
- Order-management response parsing under `src/binance/order_management.rs`.
- Local Binance JSON fixtures and unit tests.
- Repository guide and Binance flow diagram after implementation acceptance.
- No production exchange calls, credential loading, order-strategy changes, or Solana transaction work.

## Proposed Runtime And Data Flow

### Default Mode

1. Start the runtime without an execution flag.
2. Select Binance dry-run mode.
3. Permit account and market-data reads needed to evaluate the current prototype flow.
4. Log the order that would be submitted after all existing checks.
5. Do not construct or send a state-changing Binance request.

### Explicit Testnet Execution

1. Start the runtime with one documented testnet execution flag.
2. Parse the flag into a typed execution mode; fail closed on any unsupported value.
3. Run the existing account, market-data, threshold, and quantity checks.
4. Submit the LIMIT FOK request only inside the explicit testnet execution branch.
5. Keep the base URL fixed to Binance Spot Testnet.

### Order Responses

1. Deserialize open-order queries into an open-order response type containing `isWorking`, `time`, and `updateTime`.
2. Deserialize single cancellation responses into a cancellation type containing `transactTime` and cancellation-specific identifiers.
3. Deserialize cancel-all arrays into endpoint-specific item variants for ordinary orders and documented order-list reports.
4. Preserve order IDs, client IDs, status, quantities, side, type, and timestamps required for later reconciliation.

### Depth Validation

1. Fetch the Binance depth response and map request or response-body failures into contextual market-data errors.
2. Stop the flow immediately on any fetch failure; do not evaluate thresholds or construct an order.
3. Deserialize the raw Binance depth payload without treating malformed numeric strings as zero or skipping them.
4. Parse every consumed bid and ask price and quantity into `Decimal` with side, level index, and field context.
5. Reject the complete snapshot if any level is invalid.
6. Permit threshold filtering and best-price selection only on a fully fetched and validated snapshot.

### Order Submission Outcome

1. Generate a caller-controlled client order ID before constructing a state-changing request.
2. Explicitly request a status-bearing new-order response and submit the order once; do not retry automatically.
3. Deserialize the response into a LIMIT FOK result containing at least order ID, client order ID, status, original quantity, executed quantity, and optional expiry reason.
4. Report `FILLED` only when the parsed response confirms the terminal filled status and expected executed quantity.
5. Report a distinct unfilled or expired outcome for successful responses that do not confirm a complete fill.
6. Map a documented Binance rejection into a rejected outcome when the connector can prove rejection.
7. Map transport, unreadable response body, malformed success payload, or otherwise ambiguous post-submission failures into an unknown outcome containing the client order ID and error context.
8. Require later reconciliation by client order ID before any retry or replacement order.

## Decisions And Risks

- **Accidental execution:** a default or malformed mode must never submit an order. Mitigation: use a typed mode with dry-run as the default and fail closed before entering Binance execution.
- **Misleading dry-run:** logging a hypothetical order must not claim that it executed. Mitigation: use distinct dry-run messages and never call `trade::new_order` in that branch.
- **Testnet confusion:** explicit execution could later be pointed at production accidentally. Mitigation: keep the execution mode named and implemented as testnet-only and retain the fixed testnet base URL.
- **Schema conflation:** optionalizing every mismatched field could hide endpoint differences. Mitigation: use separate response types and fixture tests rather than one permissive shared struct.
- **Conditional Binance fields:** order responses may include additional fields based on order type. Mitigation: model reconciliation-critical common fields, accept additive unknown fields, and add fixtures for each endpoint shape used here.
- **Cancel-all unions:** order-list cancellation reports differ from individual order reports. Mitigation: use explicit variants and test a mixed documented response where supported.
- **Incomplete market data:** skipping malformed levels can produce a plausible but unsafe book. Mitigation: validate every level and reject the complete snapshot before evaluating thresholds.
- **Unavailable market data:** request or body errors can currently abort the process. Mitigation: return contextual snapshot errors and stop the trading flow before any order decision.
- **Unknown submission state:** a network failure can occur after Binance accepts an order. Mitigation: assign a stable client order ID before sending, return an explicit unknown outcome, and prohibit automatic retry.
- **False rejection classification:** not every connector error proves that the matching engine rejected the request. Mitigation: classify as rejected only when a structured Binance response proves rejection; otherwise classify conservatively as unknown.
- **False fill reporting:** an accepted FOK can expire without filling. Mitigation: request and parse a status-bearing response and report execution only for a confirmed complete fill.
- **Malformed success response:** inability to parse a post-submission success body leaves order state ambiguous. Mitigation: preserve the client order ID and classify the outcome as unknown rather than unfilled or rejected.
- **Financial verification:** no automated check may submit an order. Mitigation: test argument parsing, execution gating, request selection, and response fixtures without running the executable against Binance.

## Implementation Options

### Option A: Typed Dry-Run Mode And Endpoint Models (Recommended)

- Add a small typed execution mode parsed from the existing process arguments without introducing a full CLI framework.
- Default to dry-run and require `--execute-testnet` for order submission.
- Split open-order and cancellation response types and add endpoint fixtures.
- Introduce a market-data boundary that returns contextual fetch errors and fails on any malformed depth level.
- Introduce status-aware FOK response parsing, explicit order-submission outcomes, and caller-controlled reconciliation IDs without retrying. The recommended ID format is `ca-<UUIDv4 simple hex>`, which fits Binance's client-order-ID limit and avoids process-local timestamp collisions.

This directly addresses all six findings with a focused diff. A full configurable CLI remains a separate project.

### Option B: Disable Binance From The Default Runtime

- Make `cargo run` perform no exchange workflow unless any Binance flag is present.
- Apply the same endpoint-model, market-data, FOK-status, and unknown-outcome corrections as Option A.

This is safest but provides less useful read-only prototype behavior and does not establish a reusable dry-run boundary.

### Option C: Build The Full Runtime CLI Now

- Add a CLI framework and expose exchange, mode, market, side, thresholds, and quantity configuration.
- Apply the remaining Binance safety and model corrections.

This resolves broader TODOs but mixes configuration design with focused review fixes and substantially increases the financial-safety review surface. It is not recommended for this project.

## Implementation Plan

1. [pending] Record owner approval and the selected runtime option in this proposal.
2. [pending] Add typed runtime mode parsing with fail-closed defaults and focused argument tests.
3. [pending] Pass the selected mode into the Binance runner and isolate order submission behind explicit testnet execution.
4. [pending] Introduce endpoint-specific open-order, canceled-order, and cancel-all response models.
5. [pending] Add documented Binance response fixtures and deserialization tests for each used endpoint shape.
6. [pending] Add contextual depth request, body, and deserialization errors that stop the flow before execution.
7. [pending] Add strict full-snapshot depth parsing with malformed-level tests for bid and ask prices and quantities.
8. [pending] Add caller-controlled client order IDs and explicit filled, unfilled or expired, rejected, and unknown submission outcomes without automatic retry.
9. [pending] Request and deserialize status-bearing LIMIT FOK responses, including optional expiry reasons.
10. [pending] Add focused tests for depth-fetch gating, FOK status classification, unknown outcomes, and preservation of reconciliation IDs.
11. [pending] Run strict repository verification and inspect the diff for unsafe execution, incomplete-book, false-fill, and blind-retry paths.
12. [pending] Present implementation behavior, tests, limitations, and remaining testnet verification for owner review.
13. [pending] After owner acceptance, update permanent runtime and Binance documentation and remove this proposal.
14. [pending] Push the accepted closeout, reply to all six review threads, and resolve them.

Use `[pending]`, `[in-progress]`, or `[done]` so interrupted work remains resumable.

## Verification

- `cargo fmt -- --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --all-targets`
- `cargo build --release`
- `./scripts/verify.sh`
- Unit tests prove that no arguments select dry-run, `--execute-testnet` selects testnet execution, and unknown arguments fail closed.
- Unit tests prove that dry-run selection cannot choose the order-submission branch.
- Fixture tests cover current-open-orders, cancel-order, and cancel-all response shapes.
- Mocked depth request and body failures stop the flow before order construction.
- Malformed bid or ask price and quantity fixtures invalidate the entire depth snapshot.
- New-order fixtures cover confirmed `FILLED` and successful-but-unfilled or `EXPIRED` FOK responses, including optional expiry reasons.
- Submission-outcome tests preserve the client order ID and classify ambiguous transport, body, and success-payload failures as unknown.
- No test automatically retries a state-changing request.
- Do not run `cargo run` or submit any Binance order during automated verification.
- Manual Binance Spot Testnet execution remains a separately reported verification step because it has financial side effects within the test environment.

## Acceptance Criteria

- Default startup cannot submit a Binance order.
- Testnet order submission requires one explicit, documented mode selection.
- Unsupported mode input exits without account, market-data, or order requests.
- Open-order responses deserialize `isWorking`, `time`, and `updateTime` correctly.
- Cancel responses deserialize `transactTime` without requiring open-order-only fields.
- Cancel-all responses preserve all documented item variants needed by the current helper.
- Depth request, response-body, and JSON errors stop the flow before threshold evaluation or order construction.
- Any malformed numeric level invalidates the complete depth snapshot before opportunity evaluation.
- Every submitted order has a caller-controlled client order ID available for reconciliation.
- Ambiguous submission failures return an unknown outcome instead of panicking or reporting rejection.
- Unknown outcomes cannot trigger automatic retry.
- A successful Binance response is reported as filled only when parsed status and quantity fields confirm the complete FOK fill.
- Successful unfilled or expired FOK responses remain visibly distinct from filled, rejected, and unknown outcomes.
- The strict repository verification script passes without live exchange calls.
- All six follow-up PR review threads receive evidence-based replies and are resolved only after implementation acceptance and push.

## Open Questions

1. Approve Option A, B, or C. Option A is recommended.
2. For Option A, approve `--execute-testnet` as the only state-changing runtime flag for this project.
3. Should manual Spot Testnet execution be required before implementation acceptance, or explicitly deferred until real credentials are moved out of source code? Deferral is recommended.
4. Approve caller-controlled client order IDs and conservative unknown-state classification as part of this focused safety repair.
5. Approve a pinned UUID dependency and `ca-<UUIDv4 simple hex>` as the client order ID format. This is recommended over timestamp-only IDs.
6. Approve explicit `Filled`, `UnfilledOrExpired`, `Rejected`, and `Unknown` outcomes, with execution logged only for `Filled`.

## Result

- Awaiting proposal review and implementation approval.
