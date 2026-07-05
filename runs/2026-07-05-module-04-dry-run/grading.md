# Coachgremlin's dry run: Module 04 (Generics, Traits & Lifetimes) on `relay`

First real content-authoring pass for Module 04, per Coachgremlin's Workflow. Same discipline as
Modules 01-03's dry runs: a correct attempt and a deliberately naive, honest (non-adversarial)
attempt, both implementing `alert_checkpoint<N: Notifier>(notifier: &N, session: &Session, trigger:
&CheckpointTrigger) -> CheckpointAlert`. Checked fresh, not assumed to generalize from Modules
01-03's findings - this module's subject (generics/traits/lifetimes) introduces a mechanism
(lifetime annotation) none of the prior three modules touched at all.

## Step 0: ARB trigger check

`fixtures/relay/src/lib.rs` and `SPEC.md` both needed real changes: Module 04's feature (`Notifier`
trait and its three implementations, `Session::label()`, `CheckpointAlert`, `alert_checkpoint`)
didn't exist yet. `scripts/arb-trigger-check.sh --dry-run` confirmed a clean baseline before the
change, then correctly fired against `src/lib.rs` after it. Resolution: the change is purely
additive (one new trait, three new unit-struct implementations, one new inherent method, one new
struct, one new function; no existing struct, field, or function signature touched or removed),
verified by running `cargo build` (compiles, only expected unused-argument warnings on the new
`todo!()` stub) and re-running Modules 01-03's own test suites (`cargo test --no-fail-fast --test
finalize_session --test session_stats --test next_action`) - identical failure output to before the
change (5+6+5 tests, all still panicking on their own still-unsolved `todo!()`s, same as
pre-change), confirming this addition didn't silently change or break any already-shipped,
already-graded module.

## Step 1-2: Frame the exercise + rubric

Task: implement `alert_checkpoint`, which formats a per-trigger message and hands it to any
`Notifier`, reporting back whether delivery succeeded. Six required cases in
`fixtures/relay/tests/alert_checkpoint.rs`: exact message format per `CheckpointTrigger` variant
(three cases), `sent` reflecting the notifier's real return value rather than an assumed `true`,
`session_label` matching the session actually passed in, and the notifier being invoked exactly
once with the built message. Rubric: see `modules/04-generics-traits-lifetimes/README.md`.

`Notifier`, its three implementations (`DesktopNotifier`/`TerminalBellNotifier`/`StdoutNotifier`),
and `Session::label()` are all given (declared in `src/lib.rs` already) - not part of the learner's
own exercise. `Session::label()` in particular is given specifically as a worked contrast case: it
returns a reference with no explicit lifetime annotation, because elision rule 3 (a `&self` method's
output lifetime is inferred from `&self`) already covers it. The graded exercise is
`alert_checkpoint`'s message-building logic and, specifically, whatever shape the learner gives
`CheckpointAlert`'s `session_label` field - the stub ships it as an owned `String` because that's
the shape that compiles today with no lifetime parameter in sight anywhere in the file.

## Step 3: Observe the attempts

**`attempt-good/`** (`diff.patch`): changes `CheckpointAlert` to `CheckpointAlert<'a>` with
`session_label: &'a str`, and `alert_checkpoint` to `fn alert_checkpoint<'a, N: Notifier>(notifier:
&N, session: &'a Session, trigger: &CheckpointTrigger) -> CheckpointAlert<'a>` - `'a` tied only to
`session`, the one reference the output actually borrows from. `cargo test`: 22/22 pass (5 from
Module 01, 6 from Module 02, 5 from Module 03, 6 new). `cargo clippy -- -D warnings`: clean.

**`attempt-naive-clone-avoids-lifetime/`** (`diff.patch`): identical message-building logic, but
`CheckpointAlert` keeps the shipped owned-`String` shape and `alert_checkpoint` builds
`session_label: session.label.clone()` instead of borrowing - no lifetime parameter anywhere in the
diff. Behaviorally indistinguishable from `attempt-good` for every test in the suite: same instinct
as Modules 01-02's naive attempts (avoid the unfamiliar mechanism by falling back to an owned
value), this time aimed at lifetimes instead of moves/borrows. `cargo test`: **22/22 pass, identical
to `attempt-good`.** `cargo clippy -- -D warnings`: **also clean, identical to `attempt-good`.**

