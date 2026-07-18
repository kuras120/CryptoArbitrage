# Agent Briefing

Use this file first to identify the required repository context. Read every document routed for the touched area before planning, reviewing, or changing it. Detailed rules and checklists belong in `docs/**`; do not duplicate them here.

## Required Workflow

- For non-trivial changes, follow `docs/guidelines/project-lifecycle.md`, including its proposal and implementation acceptance gates.
- Keep active plans under `docs/projects/**`; remove completed plans after durable documentation and TODOs are updated.
- Use the smallest applicable set of documents from the routing table. Combine routes when a task crosses areas.
- Preserve existing user changes in the worktree.
- Keep code, documentation, plans, scripts, and user-facing messages in English.

## Repository Map

| Path | Responsibility |
| --- | --- |
| `src/main.rs` | Runtime entry point and top-level workflow selection. |
| `src/binance.rs`, `src/binance/**` | Binance Spot integration, account data, order books, and order management. |
| `src/jupiter.rs`, `src/jupiter/**` | Jupiter quote integration and response models. |
| `docs/domain/arbitrage-domain.md` | Cross-venue opportunity, inventory, execution, and risk concepts. |
| `docs/domain/binance-domain.md` | Binance Spot terminology, boundaries, invariants, and order lifecycle. |
| `docs/domain/jupiter-domain.md` | Jupiter and Solana terminology, boundaries, invariants, and transaction lifecycle. |
| `docs/guidelines/repository-guide.md` | Environment, commands, module operation, testing, and release overview. |
| `docs/guidelines/engineering-guide.md` | Application design and coding rules. |
| `docs/guidelines/production-service-guide.md` | Trading precision, execution safety, security, and production verification. |
| `docs/guidelines/project-lifecycle.md` | Proposal, approval, implementation, and closeout process. |
| `docs/guidelines/release-workflow.md` | Versioning and release process. |
| `docs/architecture/**` | Runtime and integration diagrams. |
| `docs/projects/**` | Temporary plans for active non-trivial work. |
| `docs/research/**` | Durable investigations and evaluated alternatives. |
| `scripts/**` | Repeatable verification and maintenance automation. |
| `target/` | Generated Cargo output; do not review or commit. |

## Task Routing

| Task or touched area                                               | Read before work                                                                                                                 |
|--------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------|
| Any repository change                                              | `docs/guidelines/repository-guide.md` and `docs/guidelines/engineering-guide.md`                                                 |
| `src/binance.rs` or `src/binance/**`                               | `docs/domain/binance-domain.md`, `docs/guidelines/production-service-guide.md`, and `docs/architecture/binance_spot_flow.puml` |
| `src/jupiter.rs` or `src/jupiter/**`                               | `docs/domain/jupiter-domain.md`, `docs/guidelines/production-service-guide.md`, and `docs/architecture/jupiter_swap_flow.puml` |
| Spread calculation, execution coordination, balances, or inventory | `docs/domain/arbitrage-domain.md`, both integration domain guides, `docs/guidelines/production-service-guide.md`, and both exchange flow diagrams |
| Credentials, signing, wallet keys, or secret storage               | The relevant integration domain and the security section of `docs/guidelines/production-service-guide.md`                    |
| Setup, tests, Cargo, scripts, or releases                          | Relevant sections of `docs/guidelines/repository-guide.md`; for releases also read `docs/guidelines/release-workflow.md`         |
| Architecture diagrams or runtime flow                              | Relevant files under `docs/architecture/**` and `docs/guidelines/engineering-guide.md`                                           |
| Tool selection or prior investigation                              | Relevant notes under `docs/research/**`                                                                                          |
| Documentation, review, or refactoring                              | `docs/guidelines/engineering-guide.md` plus the guide for the touched area                                                       |
| Non-trivial planning and delivery                                  | `docs/guidelines/project-lifecycle.md`                                                                                           |

## Repository-Specific Instructions

- Treat `src/**`, Cargo manifests, order schemas, exchange request signing, and balance semantics as production work.
- Never add real credentials, private keys, seed phrases, or wallet files to the repository.
- Assume exchange calls can have financial side effects. Verification must use parsing fixtures, mocks, or explicit testnet/dry-run paths unless the owner authorizes otherwise.
- Use `rust_decimal::Decimal` or integer base units for money and token quantities; do not introduce binary floating-point arithmetic for trading decisions.
- Keep repeatable verification and maintenance automation under `scripts/**`.
- Update durable domain, guideline, or architecture documentation when behavior, boundaries, configuration, or runtime flow changes.
- During review, apply `docs/guidelines/engineering-guide.md` and the production checklist for the touched area.
