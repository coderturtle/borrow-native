# Module 03: Structs, Enums & Pattern Matching

## The question this module answers

How do I model this data so illegal states are unrepresentable?

## Exercise: `next_action`

Runs against `fixtures/relay/`, the one shared project every module in this workshop builds a real
feature onto (see that directory's `SPEC.md` for the full product spec and the module-by-module
build-out table). Module 03 adds trigger/response modeling: deciding what `relay` does next once a
checkpoint fires, given why it fired and how the human responded.

> `fixtures/relay/src/lib.rs` already declares three enums for you - read them before writing any
> code:
>
> ```rust
> pub enum CheckpointTrigger {
>     TimeElapsed(u64),
>     ToolCallCount(u32),
>     ContextBudget(f64),
> }
>
> pub enum HumanResponse {
>     Acknowledged,
>     Snoozed(u64),
>     Ignored,
> }
>
> pub enum NextAction {
>     Continue,
>     PauseFor(u64),
>     EndSession,
> }
> ```
>
> Each is an enum, not a struct with `bool`/`Option` flags, because at most one variant is ever true
> at a time: a checkpoint fires for exactly one reason, a human gives exactly one response, `relay`
> takes exactly one next action. That's this module's real lesson, already made for you in the type
> design - your job is `next_action(trigger: &CheckpointTrigger, response: &HumanResponse) ->
> NextAction`, per this rule table (also in `fixtures/relay/SPEC.md`):
>
> 1. `Acknowledged` → always `Continue`, regardless of trigger.
> 2. `Snoozed(secs)` → always `PauseFor(secs)`, regardless of trigger.
> 3. `Ignored` + `ContextBudget(_)` → `EndSession` (unsafe to let a blown context budget ride on
>    silence).
> 4. `Ignored` + `TimeElapsed(_)` → `Continue` (a soft nudge, safe to proceed).
> 5. `Ignored` + `ToolCallCount(_)` → `Continue` (same reasoning as 4).
>
> Five edge cases are checked by the provided test suite (`fixtures/relay/tests/next_action.rs`):
> `Acknowledged` continuing regardless of trigger (checked against two different triggers, not one),
> `Snoozed` pausing for exactly the given duration regardless of trigger, and all three `Ignored`
> outcomes. Run everything from `fixtures/relay/` (`cd fixtures/relay && cargo test`). Get `cargo
> test` green, from your own harness, without narrating the fix as you go, then check it against the
> rubric below.

## Rubric

1. **`cargo test` green (gate, deterministic).** All 16 tests pass (5 from Module 01, 6 from Module
   02, 5 new).
2. **`cargo clippy -- -D warnings` clean (gate, deterministic).** Zero output.
3. **Diff touches only `fixtures/relay/src/lib.rs`, not the test file (gate, anti-gaming).**
4. **`Ignored` + `ContextBudget` correctly resolves to `EndSession` (gate, deterministic).** The one
   behavioral case this exercise's own test suite catches directly - a wrong default here fails
   `cargo test`, no conceptual judgment required.
5. **Every `CheckpointTrigger` variant is listed explicitly under `Ignored`, not covered by a `_`
   wildcard (scored, conceptual).** `TimeElapsed` and `ToolCallCount` both resolve to `Continue`
   today - the same outcome, reached by two separate, explicit arms rather than one wildcard arm.
   That's deliberate, not padding: see "Why this is hard" below for what a wildcard arm actually
   costs here, and why `clippy::pedantic` will try to talk you out of it.

**Before trusting a green `cargo test` and a clean `cargo clippy` as proof you're done:** they are
not the same claim as "this modeling will still be correct after the next change." A solution that
covers `TimeElapsed`/`ToolCallCount` with `_ => NextAction::Continue` passes both gates identically to
one that lists each variant explicitly. This isn't hypothetical: it's this exercise's own dry run
(`runs/2026-07-05-module-03-dry-run/grading.md`), and it holds up on a second, unrelated
order/refund-status example too (`runs/2026-07-05-module-03-dry-run/takeaway-validation/`). Criterion
5 exists because criteria 1 and 2 provably can't catch this on their own.

## Required to advance / stop condition

Produce an implementation of `next_action` that passes `cargo test` and `cargo clippy -- -D
warnings`, touches only `fixtures/relay/src/lib.rs`, and lists every `CheckpointTrigger` variant
explicitly under the `Ignored` arm rather than folding any of them into a `_` wildcard. Reading this
page does not count: you advance on a working implementation Coachgremlin has actually reviewed
against the rubric above, not on having read it.

**Valid alternate terminal:** if your first working solution does use a `_` wildcard somewhere in
this match, that's not a failure, it's the actual exercise. Go back to the diff and ask: if a new
`CheckpointTrigger` variant were added to this enum tomorrow, would this arm silently swallow it, or
would the compiler force someone to decide what it should do? If the former, replace the wildcard
with the variants it was actually standing in for.

## Where it sits in the arc

Third module. Prerequisite: [Module 02, Borrowing & References](../02-borrowing-references/README.md)
- `next_action` takes both its arguments by reference (`&CheckpointTrigger`, `&HumanResponse`), so
writing a correct method already depends on borrowing fluency; methods on the types this module adds
also take `&self`/`&mut self` in later modules. Next:
[Module 04, Generics, Traits & Lifetimes](../04-generics-traits-lifetimes/README.md), which implements
behavior *on* custom types generically - the hinge is that a concrete struct/enum vocabulary has to
exist first, which this module is what builds it. See [`modules/README.md`](../README.md) for the
full arc and why this order.

