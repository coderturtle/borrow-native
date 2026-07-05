# Coachgremlin's dry run: Module 05 (Error Handling as Idiomatic Control Flow) on `relay`

First real content-authoring pass for Module 05, per Coachgremlin's Workflow. Same discipline as
Modules 01-04's dry runs: a correct attempt and a deliberately naive, honest (non-adversarial)
attempt, both implementing `parse_config(input: &str) -> Result<RelayConfig, ConfigError>` and
`write_handoff_summary(path: &Path, session: &Session) -> Result<(), HandoffError>`. Checked fresh,
not assumed to generalize from Modules 01-04's findings - this module's subject (custom error types
and `?`-based propagation) introduces a mechanism none of the prior four modules touched at all.

## Step 0: ARB trigger check

`fixtures/relay/src/lib.rs` and `SPEC.md` both needed real changes: Module 05's given code
(`RelayConfig`, `NotifierKind`, `ConfigError` with its `Display`/`Error` impls, `HandoffError`'s
shipped `Io(String)` shape with its own `Display`/`Error` impls, and the two `todo!()` stubs) didn't
exist yet. `scripts/arb-trigger-check.sh --dry-run` confirmed a clean baseline before the change,
then correctly fired against `src/lib.rs` after it. Resolution: the change is purely additive (two
new structs, one new enum, two new error enums with their trait impls, two new functions; no
existing struct, field, function signature, or trait touched or removed) - verified by running
`cargo build` (compiles once `#[derive(Debug)]` was added to `ConfigError`/`HandoffError`/
`RelayConfig`/`NotifierKind`, a real requirement surfaced immediately: `std::error::Error` requires
`Debug + Display`, and `Result::unwrap_err`'s bound requires the `Ok` type to be `Debug` too) and
re-running Modules 01-04's own test suites (`cargo test --no-fail-fast --test finalize_session
--test session_stats --test next_action --test alert_checkpoint`) - identical failure output to
before the change (5+6+5+6 tests, all still panicking on their own still-unsolved `todo!()`s, same as
pre-change), confirming this addition didn't silently change or break any already-shipped,
already-graded module.

## Step 1-2: Frame the exercise + rubric

Two functions, one shared theme: propagate failure via `Result`/custom error types instead of
panicking, in a form a caller could act on. `parse_config` parses a `key=value` config file into
`RelayConfig`, failing with one of three named `ConfigError` variants depending on what's wrong.
`write_handoff_summary` renders a session's checkpoints to disk, failing with `HandoffError` on a
real I/O error. Ten required cases across `fixtures/relay/tests/parse_config.rs` (8) and
`tests/write_handoff_summary.rs` (2). Rubric: see `modules/05-error-handling/README.md`.

`RelayConfig`, `NotifierKind`, and `ConfigError`'s three variants (with `Display`/`Error` already
implemented) are all given - not part of the learner's own exercise, same treatment as
`CheckpointTrigger`/`Notifier` in Modules 03-04. `HandoffError` is given differently, on purpose:
its variant name (`Io`) and its `Display`/`Error` impls for whatever shape `Io` holds are given, but
the stub ships `Io(String)` - the shape that compiles with the least ceremony (stringify the
underlying `std::io::Error` the moment it's caught). Whether that's the shape kept is exactly this
module's real exercise, mirroring Module 04's `CheckpointAlert.session_label: String` stub.

## Step 3: Observe the attempts

**`attempt-good/`** (`diff.patch`, isolated to Module 05's own addition against a baseline with
Modules 01-04 already solved - see the baseline note below): `parse_config` uses two small private
helpers (`parse_interval`, `parse_notifier_kind`) and a straight-line `?` chain, no manual `match`
anywhere. `write_handoff_summary` changes `HandoffError::Io` to wrap the real `std::io::Error`
rather than the shipped `String`, adds `impl From<std::io::Error> for HandoffError` (making a bare
`std::fs::write(path, content)?` compile), and overrides `Error::source()` to return the wrapped
error. `cargo test`: 32/32 pass (5 Module 01, 6 Module 02, 5 Module 03, 6 Module 04, 10 new).
`cargo clippy -- -D warnings`: clean.

**`attempt-naive-stringified-io-error/`** (`diff.patch`): identical `parse_config` (this attempt
isolates the finding to `write_handoff_summary` alone, same discipline as Module 04 isolating its
two naive shapes to one contrast each). `write_handoff_summary` keeps the shipped `Io(String)`
shape and converts the I/O error to a message the moment it's caught (`Err(e) =>
Err(HandoffError::Io(e.to_string()))`) instead of reaching for `From`/`?`. Behaviorally
indistinguishable from `attempt-good` for every test in the suite - same instinct as Modules 01/02's
naive attempts (avoid an unfamiliar mechanism by falling back to a plainer, already-compiling
shape), this time aimed at error-type design rather than moves/borrows. `cargo test`: **32/32 pass,
identical to `attempt-good`.** `cargo clippy -- -D warnings`: **also clean, identical.**

**A second, lighter check was also run**
(`extra-check-manual-match-no-question-mark/`, no separate `diff.patch` - see its
`fixture/transcript.txt`), targeting the module's *other* learning objective (use `?` rather than
manual match boilerplate) in isolation from the `HandoffError` finding above: started from
`attempt-good`, rewrote all four of `parse_config`'s `?` uses as explicit `match` blocks with
identical behavior. `cargo test`: 32/32 pass, identical. **`cargo clippy -- -D warnings`: 2 errors**
- `clippy::question_mark` (default "style" group) fired on both `Result -> Result` passthrough
matches (`Ok(v) => v, Err(e) => return Err(e)`, no error transformation), but not on either
`Option -> Result` match (`Some(v) => v, None => return Err(..)`) - confirmed by isolating the two
Option matches alone (Result matches reverted to `?`): zero warnings. Full transcript:
`extra-check-manual-match-no-question-mark/fixture/transcript.txt`.

**Baseline used for diffing:** `attempt-good`'s and `attempt-naive-stringified-io-error`'s
`diff.patch` files are both generated against a reference copy of `src/lib.rs` with Modules 01-04
already solved (their own reference implementations from `runs/2026-07-05-module-0{1,2,3,4}-dry-run/`)
and only Module 05's two functions left as the shipped `todo!()` stub - matching a real learner
arriving at Module 05 with the prior four modules already in their own solved repo state, and
matching Module 04's precedent of a diff scoped to that module's own addition only.

## Step 4: Score against the rubric

| # | Criterion | attempt-good | attempt-naive-stringified-io-error |
|---|---|---|---|
| 1 | `cargo test` green (gate, deterministic) | Pass. 32/32. | Pass. 32/32. |
| 2 | `cargo clippy -- -D warnings` clean (gate, deterministic) | Pass. | **Pass - deterministic gate cannot distinguish them.** |
| 3 | Diff touches only `fixtures/relay/src/lib.rs`, not the test file (gate, anti-gaming) | Pass. | Pass. |
| 4 | Every `ConfigError`/`HandoffError` case is correct: exact variant, exact key/value named, real I/O failure returns `Err` not a panic (gate, deterministic - the behavioral cases this exercise's own test suite catches directly) | Pass. | Pass - both attempts get every behavioral case right. |
| 5 | `parse_config`/`write_handoff_summary` propagate failure via `?`, not `.unwrap()`/`.expect()`/`panic!()` on a recoverable error (gate, deterministic-adjacent - a panicking implementation fails test case 8 directly) | Pass. | Pass. |
| 6 | `HandoffError::Io` preserves the real underlying error (wraps `std::io::Error` via `From`, exposes it through `source()`) rather than discarding its structure into a `String` the moment it's caught (scored, conceptual) | Pass. | **Fails.** `e.to_string()` throws away `std::io::Error`'s `ErrorKind` and chainable `source()` the instant it's caught - correct output today, no way for a caller (or this type's own `source()`) to recover the original error afterward. |
| 7 | Fallible steps are propagated with `?`, not hand-rolled `match`/`return Err` boilerplate that adds no behavior (scored, conceptual - checked in isolation, see the extra-check above) | Pass. | Pass (naive attempt's `parse_config` is unchanged from `attempt-good`, so it doesn't independently fail this criterion - see the extra-check for this criterion's own failure mode, isolated). |

**Result: a genuinely new failure mode, not a fifth instance of an old one under a new name.**
Modules 01-02 were defensive over-cloning; Module 03 was exhaustiveness erosion; Module 04 was
lifetime avoidance/over-annotation. Module 05's primary naive attempt touches none of those
mechanisms - no clone, no match arm removed, no lifetime anywhere in sight. Its failure is specific
to error-type design: discarding a real error's structure into a string the moment it's caught,
which reads as "handled" (there's still an `Err`, the message is still correct) while quietly
deciding the module's actual question (does this error type preserve enough for a caller to inspect
it structurally?) by avoidance rather than by choice.

**Checked, not assumed, whether a stricter lint or a broader search closes the gap - and this
time the picture is genuinely mixed, not uniformly "nothing helps" like Module 04.** For the
`HandoffError` finding specifically: `clippy::pedantic` and `clippy::nursery` give **zero**
discriminating signal - diffing full output between `attempt-good` and
`attempt-naive-stringified-io-error` shows identical warning-type sets for both (12 pedantic
warnings each, same lints; nursery identical). The full `clippy::restriction` group (not spot-checked
- run in full and diffed via `--message-format=json`, comparing lint code + file/line across both
attempts) does contain one lint that indirectly catches part of this: `clippy::missing_trait_methods`
fires one additional time on the naive attempt's `impl Error for HandoffError` block, because that
generic "did you skip any default trait method" lint counts `source()` (which has a default no-op
impl on `std::error::Error`) as unoverridden there, while `attempt-good`'s block overrides it and so
is "missing" one fewer method. This is real, but weak as a signal: the lint is not aimed at
source-chain preservation at all, fires the same way on every trait impl in the file regardless of
which optional methods are skipped, and a learner would have to isolate the count at one specific
`impl` block, not just notice a file-wide total differs, to actually use it. `restriction` also
contains `clippy::question_mark_used` - which, as in Module 04's `single_char_lifetime_names`
finding, points the *wrong* direction: it flags `attempt-good` for using `?` at line 406
(`std::fs::write(path, content)?`), not the naive attempt for avoiding it, because this particular
restriction lint exists to flag *any* use of `?` for teams that want none. A handful of other
restriction-lint count differences (`absolute_paths`, `pattern_type_mismatch`,
`std_instead_of_core`, `missing_inline_in_public_items`, `min_ident_chars`) are incidental artifacts
of the two solutions' different code shape (more/fewer fully-qualified `std::` paths, one extra
`pub fn from` in the good attempt) rather than anything semantically aimed at this finding - listed
for completeness in `lint-evidence/restriction-attempt-good.txt` and
`lint-evidence/restriction-attempt-naive-stringified-io-error.txt`, not treated as rescues.

**For the second, `?`-avoidance shape (extra-check, LO3), the result is the opposite of Module 04's
across-the-board "nothing helps": default `cargo clippy -- -D warnings` - no opt-in group required -
genuinely fails a hand-rolled `Result -> Result` passthrough match with no error transformation
(`clippy::question_mark`, part of the default "style" group), while staying silent on the
mechanically similar `Option -> Result` case. This is the first time in this workshop's four prior
dry runs that the *default* lint set (not `pedantic`, not a manually-searched `restriction` lint)
outright rescues part of a naive shape, rather than giving a noisy partial signal (Module 01) or
requiring an off-by-default lint enabled by name (Module 03). It's also the reason the primary
attempt-good/attempt-naive-stringified-io-error comparison above holds both attempts' `parse_config`
identical: had the naive attempt also skipped `?` in `parse_config`, `cargo clippy -- -D warnings`
alone would already have separated the two attempts on the deterministic tier, muddying whether the
`HandoffError` finding was real or just co-occurring with a gate-catchable mistake. Isolating one
naive shape per contrast, per Module 04's own discipline, is what kept this clean.

**The clone-count-check pre-filter, run per Workflow step 3, reports a true negative here - the
first since Module 03's.** `scripts/clone-count-check.sh` against both `diff.patch` files
(path-filtered to `lib.rs`, baseline 0 clones - the reference implementation adds none) reports 0
clones in both, correctly staying silent: unlike Modules 01, 02, and 04, this module's real naive
failure mode doesn't route through `.clone()` at all, so a clone-count check has nothing to flag,
exactly matching its own stated scope (`scripts/clone-count-check.sh`'s Limitations note) rather
than a gap in the tool.

## Step 5: Confirm or loop

- **attempt-good:** rubric met.
- **attempt-naive-stringified-io-error:** rubric not met on the conceptual tier (criterion 6).
- **extra-check-manual-match-no-question-mark:** not scored against the full rubric (error-type
  shape and behavior are both already correct here) - checked specifically against criterion 7,
  which it does *not* fail outright (default clippy already catches the `Result`-to-`Result` half of
  it) but reveals is only partially deterministic-gate-visible. Recorded as a real, checked, mixed
  result for that criterion, not treated as a full independent attempt requiring its own
  confirm/loop cycle - same treatment Module 04 gave its own lighter over-annotation check.

## Step 6: Takeaway

A reusable custom-error-type template, packaged as a Claude Code Skill:
[`.claude/skills/custom-error-type-template/SKILL.md`](../../.claude/skills/custom-error-type-template/SKILL.md),
built directly from this finding (including the mixed default-clippy-partially-catches-LO3 result
and the missing_trait_methods/question_mark_used restriction-lint findings for LO2) and validated
against a second, unrelated problem before being called done
(`takeaway-validation/` in this same run directory).

## Step 7: Batch-review cadence

Modules 03+04 had the second content-level panel batch
(`docs/review-panel/2026-07-05-modules-03-04-content.md`). Module 05 is the first module of content
since that batch (1 of the 2-3-module window) - not due yet. Next due once Module 06 or 07 completes
the window, per `docs/next-actions.md`.

## Human Gate

Recommendation only. `human_confirmed: false` in the run record, per Coachgremlin's Human Gate.

## Addendum (same day, post-review): `thiserror` migration, re-verified

coderturtle reviewed this dry run (go) and, in the same session, authorized adding `thiserror` as a
real Cargo dependency - previously avoided (see the original "Step 1-2" section above) solely
because this project's `.hekton/governance.yaml` gates dependency changes as human-required, not
because of a design objection to `thiserror` itself. `ConfigError`/`HandoffError` were migrated from
a manual `impl Error`/`Display` to `#[derive(thiserror::Error)]` in the shipped stub (`Io(String)`
retains its `#[error(...)]` message; the shipped shape is otherwise unchanged).

