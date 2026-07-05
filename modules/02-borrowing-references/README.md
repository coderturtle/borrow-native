# Module 02: Borrowing & References

## The question this module answers

When do I need a reference instead of ownership, and what can the borrow checker actually see?

## Exercise: `session_stats`

Runs against `fixtures/relay/`, the one shared project every module in this workshop builds a real
feature onto (see that directory's `SPEC.md` for the full product spec and the module-by-module
build-out table). Module 02 adds session statistics computed by *borrowing* the session history
`finalize_session` (Module 01) already built, rather than consuming it.

> Implement `session_stats(session: &Session) -> SessionStats` in `fixtures/relay/src/lib.rs`:
> compute the average and longest gap between checkpoints, and which checkpoint produced the
> longest one, without taking ownership of `session`. Six edge cases are checked by the provided
> test suite (`fixtures/relay/tests/session_stats.rs`): an empty session, a single checkpoint, an
> average computed as the true mean (not the last value or a running total), the longest gap
> correctly identifying *which* checkpoint produced it (not just the number), a tie for longest gap
> resolving to the first occurrence, and `session` still being usable after the call. Run everything
> from `fixtures/relay/` (`cd fixtures/relay && cargo test` - there's no top-level `Cargo.toml`, so
> `cargo test` from the repo root won't find this crate). Get `cargo test` green, from your own
> harness, without narrating the fix as you go, then check it against the rubric below.

## Rubric

1. **`cargo test` green (gate, deterministic).** All 6 provided tests pass.
2. **`cargo clippy -- -D warnings` clean (gate, deterministic).** Zero output.
3. **Diff touches only `fixtures/relay/src/lib.rs`, not the test file (gate, anti-gaming).**
4. **`session` is read through the borrow, not copied into an owned local first (scored,
   conceptual).** `session: &Session` arrives borrowed; every field you need to read
   (`session.checkpoints`, and each checkpoint's `elapsed_secs`/`goal`) is reachable directly
   through that reference. Building an owned `Vec<CheckpointRecord>` (or cloning the whole
   `Session`) before working with the data is the habit this module targets.
5. **Only the one necessary clone exists (scored, conceptual).** `SessionStats` (as currently
   defined, before this workshop reaches lifetimes in Module 04) has no lifetime parameter, so
   `longest_gap_goal: String` must be owned to leave the function - cloning *that one field, once*
   is the correct move given that type shape, not a smell. Cloning anything else - or cloning that
   same field once per checkpoint instead of once at the end - is the defensive-cloning habit, not
   the fix. (A `SessionStats<'a> { longest_gap_goal: &'a str, .. }` would remove even this clone -
   out of scope here, but worth noticing once you've met Module 04.)

**Before trusting a green `cargo test` and a clean `cargo clippy` as proof you're done:** they are
not the same claim as "this solution demonstrates borrowing, not habit." A solution that clones the
entire checkpoint history before reading it passes both gates identically to one that reads through
the reference directly. This isn't hypothetical: it's this exercise's own dry run
(`runs/2026-07-05-module-02-dry-run/grading.md`), and escalating the deterministic gate to
`clippy::pedantic` doesn't help either - it gives *zero* additional signal here, a starker result
than Module 01's (where pedantic at least caught a noisy proxy lint). Criteria 4 and 5 exist because
criteria 1 and 2 provably can't catch this on their own.

## Required to advance / stop condition

Produce an implementation of `session_stats` that passes `cargo test` and `cargo clippy -- -D
warnings`, touches only `fixtures/relay/src/lib.rs`, reads `session` through the borrow rather than
copying it, and contains exactly the one clone this type shape currently requires
(`longest_gap_goal`). Reading this page does not count: you advance on a working implementation
Coachgremlin has actually reviewed against the rubric above, not on having read it.

**Valid alternate terminal:** if your first working solution does clone the whole collection
somewhere, that's not a failure, it's the actual exercise. Go back to the diff and ask: does the
value I cloned need to outlive this borrow, or was I just cloning to feel safer about reading
through a reference? If the latter, delete the clone and read `session.checkpoints` directly.

## Where it sits in the arc

Second module. Prerequisite: [Module 01, Ownership & Move Semantics](../01-ownership-move-semantics/README.md)
- borrowing is unusable without ownership underneath it; a reference is a promise about a value
someone else owns, and `session_stats` only exists because `finalize_session` already produced a
`Session` worth borrowing. Next: [Module 03, Structs, Enums & Pattern Matching](../03-structs-enums-pattern-matching/README.md),
which adds types modeled with methods that take `&self`/`&mut self` - the hinge is that modeling
data correctly depends on already being fluent in which kind of access a method needs. See
[`modules/README.md`](../README.md) for the full arc and why this order.

## Learning objectives

- Choose `&T` vs `&mut T` vs owned `T` deliberately, based on what the function actually needs to
  do (read vs. mutate vs. keep beyond the borrow's lifetime), not by trial and error against
  compiler errors.
- Recognize the difference between a clone that's load-bearing (the value must outlive the borrow
  that produced it) and one that's defensive habit (cloning a whole collection just to feel safer
  reading through a reference) - the specific shape this exercise's own dry run found.
- Recognize the two core borrow-checker rules (one mutable reference, or any number of immutable
  ones - never both at once) well enough to predict, before compiling, whether a given borrow is
  legal. (This exercise's own `&Session` borrow never actually conflicts with anything, so it won't
  hand you a real borrow-checker error to read - that's Module 06's territory, where concurrent
  access makes conflicts unavoidable. Here, the skill is choosing the right access level up front,
  not resolving a fight after the fact.)

## Why this is hard, and what actually turned out to matter

**Don't read this section before your first attempt either** - it names the diagnosis directly,
same as the Takeaway Skill below. Attempt the exercise first; come back here once you have a
working `cargo test` pass (or you're genuinely stuck on tooling, not the concept).

An experienced developer picking up Rust already knows how to compute stats over a collection in
whatever language they came from: loop over it, track a running max and sum. Rust's version of that
same task has a landmine in it precisely because it *compiles* either way: cloning the whole
`&session.checkpoints` into an owned copy first, then computing over that copy, produces the exact
same `SessionStats` as reading through the reference directly - and nothing mechanical in this
exercise's own gate tells you which one you wrote.

That claim was tested directly, not just asserted: a correct implementation that reads
`session.checkpoints` through the borrow, and a deliberately naive, honest (non-adversarial) one
that clones every field of every checkpoint into an owned `Vec` first "to be safe," pass `cargo
test` (11/11) and default `cargo clippy -- -D warnings` (zero output) identically. Escalating to
`clippy::pedantic` - which at least gave Module 01 a noisy partial signal (`needless_pass_by_value`)
- gives *no* discriminating signal at all here: both attempts produce the exact same five pedantic
warnings, on the same lines, none of them about the wasteful clone. `clippy::nursery`'s
`redundant_clone` lint was also checked specifically, since it targets clone-instead-of-move
patterns - clean on both, because it's designed to catch cloning an owned local you could have
moved, not cloning data reached through a shared reference. Full evidence:
`runs/2026-07-05-module-02-dry-run/grading.md`.

What actually distinguishes the two solutions, concretely: the naive one writes
`session.checkpoints.iter().map(|c| CheckpointRecord { goal: c.goal.clone(), summary:
c.summary.clone(), risks: c.risks.clone(), elapsed_secs: c.elapsed_secs }).collect()` before doing
anything else - rebuilding, field by field, a collection it was already handed read access to. The
borrowing-based solution just reads `session.checkpoints` directly and clones exactly one thing,
once, at the point where an owned value genuinely needs to escape the borrow
(`longest_gap_goal`). The skill this module teaches isn't "never clone under a borrow," it's "clone
only the specific piece that must outlive the borrow, not the whole structure as a first
step."

## Harness

Agnostic. Nothing in this exercise is specific to one coding-agent tool; it's a standard cargo
crate, run with whatever harness (Claude Code, Codex, or equivalent) you already use daily.

## Takeaway

Don't open this Skill before your first attempt: it names the diagnosis directly, and the exercise
is partly about finding it yourself.

A reusable borrow-checker diagnostic playbook, packaged as a Claude Code Skill:
[`.claude/skills/borrow-checker-playbook/SKILL.md`](../../.claude/skills/borrow-checker-playbook/SKILL.md).
Validated against a second, unrelated borrowing problem before being called done, not just written
and left alone (`runs/2026-07-05-module-02-dry-run/takeaway-validation/`).

> Content status: this module's core exercise is real, not a placeholder: authored, actually run
> once so far (a correct attempt and a deliberately naive, honest one, not a rubric-gaming attempt),
> and the resulting finding (the deterministic gate alone cannot distinguish the two, and pedantic
> clippy gives no additional signal for this particular naive-clone shape either) is evidenced in
> `runs/2026-07-05-module-02-dry-run/`, not asserted. Reviewed against this workshop's teaching
> agent's own required process, and by the Workshop Review Panel's first content-level batch:
> `docs/review-panel/2026-07-05-modules-01-02-content.md`. The one open finding from that review -
> the stated "hard prerequisite" on Module 01 was conceptual only, not mechanically enforced - is
> now resolved: `fixtures/relay/tests/session_stats.rs` builds every `Session` through
> `finalize_session` rather than a `Session { .. }` literal, so this module is genuinely ungradable
> until Module 01's exercise is solved. Verified both directions: with `finalize_session` still its
> unsolved `todo!()` stub, all 6 of this module's tests panic before `session_stats` is ever reached;
> with a correct `finalize_session` in place, all 6 pass unchanged. See `docs/decisions.md`.
