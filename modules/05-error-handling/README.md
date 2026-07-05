# Module 05: Error Handling as Idiomatic Control Flow

## The question this module answers

How do I propagate failure without panicking, in a way callers can actually act on?

## Exercise: `parse_config` and `write_handoff_summary`

Runs against `fixtures/relay/`, the one shared project every module in this workshop builds a real
feature onto (see that directory's `SPEC.md` for the full product spec and the module-by-module
build-out table). Module 05 adds two features that both fail in real, recoverable ways: parsing a
config file, and writing a handoff summary to disk.

> `fixtures/relay/src/lib.rs` already declares the following for you - read it before writing any
> code:
>
> ```rust
> pub struct RelayConfig {
>     pub checkpoint_interval_secs: u64,
>     pub notifier_kind: NotifierKind,
> }
>
> pub enum NotifierKind {
>     Desktop,
>     TerminalBell,
>     Stdout,
> }
>
> #[derive(Debug, thiserror::Error)]
> pub enum ConfigError {
>     #[error("missing required config key: {0}")]
>     MissingKey(String),
>     #[error("invalid value for {key}: {value:?} is not a valid number of seconds")]
>     InvalidInterval { key: String, value: String },
>     #[error("unknown notifier: {0:?} (expected \"desktop\", \"bell\", or \"stdout\")")]
>     UnknownNotifier(String),
> }
>
> pub fn parse_config(input: &str) -> Result<RelayConfig, ConfigError> {
>     todo!()
> }
>
> #[derive(Debug, thiserror::Error)]
> pub enum HandoffError {
>     #[error("failed to write handoff summary: {0}")]
>     Io(String), // shipped shape - whether you keep it is the exercise
> }
>
> pub fn write_handoff_summary(path: &std::path::Path, session: &Session) -> Result<(), HandoffError> {
>     todo!()
> }
> ```
>
> `relay`'s `Cargo.toml` depends on [`thiserror`](https://docs.rs/thiserror) - added specifically for
> this module, with coderturtle's explicit approval (this project's `.hekton/governance.yaml` gates
> dependency changes as human-required, so it wasn't added unilaterally). `RelayConfig`,
> `NotifierKind`, and `ConfigError`'s three variants (with their `#[error(...)]` messages already
> written) are given because their shape isn't this module's teaching subject - a caller matching on
> *which* config error it got is the point, not the exact variant names. `HandoffError` is given
> differently, on purpose: its variant name (`Io`) and its own `#[error(...)]` message for whatever
> shape `Io` holds are given, but the stub ships `Io(String)` - the shape that compiles today with the
> least ceremony (stringify the underlying `std::io::Error` the moment it's caught). Whether that's
> the shape you keep is exactly this module's real exercise - same treatment Module 04 gave
> `CheckpointAlert.session_label: String`. Reaching for `#[from]` on `Io`'s field (turning it into
> `Io(#[from] std::io::Error)`) is the natural `thiserror`-idiomatic way to take this exercise's
> "keep" path - it auto-derives both `From<std::io::Error>` (so `?` compiles unchanged) and
> `source()` in one attribute, less ceremony than doing either by hand.
>
> Your job is the two functions' bodies. `parse_config` reads newline-separated `key=value` lines
> (blank lines skipped, unrecognized keys ignored - forward compatible). Two required keys:
> `checkpoint_interval_secs` (must parse as `u64`) and `notifier` (must be exactly `"desktop"`,
> `"bell"`, or `"stdout"`). Missing either key comes back as `ConfigError::MissingKey`; present but
> malformed comes back as `ConfigError::InvalidInterval`/`UnknownNotifier` naming the exact key and
> value. `write_handoff_summary` renders `session`'s checkpoints (one line per checkpoint: `- {goal}:
> {summary} (risks: {joined, or "none" if empty})`) and writes it to `path` via `std::fs::write`. A
> real I/O failure must come back as `Err`, never a panic. Full format spec:
> `fixtures/relay/SPEC.md`'s Module 05 section.
>
> Eight edge cases are checked by the provided test suites (`fixtures/relay/tests/parse_config.rs`,
> `tests/write_handoff_summary.rs`): every `ConfigError` variant named exactly with the right
> key/value, an unrecognized extra key ignored rather than erroring, blank lines tolerated, the
> handoff file's exact content (including an empty-risks checkpoint rendering `none`), and a real I/O
> failure (a missing parent directory) coming back as `Err`, not a panic. Run everything from
> `fixtures/relay/` (`cd fixtures/relay && cargo test`). Get `cargo test` green, from your own
> harness, without narrating the fix as you go, then check it against the rubric below.

## Rubric

1. **`cargo test` green (gate, deterministic).** All 32 tests pass (5 Module 01, 6 Module 02, 5
   Module 03, 6 Module 04, 10 new).
2. **`cargo clippy -- -D warnings` clean (gate, deterministic).** Zero output.
3. **Diff touches only `fixtures/relay/src/lib.rs`, not the test files (gate, anti-gaming).**
4. **Every `ConfigError`/`HandoffError` case is correct (gate, deterministic).** The behavioral cases
   this exercise's own test suite catches directly - a wrong variant, a missing/wrong key or value
   named in the error, or a real I/O failure that panics instead of returning `Err`, all fail here,
   no conceptual judgment required.
5. **Failure propagates via `Result`/`?`, never `.unwrap()`/`.expect()`/`panic!()` on a recoverable
   error (gate, deterministic-adjacent).** A panicking implementation fails test case 8
   (`io_failure_comes_back_as_err_not_a_panic`) directly.
6. **`HandoffError::Io` preserves the real underlying error rather than discarding its structure into
   a `String` the instant it's caught (scored, conceptual).** The shipped stub's `Io(String)` shape,
   built via `.to_string()`, passes every test above identically to a version that wraps the real
   `std::io::Error` and exposes it through `source()` - see "Why this is hard" below for what
   changing it actually buys you, and why almost no clippy lint at any level catches the difference.
7. **Fallible steps are propagated with `?`, not hand-rolled `match`/`return Err` boilerplate that
   adds no behavior (scored, conceptual - partially checked by clippy).** A `match { Ok(v) => v,
   Err(e) => return Err(e) }` in place of `?` on a **same-type** `Result` *is* caught by default
   `cargo clippy -- -D warnings` (`clippy::question_mark`) - but the same manual-match shape on an
   `Option` (`Some(v) => v, None => return Err(..)`), or on a `Result` whose `Err` arm *converts* the
   error type, is not. Don't assume a green `cargo clippy -D warnings` means every fallible step uses
   `?` - it only guarantees the same-type-passthrough cases do.

**Before trusting a green `cargo test` and a clean `cargo clippy` as proof you're done:** they prove
today's behavior is correct, never that `HandoffError`'s shape is one that preserves what a caller
might need later. This isn't hypothetical: it's this exercise's own dry run
(`runs/2026-07-05-module-05-dry-run/grading.md`), and the gap holds under `clippy::pedantic` and
`clippy::nursery` too - checked directly, not assumed. Criterion 6 exists because criteria 1, 2, 4,
and 5 provably can't catch this particular failure mode on their own. Criterion 7 is different from
every prior module's conceptual criteria: it's *partially* catchable by the deterministic tier
itself, checked and confirmed rather than assumed to be as invisible as Module 04's lifetime finding.

## Required to advance / stop condition

Produce implementations of `parse_config` and `write_handoff_summary` that pass `cargo test` and
`cargo clippy -- -D warnings`, touch only `fixtures/relay/src/lib.rs`, propagate every failure via
`Result`/`?` rather than panicking, and give `HandoffError::Io` a shape that preserves the real
underlying `std::io::Error` (via `#[from]`, exposed through `source()`) rather than discarding it
into a `String` at the point it's caught. Reading this page does not count: you advance on a working
implementation Coachgremlin has actually reviewed against the rubric above, not on having read it.

