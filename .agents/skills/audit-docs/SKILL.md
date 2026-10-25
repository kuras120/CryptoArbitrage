---
name: audit-docs
description: Audit and compact repository documentation while preserving durable decisions and evidence. Use when documentation or agent instructions appear duplicated, stale, overgrown, misrouted, inconsistent with implemented behavior, or hard to maintain.
---

# Documentation Audit

Reduce documentation entropy. Prefer one authoritative home for each fact or rule and delete repetition after preserving unique evidence.

Apply repository-specific documentation contracts and verification commands when
present. Keep this audit usable without a named repository guide.

## Guardrails

- Read every document routed for documentation work.
- Preserve user edits and sourced evidence. Do not delete an active project merely to reduce file count.
- Never add a durable inbound link to a specific temporary project plan.
- Treat line count as a signal, not a target. Clarity and ownership matter more than arbitrary brevity.

## Audit

1. Inventory root guidance, documentation, and repository skills when present. Record
   headings, links, status/date, and likely owner category.
2. Assign each fact or rule to one authoritative location by document purpose; follow
   the repository map and documentation policy when present.
   For each affected document, compare its structure, metadata, content ownership,
   and provenance treatment with the repository's guide for that document type when
   one exists. Flag mismatches even when structural scripts pass.
3. Flag:
   - the same rule or decision in multiple owners;
   - proposals presented as current behavior;
   - plans or implementation sequences inside research/guidelines;
   - one-off tasks inside root agent instructions;
   - detailed research/project links in root README;
   - stale open questions, completed remediation history, orphan files/links;
   - specific temporary-plan links outside their own plan;
   - long prose that repeats a table or conclusion;
   - dense paragraphs, long sentences, or flat lists that make distinct rules,
     stages, or comparisons hard to scan.
4. For each flag choose `keep`, `move`, `merge`, `compress`, `delete`, or `leave temporary`.

## Cleanup Order

1. Preserve unique facts, decisions, dates, sources, warnings, and accepted constraints in their authoritative owner.
2. Replace duplicate prose with a link only when both documents are durable and the dependency is stable.
3. Remove plan/checklist language from research after its conclusion is durable.
4. Compress old audits to the current finding, lasting boundary, and reusable gate; drop superseded operational narration.
5. Keep project plans until implementation and owner acceptance. Then move lasting decisions and delete the plan.
6. Improve presentation where it helps readers find and compare information. Use
   short sentences, focused sections, lists, or tables to match the content. Preserve
   meaning and avoid splitting connected rules into a rigid template.
7. Do not create an audit report document unless the owner requests one; summarize findings in the task result.

## Verification

- Search for removed filenames, duplicate key phrases, inbound links to temporary
  plans, unresolved TODOs, and broken relative Markdown targets.
- Compare pre/post line counts and explain material deletions.
- In Git repositories, run `git diff --check`. Validate changed skills and run the
  repository verification command when declared; otherwise use proportionate checks.
  Include any plan-specific checks.
- Report each document as retained, compressed, merged, deleted, or temporary, plus any remaining ambiguity requiring owner judgment.
