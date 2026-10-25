---
name: maintain-docs
description: Create or update durable repository documentation so it matches implemented behavior. Use when behavior, configuration, architecture, data meaning, setup, verification, portability, research conclusions, or repository routing changes.
---

# Documentation

Apply repository-specific document contracts and verification commands when present.
Keep this workflow usable without a named repository guide.

1. Read every document routed for the affected area, the implementation, and existing
   docs; describe observed behavior, not intentions.
2. Use repository routing when present to select one authoritative home for each
   fact. Do not duplicate project-specific facts in reusable skills or general guides.
3. When present, keep the root README lean: purpose, safety boundary, quick start,
   and essential documentation entry points. Do not add links to individual
   temporary plans.
4. Distinguish current support, known limitations, and proposals. Add dates and source links to time-sensitive research.
5. Update all affected links and commands. Mark generated artifacts clearly and do not edit them as source.
6. Run the repository verification command when declared; otherwise choose
   proportionate checks. Include plan-specific checks, then check Markdown links
   and paths before closeout.
7. Remove an accepted temporary project plan only after durable documentation
   contains its lasting decisions.
8. Treat temporary plans as one-way references: they may link to durable docs, but
   root guidance, reusable skills, durable docs, and other plans must not link to a
   specific temporary plan.
9. Before closeout, audit the affected documents for duplicated facts, stale history,
   routing mistakes, and violations of their document-type guides. Use `audit-docs`
   when available; otherwise perform these checks directly. Keep the audit focused
   on the changed area.
