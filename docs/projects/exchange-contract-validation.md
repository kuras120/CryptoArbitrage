# Exchange Contract Validation

## Status

- Phase: PROPOSED
- Proposal maturity: DRAFT

## Problem And Intended Outcome

Binance and Jupiter can change their HTTP contracts independently of this repository. Handwritten response models and an exchange connector dependency can therefore drift until a runtime response fails to deserialize or, worse, an execution path interprets changed data incorrectly.

The intended outcome is an automated, repeatable contract-maintenance workflow that detects relevant upstream API changes before deployment, keeps generated or handwritten integration models aligned with authoritative specifications, and makes dependency upgrades explicit and reviewable.

This is a separate project because it introduces external specification acquisition, code-generation or schema-validation tooling, CI policy, generated artifact ownership, and dependency-upgrade decisions. It is materially larger than correcting the three review findings in PR 1.

## Scope

- Treat the official Binance Spot OpenAPI specification and official Jupiter OpenAPI specifications as upstream contract sources where they cover the endpoints in use.
- Validate only the endpoints and payloads used by this repository before expanding coverage.
- Detect upstream specification changes on a schedule and during dependency-update work.
- Establish deterministic generation or validation commands suitable for local use and CI.
- Define how upstream specification revisions are pinned, reviewed, and updated.
- Evaluate and upgrade `binance_spot_connector_rust` independently from schema drift detection.
- Preserve an explicit adapter boundary between external transport contracts and internal trading-domain types.
- Keep live order submission and signed Solana transaction submission outside contract verification.

## Initial Endpoint Inventory

### Binance Spot

- Account information used to verify permissions and balances.
- Order-book depth used to select executable prices and quantities.
- New order submission used for LIMIT FOK execution.
- Open-order query and cancellation helpers already present in the integration.

### Jupiter

- Swap V1 quote response currently used by the prototype.
- Any replacement quote and swap endpoints selected by a future Swap V2 migration proposal.

## Upstream Sources

- Binance publishes an official Spot OpenAPI YAML specification in `binance/binance-api-swagger` and maintains the authoritative Spot API documentation and changelog in `binance/binance-spot-api-docs`.
- Jupiter publishes versioned OpenAPI specifications under `jup-ag/docs/openapi-spec`, including Swap V1 and Swap V2 directories.
- Upstream specifications are inputs, not automatically trusted executable changes. Every revision must be reviewed for endpoint, schema, authentication, precision, and error-semantics changes.

## Boundaries And Design Principles

- External generated DTOs, when used, remain exchange-specific and do not become arbitrage-domain models.
- Domain types retain validated `Decimal` or integer base-unit representations even if upstream schemas describe monetary values as strings or generic numbers.
- Generated code must never contain credentials, environment-specific URLs, or runtime configuration.
- CI drift detection must not call state-changing endpoints.
- A specification update must not automatically update production dependencies or merge generated code.
- OpenAPI coverage is not assumed complete. Fixture tests and read-only integration checks remain required for behaviors the specifications do not express accurately.

## Implementation Options

### Option A: Selected-Schema Code Generation With Adapters (Recommended Direction)

1. Pin authoritative upstream specification revisions.
2. Extract only the operations and component schemas used by this repository.
3. Generate exchange transport DTOs deterministically into an isolated module or temporary verification directory.
4. Map generated DTOs into handwritten, validated domain-facing integration types.
5. Fail CI when regeneration produces a diff or the adapter no longer compiles.

Advantages:

- Rust compilation detects many contract changes directly.
- Generated transport types reduce manual transcription errors.
- Narrow endpoint selection limits generated-code volume.

Costs and risks:

- Requires choosing and pinning a Rust OpenAPI generator.
- Generator upgrades can create noisy diffs unrelated to upstream API changes.
- Authentication, custom serialization, precision, and incomplete schemas still require handwritten code and fixtures.

Estimated size: medium to large. Expect a tooling spike followed by integration work across scripts, CI, generated modules, adapters, tests, and documentation.

### Option B: Schema And Fixture Validation Without Committed Generated Code

1. Pin or fetch upstream specifications in CI.
2. Validate curated response fixtures against the relevant OpenAPI schemas.
3. Deserialize the same fixtures into handwritten Rust DTOs.
4. Run a scheduled job that reports upstream spec changes and opens a maintenance task.

Advantages:

- Smaller repository diff and simpler ownership.
- Keeps current handwritten models and connector architecture.
- Good first milestone for detecting known payload incompatibilities.

Costs and risks:

- Does not prove that every Rust field matches every schema branch.
- Fixture selection can miss newly added variants.
- Requires tooling to resolve OpenAPI references and extract response schemas.

Estimated size: medium. This is a practical first delivery if the generation spike does not produce stable Rust output.

### Option C: Generate Complete Exchange Clients

Replace the current request layer and connector with fully generated clients for both venues.

Advantages:

- Maximum mechanical coverage of documented endpoints.

