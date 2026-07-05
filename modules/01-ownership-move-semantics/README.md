# Module 01: Ownership & Move Semantics

## The question this module answers

Who owns this value, and what happens when it moves?

## Exercise: `merge_customer_totals`

Runs against `exercise/`, a real cargo crate in this directory (`SPEC.md` has the full spec).

> Implement `merge_customer_totals(orders: Vec<Order>) -> HashMap<String, u64>`: merge order
> totals by customer name, consuming the input. Six edge cases are checked by the provided test
> suite (`exercise/tests/merge_customer_totals.rs`): empty input, a single order, duplicate
> customers summing, multiple distinct customers not leaking totals across each other,
> case-sensitive names, and a zero-cent order still producing an entry. Get `cargo test` green,
> from your own harness, without narrating the fix as you go, then check it against the rubric
> below.

## Rubric

1. **`cargo test` green (gate, deterministic).** All 6 provided tests pass.
2. **`cargo clippy -- -D warnings` clean (gate, deterministic).** Zero output.
3. **Diff touches only `exercise/src/lib.rs`, not the test file (gate, anti-gaming).**
4. **No `.clone()` on data the function could have moved instead (scored, conceptual).** `orders`
   arrives by value; every `Order` and every field of every `Order` in it is yours to move.
5. **Ownership shape matches what the body actually needs (scored, conceptual).** If your loop
   only ever borrows `orders` (`for order in &orders`), ask why the function still takes it by
   value - a `Vec<Order>` parameter that's never consumed is a signal you borrowed when you could
   have owned.

**Before trusting a green `cargo test` and a clean `cargo clippy` as proof you're done:** they
are not the same claim as "this solution demonstrates ownership, not habit." A solution that
clones `order.customer` defensively passes both gates identically to one that doesn't. This isn't
hypothetical: it's this exercise's own dry run (`runs/2026-07-05-module-01-dry-run/grading.md`),
run for real, not asserted. Criteria 4 and 5 exist because criteria 1 and 2 provably can't catch
this on their own.

## Required to advance / stop condition

Produce an implementation of `merge_customer_totals` that passes `cargo test` and `cargo clippy --
-D warnings`, touches only `exercise/src/lib.rs`, and moves rather than clones every `Order` field
used to build the result. Reading this page does not count: you advance on a working
implementation Coachgremlin has actually reviewed against the rubric above, not on having read it.

**Valid alternate terminal:** if your first working solution does clone somewhere, that's not a
failure, it's the actual exercise. Go back to the diff, name which clone it is, and check: is
`orders` (or the field you cloned) ever used again after that point? If not, rewrite the borrow
to a move and remove the clone. If it genuinely is used again elsewhere, in a way this specific
exercise doesn't require, that's real signal your function's shape may need a second look, not
grounds to keep the clone unexamined.

## Where it sits in the arc

First module. No prior module: ownership is the concept everything else in Rust is a consequence
of. Next: [Module 02, Borrowing & References](../02-borrowing-references/README.md) - the hinge is
that borrowing only makes sense once you can already answer "who owns this" without hesitating.
See [`modules/README.md`](../README.md) for the full arc and why this order.

## Learning objectives

- Predict, before running the compiler, whether a given line moves, copies, or borrows a value.
- Fix a move error (or a borrow you didn't need) by restructuring ownership, not by reflexively
  reaching for `.clone()`.
- Recognize when a "the compiler won't let me" moment is actually "I borrowed when I could have
  owned," the specific shape this exercise's own dry run found.

## Why this is hard, and what actually turned out to matter

An experienced developer picking up Rust already knows how to solve `merge_customer_totals` in
whatever language they came from, usually by iterating a collection and building up a map, a
pattern that works by reference in most garbage-collected languages without a second thought.
Rust's version of that same pattern has a landmine in it precisely because it *compiles* either
way: borrow-and-clone builds the exact same `HashMap<String, u64>` as move-and-own, and nothing
mechanical in this exercise's own gate tells you which one you wrote.

That claim was tested directly, not just asserted: a correct move-based implementation and a
correct-but-clone-heavy one were both built and run for real against this exercise's actual test
suite and `cargo clippy -- -D warnings`. Both passed every check, identically. A stricter, opt-in
lint group (`clippy::pedantic`) did surface something, but through a different, more general lint
(`needless_pass_by_value`) than the clone itself, bundled with an unrelated stylistic nit, and
`pedantic` isn't what most real Rust projects gate on by default. Full evidence:
`runs/2026-07-05-module-01-dry-run/grading.md`.

What actually distinguishes the two solutions, concretely: the clone-heavy one iterates `for order
in &orders`, a borrow, out of habit, even though `orders: Vec<Order>` arrived owned and is never
used again after the loop. The move-based one just... doesn't borrow, because nothing required it
to. The skill this module teaches isn't "avoid `.clone()`," it's "before writing one, check whether
you already own what you're about to clone."

## Harness

Agnostic. Nothing in this exercise is specific to one coding-agent tool; it's a standard cargo
crate, run with whatever harness (Claude Code, Codex, or equivalent) you already use daily.

## Takeaway

Don't open this Skill before your first attempt: it names the diagnosis directly, and the
exercise is partly about finding it yourself.

A reusable ownership/move diagnostic checklist, packaged as a Claude Code Skill:
[`.claude/skills/ownership-move-checklist/SKILL.md`](../../.claude/skills/ownership-move-checklist/SKILL.md).
Validated against a second, unrelated ownership problem (an early-return `Option` pattern, not a
`HashMap`-aggregation one) before being called done, not just written and left alone: see
`runs/2026-07-05-module-01-dry-run/takeaway-validation/`.

> Content status: this module's core exercise is real, not a placeholder: authored, actually run
> (a correct attempt and a deliberately naive, honest one, not a rubric-gaming attempt), and the
> resulting finding (the deterministic gate alone cannot distinguish the two) is evidenced in
> `runs/2026-07-05-module-01-dry-run/`, not asserted. Reviewed against Coachgremlin's own Workflow
> steps 3-6 (`~/hekton/gremlins/coaching/coachgremlin.md`), not yet by the Workshop Review Panel
> (deferred to a batched re-run once more modules have real content - see `docs/next-actions.md`).
