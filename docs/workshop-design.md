# Workshop Design

> **Borrow Native.** Naming pass complete (see `docs/workshop-gremlin-design.md` and `docs/decisions.md`) — this doc was drafted after the name was chosen, so it uses the final name throughout.

## The one-line problem

Agent-literate practitioners — people who already drive Claude Code, Codex, or similar daily — pick up new languages constantly, but almost always by reading and typing, not by using their agent the way they already use it for real work. Rust specifically has a body of excellent existing material (the official Book, Rustlings, Exercism) that already teaches the language well. None of it teaches Rust *the way this audience already learns everything else*: harness in hand, compiler as referee, an agent giving feedback on the actual attempt.

## Audience

Agent-literate practitioners: comfortable with git, the CLI, reading a diff, and driving a coding agent daily — but **new to Rust specifically**. Not a true-beginner programming workshop (general programming fluency is assumed) and not an intro-to-agents workshop (harness fluency is assumed) — the one thing assumed unfamiliar is Rust's own model: ownership, borrowing, the trait system, and what the compiler is actually telling you.

## Format

Self-paced, public repo. Matches `terminal-velocity`'s precedent and the "public workshop" framing — no facilitator required, scales without a cohort.

## Subject vs. method (see `~/hekton/gremlins/workshop/workshop-gremlin.md`'s "Variant: Tech/Language Workshops")

`terminal-velocity` conflated these — its subject (agentic engineering) and its method (learning by doing it in a harness) were the same thing. Here they're deliberately separate:

- **Subject:** Rust — ownership, borrowing, lifetimes, traits, error handling, concurrency.
- **Method:** agent-native — every exercise is run through the learner's own coding-agent harness, graded first by a **deterministic check** (the compiler, `cargo test`, `cargo clippy`), then by Coachgremlin's conceptual feedback on top (idiom, approach, whether the learner worked *with* the borrow checker or fought it).

The hook is the combination, not either alone: "learn Rust the way you already work, with your agent, and let the compiler — not an opinion — be the first gate."

## The teaching method: agent-native, with a deterministic gate

Every module's core exercise runs through the learner's own harness. Two gate tiers, not one — this is the load-bearing difference from `terminal-velocity`, whose subject had no equivalent objective checker:

1. **Deterministic tier (primary).** `cargo test` green and (where relevant) `cargo clippy` clean. This either passes or it doesn't — no judgment call, no rubric-reading required to know if the exercise's mechanical requirement was met.
2. **Conceptual tier (secondary, Coachgremlin).** Idiom and approach: did the learner reach for `.clone()`/`unsafe`/`Rc<RefCell<_>>` to silence the compiler rather than understanding what it was telling them? Is the solution one a reviewer would accept in real code, not just one that happens to compile?

Working hypothesis, not a proven finding (same honesty discipline `terminal-velocity` used): a deterministic first gate makes Coachgremlin's job *easier and more trustworthy* than `terminal-velocity`'s all-subjective rubric — but this hasn't been tested against a real learner attempt yet. That's the next real evidence this workshop needs, same as `terminal-velocity` needed its own first Coachgremlin dry run.

## Canonical-curriculum anchor (research pass, 2026-07-05)

Rust already has strong, current teaching material. Rather than re-derive a sequence from scratch, this workshop's arc is anchored to it directly:

