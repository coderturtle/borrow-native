# Maintainers

This is the internal/agent-facing doc. Learners should read the top-level `README.md` instead; this file is for anyone working on the workshop itself.

**Classification:** factory-output
**Lifecycle:** active
**Owner:** coderturtle
**Promotion target:** `none`

This repo has two goals:

1. **Ship a workshop** teaching Rust (ownership, borrowing, structs/enums, generics/traits/lifetimes, error handling, concurrency) to agent-literate practitioners, taught by running every exercise through a real harness with the compiler and test suite as a deterministic gate, and Coachgremlin grading the conceptual/idiomatic layer on top.
2. **Feed evidence back into the reusable machinery**: this is the Workshop Gremlin's *second* real run (`terminal-velocity` was its first) and its first on a subject other than agentic engineering. It found and wrote back a genuine augmentation — a deterministic-gate tier, concept-dependency arc, and canonical-curriculum-anchor research step — into the canonical **Workshop Gremlin** and **Coachgremlin** definitions (`~/hekton/gremlins/`), for reuse by future tech/language workshops.

## Implementation Status

- 2026-07-05 — Scaffolded as factory-output (as `rust-workshop`). Naming pass complete: **Borrow Native**. Both design docs complete: see [Workshop Design](workshop-design.md) (the workshop itself: audience, deterministic-gate method, 6-module + capstone arc anchored to the Rust Book/Rustlings/Exercism) and [Workshop Gremlin Design](workshop-gremlin-design.md) (what this run changed in the canonical Gremlin definitions).
- First [Workshop Review Panel](review-panel/2026-07-05-initial-design.md) run complete against the naming + design docs — all seven personas returned distinct findings; the cheap, design-doc-text fixes were applied in the same pass (see that report's prioritized action list for what was fixed vs. deferred).
- Module skeleton (`modules/`), brand layer (`docs/brand.md`), and this maintainers split are done. Build-log/Pages site is the remaining Completion Condition item — see [Next Actions](next-actions.md).

## Documentation Contract

Agents working here must inspect `.hekton/project.yaml` before structural changes, keep `docs/session-log.md` current, record meaningful design decisions in `docs/decisions.md`, and update `docs/next-actions.md` when the work queue changes.

Vault mutation is not allowed by default. The repo-local `mind-palace/` folder is only a mirror draft; do not write to the live vault unless explicitly authorised in-session.

## Voice and style for published content

Anything a learner reads (README, module content, build-log entries, the site once built) follows `docs/brand.md` — voice, hard rules (no em dashes, no unqualified efficacy claims, no framing the compiler as an adversary), banned phrases. Internal docs under `docs/` are working documents and are exempt.

## Key Docs

- [Workshop Design](workshop-design.md) — audience, format, deterministic-gate teaching method, curriculum-anchored module arc
- [Workshop Gremlin Design](workshop-gremlin-design.md) — what this run changed in the canonical Workshop Gremlin / Coachgremlin / Review Panel definitions
- [Brand / Style Layer](brand.md) — voice, hard rules, visual identity
- [Workshop Review Panel Report](review-panel/2026-07-05-initial-design.md) — 7-persona critique of the naming + design docs, first run
- [Modules index](../modules/README.md) — the full arc, gate tiers, and per-module skeleton status
- [Session Log](session-log.md)
- [Decisions](decisions.md)
- [Risks](risks.md)
- [Project Walkthrough](project-walkthrough.md)
- [Next Actions](next-actions.md)
- [Operating Model](operating-model.md)
- [Human Understanding Check](human-understanding-check.md)
- [Depth Decision](depth-decision.md)
- [Retire / Promote Review](retire-promote-review.md)
