# PR 1 Binance Safety Follow-Up

## Status

- Phase: PROPOSED

## Problem And Intended Outcome

A follow-up review of PR 1 identified two additional Binance risks after the first review-fix implementation was pushed:

- the default executable can reach Binance Spot Testnet order submission without an explicit execution-mode opt-in;
- one shared order model is used for open-order queries and cancellation responses even though those endpoints return different required fields.

The intended outcome is a fail-closed default runtime and endpoint-specific Binance order response models that can deserialize documented responses without losing the state required for reconciliation.

## Scope

- Make the default executable read-only with respect to exchange orders.
- Require an explicit Binance Spot Testnet execution mode before constructing or submitting an order request.
- Reject unknown or malformed runtime mode arguments without entering an execution path.
- Keep production Binance endpoints unsupported.
- Replace the shared open/cancel order response model with endpoint-specific transport models.
- Correct the open-order `isWorking` mapping and cancellation `transactTime` handling.
- Cover open-order, cancel-order, and cancel-all response deserialization with local fixtures.
- Reply to and resolve the two follow-up PR review threads after accepted changes are pushed and verified.
- Keep arbitrage coordination, configurable trading parameters, production credentials, retries, fill tracking, and Jupiter behavior out of scope.

## Confirmed Facts And Assumptions

- The current `cargo run` path calls the Binance runner directly.
- The Binance runner can construct and submit a LIMIT FOK order after account and order-book checks.
- Binance Spot Testnet is still a state-changing environment; testnet execution requires explicit intent.
- The documented current-open-orders response includes `time`, `updateTime`, and `isWorking`.
- The documented cancel-order response uses `transactTime` and does not require the open-order timing and working-state fields.
- Cancel-all responses can contain cancellation reports for individual orders and order lists; the implemented model must not assume every array item has the open-order shape.

## Components And Boundaries

- Runtime mode selection in `src/main.rs`.
- Binance runner execution gating in `src/binance.rs`.
- Binance order transport models under `src/binance/model/**`.
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

## Decisions And Risks

- **Accidental execution:** a default or malformed mode must never submit an order. Mitigation: use a typed mode with dry-run as the default and fail closed before entering Binance execution.
- **Misleading dry-run:** logging a hypothetical order must not claim that it executed. Mitigation: use distinct dry-run messages and never call `trade::new_order` in that branch.
- **Testnet confusion:** explicit execution could later be pointed at production accidentally. Mitigation: keep the execution mode named and implemented as testnet-only and retain the fixed testnet base URL.
- **Schema conflation:** optionalizing every mismatched field could hide endpoint differences. Mitigation: use separate response types and fixture tests rather than one permissive shared struct.
- **Conditional Binance fields:** order responses may include additional fields based on order type. Mitigation: model reconciliation-critical common fields, accept additive unknown fields, and add fixtures for each endpoint shape used here.
- **Cancel-all unions:** order-list cancellation reports differ from individual order reports. Mitigation: use explicit variants and test a mixed documented response where supported.
- **Financial verification:** no automated check may submit an order. Mitigation: test argument parsing, execution gating, request selection, and response fixtures without running the executable against Binance.

## Implementation Options

### Option A: Typed Dry-Run Mode And Endpoint Models (Recommended)

- Add a small typed execution mode parsed from the existing process arguments without introducing a full CLI framework.
- Default to dry-run and require `--execute-testnet` for order submission.
- Split open-order and cancellation response types and add endpoint fixtures.

This directly addresses both findings with a focused diff. A full configurable CLI remains a separate project.

### Option B: Disable Binance From The Default Runtime

- Make `cargo run` perform no exchange workflow unless any Binance flag is present.
- Apply the same endpoint-specific model corrections as Option A.

This is safest but provides less useful read-only prototype behavior and does not establish a reusable dry-run boundary.

### Option C: Build The Full Runtime CLI Now

- Add a CLI framework and expose exchange, mode, market, side, thresholds, and quantity configuration.
- Apply the endpoint-specific model corrections.

This resolves broader TODOs but mixes configuration design with focused review fixes and substantially increases the financial-safety review surface. It is not recommended for this project.

## Implementation Plan

1. [pending] Record owner approval and the selected runtime option in this proposal.
2. [pending] Add typed runtime mode parsing with fail-closed defaults and focused argument tests.
3. [pending] Pass the selected mode into the Binance runner and isolate order submission behind explicit testnet execution.
4. [pending] Introduce endpoint-specific open-order, canceled-order, and cancel-all response models.
5. [pending] Add documented Binance response fixtures and deserialization tests for each used endpoint shape.
6. [pending] Run strict repository verification and inspect the diff for any path that can submit without explicit intent.
7. [pending] Present implementation behavior, tests, limitations, and remaining testnet verification for owner review.
8. [pending] After owner acceptance, update permanent runtime and Binance documentation and remove this proposal.
9. [pending] Push the accepted closeout, reply to both review threads, and resolve them.

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
- Do not run `cargo run` or submit any Binance order during automated verification.
- Manual Binance Spot Testnet execution remains a separately reported verification step because it has financial side effects within the test environment.

## Acceptance Criteria

- Default startup cannot submit a Binance order.
- Testnet order submission requires one explicit, documented mode selection.
- Unsupported mode input exits without account, market-data, or order requests.
- Open-order responses deserialize `isWorking`, `time`, and `updateTime` correctly.
- Cancel responses deserialize `transactTime` without requiring open-order-only fields.
- Cancel-all responses preserve all documented item variants needed by the current helper.
- The strict repository verification script passes without live exchange calls.
- Both follow-up PR review threads receive evidence-based replies and are resolved only after implementation acceptance and push.

## Open Questions

1. Approve Option A, B, or C. Option A is recommended.
2. For Option A, approve `--execute-testnet` as the only state-changing runtime flag for this project.
3. Should manual Spot Testnet execution be required before implementation acceptance, or explicitly deferred until real credentials are moved out of source code? Deferral is recommended.

## Result

- Awaiting proposal review and implementation approval.