**A second, lighter naive shape was also checked** (`extra-check-overannotated-lifetime/`, no
separate `diff.patch` - see its `fixture/src/lib.rs`): keeps the correct borrowed
`CheckpointAlert<'a>` shape, but annotates `alert_checkpoint<'a, N: Notifier>(notifier: &'a N,
session: &'a Session, trigger: &'a CheckpointTrigger) -> CheckpointAlert<'a>` - unifying all three
parameters under one lifetime instead of tying `'a` only to `session`. This targets the module's
*other* learning objective (recognize elision rather than annotating defensively) rather than the
first naive attempt's objective (recognize when a lifetime is genuinely required at all). `cargo
test`: 22/22 pass, identical. `cargo clippy -- -D warnings`: clean, identical. Full transcript:
`extra-check-overannotated-lifetime/fixture/transcript.txt`.

## Step 4: Score against the rubric

| # | Criterion | attempt-good | attempt-naive-clone-avoids-lifetime |
|---|---|---|---|
| 1 | `cargo test` green (gate, deterministic) | Pass. 22/22. | Pass. 22/22. |
| 2 | `cargo clippy -- -D warnings` clean (gate, deterministic) | Pass. | **Pass - deterministic gate cannot distinguish them.** |
| 3 | Diff touches only the implementation file (gate, anti-gaming) | Pass. | Pass. |
| 4 | Message format exact for all three `CheckpointTrigger` variants, `sent` reflects the real notifier return value, notifier invoked exactly once (gate, deterministic - the behavioral cases this exercise's own test suite catches directly) | Pass. | Pass - both attempts get every behavioral case right. |
| 5 | `alert_checkpoint` is generic (`<N: Notifier>`), not `&dyn Notifier` or a concrete type (scored, structural - checked by reading the signature, not by `cargo test`, since both forms would pass the suite identically) | Pass. | Pass (naive attempt didn't touch this signature element). |
| 6 | `CheckpointAlert.session_label` borrows `session` (an explicit, minimally-scoped `&'a str`), not an owned/cloned `String` (scored, conceptual) | Pass. | **Fails.** `session.label.clone()` sidesteps the lifetime question entirely rather than answering it - correct output today, no compiler-enforced connection between the alert and the session it came from. |
| 7 | Where a lifetime is used, it's scoped to only the reference actually borrowed into the output - not unified across every reference parameter defensively (scored, conceptual) | Pass. | N/A (attempt has no lifetime at all - criterion 6 is the one it fails, not this one). See the separate `extra-check-overannotated-lifetime` check for this criterion's own failure mode, isolated. |

**Result: a genuinely different failure mode from Modules 01-03, not a fourth instance of the same
symptom re-appearing under a new name.** Modules 01-02 were about defensive over-cloning where a
borrow would do; Module 03 was about exhaustiveness erosion in a match. Module 04's primary naive
attempt is clone-shaped again on the surface (`.clone()` appears in the diff), but the underlying
mechanism is new: it isn't cloning to avoid a borrow-checker complaint about ownership, it's cloning
to avoid ever having to reason about lifetime elision and explicit annotation at all - a mechanism
that didn't exist as an option before this module introduced explicit lifetimes to the codebase.
The overannotation check further shows a second, structurally different way to get this module's
concept wrong (defensive over-scoping instead of avoidance) that has no cloning in it whatsoever.

**Checked, not assumed, whether a stricter lint or a broader search closes the gap - and this time
nothing does, at any level tried.** `clippy::pedantic` gives **zero** discriminating signal:
diffing the full pedantic output between `attempt-good` and `attempt-naive-clone-avoids-lifetime`
shows the two are byte-for-byte identical (aside from the path/timestamp lines) - eight warnings,
same lines, same suggestions, on both. `clippy::nursery`'s only difference between the two is a
`too_long_first_doc_paragraph` nit on a doc comment specific to `attempt-good`'s longer prose - an
artifact of how that comment happens to be worded, not a signal about the lifetime-vs-clone
distinction at all. The full `clippy::restriction` group was also run (not just spot-checked with a
few named lints) and diffed by warning type between both attempts: 67 warnings on the naive attempt,
64 on the good one, and the *only* type-level difference is `single_char_lifetime_names` - which
fires on `attempt-good` for naming its lifetime `'a`, not on the naive attempt for lacking one. That
is not a rescue; it is `restriction`'s own opinion pointed at the correct attempt for following
ordinary Rust convention, while staying silent on the actual clone-vs-borrow question. `restriction`
also flags `exhaustive_structs` on `CheckpointAlert` in both attempts identically (unrelated to this
finding). A further manual search (`-W clippy::redundant_clone`, `-W clippy::clone_on_copy`, the
full `style`/`complexity`/`perf`/`suspicious` groups) found nothing else that flags
`session.label.clone()` or the `CheckpointAlert` struct shape specifically either. **This is a
sharper result than any prior module's**: Module 01's `pedantic` at least gave a noisy partial
signal, Module 03's off-by-default `restriction` group had a lint that caught its smell once enabled
by name; Module 04's naive attempt has no lint-shaped rescue anywhere across every group checked,
including the full `restriction` group by name, not just a few lints selected from it.

