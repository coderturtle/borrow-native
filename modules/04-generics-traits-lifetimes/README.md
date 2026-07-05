# Module 04: Generics, Traits & Lifetimes

## The question this module answers

How do I write one function that works across types, safely, without the compiler losing track of
how long references live?

## Exercise: `alert_checkpoint`

Runs against `fixtures/relay/`, the one shared project every module in this workshop builds a real
feature onto (see that directory's `SPEC.md` for the full product spec and the module-by-module
build-out table). Module 04 adds checkpoint notification: telling a human a checkpoint fired,
through any of three real channels, generically.

> `fixtures/relay/src/lib.rs` already declares the following for you - read it before writing any
> code:
>
> ```rust
> pub trait Notifier {
>     fn notify(&self, message: &str) -> bool;
> }
>
> pub struct DesktopNotifier;
> pub struct TerminalBellNotifier;
> pub struct StdoutNotifier;
> // all three implement Notifier already
>
> impl Session {
>     pub fn label(&self) -> &str {
>         &self.label
>     }
> }
> ```
>
> `Notifier` and its three implementations are given because they're not this module's teaching
> subject - shelling out to a real desktop notifier is an OS-integration concern, not a
> generics/traits/lifetimes one; don't modify them. `Session::label()` is given for a more specific
> reason: read it closely. It returns `&str` with **no explicit lifetime annotation**, and it
> compiles - because elision rule 3 (a `&self` method's elided output lifetime is assigned `&self`'s
> own) already covers it. Keep that fact in view; it's the contrast case for what you're about to
> write.
>
> Your job is `alert_checkpoint`, which formats a message from a `CheckpointTrigger` and hands it to
> any `Notifier`:
>
> ```rust
> pub struct CheckpointAlert {
>     pub session_label: String,
>     pub message: String,
>     pub sent: bool,
> }
>
> pub fn alert_checkpoint<N: Notifier>(
>     notifier: &N,
>     session: &Session,
>     trigger: &CheckpointTrigger,
> ) -> CheckpointAlert {
>     todo!()
> }
> ```
>
> That's the shape the stub ships in - it compiles today because `session_label` is owned, and
> nothing anywhere in the file has an explicit lifetime yet. Whether that's the shape you keep is
> the actual exercise. Message format (also in `fixtures/relay/SPEC.md`):
>
> 1. `TimeElapsed(secs)` → `format!("Checkpoint: {secs}s since last checkpoint")`.
> 2. `ToolCallCount(n)` → `format!("Checkpoint: {n} tool calls since last checkpoint")`.
> 3. `ContextBudget(frac)` → `format!("Checkpoint: {:.0}% of context budget used", frac * 100.0)` -
>    rounded to the nearest whole percent.
>
> `alert_checkpoint` must call `notifier.notify(&message)` exactly once, and report back whatever it
> actually returned in `sent` - not an assumed `true`. Six edge cases are checked by the provided
> test suite (`fixtures/relay/tests/alert_checkpoint.rs`): all three message formats exact, `sent`
> reflecting a real failure (not just a real success), `session_label` matching the session actually
> passed in (checked against two different sessions), and the notifier genuinely invoked once with
> the built message. Run everything from `fixtures/relay/` (`cd fixtures/relay && cargo test`). Get
> `cargo test` green, from your own harness, without narrating the fix as you go, then check it
> against the rubric below.

## Rubric

1. **`cargo test` green (gate, deterministic).** All 22 tests pass (5 from Module 01, 6 from Module
   02, 5 from Module 03, 6 new).
2. **`cargo clippy -- -D warnings` clean (gate, deterministic).** Zero output.
3. **Diff touches only `fixtures/relay/src/lib.rs`, not the test file (gate, anti-gaming).**
4. **Every message format, the real `sent` value, and the notifier invocation count are all correct
   (gate, deterministic).** The behavioral cases this exercise's own test suite catches directly - a
   wrong format string, a hardcoded `sent: true`, or a fabricated alert that never calls `notify`
   all fail here, no conceptual judgment required.
