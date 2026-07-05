# Module 05: Error Handling as Idiomatic Control Flow

## The question this module answers

How do I propagate failure without panicking, in a way callers can actually act on?

## Where it sits in the arc

Fifth module. Prerequisite: [Module 03, Structs, Enums & Pattern Matching](../03-structs-enums-pattern-matching/README.md) (`Result`/`Option` are enums) and [Module 04, Generics, Traits & Lifetimes](../04-generics-traits-lifetimes/README.md) (idiomatic `?` propagation across generic/boxed error types). This deliberately reorders the Book's own ch9-before-ch10 sequence: the Book teaches error handling with concrete error types before generics exist yet; this module specifically wants generic/`Box<dyn Error>` propagation, which needs Module 04 first. Next: [Module 06, Fearless Concurrency](../06-fearless-concurrency/README.md). See [`modules/README.md`](../README.md) for the full arc and why this order.

## Learning objectives (placeholder - finalized when content is authored)

- Choose `panic!` vs `Result` deliberately, based on whether the failure is recoverable by the caller, not by default habit.
- Design a custom error type a caller could actually match on, not a stringly-typed catch-all.
- Use `?` to propagate errors across a call chain without manual `match` boilerplate at every step.

## Exercise material to draw from (not a spec - Coachgremlin authors the real exercise later)

Rust Book ch9 (error handling), Rustlings' `12_options` and `13_error_handling` topic directories. See [`docs/workshop-design.md`](../../docs/workshop-design.md)'s curriculum-anchor section.

## Required gate (placeholder - shape decided now, real rubric written later)

- **Deterministic tier:** `cargo test` green on an implementation that propagates errors via `Result`/`?` rather than panicking on a recoverable failure.
- **Conceptual tier (Coachgremlin):** confirms the error type is one a caller could meaningfully match on or handle differently by variant, not a single opaque string wrapped in `Err`.

## Takeaway

A reusable custom-error-type template (`thiserror`-based or a manual `impl Error`), built from the real error type designed during the exercise. Packaged by Coachgremlin once the rubric is met.

## Stop condition (placeholder)

The learner's implementation passes the deterministic tier and Coachgremlin confirms the conceptual tier, per the gate above.

---

> **Skeleton only.** This module has a decided question, arc position, gate shape, and takeaway shape. It has no authored exercise, fixture, or rubric yet - that's Coachgremlin's job, run later. See [`modules/README.md`](../README.md) for workshop-wide status.