**The clone-count-check pre-filter, run per Workflow step 3, is where this module's story actually
diverges from Module 03's.** Module 03's naive mistake (a wildcard match arm) wasn't clone-shaped at
all, so the pre-filter correctly stayed silent - a true negative, outside its own stated scope.
Module 04's naive mistake *is* clone-shaped, and once the script's expected-max baseline is
calibrated against the reference implementation's own diff (2 - both inherited from Module 02's
`session_stats`, unrelated to this module's own addition) rather than a higher number assumed from
rubric prose, `scripts/clone-count-check.sh` correctly flags `attempt-naive-clone-avoids-lifetime`'s
diff (3 clones, exceeds the baseline of 2) and correctly passes `attempt-good`'s (2, at the
baseline). **A genuine true positive - precisely: the first one produced during a module's own
graded dry-run scoring (this Workflow step), as distinct from the pre-adoption calibration trial
that already established the mechanism works** (`docs/next-actions.md`, 2026-07-05: the script was
"trialed against Modules 01+02's real dry-run diffs; discriminates good/naive cleanly" before being
adopted starting Module 03). That trial was real evidence the mechanism functions, but it wasn't run
as part of either module's own graded dry-run scoring the way this check now is - Module 03 was the
first module where the pre-filter was actually exercised inside Workflow step 3 itself, and it
correctly reported a true negative there (the naive mistake wasn't clone-shaped). Module 04 is the
first case inside that same in-dry-run usage where it reports a true positive. Worth being precise
about what this does and doesn't prove: the pre-filter catches this particular
naive attempt because it happens to route through `.clone()`; it would not catch a differently-shaped
avoidance of the same underlying mistake (e.g. reaching for `'static`, or a global/`Rc`-based
workaround) - a textual heuristic, not a semantic one, exactly as `scripts/clone-count-check.sh`'s
own Limitations note already says. It is real evidence *for* running the pre-filter before the
conceptual read where relevant, not evidence that the conceptual tier is now optional.

## Step 5: Confirm or loop

- **attempt-good:** rubric met.
- **attempt-naive-clone-avoids-lifetime:** rubric not met on the conceptual tier (criterion 6).
- **extra-check-overannotated-lifetime:** not scored against the full rubric (message-building logic
  and struct shape are both already correct here) - checked specifically against criterion 7, which
  it fails. Recorded as a real, checked failure mode for that criterion, not treated as a full
  independent attempt requiring its own confirm/loop cycle.

## Step 6: Takeaway

A trait-bound/lifetime-annotation cheat-sheet,
`.claude/skills/trait-lifetime-cheatsheet/SKILL.md`, built directly from this finding (including the
pedantic-gives-zero-signal result and the clone-count-check true positive) and validated against a
second, unrelated problem (a generic config-store lookup) before being called done
(`takeaway-validation/` in this same run directory).

## Step 7: Batch-review cadence

Modules 01+02 had the first content-level panel batch
(`docs/review-panel/2026-07-05-modules-01-02-content.md`). Module 03 was the first module of content
since that batch (1 of the 2-3-module window). Module 04 completes this window (2 modules: 03+04) -
due now, per `docs/next-actions.md`'s own note that the cadence was "due once Module 04 or 05
completes the next 2-3-module window." See `retro.md` and
`docs/review-panel/2026-07-05-modules-03-04-content.md` for the panel run this triggers.

## Human Gate

Recommendation only. `human_confirmed: false` in the run record, per Coachgremlin's Human Gate.