5. **`alert_checkpoint` is generic over `N: Notifier` (scored, structural).** Not `&dyn Notifier`,
   not a hardcoded concrete `Notifier` type - checked by reading the signature, not by `cargo test`,
   since a `&dyn Notifier` version would compile and pass every test in this suite identically.
6. **It is a compile error, not merely unlikely, for a `CheckpointAlert` to be used after the
   `Session` it describes has been dropped (scored, conceptual).** The shipped stub's owned-`String`
   shape passes every test above without this property holding at all - see "Why this is hard"
   below for what actually determines whether your implementation has it, and why no clippy lint at
   any level catches the difference.
7. **A caller may pass `notifier` and `trigger` with a strictly shorter lifetime than `session`, and
   `alert_checkpoint` still compiles (scored, conceptual).** Checked directly
   (`runs/2026-07-05-module-04-dry-run/grading.md`'s over-annotation check): a signature that
   doesn't have this property compiles and passes every test above identically to one that does -
   but over-constrains a caller whose three arguments don't happen to share a lifetime already.

**Before trusting a green `cargo test` and a clean `cargo clippy` as proof you're done:** they prove
today's behavior is correct, never that `CheckpointAlert`'s shape is the one a reviewer would want.
This isn't hypothetical: it's this exercise's own dry run
(`runs/2026-07-05-module-04-dry-run/grading.md`), and the gap holds even under `clippy::pedantic`,
`clippy::nursery`, and the full `clippy::restriction` group - checked directly, not assumed.
Criteria 6 and 7 exist because criteria 1, 2, and 4 provably can't catch either failure mode on
their own.

## Required to advance / stop condition

Produce an implementation of `alert_checkpoint` that passes `cargo test` and `cargo clippy -- -D
warnings`, touches only `fixtures/relay/src/lib.rs`, stays generic over `N: Notifier`, and makes it
a compile error for a `CheckpointAlert` to outlive the `Session` it describes. Reading this page
does not count: you advance on a working implementation Coachgremlin has actually reviewed against
the rubric above, not on having read it.

**Valid alternate terminal:** if your first working solution does duplicate `session.label` into an
owned `String` to sidestep the question, that's not a failure, it's the actual exercise. Go back to
the diff and ask: does `session_label` need to outlive `session` itself anywhere a caller would
actually use this? If not - and in this exercise, it doesn't - replace the copy with a borrow, add
the lifetime parameter the compiler will now ask for, and connect it to `session` alone, not to
every reference parameter in the signature.

## Where it sits in the arc

Fourth module. Prerequisite: [Module 03, Structs, Enums & Pattern
Matching](../03-structs-enums-pattern-matching/README.md) - this module's exercise takes a
`CheckpointTrigger` (defined in Module 03) and implements behavior generically around it, so a
concrete type vocabulary has to exist first. Unlike Module 02's now-mechanically-enforced dependency
on Module 01, this prerequisite is conceptual/type-level, not mechanically gated - `alert_checkpoint`
doesn't require `next_action` to be solved, only for `CheckpointTrigger` to exist as a type, which it
already does regardless of whether Module 03's own exercise is complete. Next: two branches -
[Module 05, Error Handling](../05-error-handling/README.md) (needs this module's generic/boxed error
propagation) and [Module 06, Fearless Concurrency](../06-fearless-concurrency/README.md) (does *not*
depend on this module - `Send`/`Sync` are taught directly within it). See
[`modules/README.md`](../README.md) for the full arc and why this order.

## Learning objectives

- Write a generic function with a trait bound that the compiler accepts on the first attempt, or
  diagnose exactly why it doesn't. **Directly gated by the required exercise above** (rubric
  criterion 5) - `alert_checkpoint<N: Notifier>` must actually be generic, not a `&dyn Notifier`
  shortcut that happens to pass the same tests.