Costs and risks:

- Very large generated surface and review burden.
- Higher migration risk for signing, authentication, precision, and error handling.
- Upstream OpenAPI limitations become runtime client limitations.

Estimated size: large. This is not recommended for the current prototype.

## Proposed Delivery Stages

### Stage 1: Contract Tooling Spike

- Confirm exact upstream files, licenses, update cadence, and endpoint coverage.
- Compare Rust generators and schema validators for deterministic output and OpenAPI reference support.
- Generate or validate the current Binance account/depth and Jupiter quote schemas in a temporary workspace.
- Decide whether Option A is stable enough or Option B should become the initial implementation.

### Stage 2: Narrow Contract Gate

- Add pinned source metadata and reproducible fetch/update scripts.
- Cover Jupiter quote and Binance account/depth responses first.
- Add fixture validation and Rust deserialization tests.
- Add a local verification command that does not require network access after source update.

### Stage 3: Scheduled Drift Detection

- Add scheduled CI that compares the pinned revision with authoritative upstream sources.
- Report changed paths, operations, schemas, and source revisions without automatically changing production code.
- Require human review and explicit regeneration before accepting a contract update.

### Stage 4: Binance Connector Upgrade

- Review the current official Rust connector release, changelog, API changes, and transitive dependencies.
- Upgrade in a dedicated commit with account, depth, signing, and testnet verification.
- Confirm that connector behavior and the independently validated OpenAPI contract agree for repository-used endpoints.

### Stage 5: Execution Contract Coverage

- Extend coverage to order submission, query, and cancellation only after read-only contracts are stable.
- Add testnet verification for signing and request construction without enabling production endpoints.
- Add Jupiter transaction endpoints only as part of an approved execution or Swap V2 migration project.

## Decisions And Risks

- **Specification freshness:** pinning makes builds reproducible but does not detect new upstream revisions by itself. Mitigation: separate deterministic validation from scheduled drift monitoring.
- **Specification accuracy:** OpenAPI can lag runtime behavior or omit semantic constraints. Mitigation: retain curated fixtures, read-only smoke checks, changelog monitoring, and domain validation.
- **Generated-code noise:** broad generation can obscure meaningful changes. Mitigation: select only used operations and pin the generator version.
- **Supply-chain exposure:** fetching tools or specifications during every build adds external dependencies. Mitigation: pin versions and checksums, review licenses, and keep ordinary verification offline and reproducible.
- **Dependency coupling:** upgrading the Binance connector does not prove the API contract is current. Mitigation: treat dependency updates and contract validation as complementary, independently failing checks.
- **False confidence:** schema compatibility does not validate trading semantics such as fee interpretation, precision, stale data, or unknown order state. Mitigation: keep domain invariants and integration tests separate from schema checks.

## Implementation Plan

1. [pending] Complete the tooling spike and record the selected generator or validator with version and license.
2. [pending] Record the exact Binance and Jupiter upstream specification files and pinning strategy.
3. [pending] Define the initial endpoint subset and external-to-internal adapter boundaries.
4. [pending] Implement deterministic local contract generation or validation for the initial subset.
5. [pending] Add fixture and Rust deserialization tests using the same contract examples.
6. [pending] Add offline verification and scheduled upstream drift detection.
7. [pending] Upgrade the Binance Rust connector in an isolated commit and verify compatibility.
8. [pending] Document maintenance, failure triage, and contract-update review steps.

Use `[pending]`, `[in-progress]`, or `[done]` so interrupted work remains resumable.

## Verification

- A pinned-source contract check runs deterministically without exchange credentials.
- Re-running generation with unchanged inputs produces no diff.
- A deliberately incompatible fixture or DTO change fails the contract gate.
- Jupiter quote and Binance account/depth fixtures pass both schema and Rust deserialization validation.
- Scheduled drift detection reports upstream changes without modifying or merging production code.
- `./scripts/verify.sh`
- Binance execution verification remains limited to mocks or Spot Testnet and requires a separately approved implementation stage.

## Acceptance Criteria

- The repository can identify when an authoritative upstream schema revision changes.
- The used endpoint subset has deterministic contract checks and Rust deserialization coverage.
- External contract DTOs remain isolated from normalized trading-domain types.
- Ordinary verification does not depend on network availability.
- Dependency upgrades and upstream contract updates are visible, reviewable changes.
- No contract check can submit an order or transaction.

## Open Questions

1. Should the tooling spike target selected-schema generation first, or begin with schema-plus-fixture validation?
2. Should generated DTOs be committed for review, or generated temporarily and checked for reproducibility?
3. Which CI system and schedule should own upstream drift detection?
4. Should the Binance connector upgrade remain Stage 4 of this project or become its own smaller project after the spike?
5. Should Jupiter Swap V1 receive only maintenance coverage while a separate proposal evaluates migration to Swap V2?

## Result

- Draft proposal only. No implementation is authorized.
