# Agent Briefing

For implementation tasks, keep a temporary Markdown plan in the repository root (for example `TASK_PLAN.md`). Prefer diagrams, tables, and concise checklists; choose sections to suit the task. Track implementation as checklist items and update progress as you work. Before opening a PR, preserve lasting decisions in docs and unfinished work in `INBOX.md`, then delete the plan. This is the only workflow gate; no separate proposal or implementation-acceptance stages are required.

## Context

Read the relevant retained docs before planning, changing, or reviewing an area. Combine routes only when needed.

| Area | Read / location |
| --- | --- |
| Purpose, setup, verification | `README.md`, `scripts/verify.sh` |
| Runtime and integration flow | `docs/architecture/README.md`; `src/main.rs` selects the workflow |
| Binance: `src/binance.rs`, `src/binance/**` | `docs/domain/binance-domain.md`, `docs/architecture/binance_spot_flow.puml` |
| Jupiter: `src/jupiter.rs`, `src/jupiter/**` | `docs/domain/jupiter-domain.md`, `docs/architecture/README.md#jupiter-quote-workflow` |
| Spreads, execution coordination, balances, inventory | `docs/domain/arbitrage-domain.md` and both integration domains/flows |
| Prior investigations and tool choices | Relevant notes in `docs/research/**`; retain dated findings and sources |
| Follow-ups and open decisions | `INBOX.md` |
| Documentation and requested reviews | Relevant skills in `.agents/skills/**` |

## Working Rules

- Preserve existing user changes. Keep code, docs, plans, scripts, and user-facing messages in English.
- Treat exchange schemas, signing, balances, and order execution as production-sensitive. Keep exchange adapters separate from arbitrage policy and make side effects explicit.
- Use `rust_decimal::Decimal` or integer base units for money and token quantities; never binary floating point for trading decisions.
- Never commit or log credentials, private keys, seed phrases, wallet files, or signed transactions.
- Verify using fixtures, mocks, or explicit testnet/dry-run paths. Exchange execution requires owner authorization; automated verification must not place orders or submit transactions.
- Review precision, data freshness, venue filters, partial fills, and unknown outcomes. Preserve reconciliation IDs and never blindly retry state-changing requests.
- Keep repeatable validation in `scripts/**`; run `./scripts/verify.sh` for code changes and proportionate reference/format checks for docs. Report failures and unverified behavior accurately. Ignore generated `target/` output in reviews.
- Keep durable docs short and aligned with implemented behavior. Prefer tables and diagrams where useful; distinguish current support from intended domain boundaries and future work.
