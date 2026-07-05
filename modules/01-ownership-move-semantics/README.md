# Module 01: Ownership & Move Semantics

## The question this module answers

Who owns this value, and what happens when it moves?

## Exercise: `finalize_session`

Runs against `fixtures/relay/`, the one shared project every module in this workshop builds a real
feature onto (see that directory's `SPEC.md` for the full product spec and the module-by-module
build-out table). Module 01 adds the project's core domain types and its first function.

> Implement `finalize_session(label: String, drafts: Vec<DraftCheckpoint>) -> Session` in
> `fixtures/relay/src/lib.rs`: seal a session's drafted checkpoints into finalized records. Five
> edge cases are checked by the provided test suite (`fixtures/relay/tests/finalize_session.rs`):
> empty drafts, a single draft, multiple drafts preserving order, empty risks preserved (not
> dropped), and multiple risks preserved in order. Run everything from `fixtures/relay/`
> (`cd fixtures/relay && cargo test` - there's no top-level `Cargo.toml`, so `cargo test` from the
> repo root won't find this crate). Get `cargo test` green, from your own harness, without
> narrating the fix as you go, then check it against the rubric below.

## Rubric

1. **`cargo test` green (gate, deterministic).** All 5 provided tests pass.
2. **`cargo clippy -- -D warnings` clean (gate, deterministic).** Zero output.
3. **Diff touches only `fixtures/relay/src/lib.rs`, not the test file (gate, anti-gaming).**
4. **Every `String`/`Vec` field in your output `CheckpointRecord`s is the same allocation the
   corresponding `DraftCheckpoint` field already held - none is a new buffer holding a duplicate of
   data this function already owned (scored, conceptual).** `drafts` arrives by value; check
   whether your implementation actually uses that fact.
5. **Nothing in your implementation reads `drafts` through a shared reference while the function
   itself already holds full ownership of it (scored, conceptual).** Trace where each field of your
   output came from - if the path runs through `&drafts` or `&draft` anywhere, ask why, given that
   `drafts: Vec<DraftCheckpoint>` was never going to be used again after this call.

**Before trusting a green `cargo test` and a clean `cargo clippy` as proof you're done:** they are
not the same claim as "this solution demonstrates ownership, not habit." A solution that clones
every field of every draft passes both gates identically to one that doesn't. This isn't
hypothetical: it's this exercise's own dry run, run twice now (once against the retired standalone
`merge_customer_totals` toy, once against this exact project - see
`runs/2026-07-05-module-01-dry-run/grading.md` and `runs/2026-07-05-module-01-relay-dry-run/
grading.md`), the same finding both times. Criteria 4 and 5 exist because criteria 1 and 2 provably
can't catch this on their own.

## Required to advance / stop condition

Produce an implementation of `finalize_session` that passes `cargo test` and `cargo clippy -- -D
warnings`, touches only `fixtures/relay/src/lib.rs`, and allocates nothing new for data this
function already owns outright. Reading this page does not count: you advance on a working
implementation Coachgremlin has actually reviewed against the rubric above, not on having read it.

**Valid alternate terminal:** if your first working solution does duplicate some data it already
owned, that's not a failure, it's the actual exercise. Go back to the diff, name the spot, and
check: is `drafts` (or the field in question) ever used again after that point? If not, rewrite
whatever reads it through a reference so it takes ownership directly instead.

## Where it sits in the arc

First module. No prior module: ownership is the concept everything else in Rust is a consequence
of. Next: [Module 02, Borrowing & References](../02-borrowing-references/README.md), which adds
session statistics computed by *borrowing* `Session`'s history rather than consuming it - the hinge
is that borrowing only makes sense once you can already answer "who owns this" without hesitating.
See [`modules/README.md`](../README.md) for the full arc and why this order.

## Learning objectives

- Predict, before running the compiler, whether a given line moves, copies, or borrows a value.
- Fix a move error (or a borrow you didn't need) by restructuring ownership, not by reflexively
  reaching for `.clone()`.
- Recognize when a "the compiler won't let me" moment is actually "I borrowed when I could have
  owned," the specific shape this exercise's own dry run found, twice.

## Why this is hard, and what actually turned out to matter

**Don't read this section before your first attempt either** - it names the diagnosis directly,
same as the Takeaway Skill below. Attempt the exercise first; come back here once you have a
working `cargo test` pass (or you're genuinely stuck on tooling, not the concept).

An experienced developer picking up Rust already knows how to solve `finalize_session` in whatever
language they came from: iterate a list, build up a new one. Rust's version of that same pattern
has a landmine in it precisely because it *compiles* either way: borrow-and-clone builds the exact
same `Session` as move-and-own, and nothing mechanical in this exercise's own gate tells you which
one you wrote.

That claim was tested directly, not just asserted, and tested **twice**: once against an earlier,
now-retired standalone exercise (`merge_customer_totals`, an unrelated `Order`/`HashMap` toy), and
again against this project's own real `finalize_session` after the workshop pivoted to one shared
throughline project (see `modules/README.md` for why). Both times, a correct move-based
implementation and a correct-but-clone-heavy one passed every check identically. A stricter,
opt-in lint group (`clippy::pedantic`) did surface something in the original run, but through a
different, more general lint (`needless_pass_by_value`) than the clone itself, bundled with an
unrelated stylistic nit, and `pedantic` isn't what most real Rust projects gate on by default.
Full evidence: `runs/2026-07-05-module-01-dry-run/grading.md` (original) and
`runs/2026-07-05-module-01-relay-dry-run/grading.md` (re-run against `relay`).

What actually distinguishes the two solutions, concretely: the clone-heavy one iterates `for draft
in &drafts`, a borrow, out of habit, even though `drafts: Vec<DraftCheckpoint>` arrived owned and is
never used again after the loop. The move-based one just... doesn't borrow, because nothing
required it to. The skill this module teaches isn't "avoid `.clone()`," it's "before writing one,
check whether you already own what you're about to clone."

**One honest question before you move on, not scored, not gated:** if you handed this exercise's
prompt to your own coding agent with no attempt of your own first, it would very likely have
produced the move-based version in one shot - it's not a hard problem for a model that already
knows Rust. That's not cheating; the deterministic gate doesn't care how the diff got written. But
if that's what happened here, what did *you* just learn, versus what did your agent just
demonstrate? There's no rubric line for that question on purpose - it's yours to answer honestly,
not Coachgremlin's to grade. (See `.claude/skills/agentic-learning-discipline/SKILL.md` if you want
a concrete way to check your own answer before moving to Module 02.)

## Harness

Agnostic. Nothing in this exercise is specific to one coding-agent tool; it's a standard cargo
crate, run with whatever harness (Claude Code, Codex, or equivalent) you already use daily.

## Takeaway

Don't open this Skill before your first attempt: it names the diagnosis directly, and the exercise
is partly about finding it yourself.

A reusable ownership/move diagnostic checklist, packaged as a Claude Code Skill:
[`.claude/skills/ownership-move-checklist/SKILL.md`](../../.claude/skills/ownership-move-checklist/SKILL.md).
Validated against a second, unrelated ownership problem before being called done, not just written
and left alone (`runs/2026-07-05-module-01-dry-run/takeaway-validation/`), and re-confirmed to
still apply without modification when this module's own exercise migrated to `relay`
(`runs/2026-07-05-module-01-relay-dry-run/retro.md`).

> Content status: this module's core exercise is real, not a placeholder: authored, actually run
> twice (a correct attempt and a deliberately naive, honest one, not a rubric-gaming attempt, both
> before and after the shared-project pivot), and the resulting finding (the deterministic gate
> alone cannot distinguish the two) is evidenced in `runs/2026-07-05-module-01-dry-run/` and
> `runs/2026-07-05-module-01-relay-dry-run/`, not asserted. Reviewed against Coachgremlin's own
> Workflow steps 3-6 by this workshop's teaching agent, and by the Workshop Review Panel's first
> content-level batch: `docs/review-panel/2026-07-05-modules-01-02-content.md`.
