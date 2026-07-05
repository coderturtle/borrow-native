# Coachgremlin's dry run: Module 01 on `relay` (post-pivot), graded

Re-run of the Module 01 exercise against the new shared throughline project (`fixtures/relay/`),
after retiring the standalone `merge_customer_totals` toy fixture in favor of one growing project
across all 8 modules (see `docs/decisions.md` for the pivot decision). Same discipline as the
original dry run (`runs/2026-07-05-module-01-dry-run/`): a correct move-based attempt and a
deliberately naive, honest (non-adversarial) clone-heavy attempt, both against `finalize_session`.

## Step 3: Observe the attempt

**`attempt-good/`**: one-file diff, `.into_iter().map(...)` consumes `drafts` by value, moving each
`DraftCheckpoint`'s fields directly into its `CheckpointRecord`. No `.clone()` anywhere. `cargo
test`: 5/5 pass. `cargo clippy -- -D warnings`: clean.

**`attempt-naive-clone/`**: also a one-file diff, `for draft in &drafts` borrows, forcing
`.clone()` on all three fields (`goal`, `summary`, `risks`) to build each record. `cargo test`:
5/5 pass, identical to `attempt-good`. **`cargo clippy -- -D warnings`: also clean.**

## Step 4: Score against the rubric

Same rubric as the original dry run (`../2026-07-05-module-01-dry-run/grading.md`), reapplied here:

| # | Criterion | attempt-good | attempt-naive-clone |
|---|---|---|---|
| 1 | `cargo test` green (gate, deterministic) | Pass. 5/5. | Pass. 5/5. |
| 2 | `cargo clippy -- -D warnings` clean (gate, deterministic) | Pass. | **Pass — confirms the original finding transfers.** |
| 3 | Diff touches only the implementation file (gate, anti-gaming) | Pass. | Pass. |
| 4 | No `.clone()` on data the function could have moved instead (scored, conceptual) | Pass. | **Fails.** Three separate unnecessary clones (`goal`, `summary`, `risks`), all avoidable by consuming `drafts` by value. |
| 5 | Ownership shape matches what the body needs (scored, conceptual) | Pass. | **Fails.** `drafts` is taken by value but only ever borrowed in the loop. |

**Result: identical outcome to the original dry run.** The finding (deterministic gate alone can't
distinguish these two attempts) is not an artifact of the specific `merge_customer_totals` toy — it
reproduces exactly against the real project's own domain types, which is the actual point of this
re-run: confirm the lesson (and the exercise design) survived the migration, not just the fixture
code.

## Step 5: Confirm or loop

- **attempt-good:** rubric met.
- **attempt-naive-clone:** rubric not met on the conceptual tier, same as before.

## Takeaway

Unchanged: `.claude/skills/ownership-move-checklist/SKILL.md` (already packaged and validated
against a second, unrelated ownership problem in the original dry run) applies here without
modification — the checklist was written generally ("do I already own this," not "is this
`Order`/`HashMap` shaped"), and this re-run is itself further validation that it generalizes: a
third, real-domain instance of the same lesson, not just the one it was extracted from plus one
synthetic check.

## What this re-run is and isn't evidence of

**Is:** confirmation that retiring the toy fixture in favor of the shared `relay` project didn't
lose or weaken Module 01's actual pedagogical content — the exercise, rubric, and finding all
transferred cleanly.

**Isn't:** a new, independent data point toward Coachgremlin's 3-run Review Trigger. This is the
same lesson re-instantiated, not a new concept or a new failure mode — `~/hekton/gremlins/coaching/
coachgremlin.md`'s run count stays at 2 (terminal-velocity Module 04, borrow-native Module 01),
not 3.

## Human Gate

Recommendation only. `human_confirmed: false` in `runs/run-20260705-RW-002.yaml`, per Coachgremlin's
Human Gate.