**Valid alternate terminal:** if your first working solution does convert the I/O error to a
`String` the moment `write_handoff_summary` catches it (`Err(e) => Err(HandoffError::Io(e.to_string()))`),
that's not a failure, it's the actual exercise. Go back to the diff and ask: does anything downstream
of this error ever need more than a display message - a specific `ErrorKind` to match on, a full
`source()` chain to log? If the shipped stub already compiles and every test already passes with the
`String` shape, nothing is *forcing* the question - which is exactly why asking it yourself, rather
than waiting for the compiler to, is the actual skill. If the answer is yes (and in real code, it
usually is), replace `Io(String)` with `Io(#[from] std::io::Error)` - one attribute change gets you
both `From<std::io::Error>` (so `?` compiles again) and a working `source()` for free, courtesy of
`thiserror`'s derive macro.

## Where it sits in the arc

Fifth module. Prerequisite: [Module 03, Structs, Enums & Pattern Matching](../03-structs-enums-pattern-matching/README.md)
(`Result`/`Option` are enums, and `ConfigError`/`HandoffError` are matched exhaustively the same way
`CheckpointTrigger` was) and [Module 04, Generics, Traits & Lifetimes](../04-generics-traits-lifetimes/README.md)
(idiomatic `?` propagation and `From`-based conversion build on the same generic/trait vocabulary
`alert_checkpoint` introduced). This deliberately reorders the Book's own ch9-before-ch10 sequence:
the Book teaches error handling with concrete error types before generics exist yet; this module
specifically wants `?`/`From`-based propagation, which reads more naturally with Module 04 already
in place. Next: [Module 06, Fearless Concurrency](../06-fearless-concurrency/README.md). See
[`modules/README.md`](../README.md) for the full arc and why this order.