- **The Rust Programming Language ("the Book")** — 20 chapters, canonical order: Getting Started → Guessing Game → Common Concepts → **Ownership** (ch4) → Structs (ch5) → Enums/Pattern Matching (ch6) → Modules (ch7) → Collections (ch8) → Error Handling (ch9) → **Generics, Traits, and Lifetimes** (ch10, taught as one chapter — not three) → Testing (ch11) → an I/O project (ch12) → Iterators/Closures (ch13) → Cargo (ch14) → Smart Pointers (ch15) → **Fearless Concurrency** (ch16) → Async (ch17) → OOP (ch18) → Patterns (ch19) → Advanced/Unsafe (ch20).
- **Rustlings** — 23 numbered exercises, each mapped by the project itself to a Book section (`01_variables`→§3.1 ... `16_lifetimes`→§10.3 ... `20_threads`→§16.1-3), fixed one at a time against real compiler errors. Confirms the compiler-driven, one-exercise-per-concept shape this workshop's deterministic gate also uses — Rustlings already validates that shape works for Rust specifically.
- **Exercism's Rust track** — its own `config.json` encodes an explicit prerequisite *graph*, not just a list: e.g. `entry-api` requires `[functions, option]`; `vec-stack` requires `[functions, integers, option, enums]`. Real, current evidence that Rust's own teaching community already treats prerequisite-graph sequencing (Enhancement 2 in the Gremlin's tech/language variant) as necessary, not a nice-to-have this workshop invented.

**Differentiator against all three:** none of them are agent-native (you read and type; nothing frames the exercise for or grades your agent's attempt), none give conceptual feedback beyond a pass/fail compile, and none leave you with a keepable takeaway artifact beyond the solved exercise itself. Borrow Native's bet is that layering agent-native delivery, Coachgremlin's conceptual tier, and per-module takeaways on top of Rust's *already excellent* deterministic-checker story is a genuine addition, not a rebuild of what already works.

## The module arc

Anchored to the Book/Rustlings sequence above, but scoped for an audience with general programming fluency already — modules that are mostly generic-programming refresher for a beginner (variables, control flow, basic collections) are compressed into the modules that actually need them as a stepping stone, not given standalone treatment. Each module names its **hard prerequisite** explicitly, per the Gremlin's concept-dependency-arc requirement — not just a position in a plausible-looking list.

| # | Module | Hard prerequisite | Book anchor | Rustlings anchor |
|---|---|---|---|---|
| 01 | Ownership & Move Semantics | none (general programming fluency assumed) | ch4.1 | `06_move_semantics` |
| 02 | Borrowing & References | 01 | ch4.2-4.3 | `05_vecs`, `06_move_semantics` |
| 03 | Structs, Enums & Pattern Matching | 02 (methods take `&self`/`&mut self`) | ch5-6, §19.3 | `07_structs`, `08_enums` |
| 04 | Generics, Traits & Lifetimes | 03 | ch10 (kept as one module — the Book itself teaches these three together, not a split this workshop invented) | `14_generics`, `15_traits`, `16_lifetimes` |
| 05 | Error Handling as Idiomatic Control Flow | 03 (`Result`/`Option` are enums) and 04 (`?` across generic error types) | ch9 | `12_options`, `13_error_handling` |
| 06 | Fearless Concurrency | 01+02 (ownership/borrowing is what makes this safe) and 04 (`Send`/`Sync` are traits) | ch16 | `20_threads` |
| 07 | Synthesis capstone | all of the above | — | — |

**Scoping decision, made now rather than discovered late (mirrors `terminal-velocity`'s own honest-scope notes):** Iterators & Closures (Book ch13) is real and important, but doesn't get a standalone module. It's folded into Module 04 as an **optional graded extension** (Iterator is a trait; closures interact directly with borrowing) — the same "required core + optional extensions" shape `terminal-velocity`'s Module 04 validated, reused here rather than re-invented.

### Why this order (the dependency reasoning, not just the Book's)

Ownership has no prerequisite because everything else in Rust is a consequence of it. Borrowing is unusable without ownership underneath it. Structs/enums need `&self`/`&mut self`, so they wait on borrowing — even though a beginner-Rust book could teach struct *syntax* earlier, this workshop is sequencing by what's actually required to write a correct method, not by surface syntax difficulty. Generics/traits/lifetimes stay one module because they're mutually entangled in real signatures (`fn largest<T: PartialOrd>(list: &[T]) -> &T` needs a trait bound and implicitly a lifetime) — splitting them the way this workshop split harness/loop engineering in `terminal-velocity` would misrepresent how tightly coupled they actually are in Rust, unlike that split, which *was* real. Error handling waits on both enums (module 03) and generic error propagation (module 04). Concurrency comes last among the core modules because "fearless" is a direct payoff of ownership+borrowing (module 1-2) plus `Send`/`Sync` (module 4) — it's the module where the earlier investment visibly pays off, which is also why it's a strong note to end the core arc on before the capstone.

## What you keep

Per the Gremlin's takeaway requirement (`workshop-gremlin.md` Design Principle 4): every module's gate produces something reusable, not just a passed check. Concrete takeaways are Coachgremlin's job at content-building time (not this design pass), but the intended *shape* per module:

| # | Module | Intended takeaway shape |
|---|---|---|
| 01 | Ownership & Move Semantics | A personal "who owns this" checklist/Skill for diagnosing move errors fast |
| 02 | Borrowing & References | A reusable borrow-checker diagnostic playbook |
| 03 | Structs, Enums & Pattern Matching | A idiomatic-modeling checklist (when to reach for an enum vs. a struct with a bool flag) |
| 04 | Generics, Traits & Lifetimes | A reusable trait-bound/lifetime-annotation cheat-sheet, built from real errors hit, not copied from docs |
| 05 | Error Handling | A reusable custom-error-type template (`thiserror`/manual `impl Error`) |
| 06 | Fearless Concurrency | A concurrency-pattern Skill (channel vs. shared-state decision guide) |
| 07 | Synthesis capstone | A personal Rust diagnostic playbook compressing the whole arc into one checklist |

## Build-in-public build log

Published as a dated build-log/journal via GitHub Pages (Astro, see `docs/workshop-gremlin-design.md`), same pipeline as `terminal-velocity`.

## What's explicitly out of scope for this design pass

- Exact exercise specs, fixtures, and rubrics per module (Coachgremlin's job, run later, one concept at a time).
- The Iterators & Closures optional extension's actual content (scoped above, not authored).
- The actual Astro site content and first Pages deploy.
