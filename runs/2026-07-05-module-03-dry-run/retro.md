# Retro: Coachgremlin's Module 03 dry run (Structs, Enums & Pattern Matching)

Go/no-go on Module 03's real content, and a direct test of `docs/next-actions.md`'s own instruction:
don't assume Modules 01-02's finding (deterministic gate can't distinguish defensive over-cloning)
generalizes to a concept with no cloning in it at all, without checking it again.

## Checklist, checked against evidence

**Coachgremlin can frame Module 03's exercise from the module README's stated gate and takeaway,
without inventing an unrelated scenario.**
Yes. `next_action` (`fixtures/relay/SPEC.md`'s Module 03 section) is a direct instance of the
module's own stated question ("how do I model this data so illegal states are unrepresentable") -
`CheckpointTrigger`/`HumanResponse`/`NextAction` are enums specifically because at most one variant of
each can ever be true at once, not a substitute scenario bolted on.

**Given a deliberately good attempt and a deliberately naive (not gaming) attempt, the deterministic
tier alone does not discriminate, and Coachgremlin's conceptual tier does - checked fresh, not assumed
from Modules 01-02.**
Yes, and it's a genuinely different mechanism, not a third instance of the same clone-shaped symptom.
`attempt-good` and `attempt-naive-wildcard-match` both pass `cargo test` (16/16) and default `cargo
clippy -- -D warnings` (zero output) identically - see `grading.md`. The naive attempt's actual
mistake (`_ => NextAction::Continue` standing in for two `CheckpointTrigger` variants under `Ignored`)
is an exhaustiveness-erosion smell, not a cloning one: correct for every variant that exists today,
but silently absorbing any future variant without a compile error forcing a decision. **Is
consistent with** Coachgremlin's two-tier model generalizing to a concept with no ownership/borrowing
content in it at all, not just to different flavors of the same failure mode - a broader claim than
Modules 01-02 alone could support, and named at that strength deliberately, not overstated further;
see this retro's "What's still open" section.

**Checked whether a stricter deterministic tier would close the gap, rather than assuming Module
02's `clippy::pedantic`-gives-no-signal result carries over unchanged.**
Yes, and it didn't carry over the way either prior module would have predicted - `clippy::pedantic`
doesn't just fail to help here, it recommends the wrong direction. It flags `attempt-good`'s two
explicit, intentionally-separate arms (`match_same_arms`) and suggests merging them - the same
simplification `attempt-naive-wildcard-match` already made. Followed uncritically, pedantic's own
advice here would push a learner toward the anti-pattern this module's rubric exists to catch, not
away from it. `clippy::nursery` gives no discriminating signal. The lint that does catch it,
`clippy::wildcard_enum_match_arm`, lives in clippy's `restriction` group - off by default, not bundled
into `pedantic` or `nursery`, and clippy's own documentation says `restriction` lints are meant to be
opted into individually, not as a group, since some conflict with idiomatic style outright. Decision:
this module's stated deterministic gate stays at default `cargo clippy`, same as Modules 01-02 - the
conceptual tier is doing the real discriminating work, and in this case a stricter default lint tier
would have actively misled rather than merely fallen short.

**Coachgremlin's feedback references the actual diff, gives one concrete next try, and never hands
the fix over.**
Yes. `grading.md`'s scoring table points at the specific line (`_ => NextAction::Continue` under the
`Ignored` arm) and names what it costs (a future trigger variant silently inheriting `Continue`
instead of forcing a decision), without handing over the fixed code.

**The terminal state (test + clippy output) is observed running, not asserted.**
Yes. `transcript.txt` in each `attempt-*/fixture/` directory is real, captured `cargo test`/`cargo
clippy` output across all four lint levels checked (default, pedantic, nursery,
`wildcard_enum_match_arm`), not narrated.

**Step 6 produces a takeaway that actually helps on a second, different problem, not just a file that
was written.**
Yes. See `.claude/skills/enum-modeling-playbook/SKILL.md` and `takeaway-validation/` in this run
directory - the packaged playbook was applied to an unrelated modeling scenario before being called
done.

**The Human Gate holds: completion is a recommendation with `human_confirmed: false` in a `runs/`
entry, not a self-certified "complete."**
Yes - see the new `run-*.yaml` entry for this run.

**Step 0 (ARB check) and the batch-review cadence (step 7) were both actually checked, not just
referenced.**
Yes. Step 0: clean baseline confirmed before the change, correct fire after adding the new enums/
function to `src/lib.rs`, resolved by confirming the change was purely additive (build succeeds,
Modules 01-02's own tests fail identically to before - nothing re-broken). Step 7: checked and
correctly deferred - Module 03 is only the first module of content since the Modules 01+02 batch,
inside the 2-3-module window, not yet due; recorded as a pending trigger in `docs/next-actions.md`
rather than silently skipped.

**New this run: the clone-count-check pre-filter (Workflow step 3's new sub-step) was actually run,
not assumed irrelevant.**
Yes. Run against both attempts' diffs; correctly reports 0 clone-count matches for both, a true
negative rather than a false sense of coverage - this module's failure mode isn't clone-shaped, and
the script was never meant to catch an exhaustiveness smell. Recorded in `grading.md` rather than
skipped without comment, since silently not running a stated step is exactly the kind of gap this
workshop's own retros have caught before (`docs/decisions.md`, `arb-trigger-check.sh`'s missing
script).

## Go/no-go

**Go.** Module 03's exercise, rubric, and finding are real, evidenced, and mechanistically distinct
from Modules 01-02's - a genuinely different way the deterministic gate can miss something the
conceptual tier catches, not a restatement of "clippy doesn't see defensive cloning" in new clothes.
Proceeding with the same per-module dry-run discipline for Modules 04-08.

## Revision fed back

1. **`~/hekton/gremlins/coaching/coachgremlin.md`**: this is Coachgremlin's fourth real dry run and
   third workshop-internal one for `borrow-native` (fifth counting the same-day clone-count-check
   trial as a distinct, if narrower, extension) - Version/Status line updated to cite this run's
   finding (a third, mechanically distinct instance of "the deterministic gate misses a real
   idiom/design smell," this time with `clippy::pedantic` actively recommending the anti-pattern
   rather than merely failing to catch it).
2. **`modules/03-structs-enums-pattern-matching/README.md`**: rewritten from skeleton to real
   authored content reflecting this dry run's actual evidence.
3. **Takeaway packaged and validated**: `.claude/skills/enum-modeling-playbook/SKILL.md` plus
   `takeaway-validation/` in this run's own directory.

## Toward Coachgremlin's Review Trigger

This run's finding is a new, mechanically distinct instance of "the two-tier model catches something
a single deterministic gate (even an escalated one) would miss" - the third such instance for this
workshop (ownership/Module 01, borrowing/Module 02, now enums-and-exhaustiveness/Module 03), and the
first with no cloning involved at all, which is real breadth-*within*-the-workshop evidence that the
model isn't specific to memory-management concepts. Per the same standard already applied to Module
02 (`docs/decisions.md`, 2026-07-05): this still counts as **depth** evidence for the two-tier
model generalizing across concepts within `borrow-native`, not a new **breadth** data point toward
Coachgremlin's "3+ runs across 2+ workshops" bar, which stays at 2 - flagged here, not
self-certified, per coderturtle's own prior ruling on this exact distinction.

## What's still open

- Modules 04-08 still need the same treatment.
- Same single-grader limitation as Modules 01-02's retros named: this dry run used one session (this
  one) constructing and grading both attempts. An independent blind pass would be a stronger test.
- The `clippy::pedantic`-recommends-the-anti-pattern finding is itself only one data point. Whether
  this is a general property of `match_same_arms` whenever explicit-for-forward-compatibility
  arms happen to share a body, or specific to this exercise's shape, is not yet checked - worth
  watching for in Modules 04-08 if a similar exhaustive-match situation recurs.
