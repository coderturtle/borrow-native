# Retro: re-validating Module 01 after the shared-project pivot

Go/no-go on whether retiring the standalone `merge_customer_totals` toy fixture in favor of
`fixtures/relay/` (the shared throughline project, see `docs/decisions.md`) preserved Module 01's
actual content, or quietly lost something in the migration.

## Checklist, checked against evidence

**The new exercise (`finalize_session`) is the same lesson as the retired one
(`merge_customer_totals`), not a weaker or different substitute.**
Yes. Same shape (consume a `Vec<T>` by value, produce an aggregate, the naive failure mode is
borrowing-then-cloning instead of moving), now against the project's own real domain types
(`DraftCheckpoint`/`CheckpointRecord`/`Session`) instead of an unrelated `Order`/`HashMap` toy.

**The deterministic-gate-can't-distinguish finding reproduces, not just the exercise's surface
shape.**
Yes, exactly: both `cargo test` (5/5) and `cargo clippy -- -D warnings` (clean) are identical
between `attempt-good` and `attempt-naive-clone`, reproducing the original dry run's core result
against new code. See `grading.md`.

**The packaged takeaway (`ownership-move-checklist` Skill) still applies without modification.**
Yes — it was written at the right level of generality ("do I already own this," not tied to any
one domain), and this re-run is itself a second real validation of that, on top of the original
`takeaway-validation/` check against an unrelated `Option`-pattern problem.

**This re-run does not get double-counted as new evidence toward Coachgremlin's 3-run bar.**
Correctly not claimed as such — see `grading.md`'s closing section. Re-validating a migration is a
different kind of evidence than a new concept or a new failure mode.

## Go/no-go

**Go.** The pivot to a shared throughline project didn't cost this workshop anything at the content
level — Module 01's real exercise, rubric, and finding all survived intact, and the retired toy's
own dry run (`runs/2026-07-05-module-01-dry-run/`) remains valid supporting evidence for the general
finding, just no longer the shipped fixture. Proceeding to build Module 02's feature (session
statistics, borrowing `&[CheckpointRecord]`) against `fixtures/relay/` next.

## What's still open

- Modules 02-08 still need their features built against `fixtures/relay/`, each with its own
  dry-run discipline (a real attempt plus at least one honest-naive attempt where plausible), not
  assumed to inherit Module 01's specific finding.
- The original `runs/2026-07-05-module-01-dry-run/` (against the retired `merge_customer_totals`
  fixture) is kept, not deleted — historical record of a still-true finding, cited from both
  `coachgremlin.md` and this run's own `grading.md`.
