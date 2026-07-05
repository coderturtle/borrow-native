# Module 08: Synthesis Capstone

## The question this module answers

Given a broken or non-idiomatic Rust program, which concept is actually the root cause?

## Where it sits in the arc

Eighth and final module. Prerequisite: all seven core modules - this capstone only makes sense once ownership, borrowing, structs/enums, generics/traits/lifetimes, error handling, concurrency, and async are each individually fluent; diagnosing which one is the actual bottleneck requires being able to rule the others out. No next module. See [`modules/README.md`](../README.md) for the full arc and why this order.

## Learning objectives (placeholder - finalized when content is authored)

- Given a Rust program that fails to compile, misbehaves, or merely "works but isn't idiomatic," correctly identify which single arc concept is the actual root cause.
- Defend that diagnosis in writing, distinguishing the root cause from a symptom that would disappear if a different, wrong fix were applied.
- Fix the identified root cause without introducing a new violation of an earlier module's concept (e.g. silencing a lifetime error with `.clone()` while "fixing" a concurrency bug).

## Exercise material to draw from (not a spec - Coachgremlin authors the real exercise later)

A small, multi-file, deliberately broken or non-idiomatic Rust program spanning 3+ arc concepts - mirrors `terminal-velocity`'s own capstone format (diagnose the bottleneck, fix it, defend the diagnosis in writing). Not yet built; Coachgremlin's content-building pass should decide how many of the seven core concepts the shipped fixture actually covers, honestly, the way `terminal-velocity`'s own capstone shipped 2 of 4 layers as required and documented the other 2 as negative controls rather than silently pretending full coverage.

## Required gate (added 2026-07-05 during this workshop's own design review - the initial draft left this undefined, a Review Panel finding)

- **Deterministic tier:** `cargo test`/`cargo clippy` green on the learner's fix to the broken/non-idiomatic program.
- **Conceptual tier (Coachgremlin):** confirms the learner's written diagnosis correctly names the root-cause concept (not a symptom), before or alongside the fix - a correct fix reached by trial and error without a correct diagnosis does not meet this module's actual gate.

## Takeaway

A personal Rust diagnostic playbook, built *from* the defended diagnosis above, compressing the whole arc into one repeatable checklist - not a substitute for actually producing that diagnosis. Packaged by Coachgremlin once the rubric is met.

## Stop condition (placeholder)

The learner's fix passes the deterministic tier and Coachgremlin confirms the written diagnosis correctly identifies the root-cause concept, per the gate above.

---

> **Skeleton only.** This module has a decided question, arc position, gate shape, and takeaway shape. It has no authored exercise, fixture, or rubric yet - that's Coachgremlin's job, run later. See [`modules/README.md`](../README.md) for workshop-wide status.
