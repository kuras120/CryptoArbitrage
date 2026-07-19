# Release Workflow

CryptoArbitrage does not currently publish a package or deploy through automated infrastructure. Releases are manual source/binary milestones and must remain testnet-only until the production-readiness requirements below are met.

## Versioning

- The package version lives in `Cargo.toml`.
- Use semantic versions (`MAJOR.MINOR.PATCH`).
- Keep pull request titles and commit subjects in the form `<type>(<optional-scope>): <short summary>`.
- Prefer `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `ci`, `build`, `perf`, or `security` as types.

## Release Steps

1. Confirm the target change has completed the project lifecycle when non-trivial.
2. Update `Cargo.toml`, durable documentation, and release notes when a changelog exists.
3. Run:

   ```bash
   ./scripts/verify.sh
   cargo build --release
   ```

4. Review dependency changes and confirm no secrets or wallet material are tracked.
5. Exercise read-only and execution paths against explicit testnet/dry-run configuration.
6. Create an annotated Git tag from the accepted commit.
7. Publish checksums with any shared binary artifact.

## Production-Readiness Gate

Do not enable production endpoints or label a release production-ready until all of these exist:

- external secret and configuration management;
- explicit dry-run and execution modes;
- automated tests for arithmetic, exchange schemas, and failure states;
- exchange filter and token-decimal validation;
- timeouts, safe retry policy, idempotency, and partial-execution recovery;
- balance reconciliation, risk limits, and operational monitoring;
- an owner-approved runbook for stopping execution and recovering inventory.

## Future Automation

When releases become repeatable, introduce an automated workflow backed by deterministic release tooling. Define version selection, artifacts, tags, rollback, and deployment ownership before enabling it.
