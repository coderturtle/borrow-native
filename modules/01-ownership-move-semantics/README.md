# Module 01: Ownership & Move Semantics

## The question this module answers

Who owns this value, and what happens when it moves?

## Where it sits in the arc

First module. No prior module: ownership is the concept everything else in Rust is a consequence of. Next: [Module 02, Borrowing & References](../02-borrowing-references/README.md) - the hinge is that borrowing only makes sense once you can already answer "who owns this" without hesitating. See [`modules/README.md`](../README.md) for the full arc and why this order.

## Learning objectives (placeholder - finalized when content is authored)

- Predict, before running the compiler, whether a given line moves, copies, or borrows a value.
- Fix a move error by restructuring ownership, not by reflexively reaching for `.clone()`.
- Recognize when a type's `Copy`-vs-`Clone` status is the actual reason an error appears.

## Exercise material to draw from (not a spec - Coachgremlin authors the real exercise later)

Real material this module's exercise should be built from, not invented from scratch: the Rust Book's own ownership chapter (ch4.1), Rustlings' `06_move_semantics` topic directory (its early exercises specifically - later ones in that same directory anchor Module 02 instead), and Exercism's Rust track concept exercises tagged with ownership/move-related concepts. See [`docs/workshop-design.md`](../../docs/workshop-design.md)'s curriculum-anchor section for the full research this arc is grounded in.

## Required gate (placeholder - shape decided now, real rubric written later)

- **Deterministic tier:** `cargo test` green on an implementation that compiles without a move error and without gratuitous `.clone()` calls added just to make the compiler stop complaining.
- **Conceptual tier (Coachgremlin):** confirms the learner fixed the actual ownership structure rather than defensively cloning around every error the compiler raised - a passing compile that's built entirely out of `.clone()` calls has not demonstrated this module's concept, even though `cargo test` is green.

## Takeaway

A personal "who owns this" checklist/Skill for diagnosing move errors fast - built from the real errors hit during the exercise, not copied from documentation. Packaged by Coachgremlin once the rubric is met (see `~/hekton/gremlins/coaching/coachgremlin.md`'s Workflow step 6).

## Stop condition (placeholder)

The learner's implementation passes the deterministic tier and Coachgremlin confirms the conceptual tier, per the gate above. Reading this page does not count: advancement requires a working, compiler-verified attempt Coachgremlin has actually observed.

---

> **Skeleton only.** This module has a decided question, arc position, gate shape, and takeaway shape. It has no authored exercise, fixture, or rubric yet - that's Coachgremlin's job, run later, per the Workshop Gremlin's Completion Condition (it stops before content exists). See [`modules/README.md`](../README.md) for workshop-wide status.
