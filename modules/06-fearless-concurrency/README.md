# Module 06: Fearless Concurrency

## The question this module answers

How does the compiler let me share state across threads safely, and when should I reach for a channel instead?

## Where it sits in the arc

Sixth module - last of the core arc before the capstone. Prerequisite: [Module 01](../01-ownership-move-semantics/README.md) and [Module 02](../02-borrowing-references/README.md) - "fearless" concurrency is a direct payoff of ownership and borrowing, not a new set of rules; `Send`/`Sync` are taught directly within this module, not derived from Module 04's generics/traits machinery (an earlier draft of this arc overstated that dependency - corrected during this workshop's own design review). Next: [Module 07, Async Programming](../07-async-programming/README.md) - the hinge is that async solves a variant of the same "run many things at once" problem as threads, and the two are easiest to tell apart once you've done one of them for real. See [`modules/README.md`](../README.md) for the full arc and why this order.

## Learning objectives (placeholder - finalized when content is authored)

- Choose between message-passing (channels) and shared-state (`Arc<Mutex<T>>`) concurrency deliberately, based on the actual data-flow shape, not by copying whichever pattern was seen most recently.
- Explain why a type that isn't `Send`/`Sync` can't cross a thread boundary, in terms of ownership and borrowing, not as a memorized rule.
- Recognize a genuine data race the compiler prevented, versus a false alarm caused by a design that could be restructured to avoid sharing at all.

## Exercise material to draw from (not a spec - Coachgremlin authors the real exercise later)

Rust Book ch16 (Fearless Concurrency - its own final section introduces `Send`/`Sync` directly), Rustlings' `20_threads` topic directory. See [`docs/workshop-design.md`](../../docs/workshop-design.md)'s curriculum-anchor section.

## Required gate (placeholder - shape decided now, real rubric written later)

- **Deterministic tier:** `cargo test` green, with no data race across repeated runs (e.g. varying `--test-threads`), on an implementation sharing state safely across threads.
- **Conceptual tier (Coachgremlin):** confirms no `unsafe impl Send`/`Sync` was used to bypass the compiler's check rather than genuinely restructuring the data to be thread-safe - the concurrency-specific version of this workshop's standing escape-hatch warning, and the module where it matters most.

## Takeaway

A concurrency-pattern Skill: a channel-vs-shared-state decision guide, built from the real tradeoff faced during the exercise. Packaged by Coachgremlin once the rubric is met.

## Stop condition (placeholder)

The learner's implementation passes the deterministic tier and Coachgremlin confirms the conceptual tier, per the gate above.

---

> **Skeleton only.** This module has a decided question, arc position, gate shape, and takeaway shape. It has no authored exercise, fixture, or rubric yet - that's Coachgremlin's job, run later. See [`modules/README.md`](../README.md) for workshop-wide status.
