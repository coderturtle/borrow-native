# Coachgremlin's dry run: Module 03 (Structs, Enums & Pattern Matching) on `relay`

First real content-authoring pass for Module 03, per Coachgremlin's Workflow. Same discipline as
Modules 01 and 02's dry runs: a correct attempt and a deliberately naive, honest (non-adversarial)
attempt, both implementing `next_action(trigger: &CheckpointTrigger, response: &HumanResponse) ->
NextAction`. Checked fresh, not assumed to generalize from Modules 01-02's clone-shaped finding -
this module's subject (enums/pattern matching) has no cloning in it at all, so the failure mode had
to be found on its own terms.

## Step 0: ARB trigger check

`fixtures/relay/src/lib.rs` and `SPEC.md` both needed real changes: Module 03's feature
(`CheckpointTrigger`/`HumanResponse`/`NextAction` enums, `next_action`) didn't exist yet.
`scripts/arb-trigger-check.sh --dry-run` confirmed a clean baseline before the change, then correctly
fired against `src/lib.rs` after it. Resolution: the change is purely additive (three new enums, one
new function; no existing struct, field, or function signature touched), verified by running
`cargo build` (compiles, only expected unused-argument warnings on the new `todo!()` stub) and
re-running Module 01/02's own test suites (`cargo test --test finalize_session --test
session_stats`) - identical failure output to before the change (same 5+6 tests still panicking on
their own still-unsolved `todo!()`s), confirming this addition didn't silently change or break either
already-shipped, already-graded module.

## Step 1-2: Frame the exercise + rubric

Task: implement `next_action`, deciding what `relay` does next given why a checkpoint fired
(`CheckpointTrigger`) and how the human responded (`HumanResponse`). Five required cases in
`fixtures/relay/tests/next_action.rs`: `Acknowledged` continues regardless of trigger (checked
against two different triggers), `Snoozed(secs)` pauses for exactly that duration regardless of
trigger, and `Ignored` resolves three different ways depending on trigger - `ContextBudget` ends the
session (the safety-critical case), `TimeElapsed`/`ToolCallCount` both continue. Rubric: see
`modules/03-structs-enums-pattern-matching/README.md`.

The three enums themselves (`CheckpointTrigger`, `HumanResponse`, `NextAction`) are given in the
shipped stub, not part of the learner's own exercise - the module's stated question ("how do I model
this data so illegal states are unrepresentable") is answered by their *existence as enums rather
than structs with `bool`/`Option` flags*, which the learner reads before starting, not designs from
scratch. The graded exercise is the match logic against them.

## Step 3: Observe the attempts

**`attempt-good/`** (`diff.patch`): matches on `response` first, then on `trigger` only inside the
`Ignored` arm, listing all three `CheckpointTrigger` variants explicitly - no `_` wildcard anywhere.
`cargo test`: 16/16 pass (5 from Module 01, 6 from Module 02, 5 new). `cargo clippy -- -D warnings`:
clean.

**`attempt-naive-wildcard-match/`** (`diff.patch`): identical outer structure, but the `Ignored` arm
uses `CheckpointTrigger::ContextBudget(_) => NextAction::EndSession, _ => NextAction::Continue` -
covering `TimeElapsed`/`ToolCallCount` with a single wildcard instead of listing them. Behaviorally
indistinguishable from `attempt-good` for every input the current enum defines - the same instinct as
Modules 01-02's naive attempts (do the minimum that makes today's tests pass), aimed at exhaustiveness
instead of ownership this time. `cargo test`: **16/16 pass, identical to `attempt-good`.** `cargo
clippy -- -D warnings`: **also clean, identical to `attempt-good`.**

## Step 4: Score against the rubric

| # | Criterion | attempt-good | attempt-naive-wildcard-match |
|---|---|---|---|
| 1 | `cargo test` green (gate, deterministic) | Pass. 16/16. | Pass. 16/16. |
| 2 | `cargo clippy -- -D warnings` clean (gate, deterministic) | Pass. | **Pass - deterministic gate cannot distinguish them.** |
| 3 | Diff touches only the implementation file (gate, anti-gaming) | Pass. | Pass. |
| 4 | `Ignored` + `ContextBudget` correctly ends the session (gate, deterministic - this one behavioral case the test suite catches directly) | Pass. | Pass - both attempts get today's answer right. |
| 5 | Every `CheckpointTrigger` variant listed explicitly under `Ignored`, no `_` wildcard standing in for variants that could diverge later (scored, conceptual) | Pass. | **Fails.** `_ => NextAction::Continue` silently absorbs `TimeElapsed`/`ToolCallCount` today, and would silently absorb any *future* trigger variant too, without a compile error forcing a deliberate decision. |

**Result: a genuinely different failure mode from Modules 01-02, not the same clone-shaped symptom
re-appearing.** Modules 01-02's naive attempts were about *defensive over-cloning where a borrow would
do*. Module 03's is about *exhaustiveness erosion* - a `_` arm that happens to be correct for every
variant that exists today, but trades away the compiler's forced-decision guarantee the moment a new
variant is added elsewhere in the codebase. Neither `cargo test` nor default `cargo clippy -- -D
warnings` can see this, for the same structural reason cloning was invisible to them: it's a property
of the *code's shape*, not of any input/output pair a test can observe.

**Checked, not assumed, whether a stricter lint closes the gap - and this time the result runs in an
unexpected direction, though not as strong a direction as first claimed here.** `clippy::pedantic`
does **not** flag the naive attempt at all. It instead flags **`attempt-good`**, with
`match_same_arms` ("these match arms have identical bodies") on the two explicit, intentionally-
separate `TimeElapsed`/`ToolCallCount` arms - and its own suggested fix is to merge them into an
or-pattern (`TimeElapsed(_) | ToolCallCount(_) => Continue`). **Corrected 2026-07-05** (Modules
03+04 Workshop Review Panel batch, AI/ML Practitioner persona, verified by compiling both forms
after adding a fourth `CheckpointTrigger` variant): the or-pattern is *not* "functionally the same
simplification the naive attempt already made" - an or-pattern still names every variant explicitly,
so it still fails to compile (`E0004`, non-exhaustive) on the new variant, exactly like the two-
separate-arms version. Following pedantic's suggestion here does not reintroduce rule 5's
exhaustiveness gap. What it *does* cost is the readability signal that `TimeElapsed` and
`ToolCallCount` were each considered on their own - a real but smaller caution than "pedantic
actively recommends the anti-pattern," which overstated what the suggested fix actually changes.
`clippy::nursery` gives no discriminating signal (one `const fn` suggestion, identical on both). The
one lint that does catch the actual wildcard risk,
`clippy::wildcard_enum_match_arm`, lives in clippy's `restriction` group - off by default, not
included in `pedantic` or `nursery`, and clippy's own documentation is explicit that `restriction`
lints are meant to be individually opted into, not bundled, since some conflict with idiomatic style
outright. Enabled directly (`cargo clippy -- -W clippy::wildcard_enum_match_arm`), it flags
`attempt-naive-wildcard-match`'s `_` arm by name and is silent on `attempt-good`. Full transcripts,
all four lint levels captured for both attempts: `attempt-good/fixture/transcript.txt`,
`attempt-naive-wildcard-match/fixture/transcript.txt`.

**Clone-count-check pre-filter, run per the new Workflow step 3 sub-step:** correctly a non-event
here - this module's failure mode isn't clone-shaped, so `scripts/clone-count-check.sh` finds 0
`.clone()` calls in both attempts' `next_action` additions and flags neither. Recorded as a true
negative, not a check failure: the script was never designed to catch an exhaustiveness smell, only
a defensive-cloning one, and correctly stays silent outside its own scope.

## Step 5: Confirm or loop

- **attempt-good:** rubric met.
- **attempt-naive-wildcard-match:** rubric not met on the conceptual tier.

## Step 6: Takeaway

An enum-modeling and exhaustive-match checklist, `.claude/skills/enum-modeling-playbook/SKILL.md`,
built directly from this finding (including the pedantic-points-the-wrong-way caution) and validated
against a second, unrelated modeling problem before being called done (`takeaway-validation/` in this
same run directory).

## Step 7: Batch-review cadence

Modules 01+02 already had the first content-level panel batch
(`docs/review-panel/2026-07-05-modules-01-02-content.md`). Module 03 is the first module of content
since that batch - 1 of the 2-3-module window Workflow step 7 sets, not yet due. Deferred, not
skipped: recorded in `docs/next-actions.md` for whenever Module 04 or 05 completes the next batch.

## Human Gate

Recommendation only. `human_confirmed: false` in the run record, per Coachgremlin's Human Gate.
