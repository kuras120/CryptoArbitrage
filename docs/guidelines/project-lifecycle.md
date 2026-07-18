# Project Lifecycle

Use this workflow for non-trivial changes, especially work that requires architecture, product, risk, or execution decisions before implementation.

## 1. Project Proposal

Create one Markdown project proposal. The proposal must describe:

- the problem and intended outcome;
- confirmed decisions and assumptions;
- affected components and boundaries;
- the proposed runtime or data flow;
- financial and operational failure handling;
- user-visible behavior and safety controls;
- implementation stages;
- tests and acceptance criteria;
- open questions requiring owner decisions.

Set the status to `PROPOSED`. Do not start production implementation during this stage.

## 2. Proposal Review

The repository owner reviews the project file. Apply decisions and corrections to the proposal so the accepted design is explicit.

Implementation starts only after owner acceptance. Set the status to `APPROVED FOR IMPLEMENTATION` when approval is given.

## 3. Implementation

Implement only the accepted scope. Keep the project proposal during development and record material deviations or newly discovered risks. Do not finalize permanent documentation yet.

Run verification proportionate to the change and prepare a review summary containing:

- changed files and behavior;
- design deviations;
- automated test results;
- testnet, simulation, or manual verification still required;
- known limitations and follow-up work.

Set the status to `IMPLEMENTED`.

## 4. Implementation Review

The repository owner reviews the implementation and verification results. Requested changes return to implementation.

Do not finalize durable documentation or delete the project file until the owner explicitly accepts the implementation. Set the status to `APPROVED FOR DOCUMENTATION` after acceptance.

## 5. Documentation And Cleanup

After implementation acceptance:

1. Update permanent documentation to match the accepted behavior and decisions.
2. Update visitor-facing documentation when purpose, maturity, or usage changed.
3. Preserve reusable investigation outcomes as permanent research notes.
4. Remove the completed project proposal.
5. Run link, formatting, lint, test, and build checks.
6. Prepare the final change set for merge or release approval.

Git history preserves the reviewed plan while the working tree retains only current documentation.

## Rules

- A proposal is not authorization to implement.
- A passing build is not implementation acceptance.
- Permanent documentation describes implemented behavior, clearly separating future work.
- One project file covers one cohesive change.
- Unrelated improvements become separate projects or follow-up items.
- Any plan that can submit orders or transactions must specify testnet/dry-run verification and recovery from partial execution.

## Project Template

```markdown
# <Change Name>

## Status

- Phase: PROPOSED | APPROVED FOR IMPLEMENTATION | IMPLEMENTED | APPROVED FOR DOCUMENTATION | DONE

## Scope

- <What changes>
- <What stays out of scope>

## Decisions And Risks

- <Confirmed decision or assumption>
- <Financial, security, or operational risk and mitigation>

## Implementation Plan

1. [pending] <Step>
2. [pending] <Step>

Use `[pending]`, `[in-progress]`, or `[done]` so interrupted work remains resumable.

## Verification

- `<command>`
- <Testnet, simulation, or manual acceptance step>

## Result

- <Temporary summary. Delete this file after owner acceptance, cleanup, and permanent documentation updates.>
```
