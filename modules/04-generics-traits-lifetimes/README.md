# Module 04: Generics, Traits & Lifetimes

## The question this module answers

How do I write one function that works across types, safely, without the compiler losing track of how long references live?

## Where it sits in the arc

Fourth module. Prerequisite: [Module 03, Structs, Enums & Pattern Matching](../03-structs-enums-pattern-matching/README.md) - this module's exercises implement traits on custom types and take generic/lifetime-bounded references to them, so a concrete type vocabulary has to exist first (the Book's own canonical `fn largest<T: PartialOrd>(list: &[T]) -> &T` example needs no struct, but idiomatic trait-implementation practice does). Next: two branches - [Module 05, Error Handling](../05-error-handling/README.md) (needs this module's generic/boxed error propagation) and [Module 06, Fearless Concurrency](../06-fearless-concurrency/README.md) (does *not* depend on this module - `Send`/`Sync` are taught directly within it, corrected during this workshop's own design review). See [`modules/README.md`](../README.md) for the full arc and why this order.

## Learning objectives (placeholder - finalized when content is authored)

- Write a generic function with a trait bound that the compiler accepts on the first attempt, or diagnose exactly why it doesn't.
- Annotate a lifetime only when the compiler actually requires it - recognize elision rules rather than annotating defensively everywhere.
- Distinguish "the borrow checker is wrong about my lifetimes" (it never is) from "my data structure doesn't actually support what I'm trying to do."

## Exercise material to draw from (not a spec - Coachgremlin authors the real exercise later)

Rust Book ch10 (Generic Types, Traits, and Lifetimes - kept as one chapter, not split, matching how the Book itself teaches it), Rustlings' `14_generics`, `15_traits`, and `16_lifetimes` topic directories. See [`docs/workshop-design.md`](../../docs/workshop-design.md)'s curriculum-anchor section.

## Required gate (placeholder - shape decided now, real rubric written later)

- **Deterministic tier:** `cargo test`/`cargo clippy` clean on a correctly bounded, correctly lifetime-annotated generic function.
- **Conceptual tier (Coachgremlin):** confirms no `'static` or `.clone()` was used to make a lifetime error disappear without understanding it - the single most common way to pass this module's compiler check without learning its actual lesson.

## Optional graded extension: Iterators & Closures

Book ch13. `Iterator` is a trait; closures interact directly with borrowing - real and important, but scoped as an optional extension rather than a standalone module (mirrors `terminal-velocity`'s validated "required core + optional extensions" pattern). Not authored yet.

## Takeaway

A reusable trait-bound/lifetime-annotation cheat-sheet, built from real compiler errors hit during the exercise, not copied from documentation. Packaged by Coachgremlin once the rubric is met.

## Stop condition (placeholder)

The learner's implementation passes the deterministic tier and Coachgremlin confirms the conceptual tier, per the gate above.

---

> **Skeleton only.** This module has a decided question, arc position, gate shape, and takeaway shape. It has no authored exercise, fixture, or rubric yet - that's Coachgremlin's job, run later. See [`modules/README.md`](../README.md) for workshop-wide status.