## Learning objectives

- Choose `panic!` vs `Result` deliberately, based on whether the failure is recoverable by the
  caller, not by default habit. **Directly gated by the required exercise above** (rubric criterion
  5) - a real I/O failure (a missing parent directory) must come back as `Err`, checked with a test
  that would catch a panicking implementation directly.
- Design a custom error type a caller could actually match on, not a stringly-typed catch-all, and
  preserve a wrapped lower-level error's own structure rather than discarding it into a message
  early. **Directly gated by the required exercise above** (rubric criterion 6) - `ConfigError`'s
  three named variants (given) demonstrate the "match on it" half; `HandoffError::Io`'s shipped
  `String` shape versus a `std::io::Error`-wrapping one is the "don't discard structure" half, and
  the one this module's exercise actually asks you to reconsider.
- Use `?` to propagate errors across a call chain without manual `match` boilerplate at every step,
  and know which shapes of that boilerplate the compiler's own default lints already catch.
  **Directly gated by the required exercise above** (rubric criterion 7) - checked, not assumed:
  default `clippy` catches a same-type `Result` passthrough written as a manual `match`, but not the
  same shape over an `Option`, or over a `Result` whose `Err` arm converts the error type.

## Why this is hard, and what actually turned out to matter

**Don't read this section before your first attempt either** - it names the diagnosis directly, same
as the Takeaway Skill below. Attempt the exercise first; come back here once you have a working
`cargo test` pass (or you're genuinely stuck on tooling, not the concept).

An experienced developer catching a `std::io::Error` for the first time in Rust already has a
working instinct from other languages: format the error into a message and move on, because the
message is all anyone's going to print anyway. In Rust, that instinct compiles immediately and
passes every test: `Err(e) => Err(HandoffError::Io(e.to_string()))` needs no new trait impl, no
`From`, nothing beyond what the shipped stub already has in scope. It also quietly answers a question
the exercise was actually asking (can whatever's downstream of this error recover more than a
message?) with "no, by construction" rather than by a real decision - the string conversion doesn't
just format the error, it destroys the only copy of the structured information a caller might have
wanted.

That claim was tested directly, not just asserted - twice, in fact, since the given `HandoffError`
code was migrated from a manual `impl Error`/`Display` to `#[derive(thiserror::Error)]` after this
module's original dry run (coderturtle authorized `thiserror` as a real dependency once the
`dependency_changes: human_required` governance gate was the only thing blocking it). An
implementation using `HandoffError::Io(#[from] std::io::Error)` (`#[from]` derives both
`From<std::io::Error>` and `source()` in one attribute - less ceremony than either by hand) and one
using `HandoffError::Io(String)` (built via `.to_string()` at the catch site) pass `cargo test`
(32/32) and `cargo clippy -- -D warnings` identically. Escalating to `clippy::pedantic` and
`clippy::nursery` doesn't help either - diffed directly, both attempts' output is identical at both
levels. **Re-checked, not assumed to carry over, against the full `clippy::restriction` group after
the `thiserror` migration**: the one weak partial signal the original manual-`impl`-based dry run
found (`clippy::missing_trait_methods`, flagging an unoverridden `source()`) is **gone entirely**
once the `Error` impl is derived rather than hand-written - `thiserror`'s derive macro doesn't
trigger that lint at all, on either attempt (0 occurrences, both). This makes the finding *starker*
after the dependency migration, not just differently-shaped: no lint at any level checked - default,
`pedantic`, `nursery`, or the full `restriction` group - distinguishes the two attempts on this axis
anymore. `restriction` does still contain `clippy::question_mark_used`, which - as in Module 04's
`single_char_lifetime_names` finding - flags the *correct* attempt (for using `?` at all) rather than
the naive one, since that particular restriction lint exists to discourage `?` outright for
codebases that want none. Full evidence, both attempts and every lint level checked, from both before
and after the `thiserror` migration: `runs/2026-07-05-module-05-dry-run/`.

