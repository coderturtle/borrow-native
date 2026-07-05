# Retro: Coachgremlin's Module 04 dry run (Generics, Traits & Lifetimes)

Go/no-go on Module 04's real content, and a direct test of whether Modules 01-03's two-tier finding
(the deterministic gate can't distinguish a correct attempt from a naive one) holds for a concept
none of them touched: explicit lifetime annotation. Not assumed to carry over just because the
surface shape (a naive attempt reaching for `.clone()`) looks familiar.

## Checklist, checked against evidence

**Coachgremlin can frame Module 04's exercise from the module README's stated gate and takeaway,
without inventing an unrelated scenario.**
Yes. `alert_checkpoint` (`fixtures/relay/SPEC.md`'s Module 04 section) is a direct instance of the
module's own stated question ("how do I write one function that works across types, safely, without
the compiler losing track of how long references live") - `Notifier` is the trait-bound half
(generic over three real implementations plus a test double), `CheckpointAlert`'s `session_label`
field is the lifetime half, not a substitute scenario bolted on.

**Given a deliberately good attempt and a deliberately naive (not gaming) attempt, the deterministic
tier alone does not discriminate, and Coachgremlin's conceptual tier does - checked fresh, not
assumed from Modules 01-03.**
Yes, and the underlying mechanism is genuinely new, not a fourth coat of paint on the same finding.
`attempt-good` and `attempt-naive-clone-avoids-lifetime` both pass `cargo test` (22/22) and default
`cargo clippy -- -D warnings` (zero output) identically - see `grading.md`. The naive attempt's
actual mistake (`session_label: session.label.clone()`, no lifetime parameter anywhere) is lifetime
avoidance, not the ownership-move or borrow-cloning failure modes Modules 01-02 found, and not the
exhaustiveness erosion Module 03 found - this module introduces explicit lifetime annotation as a
mechanism for the first time, so the failure mode being new is the expected result of checking fresh
rather than assuming, not a coincidence.

**Checked whether a stricter deterministic tier, or a broader lint search, would close the gap,
rather than assuming any prior module's escalation result carries over unchanged.**
Yes, and this module's result is the starkest yet. `clippy::pedantic`'s output is byte-for-byte
identical between the two attempts (diffed directly, not eyeballed) - not a noisy partial signal
(Module 01), not zero signal that at least stays neutral (Module 02), not a suggestion that costs
only readability without touching safety (Module 03, corrected 2026-07-05 from this workshop's
original overstated framing - see that module's own retro), just nothing at all. `clippy::nursery`'s
only difference is a doc-comment-length nit unrelated to the lifetime-vs-clone question. The full
`clippy::restriction` group was run and diffed by warning type, not spot-checked with a few named
lints: the only type-level difference between the two attempts is `single_char_lifetime_names`,
which fires on the *correct* attempt for naming its lifetime `'a` (ordinary convention) and is silent
on the naive one - `restriction`'s one relevant difference points at the wrong attempt, not a rescue
at all. A further manual search across `redundant_clone` and `clone_on_copy` found no lint anywhere
that flags the clone or the struct shape either. Decision: this module's stated deterministic gate
stays at default `cargo clippy`, same as every prior module - here, unlike Module 03, there is no
`restriction`-group lint that would rescue this if enabled, checked directly rather than assumed from
Module 03's different case; the conceptual tier is doing 100% of the discriminating work with no
compiler-tooling assist available at any strictness level checked.

**New this run: is the naive attempt's mistake actually catchable by the clone-count pre-filter, and
does checking that change anything about the workshop's story of how that pre-filter behaves?**
Checked directly, not assumed either way. Yes - `scripts/clone-count-check.sh`, calibrated against
`attempt-good`'s own diff (baseline 2, both inherited from Module 02's `session_stats`), correctly
flags `attempt-naive-clone-avoids-lifetime`'s diff (3 clones) and correctly passes `attempt-good`'s.
**Corrected 2026-07-05** (Modules 03+04 Workshop Review Panel batch, Skeptical Critic persona): this
retro originally called it "the pre-filter's first genuine true positive in this workshop," which
overstated things against `docs/next-actions.md`'s own record that Modules 01-02's pre-adoption
trial already "discriminates good/naive cleanly" on their diffs - that trial was already a true
positive on the mechanism, just not one produced during a module's own graded dry-run scoring the
way Workflow step 3 now runs it. Precisely: Module 03 was the first case the pre-filter was actually
exercised *inside* a module's own dry-run (a true negative, correctly silent, since that mistake
wasn't clone-shaped); Module 04 is the first case inside that same in-dry-run usage where it reports
a true positive. Recorded precisely, not overclaimed: this is a true positive for *this specific
naive attempt*, not evidence the pre-filter would catch
every way a learner might avoid an explicit lifetime (`'static`, a global, an `Rc` - none of which
route through `.clone()`) - the script's own Limitations note already says as much, and this run
doesn't contradict it.

**Checked, not assumed: does the module's second learning objective (recognize elision rather than
annotating defensively) have its own real, checked failure mode, distinct from the first naive
attempt's?**
Yes. `extra-check-overannotated-lifetime` unifies all three of `alert_checkpoint`'s reference
parameters under one `'a` instead of scoping it to `session` alone - compiles, passes all 22 tests,
and clippy stays clean at every level checked, identically to the correctly-scoped version. This is
a structurally different mistake from the primary naive attempt (over-application of a mechanism
the learner does use, versus complete avoidance of it) and was checked directly rather than assumed
to be "basically the same finding" because both involve lifetimes.

**Coachgremlin's feedback references the actual diff, gives one concrete next try, and never hands
the fix over.**
Yes. `grading.md`'s scoring table points at the specific line
(`session_label: session.label.clone()`) and names what it costs (no compiler-enforced connection
between the alert and the session it came from, despite identical behavior today), without handing
over the fixed code.

**The terminal state (test + clippy output) is observed running, not asserted.**
Yes. `transcript.txt` in each `attempt-*/fixture/` directory (and
`extra-check-overannotated-lifetime/fixture/`) is real, captured `cargo test`/`cargo clippy` output
across default, `pedantic`, and `nursery` lint levels, not narrated - and the pedantic/nursery
diffs between attempts were generated with `diff`, not summarized from memory.

**Step 6 produces a takeaway that actually helps on a second, different problem, not just a file
that was written.**
Yes. See `.claude/skills/trait-lifetime-cheatsheet/SKILL.md` and `takeaway-validation/` in this run
directory - the packaged cheat-sheet was applied to an unrelated generic config-store lookup before
being called done, and correctly predicted that both the clone-avoidance naive shape and the
correctly-annotated shape would compile and lint identically there too.

**The Human Gate holds: completion is a recommendation with `human_confirmed: false` in a `runs/`
entry, not a self-certified "complete."**
Yes - see the new `run-*.yaml` entry for this run.

**Step 0 (ARB check) and the batch-review cadence (step 7) were both actually checked, not just
referenced.**
Yes. Step 0: clean baseline confirmed before the change, correct fire after adding the new trait/
struct/function to `src/lib.rs`, resolved by confirming the change was purely additive (build
succeeds, Modules 01-03's own tests fail identically to before - nothing re-broken). Step 7: checked
and found due - Module 04 completes the 2-3-module window since the Modules 01+02 batch (03+04 = 2
modules), matching `docs/next-actions.md`'s own note that the cadence was due "once Module 04 or 05"
completed it. See `docs/review-panel/2026-07-05-modules-03-04-content.md` for the panel run this
triggers, run as part of this same session rather than deferred again.

## Go/no-go

**Go.** Module 04's exercise, rubric, and finding are real, evidenced, and mechanistically distinct
from Modules 01-03's - lifetime avoidance and lifetime over-application are both new failure modes
this workshop hadn't exercised before, and the deterministic tier's blindness to them is, if
anything, more complete here (zero lint signal at any level, versus a noisy or misdirected signal in
three of the four prior findings) than any prior module found. Proceeding with the same per-module
dry-run discipline for Modules 05-08.

## Revision fed back

1. **`~/hekton/gremlins/coaching/coachgremlin.md`**: this is Coachgremlin's fifth real dry run and
   fourth workshop-internal one for `borrow-native` - Version/Status line updated to cite this run's
   finding (a fourth, mechanically distinct instance of "the deterministic gate misses a real
   idiom/design smell," this time with zero lint signal at any level rather than a noisy, absent, or
   misdirected one, plus the clone-count pre-filter's first genuine true positive in this workshop).
2. **`modules/04-generics-traits-lifetimes/README.md`**: rewritten from skeleton to real authored
   content reflecting this dry run's actual evidence.
3. **Takeaway packaged and validated**: `.claude/skills/trait-lifetime-cheatsheet/SKILL.md` plus
   `takeaway-validation/` in this run's own directory.

## Toward Coachgremlin's Review Trigger

This run's finding is a new, mechanically distinct instance of "the two-tier model catches something
a single deterministic gate (even an escalated one) would miss" - the fourth such instance for this
workshop (ownership/Module 01, borrowing/Module 02, enums-and-exhaustiveness/Module 03, now
lifetimes-and-generics/Module 04), and the first where escalating the lint tier gives literally zero
signal rather than a noisy, absent-but-neutral, or actively-misdirecting one. Per the same standard
already applied to Modules 02-03 (`docs/decisions.md`, 2026-07-05): this still counts as **depth**
evidence for the two-tier model generalizing across concepts within `borrow-native`, not a new
**breadth** data point toward Coachgremlin's "3+ runs across 2+ workshops" bar, which stays at 2 -
flagged here, not self-certified, per coderturtle's own prior ruling on this exact distinction.

## What's still open

- Modules 05-08 still need the same treatment.
- Same single-grader limitation as Modules 01-03's retros named: this dry run used one session
  (this one) constructing and grading all three variants (good, naive-clone, naive-overannotated).
  An independent blind pass would be a stronger test.
- The clone-count pre-filter's true positive here is specific to a clone-shaped avoidance. Worth
  watching in Modules 05-08 for a naive attempt that avoids the module's target concept through a
  route the pre-filter (or any single cheap deterministic check) can't see at all - Module 04's own
  overannotation check is already one such case, just not one the pre-filter was ever designed to
  catch (no cloning involved).
- The batch-review cadence fires with this module (see Step 7 above); its findings and triage are
  recorded separately in `docs/review-panel/2026-07-05-modules-03-04-content.md`, not repeated here.