- Annotate a lifetime only when the compiler actually requires it - recognize elision rules rather
  than annotating defensively everywhere. **Directly gated by the required exercise above** (rubric
  criterion 7): recognizing that `Session::label()` needs no annotation (elision rule 3) while
  `alert_checkpoint` genuinely does (no `self`, multiple reference inputs) is built into reading the
  given code before writing any, and criterion 7 scores whether a lifetime, once added, is scoped to
  only `session` rather than unified across all three parameters defensively.
- Distinguish "the borrow checker is wrong about my lifetimes" (it never is) from "my data structure
  doesn't actually support what I'm trying to do." **Directly gated by the required exercise above**
  (rubric criterion 6) - the shipped stub's owned-`String` shape isn't a borrow-checker obstacle to
  work around, it's a design choice this exercise asks you to reconsider once you understand why the
  compiler would accept a borrow here just as easily.

## Why this is hard, and what actually turned out to matter

**Don't read this section before your first attempt either** - it names the diagnosis directly, same
as the Takeaway Skill below. Attempt the exercise first; come back here once you have a working
`cargo test` pass (or you're genuinely stuck on tooling, not the concept).

An experienced developer arriving at their first Rust lifetime error already has a well-worn
instinct from every other language: when the compiler complains that a reference might not live
long enough, make a copy and the problem disappears. In Rust, that instinct works exactly as
advertised - `session.label.clone()` compiles, every test passes, and the error is gone. It also
quietly answers a question the exercise was actually asking (does `session_label` need to outlive
`session`?) with "avoid the question" rather than "no." That's the landmine: cloning to make a
lifetime error disappear and cloning because the value genuinely needs to outlive its source look
identical in the diff and identical in the test output, and only one of them is the correct call
here.

That claim was tested directly, not just asserted. An implementation using `CheckpointAlert<'a> {
session_label: &'a str, .. }` and one using `CheckpointAlert { session_label: String, .. }` (built
via `.clone()`) pass `cargo test` (22/22) and `cargo clippy -- -D warnings` identically. Escalating
to `clippy::pedantic` doesn't just fail to help here - checked fresh, the two attempts' pedantic
output is byte-for-byte identical, not even a noisy partial signal like Module 01's finding.
`clippy::nursery`'s only difference between them is an unrelated doc-comment-length nit. The full
`clippy::restriction` group was also run and diffed by warning type, not just spot-checked with a
few named lints: its one type-level difference between the two attempts is `single_char_lifetime_names`,
which fires on the *correct* attempt for naming its lifetime `'a` (ordinary Rust convention) and stays
silent on the naive one - pointing at the wrong attempt, not a rescue. Unlike Module 03, where an
off-by-default `restriction` lint (`clippy::wildcard_enum_match_arm`) at least existed to catch its
analogous smell once enabled by name, no lint anywhere in clippy's ecosystem - default, `pedantic`,
`nursery`, or the full `restriction` group by name - checks whether a struct field should borrow with
a lifetime instead of owning cloned data - it's an API-design question about a type's own shape,
outside what a per-function, local-dataflow lint pass evaluates at any strictness level.

One cheap check *does* catch this particular instance, though not for a principled reason specific
to lifetimes: `scripts/clone-count-check.sh` (Module 03's optional pre-filter, see that module's
README), calibrated against the reference implementation's own diff (2 clones, both inherited from
Module 02's `session_stats` and unrelated to this module), correctly flags the naive attempt's diff
(3 clones) and passes the correct one's - the first time, inside a module's own graded dry run, that
this pre-filter has produced a true positive rather than a true negative (Module 03's own naive
mistake wasn't clone-shaped, so that run correctly stayed silent instead). This is a coincidence of
this mistake happening to route through `.clone()`, not evidence the pre-filter would catch every way
a learner might avoid an explicit lifetime. A learner reaching for `'static` instead, or leaking data
through a global, would sail past it identically to how they'd sail past `cargo test`. Full evidence,
all attempts and lint levels: `runs/2026-07-05-module-04-dry-run/`.

What actually distinguishes the two solutions, concretely: the cloned version writes
`session_label: session.label.clone()`, producing a `CheckpointAlert` with no compiler-enforced
relationship to the `Session` it came from - it happens to hold the right string today, and nothing
stops a future refactor from passing it a different session's label by mistake, silently. The
borrowed version writes `session_label: session.label()` inside a function signature that names the
connection explicitly (`session: &'a Session, .. -> CheckpointAlert<'a>`) - more ceremony, identical
behavior today, and a compile error the moment someone tries to return a `CheckpointAlert` that
outlives the `Session` it borrowed from, catching a class of bug the cloned version can't even
express as an error. The skill this module teaches isn't "never clone a reference," it's "clone only
when the value genuinely needs to outlive its source, not whenever facing a lifetime you haven't
worked out yet."

**One honest question before you move on, not scored, not gated:** if you handed this exercise's
prompt to your own coding agent with no attempt of your own first, it would very likely have
produced the correctly-scoped `<'a>` in one shot, no clone anywhere in sight - this isn't a hard
problem for a model that already knows Rust. That's not cheating; the deterministic gate doesn't
care how the diff got written. But if that's what happened here, what did *you* just learn, versus
what did your agent just demonstrate? There's no rubric line for that question on purpose - it's
yours to answer honestly, not Coachgremlin's to grade. (See
`.claude/skills/agentic-learning-discipline/SKILL.md` if you want a concrete way to check your own
answer before moving to Module 05.)

## Optional graded extension: Iterators & Closures

Book ch13. `Iterator` is a trait; closures interact directly with borrowing - real and important,
but scoped as an optional extension rather than folded into the core exercise, mirroring
`terminal-velocity`'s validated "required core + optional extensions" pattern. Not authored yet.

## Harness

Agnostic. Nothing in this exercise is specific to one coding-agent tool; it's a standard cargo
crate, run with whatever harness (Claude Code, Codex, or equivalent) you already use daily.

## Takeaway

Don't open this Skill before your first attempt: it names the diagnosis directly, and the exercise
is partly about finding it yourself.

A trait-bound/lifetime-annotation cheat-sheet, packaged as a Claude Code Skill:
[`.claude/skills/trait-lifetime-cheatsheet/SKILL.md`](../../.claude/skills/trait-lifetime-cheatsheet/SKILL.md).
Validated against a second, unrelated problem (a generic config-store lookup) before being called
done, not just written and left alone
(`runs/2026-07-05-module-04-dry-run/takeaway-validation/`).

> Content status: this module's core exercise is real, not a placeholder: authored, actually run
> once so far (a correct attempt, a deliberately naive clone-avoidance attempt, and a lighter
> over-annotation check targeting the module's second learning objective specifically - none of them
> a rubric-gaming attempt), and the resulting finding (the deterministic gate cannot distinguish
> lifetime avoidance or lifetime over-application from the correct shape, and unlike Module 03, no
> clippy lint at any level - default, `pedantic`, `nursery`, or a manually-searched `restriction`
> lint - closes the gap) is evidenced in `runs/2026-07-05-module-04-dry-run/`, including an
> independent reproduction on a second, unrelated example, not asserted. This module's Module-03
> prerequisite is conceptual/type-level, consistent with how `modules/README.md` frames arc
> dependencies generally - like Module 03's own now-resolved case relative to Module 02, there's no
> shared-fixture mechanism here that makes it mechanically enforceable, since `alert_checkpoint`
> only needs `CheckpointTrigger` to exist as a type, not `next_action` to be solved. Reviewed by the
> Workshop Review Panel as part of the Modules 03+04 batch:
> `docs/review-panel/2026-07-05-modules-03-04-content.md`.
