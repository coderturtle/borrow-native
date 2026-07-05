# Module 03: Structs, Enums & Pattern Matching

## The question this module answers

How do I model this data so illegal states are unrepresentable?

## Where it sits in the arc

Third module. Prerequisite: [Module 02, Borrowing & References](../02-borrowing-references/README.md) - methods take `&self`/`&mut self`, so writing a correct method already depends on borrowing fluency. Next: [Module 04, Generics, Traits & Lifetimes](../04-generics-traits-lifetimes/README.md) - the hinge is that generic/trait exercises implement behavior *on* custom types, so a concrete struct/enum vocabulary has to exist first. See [`modules/README.md`](../README.md) for the full arc and why this order.

## Learning objectives (placeholder - finalized when content is authored)

- Choose between a struct, an enum, and an enum-wrapping-a-struct based on what states are actually possible, not habit carried over from another language.
- Write an exhaustive `match` and understand why the compiler forces it.
- Recognize when a `bool` flag on a struct is really a hidden enum trying to get out.

## Exercise material to draw from (not a spec - Coachgremlin authors the real exercise later)

Rust Book ch5 (structs) and ch6 + §19.3 (enums, pattern matching, refutability), Rustlings' `07_structs` and `08_enums` topic directories. See [`docs/workshop-design.md`](../../docs/workshop-design.md)'s curriculum-anchor section.

## Required gate (placeholder - shape decided now, real rubric written later)

- **Deterministic tier:** `cargo test` green on a correctly-modeled type with an exhaustive `match` (no wildcard `_` arm papering over a missed case, unless the exercise explicitly calls for one).
- **Conceptual tier (Coachgremlin):** confirms the modeling choice is idiomatic - an enum used where states are genuinely mutually exclusive, not a struct-plus-flag doing the same job worse - not just that it compiles.

## Takeaway

An idiomatic-modeling checklist: when to reach for an enum vs. a struct with a bool flag, built from the real modeling decisions made during the exercise. Packaged by Coachgremlin once the rubric is met.

## Stop condition (placeholder)

The learner's implementation passes the deterministic tier and Coachgremlin confirms the conceptual tier, per the gate above.

---

> **Skeleton only.** This module has a decided question, arc position, gate shape, and takeaway shape. It has no authored exercise, fixture, or rubric yet - that's Coachgremlin's job, run later. See [`modules/README.md`](../README.md) for workshop-wide status.
