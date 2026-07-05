# Module 02: Borrowing & References

## The question this module answers

When do I need a reference instead of ownership, and what can the borrow checker actually see?

## Where it sits in the arc

Second module. Prerequisite: [Module 01, Ownership & Move Semantics](../01-ownership-move-semantics/README.md) - borrowing is unusable without ownership underneath it; a reference is a promise about a value someone else owns. Next: [Module 03, Structs, Enums & Pattern Matching](../03-structs-enums-pattern-matching/README.md) - the hinge is that methods take `&self`/`&mut self`, so modeling data correctly depends on already being fluent in borrowing. See [`modules/README.md`](../README.md) for the full arc and why this order.

## Learning objectives (placeholder - finalized when content is authored)

- Choose `&T` vs `&mut T` vs owned `T` deliberately, based on what the function actually needs to do, not by trial and error against compiler errors.
- Read a borrow-checker error and identify which of the two core rules it's enforcing (one mutable reference, or any number of immutable ones - never both at once).
- Fix a borrow conflict by restructuring scope or splitting a borrow, not by cloning the value to sidestep the checker.

## Exercise material to draw from (not a spec - Coachgremlin authors the real exercise later)

Rust Book ch4.2-4.3 (references, borrowing, and the slice type), Rustlings' `05_vecs` and the later exercises in `06_move_semantics` (the same topic directory anchors both Module 01 and this one - its early files are pure move-semantics, its later ones specifically exercise reference/borrow fixes). See [`docs/workshop-design.md`](../../docs/workshop-design.md)'s curriculum-anchor section.

## Required gate (placeholder - shape decided now, real rubric written later)

- **Deterministic tier:** `cargo test`/`cargo clippy` clean on an implementation using references correctly, with no borrow-checker errors suppressed by working around the type system.
- **Conceptual tier (Coachgremlin):** confirms the learner resolved a real borrow conflict by restructuring code, not by cloning the value purely to avoid engaging with the borrow checker's actual complaint.

## Takeaway

A reusable borrow-checker diagnostic playbook: a personal method for reading a borrow error and finding the real fix, not a list of workarounds. Packaged by Coachgremlin once the rubric is met.

## Stop condition (placeholder)

The learner's implementation passes the deterministic tier and Coachgremlin confirms the conceptual tier, per the gate above.

---

> **Skeleton only.** This module has a decided question, arc position, gate shape, and takeaway shape. It has no authored exercise, fixture, or rubric yet - that's Coachgremlin's job, run later. See [`modules/README.md`](../README.md) for workshop-wide status.