## Learning objectives

- Recognize when a `bool`/`Option` flag on a struct is really a hidden enum trying to get out - a
  value with a small number of genuinely mutually-exclusive states belongs in an enum, so the type
  system rules out the combinations that can never actually happen, rather than a comment or a test
  hoping no one constructs them.
- Write an exhaustive `match` and understand what forcing every variant to be handled explicitly
  actually buys you: a compile error the moment a new variant is added, rather than a silent default.
- Recognize that a `_` wildcard arm covering several variants that happen to resolve the same way
  today is not free - it trades away exactly that forced-decision guarantee, in a way no test run can
  detect until a new variant actually gets added later. The specific shape this exercise's own dry run
  found.
- Recognize that a linter's own suggestion isn't automatically the idiomatic choice - checked
  directly in this exercise, `clippy::pedantic` recommends *collapsing* the explicit, forward-safe
  arms back into a wildcard, which would undo the point of criterion 5.

## Why this is hard, and what actually turned out to matter

**Don't read this section before your first attempt either** - it names the diagnosis directly, same
as the Takeaway Skill below. Attempt the exercise first; come back here once you have a working
`cargo test` pass (or you're genuinely stuck on tooling, not the concept).

An experienced developer picking up Rust already knows how to branch on a small set of cases in
whatever language they came from: an if/else chain, or a switch with a `default`. Rust's `match`
looks like that same tool, and for the `Acknowledged`/`Snoozed` arms in this exercise, treating
`trigger` as irrelevant with a leading match on `response` alone is completely correct - those two
arms really don't depend on which `CheckpointTrigger` fired. The landmine is applying that same
"doesn't matter, catch it with `_`" instinct one level down, inside the `Ignored` arm, where
`TimeElapsed` and `ToolCallCount` currently resolve the same way *by coincidence*, not because trigger
truly doesn't matter for `Ignored` - `ContextBudget` proves it does.

That claim was tested directly, not just asserted: an implementation that lists all three
`CheckpointTrigger` variants explicitly under `Ignored`, and one that covers `ContextBudget` explicitly
but folds `TimeElapsed`/`ToolCallCount` into a single `_ => NextAction::Continue`, pass `cargo test`
(16/16) and default `cargo clippy -- -D warnings` (zero output) identically. Escalating to
`clippy::pedantic` doesn't just fail to help here - checked fresh, it actively recommends the wrong
direction: it flags the explicit version's two identical-body arms (`match_same_arms`) and suggests
merging them, which is functionally the same collapse the wildcard version already made. The lint that
actually catches the wildcard risk, `clippy::wildcard_enum_match_arm`, lives in clippy's `restriction`
group - off by default, not bundled into `pedantic` or `nursery`, and clippy's own documentation says
restriction lints are meant to be enabled individually, not as a group, since some conflict with
idiomatic style outright. Full evidence, both the original finding and its independent reproduction on
an unrelated order/refund-status example: `runs/2026-07-05-module-03-dry-run/grading.md` and
`runs/2026-07-05-module-03-dry-run/takeaway-validation/`.

What actually distinguishes the two solutions, concretely: the wildcard version writes `_ =>
NextAction::Continue` under `Ignored`, which is right for exactly the two variants that exist today
and says nothing about any variant added tomorrow. The explicit version writes out
`CheckpointTrigger::TimeElapsed(_) => NextAction::Continue` and `CheckpointTrigger::ToolCallCount(_)
=> NextAction::Continue` as separate arms - more characters, identical behavior today, and a compile
error the moment a fourth `CheckpointTrigger` variant shows up anywhere in the codebase, forcing
whoever adds it to decide what `Ignored` should do for it instead of inheriting `Continue` by default.
The skill this module teaches isn't "never use `_`," it's "reach for `_` only when the cases really
are interchangeable forever, not just for however many variants happen to exist right now."

## Harness

Agnostic. Nothing in this exercise is specific to one coding-agent tool; it's a standard cargo crate,
run with whatever harness (Claude Code, Codex, or equivalent) you already use daily.

## Takeaway

Don't open this Skill before your first attempt: it names the diagnosis directly, and the exercise is
partly about finding it yourself.

An enum-modeling and exhaustive-match playbook, packaged as a Claude Code Skill:
[`.claude/skills/enum-modeling-playbook/SKILL.md`](../../.claude/skills/enum-modeling-playbook/SKILL.md).
Validated against a second, unrelated modeling problem before being called done, not just written and
left alone (`runs/2026-07-05-module-03-dry-run/takeaway-validation/`).

> Content status: this module's core exercise is real, not a placeholder: authored, actually run once
> so far (a correct attempt and a deliberately naive, honest one, not a rubric-gaming attempt), and
> the resulting finding (the deterministic gate alone cannot distinguish the two, and `clippy::pedantic`
> doesn't just fail to help - it recommends the wrong direction) is evidenced in
> `runs/2026-07-05-module-03-dry-run/`, including an independent reproduction of the pedantic finding
> on a second, unrelated example, not asserted. This module's Module-01/02 prerequisite is conceptual
> (borrowing/method-receiver fluency), consistent with how `modules/README.md` frames arc dependencies
> generally - unlike Module 02's now-resolved case, there's no shared-fixture mechanism here that could
> make it mechanically enforceable, since `next_action` doesn't take a `Session` at all. Not yet
> reviewed by the Workshop Review Panel: Module 03 is the first module of content since the Modules
> 01+02 batch, inside the 2-3-module cadence window, not yet due (`docs/next-actions.md`).
