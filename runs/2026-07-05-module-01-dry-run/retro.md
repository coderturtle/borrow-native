# Retro: Coachgremlin's first real dry run (Module 01 core)

Go/no-go before authoring Modules 02-08 on the same pattern, and the first real test of this
workshop's central bet: does the two-tier gate (deterministic + Coachgremlin conceptual) actually
do the job a single deterministic tier can't.

## Checklist, checked against evidence

**Coachgremlin can frame Module 01's exercise from the module README's stated gate and takeaway,
without inventing an unrelated scenario.**
Yes. `merge_customer_totals` (`modules/01-ownership-move-semantics/exercise/SPEC.md`) is a direct,
minimal instance of the module's own stated question ("who owns this value, and what happens when
it moves?") — not a substitute scenario.

**Given a deliberately good attempt and a deliberately naive (not gaming) attempt, the
deterministic tier alone does not discriminate, and Coachgremlin's conceptual tier does.**
Yes, and this is the load-bearing finding of this entire dry run. `attempt-good` and
`attempt-naive-clone` both pass `cargo test` (6/6) and both pass `cargo clippy -- -D warnings`
(zero output) — see `grading.md`. **The deterministic gate, as this module currently defines it,
cannot tell these two attempts apart at all.** Only Coachgremlin's conceptual read (does the diff
contain a clone that a by-value loop would have made unnecessary) catches it. This is not the same
finding `terminal-velocity`'s Module 04 dry run made (that was about a rubric-gaming attempt
editing the test file; this is an honest, non-adversarial attempt that simply didn't demonstrate
the concept while passing every mechanical check). Two different reasons a green terminal state can
mislead, both real, both now evidenced.

**Checked whether a stricter deterministic tier would close the gap, rather than assuming Coachgremlin
is the only possible answer.**
Yes. `cargo clippy -- -W clippy::pedantic -D warnings` against `attempt-naive-clone` does produce a
finding (`needless_pass_by_value`), but it's a different lint than the clone itself, it's bundled
with an unrelated stylistic nit (`must_use_candidate`) in the same run, and `clippy::pedantic` is an
opt-in group most real-world Rust projects don't enable wholesale. Decision: **this module's stated
deterministic gate stays at default `cargo clippy`**, matching how most real Rust codebases actually
gate, rather than quietly upgrading to pedantic to make this one exercise's gap disappear — the
honest conclusion is that Coachgremlin's conceptual tier is doing real work here, not that the
deterministic tier was mis-specified.

**Coachgremlin's feedback references the actual diff, gives one concrete next try, and never hands
the fix over.**
Yes. Both feedback blocks in `grading.md` point at the specific line that changed (`for order in
&orders` plus the `.clone()` call) and ask what happens if the loop becomes by-value, without
stating the fixed code.

**The terminal state (test + clippy output) is observed running, not asserted.**
Yes, for both attempts: `transcript.txt` in each `attempt-*/` directory is real `cargo test`/`cargo
clippy` output, captured directly, not narrated.

**Step 6 produces a takeaway that actually helps on a second, different ownership problem, not just
a file that was written.**
Yes. See `../../modules/01-ownership-move-semantics/takeaway/` and `takeaway-validation/` below —
the packaged checklist was applied to an unrelated ownership scenario (not `Order`/`HashMap`
shaped) and correctly identified where a clone was and wasn't necessary.

**The Human Gate holds: completion is a recommendation with `human_confirmed: false` in a `runs/`
entry, not a self-certified "complete."**
Yes. `runs/run-20260705-RW-001.yaml` records `status: done` (the dry run itself finished) with
`human_confirmed: false`.

## Go/no-go

**Go**, with a real, evidenced finding fed back into the canonical Gremlin, not just this
workshop's own docs. Proceeding to author Modules 02-08 with the same exercise-design discipline:
for each module, deliberately construct at least one naive-but-honest (non-gaming) attempt that
passes the deterministic tier, to check the conceptual tier is actually load-bearing there too,
rather than assuming this Module 01 finding generalizes without checking.

## Revision fed back

1. **`~/hekton/gremlins/coaching/coachgremlin.md`**: this run is real evidence, not just a design,
   that the two-tier grading extension (added 2026-07-05, before this run) does real work — updated
   that file's Version/Status line to cite this run and its specific finding (default clippy misses
   architecturally-unnecessary cloning; a stricter lint group catches a proxy for it via a
   different, noisier lint), closing the "designed but not yet run against a real learner attempt"
   gap that file previously flagged honestly.
2. **`modules/01-ownership-move-semantics/README.md`**: rewritten from skeleton to real authored
   content (exercise, rubric, required-to-advance, why this is hard) reflecting this dry run's
   actual evidence, not the placeholder text.
3. **Takeaway packaged and validated**, not just described: `modules/01-ownership-move-semantics/
   takeaway/` plus `takeaway-validation/` in this run's own directory.

## Toward Coachgremlin's Review Trigger

This is run 1 of the 3+ (across 2+ workshops) `coachgremlin.md` names before its contract counts as
stable enough to bump past `draft`. `terminal-velocity`'s Module 04 dry run was a different run, on
a different workshop, on a different failure mode (test-file gaming vs. honest-but-naive
over-cloning) — genuinely two data points now, not the same one counted twice. Still short of 3+.
Whether this run counts toward that total is coderturtle's call, not self-certified here (Human
Gate again): flagged in `runs/run-20260705-RW-001.yaml`'s `human_notes`.

## What's still open

- Modules 02-08 still need the same treatment: a real exercise, a real good-attempt-plus-naive-attempt
  dry run, a rubric that's actually checked for deterministic-tier blind spots, not assumed to have
  none.
- This dry run used one grader (this session, playing Coachgremlin) grading its own two constructed
  attempts, the same limitation `terminal-velocity`'s retro named. A stronger future test would have
  an independent pass grade the same transcripts blind.
- The Ardan Labs exam alignment (see `docs/workshop-design.md`) is still a stated intent; Module 01's
  content here is a step toward it, not evidence toward it yet.
