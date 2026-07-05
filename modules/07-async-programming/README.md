# Module 07: Async Programming

## The question this module answers

How do I write code that waits on many things at once without blocking a thread for each one?

## Where it sits in the arc

Seventh module, added 2026-07-05 when this workshop's arc was checked against the Ardan Labs Rust certification's real exam topics (memory safety, type systems, asynchronous programming, testing/debugging practice) and async turned out to be the one topic the original 6-module arc didn't cover. Prerequisite: [Module 04, Generics, Traits & Lifetimes](../04-generics-traits-lifetimes/README.md) (`Future` is a trait, and `async fn` desugars to a generic state machine) and [Module 06, Fearless Concurrency](../06-fearless-concurrency/README.md) (async solves a variant of the same "run many things at once" problem as threads, and the two are easiest to tell apart once you've done one of them for real). Next: [Module 08, Synthesis capstone](../08-synthesis-capstone/README.md). See [`modules/README.md`](../README.md) for the full arc and why this order.

## Learning objectives (placeholder - finalized when content is authored)

- Explain why `async fn main` doesn't work without a runtime (`tokio`, `async-std`), in terms of what `Future` actually is (a value that does nothing until polled).
- Choose between async and threads deliberately for a given workload, not by default habit.
- Diagnose a common async footgun (holding a `MutexGuard` or other non-`Send` value across an `.await` point) by reading the compiler error, not by trial and error.

## Exercise material to draw from (not a spec - Coachgremlin authors the real exercise later)

Rust Book ch17 (Fundamentals of Asynchronous Programming - the Book's own sequencing, directly after ch16 Concurrency, which is why this module follows Module 06 rather than sitting earlier in the arc), plus whatever async runtime crate (most likely `tokio`, the ecosystem default) the real exercise ends up using. Ardan Labs' own Rust Training Bundle (35+ hours, aligned to their exam) is worth reviewing for exercise shape once content-building starts, without copying its material directly.

## Required gate (placeholder - shape decided now, real rubric written later)

- **Deterministic tier:** `cargo test` green (using the chosen async test harness, e.g. `#[tokio::test]`) on an implementation that correctly awaits multiple futures without blocking.
- **Conceptual tier (Coachgremlin):** confirms the learner chose async deliberately (not because it's trendy) and can explain why a non-`Send` value can't cross an `.await` point, not just that the compiler eventually accepted the code.

## Takeaway

A personal async-vs-threads decision guide, paired with Module 06's concurrency decision guide as a matched set. Packaged by Coachgremlin once the rubric is met.

## Stop condition (placeholder)

The learner's implementation passes the deterministic tier and Coachgremlin confirms the conceptual tier, per the gate above.

---

> **Skeleton only.** This module has a decided question, arc position, gate shape, and takeaway shape. It has no authored exercise, fixture, or rubric yet - that's Coachgremlin's job, run later. See [`modules/README.md`](../README.md) for workshop-wide status.
