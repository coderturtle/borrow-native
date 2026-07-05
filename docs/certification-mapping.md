# Certification Mapping: Borrow Native → Ardan Labs Rust Certification

> **Skeleton only.** This file's shape and intent are decided; its content is not. See
> `docs/workshop-design.md`'s "Certification Alignment Retrofit (deferred phase)" section for the
> full reasoning: this mapping is deliberately not authored yet, because the only Ardan Labs exam
> information available today is a four-category marketing-page list, not the exam's actual
> structure. Building a module-by-module map from that alone would be precision this workshop can't
> back up. Real authoring is gated on coderturtle actually sitting the exam and recording a
> retrospective - see `docs/next-actions.md`'s "Later" section for the trigger condition.

## What today's placeholder categories are (pre-exam, not yet verified)

Ardan Labs' own public description of the exam names four topic areas. Listed here as the
starting point the real retrofit will check the arc against - explicitly **not** yet confirmed
against the exam's actual weighting, question style, or sub-topics:

- Memory safety
- Type systems
- Asynchronous programming
- Testing/debugging best practices

## Decided shape (once authored)

A table, one row per module (01-08 + capstone), naming which of the exam's real topic areas (as
confirmed by the post-exam retrospective, not the four-category placeholder above) that module's
exercise actually exercises, plus an honest per-module confidence note - "directly exercised,"
"touched but not gated," or "not covered by this arc" - the same gate/scored honesty distinction
this workshop already applies to its own learning objectives (see any module README's Learning
Objectives section for the pattern).

## Not decided yet

- Whether the real exam's topic breakdown turns out to need more than four rows, or a different
  grouping entirely than the marketing-page categories imply.
- Whether any module needs a follow-up exercise added to close a gap the retrospective finds (the
  same kind of gap that produced Module 07, Async Programming, from checking the original arc
  against Ardan's topic list in the first place).
- The exact schema `.claude/skills/certification-tracker/SKILL.md` will read from this file once
  both exist for real - keep them authored together, not this file first and the Skill guessing at
  its shape.

## Human Gate

Once authored for real, this file makes a learner-facing claim about exam alignment, not just an
internal design note. Treat it the same as any module's dry run: an ADR in `docs/decisions.md`, a
`runs/` entry, and `human_confirmed` sign-off before it's presented as more than a draft.
