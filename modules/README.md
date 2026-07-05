# Modules

Borrow Native's spine is Rust's own dependency structure, in order: **ownership → borrowing → structs/enums → generics/traits/lifetimes → error handling → concurrency → async**, then a synthesis capstone. Work through them in order. Each module states a hard prerequisite on an earlier one; skipping ahead means hitting compiler errors the workshop hasn't equipped you to read yet.

This arc is also checked against a real external bar: the [Ardan Labs Rust certification](https://ardanlabs.training/) (a proctored, 100-question exam covering memory safety, type systems, async programming, and testing/debugging practice). Module 07 (Async Programming) exists specifically because that check found a gap the original 6-module arc didn't cover. See [`docs/workshop-design.md`](../docs/workshop-design.md#external-validation-the-ardan-labs-rust-certification) for the full reasoning and what's actually proven versus stated intent.

Every module's core exercise is run through your own coding-agent harness (Claude Code, Codex, or equivalent). Two gates, not one: a **deterministic tier** (`cargo test` green, `cargo clippy` clean, no judgment call) and a **conceptual tier** (Coachgremlin, checking idiom and approach on top of a passing compile). See the top-level README and [`docs/workshop-design.md`](../docs/workshop-design.md) for the full thesis and the curriculum-anchor research behind this arc.

**Hands-on by design, not passive text.** No module here completes by reading it. Every module states a required gate: an artifact you produce or an action you're observed doing, checked first mechanically, then conceptually. If a module ever reduces to "read this, then move on," that's a defect. Every gate also has a stated **takeaway**: you keep something reusable, not just proof you did the exercise.

> **Content status: skeleton only.** Every module below is structure, not content - the question, arc position, gate shape, and takeaway shape are decided; the actual exercise, fixture, and rubric are Coachgremlin's job, run later, one module at a time, per the Workshop Gremlin's own Completion Condition (it stops before content exists). Don't expect a working exercise yet.

## The arc

| # | Module | Hard prerequisite | The question it answers | Required gate (once authored) |
|---|---|---|---|---|
| 01 | [Ownership & Move Semantics](01-ownership-move-semantics/README.md) | none (general programming fluency assumed) | Who owns this value, and what happens when it moves? | `cargo test` green on a move-correct implementation (deterministic) + Coachgremlin confirms no defensive over-cloning (conceptual) |
| 02 | [Borrowing & References](02-borrowing-references/README.md) | 01 | When do I need a reference instead of ownership, and what can the borrow checker actually see? | `cargo test`/`cargo clippy` clean on a correctly-borrowed implementation (deterministic) + Coachgremlin confirms borrows aren't dodged via unnecessary cloning (conceptual) |
| 03 | [Structs, Enums & Pattern Matching](03-structs-enums-pattern-matching/README.md) | 02 | How do I model this data so illegal states are unrepresentable? | `cargo test` green on a correctly-modeled type + exhaustive match (deterministic) + Coachgremlin confirms the modeling choice is idiomatic, not just compiling (conceptual) |
| 04 | [Generics, Traits & Lifetimes](04-generics-traits-lifetimes/README.md) | 03 | How do I write one function that works across types, safely, without the compiler losing track of how long references live? | `cargo test`/`cargo clippy` clean on a correctly bounded, correctly annotated generic function (deterministic) + Coachgremlin confirms no `'static`/`.clone()` used to silence a lifetime error (conceptual) |
| 05 | [Error Handling as Idiomatic Control Flow](05-error-handling/README.md) | 03 and 04 | How do I propagate failure without panicking, in a way callers can actually act on? | `cargo test` green on a `Result`-propagating implementation using `?` (deterministic) + Coachgremlin confirms the error type is one a caller could match on, not a stringly-typed catch-all (conceptual) |
| 06 | [Fearless Concurrency](06-fearless-concurrency/README.md) | 01+02 | How does the compiler let me share state across threads safely, and when should I reach for a channel instead? | `cargo test` green with no data race under `cargo test -- --test-threads` variation (deterministic) + Coachgremlin confirms no `unsafe impl Send`/`Sync` used to bypass the check (conceptual) |
| 07 | [Async Programming](07-async-programming/README.md) | 04+06 | How do I write code that waits on many things at once without blocking a thread for each one? | `cargo test` green (async test harness) on correctly-awaited futures (deterministic) + Coachgremlin confirms the learner can explain why a non-`Send` value can't cross an `.await` point (conceptual) |
| 08 | [Synthesis capstone](08-synthesis-capstone/README.md) | all of the above | Given a broken or non-idiomatic Rust program, which concept is actually the root cause? | `cargo test`/`cargo clippy` green on the fixed program (deterministic) + a written diagnosis Coachgremlin confirms correctly names the root-cause concept, not just a symptom (conceptual) |

Module 04 also carries an **optional graded extension**: Iterators & Closures (Book ch13) - real and important, but not load-bearing enough for its own module at this workshop's scope. See that module's README once authored.

## What you keep

Each module's gate produces a takeaway, not just proof: a real, keepable artifact.

| # | Module | Takeaway |
|---|---|---|
| 01 | Ownership & Move Semantics | A personal "who owns this" checklist/Skill for diagnosing move errors fast |
| 02 | Borrowing & References | A reusable borrow-checker diagnostic playbook |
| 03 | Structs, Enums & Pattern Matching | An idiomatic-modeling checklist (when to reach for an enum vs. a struct with a bool flag) |
| 04 | Generics, Traits & Lifetimes | A reusable trait-bound/lifetime-annotation cheat-sheet, built from real errors hit |
| 05 | Error Handling | A reusable custom-error-type template (`thiserror`/manual `impl Error`) |
| 06 | Fearless Concurrency | A concurrency-pattern Skill (channel vs. shared-state decision guide) |
| 07 | Async Programming | An async-vs-threads decision guide, paired with Module 06's as a matched set |
| 08 | Synthesis capstone | A personal Rust diagnostic playbook compressing the whole arc into one checklist |

## Why this order

This is this workshop's own editorial synthesis, not an independently validated pedagogical finding (same honesty caveat `terminal-velocity` applied to its own harness/loop split). Ownership has no prerequisite because everything else in Rust is a consequence of it. Borrowing is unusable without ownership underneath it. Structs/enums wait on borrowing because methods take `&self`/`&mut self`. Generics/traits/lifetimes stay one module (the Book itself teaches these three together) and depend on Module 03 because the module's real exercises implement traits on custom types, not bare primitives. Error handling waits on both enums (03) and generic error propagation (04). Concurrency depends on ownership/borrowing specifically (01-02), not on 04's machinery: `Send`/`Sync` are taught directly within it. Async (07), added after checking this arc against the Ardan Labs certification's real exam topics, depends on both 04 (`Future` is a trait; `async fn` desugars to a generic state machine) and 06 (easiest to tell apart from threads once you've done one of them for real) - and follows the Rust Book's own sequencing, which places its async chapter directly after concurrency. Full reasoning and the curriculum-anchor research behind it: [`docs/workshop-design.md`](../docs/workshop-design.md).

## Gate tiers (every module uses this vocabulary)

| Tier | What it is |
|---|---|
| Deterministic (primary) | `cargo test` green and/or `cargo clippy` clean - passes or it doesn't, no judgment call. |
| Conceptual (secondary, Coachgremlin) | Idiom and approach: did the learner work *with* the compiler or reach for `.clone()`/`unsafe`/`'static` to silence it? Graded only on what the deterministic tier can't check. |

A green deterministic tier is necessary, never sufficient on its own - same discipline `terminal-velocity`'s Coachgremlin learned the hard way (a fix that edits the test rather than the code can pass every literal gate). See [`docs/workshop-design.md`](../docs/workshop-design.md) and `~/hekton/gremlins/coaching/coachgremlin.md`'s Workflow step 3.
