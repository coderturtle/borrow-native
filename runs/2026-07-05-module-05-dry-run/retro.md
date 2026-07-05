# Retro: Coachgremlin's Module 05 dry run (Error Handling as Idiomatic Control Flow)

Go/no-go on Module 05's real content, and a direct test of whether Modules 01-04's two-tier finding
(the deterministic gate can't distinguish a correct attempt from a naive one) holds for a concept
none of them touched: custom error-type design and `?`-based propagation. Not assumed to carry over
just because the surface shape (a naive attempt reaching for a plain, already-compiling type) looks
familiar from Module 04.

## Checklist, checked against evidence

**Coachgremlin can frame Module 05's exercise from the module README's stated gate and takeaway,
without inventing an unrelated scenario.**
Yes. `parse_config`/`write_handoff_summary` (`fixtures/relay/SPEC.md`'s Module 05 section) are direct
instances of the module's own stated question ("how do I propagate failure without panicking, in a
way callers can actually act on") - `ConfigError`'s three named variants are the "caller could match
on it" half, `HandoffError`'s shipped `Io(String)` stub is the "propagate without losing structure"
half, not a substitute scenario bolted on.

**Given a deliberately good attempt and a deliberately naive (not gaming) attempt, the deterministic
tier alone does not discriminate, and Coachgremlin's conceptual tier does - checked fresh, not
assumed from Modules 01-04.**
Yes, and the underlying mechanism is genuinely new. `attempt-good` and
`attempt-naive-stringified-io-error` both pass `cargo test` (32/32) and default `cargo clippy -- -D
warnings` (zero output) identically - see `grading.md`. The naive attempt's actual mistake
(`HandoffError::Io(e.to_string())`, discarding the real `std::io::Error` the moment it's caught) is
error-type-design avoidance, not any of Modules 01-04's four prior mechanisms (ownership-move
cloning, borrow cloning, exhaustiveness erosion, lifetime avoidance/over-annotation) - this module
introduces custom error types and `?`/`From`-based propagation for the first time, so the failure
mode being new is the expected result of checking fresh, not a coincidence.

**Checked whether a stricter deterministic tier, or a broader lint search, would close the gap,
rather than assuming any prior module's escalation result carries over unchanged.**
Yes, and this module's result is more mixed than any prior module's, in a specific and useful way.
For the `HandoffError` finding: `clippy::pedantic` and `clippy::nursery` give zero discriminating
signal, matching Module 04's starkest case. The full `clippy::restriction` group, run in full and
diffed programmatically by lint code and file/line (`--message-format=json`, not spot-checked with a
few named lints), does contain `clippy::missing_trait_methods` firing one extra time on the naive
attempt - a real but weak signal, since that lint generically flags any unoverridden default trait
method file-wide and isn't aimed at source-chain preservation specifically. `restriction` also
contains `clippy::question_mark_used`, which - like Module 04's `single_char_lifetime_names` -
points at the *correct* attempt (for using `?`), not the naive one. Decision: this module's stated
deterministic gate stays at default `cargo clippy`, same as every prior module - the conceptual tier
still does effectively all of the discriminating work for this finding, with only one weak,
generic, off-by-default-group lint as a partial assist, checked directly rather than assumed to be
either "nothing helps" (Module 04) or "a real rescue" (Module 03) without looking.

**New this run: for the module's *other* naive shape (skipping `?` for manual matches), does the
deterministic gate actually help, and is that consistent with or different from every prior
module's escalation result?**
Checked directly, not assumed. Different, and notably so: `clippy::question_mark`, part of the
**default** "style" lint group (no `-W` flag needed), genuinely fails a hand-rolled `Result ->
Result` passthrough match with no error transformation (`extra-check-manual-match-no-question-mark/`,
`cargo clippy -- -D warnings` → 2 errors) - but stays silent on the mechanically similar `Option ->
Result` case, confirmed by isolating the two Option matches alone (0 warnings). This is the first
time in this workshop's five dry runs that the *default* lint set outright rescues part of a naive
shape, rather than giving a noisy partial signal (Module 01), an off-by-default lint requiring
enabling by name (Module 03), or nothing at any level (Module 04). It's also why the primary
attempt-good/attempt-naive-stringified-io-error comparison keeps both attempts' `parse_config`
identical (both use `?` throughout) - had the naive attempt also skipped `?` there, `cargo clippy -D
warnings` alone would already separate the two attempts, muddying whether the `HandoffError` finding
was real or just riding along with a gate-catchable mistake. Isolating one naive shape per contrast
is what kept this clean, same discipline Module 04 applied to its own two naive shapes.

**Checked, not assumed: is the clone-count pre-filter's silence here a true negative or a gap in the
tool?**
Checked directly. True negative. `scripts/clone-count-check.sh` against both `diff.patch` files
(baseline 0, path-filtered to `lib.rs`) reports 0 clones in both attempts and stays correctly silent
- this module's naive failure mode doesn't route through `.clone()` at all (it's about discarding
structure via `.to_string()`, not copying data), exactly matching the pre-filter's own stated scope
rather than exposing a hole in it. The first true negative since Module 03's.

**Coachgremlin's feedback references the actual diff, gives one concrete next try, and never hands
the fix over.**
Yes. `grading.md`'s scoring table points at the specific line (`HandoffError::Io(e.to_string())`)
and names what it costs (no way to recover `std::io::Error`'s structure or chain it via `source()`
afterward), without handing over the fixed code.

**The terminal state (test + clippy output) is observed running, not asserted.**
Yes. `transcript.txt` in each `attempt-*/fixture/` directory (and
`extra-check-manual-match-no-question-mark/fixture/`) is real, captured `cargo test`/`cargo clippy`
output across default, `pedantic`, `nursery`, and the full `restriction` group, not narrated - the
`restriction`-group diff was generated programmatically (`--message-format=json` piped through a
small script, not eyeballed), same rigor as Module 04's.

**Step 6 produces a takeaway that actually helps on a second, different problem, not just a file
that was written.**
Yes. See `.claude/skills/custom-error-type-template/SKILL.md` and `takeaway-validation/` in this run
directory - validated against a second, unrelated problem before being called done.

**The Human Gate holds: completion is a recommendation with `human_confirmed: false` in a `runs/`
entry, not a self-certified "complete."**
Yes - see the new `run-*.yaml` entry for this run.

**Step 0 (ARB check) and the batch-review cadence (step 7) were both actually checked, not just
referenced.**
Yes. Step 0: clean baseline confirmed before the change, correct fire after adding the new structs/
enums/functions to `src/lib.rs`, resolved by confirming the change was purely additive (build
succeeds once `#[derive(Debug)]` was added where `std::error::Error`'s `Debug + Display` bound and
`Result::unwrap_err`'s `Debug` bound required it, Modules 01-04's own tests fail identically to
before - nothing re-broken). Step 7: checked and found **not due yet** - Module 05 is only the first
module of content since the Modules 03+04 batch (1 of the 2-3-module window), per
`docs/next-actions.md`.

## Go/no-go

**Go.** Module 05's exercise, rubric, and finding are real, evidenced, and mechanistically distinct
from Modules 01-04's - discarding a real error's structure into a string the instant it's caught is
a new failure mode this workshop hadn't exercised, and unlike any prior module, this run found the
deterministic gate's blindness to be genuinely *mixed* rather than uniform: near-total for the
error-type-design half (`HandoffError`), but real and default-clippy-visible for part of the
`?`-avoidance half (the `Result`-to-`Result` case). That mix is itself useful evidence, not a
messier result to smooth over. Proceeding with the same per-module dry-run discipline for Modules
06-08.

## Revision fed back

1. **`~/hekton/gremlins/coaching/coachgremlin.md`**: this is Coachgremlin's sixth real dry run and
   fifth workshop-internal one for `borrow-native` - Version/Status line updated to cite this run's
   finding (a fifth, mechanically distinct instance of "the deterministic gate misses a real
   idiom/design smell," this time a genuinely mixed result rather than a uniformly blind or
   uniformly rescued one).
2. **`modules/05-error-handling/README.md`**: rewritten from skeleton to real authored content
   reflecting this dry run's actual evidence.
3. **Takeaway packaged and validated**: `.claude/skills/custom-error-type-template/SKILL.md` plus
   `takeaway-validation/` in this run's own directory.

## Toward Coachgremlin's Review Trigger

This run's finding is a new, mechanically distinct instance of "the two-tier model catches something
a single deterministic gate (even an escalated one) would miss" - the fifth such instance for this
workshop, and the first with a genuinely mixed deterministic-gate result (near-blind on one half,
partially sighted on the other) rather than uniform blindness or uniform silence. Per the same
standard already applied to Modules 02-04 (`docs/decisions.md`, 2026-07-05): this still counts as
**depth** evidence for the two-tier model generalizing across concepts within `borrow-native`, not a
new **breadth** data point toward Coachgremlin's "3+ runs across 2+ workshops" bar, which stays at 2.

## What's still open

- Modules 06-08 still need the same treatment.
- Same single-grader limitation as Modules 01-04's retros named: this dry run used one session
  (this one) constructing and grading both full attempts plus the extra check. An independent blind
  pass would be a stronger test.
- The `clippy::question_mark` finding (default clippy partially catches `?`-avoidance) is worth
  watching in Modules 06-08: if a future module's naive shape also happens to be a same-type
  `Result`-to-`Result` passthrough, the deterministic gate may do more of the discriminating work
  there than Coachgremlin's conceptual tier - not a reason to skip the conceptual read, but a reason
  to check which half of a naive attempt's mistake the gate actually reaches before assuming it's
  blind.
- The batch-review cadence does not fire with this module (1 of 2-3 since the last batch) - due once
  Module 06 or 07 completes the window, per `docs/next-actions.md`.

## Addendum (same day, post-review): `thiserror` authorized, finding re-verified

**coderturtle confirmed this dry run (go) and, same session, authorized `thiserror` as a real Cargo
dependency** - the `docs/decisions.md` entry above already names *why* it wasn't added unilaterally
(this project's `dependency_changes: human_required` governance gate), and that reasoning held: the
dependency was flagged, not added around. Once authorized, `ConfigError`/`HandoffError` were
migrated to `#[derive(thiserror::Error)]` in the shipped stub.

**Checked directly, not assumed to carry over: does this module's core finding still hold against
the migrated code?** Yes, and one specific sub-claim needed correcting rather than just re-asserting.
The primary finding (default/pedantic/nursery clippy can't distinguish stringify-early from
structure-preserving `HandoffError` shapes) held unchanged. But the original dry run's secondary
claim - that `clippy::restriction`'s `missing_trait_methods` gives one weak partial signal - **did
not** hold: re-run programmatically against the `thiserror`-derived code, that lint fires zero times
on either attempt, because `thiserror`'s derive macro doesn't trigger it the way a hand-written
`impl Error for HandoffError {}` did. This is a real instance of the same discipline this workshop
applies to every other claim (Module 03's pedantic/or-pattern correction, Module 04's "no lint helps"
re-verification): a claim about tooling behavior is scoped to the exact code shape it was checked
against, and a dependency migration is exactly the kind of change that can silently invalidate it.
`grading.md`'s new addendum, `modules/05-error-handling/README.md`, and
`.claude/skills/custom-error-type-template/SKILL.md` all updated to state the corrected, starker
result (no lint at any level now rescues this finding) rather than the original, now-outdated one.

Not counted as a new dry run or a new Review Trigger data point - the correct/naive attempts'
behavior and every rubric criterion's pass/fail outcome are unchanged; only one sub-claim about
which lint level detects the difference was corrected.