**A second, mechanically distinct naive shape was also checked, targeting this module's other
learning objective** (use `?` instead of manual match boilerplate) **in isolation** - and here the
result flips. Rewriting `parse_config`'s four `?` uses as explicit `match` blocks with identical
behavior gets caught, partially, by `cargo clippy -- -D warnings` itself: `clippy::question_mark`
(part of the **default** "style" lint group - no opt-in flag needed) fires on both `Result ->
Result` passthrough matches (`Ok(v) => v, Err(e) => return Err(e)`, no error transformation), but
stays silent on the mechanically similar `Option -> Result` matches (`Some(v) => v, None =>
return Err(..)`) - confirmed by isolating the two `Option` matches alone (zero warnings). This is
the first time in this workshop's five dry runs that the *default* lint set, with no escalation at
all, outright rescues part of a naive shape. The Takeaway Skill's own validation on a second,
unrelated problem (`runs/2026-07-05-module-05-dry-run/takeaway-validation/`) sharpened this further:
`clippy::question_mark` also stays silent on a manual match whose `Err` arm *converts* the error
type (e.g. `ParseIntError -> RetryPolicyError`), not just same-type passthroughs - so "default
clippy catches manual-match `?`-avoidance" is true only for the narrower same-type-passthrough case,
not manual matches generally.

What actually distinguishes the two `HandoffError` shapes, concretely: the stringified version
writes `HandoffError::Io(e.to_string())`, producing an error whose only content is a message -
nothing downstream can match on `ErrorKind::NotFound` specifically, retry only on a transient
failure, or log a `source()` chain, because none of that survived past the point it was caught. The
structured version writes `Io(#[from] std::io::Error)` in the type declaration and
`std::fs::write(path, content)?` in the function body - `#[from]` is what makes that bare `?` compile
(no `.map_err` needed) and wires `source()` to return the wrapped error automatically. With
`thiserror`, the correct shape is now genuinely *less* ceremony than the naive one, not more - one
attribute versus a `.to_string()` call - which makes "the shipped stub still compiles trivially and
nothing forces the question" the actual reason this is hard, not "the correct answer costs more
effort." The skill this module teaches isn't "never convert an error to a string," it's "convert to
a string only at the point something actually displays it to a human, not at the point you catch
it" - true regardless of which mechanism (manual `impl` or `thiserror`) is available to express it.

## Harness

Agnostic. Nothing in this exercise is specific to one coding-agent tool; it's a standard cargo
crate, run with whatever harness (Claude Code, Codex, or equivalent) you already use daily.

## Takeaway

Don't open this Skill before your first attempt: it names the diagnosis directly, and the exercise
is partly about finding it yourself.

A reusable custom-error-type template, packaged as a Claude Code Skill:
[`.claude/skills/custom-error-type-template/SKILL.md`](../../.claude/skills/custom-error-type-template/SKILL.md).
Validated against a second, unrelated problem (a retry-policy config parser) before being called
done, not just written and left alone - and that validation itself refined one of the Skill's own
claims rather than just confirming it (`runs/2026-07-05-module-05-dry-run/takeaway-validation/`).

> Content status: this module's core exercise is real, not a placeholder: authored, actually run
> once so far (a correct attempt, a deliberately naive early-stringification attempt, and a lighter
> `?`-avoidance check targeting the module's other learning objective specifically - none of them a
> rubric-gaming attempt), and the resulting finding (the deterministic gate is near-blind to the
> error-type-design half of this module's teaching subject, but genuinely partially sighted on the
> `?`-avoidance half) is evidenced in `runs/2026-07-05-module-05-dry-run/`, including an independent
> refinement on a second, unrelated example, not asserted. **`relay` now depends on `thiserror`**
> (added same-session, coderturtle's explicit approval, per this project's `dependency_changes:
> human_required` governance gate) - `ConfigError`/`HandoffError` were migrated from a manual
> `impl Error`/`Display` to `#[derive(thiserror::Error)]`, and the module's core finding was
> re-verified fresh against the migrated code rather than assumed to carry over: it does, and is
> *starker* than originally recorded - the one weak partial signal the manual-`impl` era found
> (`clippy::missing_trait_methods`) doesn't fire at all once the `Error` impl is derived instead of
> hand-written, so no clippy lint at any level (default, `pedantic`, `nursery`, or the full
> `restriction` group) now distinguishes the two `HandoffError` shapes. This module's Module-03/
> Module-04 prerequisites are conceptual/type-level, consistent with how `modules/README.md` frames
> arc dependencies generally - there's no shared-fixture mechanism here that makes them mechanically
> enforceable, since `parse_config`/`write_handoff_summary` only need `Session`/`CheckpointRecord`
> and ordinary generic/trait vocabulary to exist, not any specific prior module's own exercise to be
> solved. Reviewed and confirmed (go) by coderturtle 2026-07-05
> (`runs/run-20260705-RW-006.yaml`, `human_confirmed: true`). Not yet covered by a Workshop Review
> Panel batch - Module 05 is the first module of content since the Modules 03+04 batch (1 of the
> 2-3-module window), due once Module 06 or 07 completes it.