**This module's finding was re-verified fresh against the migrated code, not assumed to carry over
unchanged** - a real, checked distinction, per this workshop's own recurring discipline:

- Rebuilt both attempts against the `thiserror`-derived given code: a correct attempt using
  `Io(#[from] std::io::Error)` (one attribute deriving both `From<std::io::Error>` and `source()`)
  and the naive attempt keeping `Io(String)` built via `.to_string()`. Both pass `cargo test`
  (32/32) and `cargo clippy -- -D warnings` identically - the primary finding holds.
- **Re-ran the full `clippy::restriction` group programmatically (`--message-format=json`, counts
  diffed by lint code, not eyeballed) and found the one weak partial signal the original manual-`impl`
  dry run recorded - `clippy::missing_trait_methods`, flagging a `source()`-less `impl Error` -
  no longer fires at all (0 occurrences on both attempts).** `thiserror`'s derive macro doesn't
  trigger this lint the way a hand-written `impl Error for HandoffError {}` did. This makes the
  finding *starker* after the migration: no lint at any level checked (default, `pedantic`,
  `nursery`, or the full `restriction` group) distinguishes the two `HandoffError` shapes anymore,
  matching Module 04's "nothing rescues this" pattern rather than this module's own original
  "one weak generic signal" pattern.
- `clippy::question_mark_used` (flagging the *correct* attempt for using `?`, restriction group)
  still fires identically to before - unaffected by the migration, since it targets `?` usage in
  `write_handoff_summary`'s body, not the error type's own impl shape.
- The `?`-avoidance extra-check (`extra-check-manual-match-no-question-mark/`) and the clone-count
  pre-filter's true-negative result are both about `parse_config`, which the migration didn't touch
  (`ConfigError` kept its three variants and `#[error(...)]` messages, migrated the same way as
  `HandoffError`) - not re-run, since nothing about them depends on the manual-vs-derived `Error`
  impl choice.

Not re-run as a full new attempt/confirm-loop cycle (the correct/naive attempts' behavior and every
rubric criterion's pass/fail outcome are unchanged) - this addendum documents a re-verification of
one specific claim (the `restriction`-group partial signal) that the migration falsified, not a new
dry run. `docs/decisions.md` and `modules/05-error-handling/README.md` updated to reflect this;
`.claude/skills/custom-error-type-template/SKILL.md` updated with the corrected, starker finding.
