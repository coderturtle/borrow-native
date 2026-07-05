# Workshop Design

> **Borrow Native.** Naming pass complete (see `docs/workshop-gremlin-design.md` and `docs/decisions.md`) — this doc was drafted after the name was chosen, so it uses the final name throughout.

## The one-line problem

Agent-literate practitioners — people who already drive Claude Code, Codex, or similar daily — pick up new languages constantly, but almost always by reading and typing, not by using their agent the way they already use it for real work. Rust specifically has a body of excellent existing material (the official Book, Rustlings, Exercism) that already teaches the language well. None of it teaches Rust *the way this audience already learns everything else*: harness in hand, compiler as referee, an agent giving feedback on the actual attempt.

**Correction, 2026-07-05:** this workshop was originally scoped, informally, as "a companion to the Rust Foundation certification." That target doesn't exist as stated. The Rust Foundation's Trusted Training (RFTT) program, launched June 2026, accredits *training providers* (five founding organizations: Mainmatter, Integer 32, Wyliodrin, Doulos, Ferrous Systems), not individual learners — there's no exam, no individual credential, nothing an individual developer gets "certified" in from the Rust Foundation itself. See "External Validation," below, for the real target this workshop now aims at instead.

## External Validation: The Ardan Labs Rust Certification

Added 2026-07-05, at coderturtle's direction, after the Rust Foundation correction above. Rather than drop the certification-companion idea, it's redirected at a real one: **Ardan Labs' Rust certification** — a proctored, 100-question, 90-minute multiple-choice exam, 80% to pass, $99 per attempt with unlimited retakes, valid 2 years. It covers memory safety, type systems, asynchronous programming, and testing/debugging best practices — real, assessed, individually achievable, unlike RFTT.

This gives Borrow Native a genuine, falsifiable external claim `terminal-velocity` never had a way to make: **did going through this workshop's modules actually prepare a real learner to pass a real, external, human-graded exam.** coderturtle has committed to sitting the real Ardan exam personally once module content exists, as this workshop's own dogfooding evidence — the same discipline `terminal-velocity`'s Coachgremlin dry runs used (a real attempt, evidenced, not just designed). **This is a stated intent, not yet evidenced** — no exam has been attempted at time of writing; treat any claim of alignment with Ardan's topics as this workshop's own bet until that attempt happens and the result is recorded here.

Checking this arc against Ardan's published topic list is what caught the one real gap in the original 6-module design: **async programming** wasn't covered at all. Module 07 (Async Programming) exists specifically because of this check, not because async was independently identified as important during the original curriculum-anchor research. Worth naming honestly: the original research anchored to the Book/Rustlings/Exercism and still missed something a real external exam's topic list caught in one pass — a concrete argument for external validation as a check on curriculum-anchor research, not a redundant step layered on top of it.

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

Working hypothesis, not a proven finding (same honesty discipline `terminal-velocity` used): a deterministic first gate makes Coachgremlin's job *easier and more trustworthy* than `terminal-velocity`'s all-subjective rubric. Module 01's dry run gave this its first real evidence (`runs/2026-07-05-module-01-dry-run/`, re-confirmed after the shared-project pivot below in `runs/2026-07-05-module-01-relay-dry-run/`): a correct attempt and a deliberately naive, honest one passed identically at the deterministic tier, both times. That's one module's evidence, graded by this session's own constructed attempts, not an independent learner's — the next real test is a wider one, same as `terminal-velocity` needed more than its own first Coachgremlin dry run.

## The shared project: `relay`

Added 2026-07-05, at coderturtle's direction, after Module 01 had already shipped as a standalone
toy exercise (`merge_customer_totals`, unrelated to any real product). Every module now builds one
real feature onto the same project instead: `fixtures/relay/`, a CLI for **hybrid human-agent team
pacing** — pomodoro-style checkpoints during an agent session, each producing a restartable-handoff
summary (goal, changes, verification evidence, open risks) and notifying a human, a direct
implementation of the "Restartable Handoff Loop" pattern this workshop's own research already named
(see `modules/README.md`'s Sources). By Module 08, a learner has a real, usable tool, not eight
disconnected snippets. Full product spec, and the same module-by-module build-out table repeated
as the fixture's own source of truth: `fixtures/relay/SPEC.md`.

**Deliberately narrower than a multi-agent cockpit:** `agent-mission-control-lab` already exists in
this factory's `labs/` building a full multi-agent session cockpit. `relay` answers one narrower
question — has this session reached a natural checkpoint, and what's the handoff — rather than
competing with that lab's own scope. Module 06 does add multi-session tracking, but in service of
that same narrow question asked across sessions, not a cockpit reimplementation.

**This deliberately differs from `terminal-velocity`'s fixture shape**, and that difference is
worth stating rather than silently copying the pattern: `receipts` is one static function
engineered five different ways (prompted, curated-around, harnessed, fixed, sabotaged), which fits
because agentic-engineering practice really is five lenses on one problem. `relay` grows one real
feature per module instead, because Rust's concept arc (ownership → borrowing → ... → async) is an
additive dependency chain — each concept is genuinely required to build the next feature, not a
different lens on a feature that already exists.

**What happened to Module 01's original exercise:** retired, not deleted. `merge_customer_totals`'s
own dry run (`runs/2026-07-05-module-01-dry-run/`) remains valid supporting evidence for the
deterministic-gate finding, which is a fact about Rust and clippy, not about that specific fixture.
The exercise itself (own the input, move rather than clone) was re-instantiated as `relay`'s own
first feature (`finalize_session`) and re-validated there — see `runs/2026-07-05-module-01-relay-
dry-run/retro.md`.

## Canonical-curriculum anchor (research pass, 2026-07-05)

Rust already has strong, current teaching material. Rather than re-derive a sequence from scratch, this workshop's arc is anchored to it directly:

- **The Rust Programming Language ("the Book")** — 20 chapters, canonical order: Getting Started → Guessing Game → Common Concepts → **Ownership** (ch4) → Structs (ch5) → Enums/Pattern Matching (ch6) → Modules (ch7) → Collections (ch8) → Error Handling (ch9) → **Generics, Traits, and Lifetimes** (ch10, taught as one chapter — not three) → Testing (ch11) → an I/O project (ch12) → Iterators/Closures (ch13) → Cargo (ch14) → Smart Pointers (ch15) → **Fearless Concurrency** (ch16) → Async (ch17) → OOP (ch18) → Patterns (ch19) → Advanced/Unsafe (ch20).
- **Rustlings** — 23 numbered *topic directories* (`01_variables` through `23_conversions`), each mapped by the project itself to a Book section (`01_variables`→§3.1 ... `16_lifetimes`→§10.3 ... `20_threads`→§16.1-3) and containing several individual exercise files apiece (~94 exercise files total, not 23 — corrected 2026-07-05 per the Review Panel's AI/ML Practitioner finding, which caught this workshop's own first draft conflating topic count with exercise count). Confirms the compiler-driven, one-exercise-per-concept shape this workshop's deterministic gate also uses.
- **Exercism's Rust track** — its own `config.json` tags each concept exercise with a `concepts` list and a `prerequisites` list of *other concepts*, e.g. the exercise "Magazine Cutout" is tagged `concepts: [entry-api]` with `prerequisites: [functions, option]`; "RPN Calculator" is tagged `concepts: [vec-stack]` with `prerequisites: [functions, integers, option, enums]` (corrected 2026-07-05 — the first draft cited the concept tags as if they were the exercises' own names, which the Review Panel's AI/ML Practitioner caught). Real, current evidence that Rust's own teaching community already models prerequisite structure as data, not just prose — lending real (if not "necessary") weight to sequencing this workshop's own arc the same way.

**Differentiator against all three:** none of them are agent-native (you read and type; nothing frames the exercise for or grades your agent's attempt), none give conceptual feedback beyond a pass/fail compile, and none leave you with a keepable takeaway artifact beyond the solved exercise itself. Rustlings' compiler-driven shape is real prior art for *this workshop's deterministic tier specifically* — it says nothing about the agent-native delivery or Coachgremlin's conceptual tier, which is the actual, still-untested bet. Borrow Native's bet is that layering agent-native delivery, Coachgremlin's conceptual tier, and per-module takeaways on top of Rust's *already excellent* deterministic-checker story is a genuine addition, not a rebuild of what already works — a hypothesis this workshop is testing, not a finding it's reporting.

## The module arc

Anchored to the Book/Rustlings sequence above, but scoped for an audience with general programming fluency already — modules that are mostly generic-programming refresher for a beginner (variables, control flow, basic collections) are compressed into the modules that actually need them as a stepping stone, not given standalone treatment. Each module names its **hard prerequisite** explicitly, per the Gremlin's concept-dependency-arc requirement — not just a position in a plausible-looking list.

| # | Module | Hard prerequisite | Book anchor | Rustlings anchor |
|---|---|---|---|---|
| 01 | Ownership & Move Semantics | none (general programming fluency assumed) | ch4.1 | `06_move_semantics` (early exercises: move errors) |
| 02 | Borrowing & References | 01 | ch4.2-4.3 | `05_vecs`, `06_move_semantics` (later exercises: reference/borrow fixes — the same topic directory spans both moves and borrows, hence anchoring both modules) |
| 03 | Structs, Enums & Pattern Matching | 02 (methods take `&self`/`&mut self`) | ch5-6, §19.3 | `07_structs`, `08_enums` |
| 04 | Generics, Traits & Lifetimes | 03 (a concrete custom type from Module 03 is what the module's actual exercises implement traits *on* and take generic/lifetime-bounded references *to* — the bare `fn largest<T: PartialOrd>` example needs no struct, but idiomatic trait-implementation practice does) | ch10 (kept as one module — the Book itself teaches these three together, not a split this workshop invented) | `14_generics`, `15_traits`, `16_lifetimes` |
| 05 | Error Handling as Idiomatic Control Flow | 03 (`Result`/`Option` are enums) and 04 (`?` across generic/boxed error types) — note this reorders the Book's own ch9-before-ch10 sequence deliberately: the Book teaches error handling with concrete error types before generics exist yet, this workshop's Module 05 specifically wants idiomatic `Box<dyn Error>`/generic propagation, which needs Module 04 first | ch9 | `12_options`, `13_error_handling` |
| 06 | Fearless Concurrency | 01+02 (ownership/borrowing is what makes this safe) | ch16 (the chapter's own final section introduces `Send`/`Sync` directly — corrected 2026-07-05: the first draft claimed a Module 04 dependency here that overstated the link; trait *bounds* aren't needed to understand these two marker traits) | `20_threads` |
| 07 | Async Programming | 04 (`Future` is a trait; `async fn` desugars to a generic state machine) and 06 (async solves a variant of the same "run many things at once" problem as threads) | ch17 (the Book's own next chapter after Concurrency) | none dedicated — Rustlings has no async topic directory; exercise material comes from the Book plus the chosen async runtime's own docs |
| 08 | Synthesis capstone | all of the above | — | — |

**Added 2026-07-05:** Module 07 (Async Programming) exists because checking this arc against the Ardan Labs certification's real exam topics (see "External Validation," above) found a gap the Book/Rustlings/Exercism curriculum-anchor research alone hadn't flagged. It follows the Book's own sequencing (ch17 directly after ch16) rather than being inserted arbitrarily.

**Scoping decision, made now rather than discovered late (mirrors `terminal-velocity`'s own honest-scope notes):** Iterators & Closures (Book ch13) is real and important, but doesn't get a standalone module. It's folded into Module 04 as an **optional graded extension** (Iterator is a trait; closures interact directly with borrowing) — the same "required core + optional extensions" shape `terminal-velocity`'s Module 04 validated, reused here rather than re-invented.

### Why this order (the dependency reasoning, not just the Book's)

**This is this workshop's own editorial synthesis, not an independently validated pedagogical finding** — same honesty caveat `terminal-velocity` applied to its own harness/loop split. Ownership has no prerequisite because everything else in Rust is a consequence of it. Borrowing is unusable without ownership underneath it. Structs/enums need `&self`/`&mut self`, so they wait on borrowing — even though a beginner-Rust book could teach struct *syntax* earlier, this workshop is sequencing by what's actually required to write a correct method, not by surface syntax difficulty. Generics/traits/lifetimes stay one module because they're mutually entangled in real signatures (`fn largest<T: PartialOrd>(list: &[T]) -> &T` needs a trait bound and implicitly a lifetime), and depend on Module 03 because the module's actual exercises implement traits on custom types, not bare primitives — splitting generics/traits/lifetimes the way this workshop split harness/loop engineering in `terminal-velocity` would misrepresent how tightly coupled they actually are in Rust, unlike that split, which *was* real. Error handling waits on both enums (Module 03) and generic error propagation (Module 04). Concurrency doesn't need Module 04's machinery (`Send`/`Sync` are taught directly within it), but "fearless" is a direct payoff of ownership+borrowing (Modules 01-02) specifically — it's the module where that earlier investment visibly pays off. Async (Module 07) comes last among the core modules, after concurrency rather than before it: it depends on Module 04 (`Future` is a trait) and is easiest to tell apart from threads once threads are already fluent, and it follows the Book's own chapter sequencing (ch17 directly after ch16). It exists in this arc at all because checking against the Ardan Labs certification's real exam topics found the gap, not because the original curriculum-anchor research surfaced it independently — worth being honest about, since it means external validation caught something internal research alone didn't.

## What you keep

Per the Gremlin's takeaway requirement (`workshop-gremlin.md` Design Principle 4): every module's gate produces something reusable, not just a passed check. Concrete takeaways are Coachgremlin's job at content-building time (not this design pass), but the intended *shape* per module:

| # | Module | Intended takeaway shape |
|---|---|---|
| 01 | Ownership & Move Semantics | A personal "who owns this" checklist/Skill for diagnosing move errors fast |
| 02 | Borrowing & References | A reusable borrow-checker diagnostic playbook |
| 03 | Structs, Enums & Pattern Matching | An idiomatic-modeling checklist (when to reach for an enum vs. a struct with a bool flag) |
| 04 | Generics, Traits & Lifetimes | A reusable trait-bound/lifetime-annotation cheat-sheet, built from real errors hit, not copied from docs. **Escape-hatch warning applies here specifically:** reaching for `'static` or `.clone()` to make a lifetime error disappear is the single most common way to pass this module's compiler check without understanding it — Coachgremlin's conceptual tier exists precisely to catch this. |
| 05 | Error Handling | A reusable custom-error-type template (`thiserror`/manual `impl Error`) |
| 06 | Fearless Concurrency | A concurrency-pattern Skill (channel vs. shared-state decision guide). **Escape-hatch warning applies here specifically:** reaching for `unsafe impl Send`/`Sync` to silence a compiler error, instead of restructuring the data to be genuinely thread-safe, is exactly the unsafe habit this module must not let pass quietly — Coachgremlin's conceptual tier must check for it explicitly. |
| 07 | Async Programming | An async-vs-threads decision guide, paired with Module 06's as a matched set — added 2026-07-05 to close the gap the Ardan Labs certification check found. |
| 08 | Synthesis capstone | **Required gate (defined 2026-07-05, a Review Panel Instructional Designer finding; updated after the shared-project pivot to test against the real `relay` codebase instead of a synthetic broken program):** a real, seeded bug or non-idiomatic pattern in the `relay` project accumulated by Modules 01-07, spanning 3+ concepts. The learner must get it to `cargo test`/`cargo clippy` green (deterministic tier) *and* correctly diagnose, in writing, which arc concept was the actual root cause before fixing it (conceptual tier) — mirrors `terminal-velocity`'s own capstone format (diagnose the bottleneck, fix it, defend the diagnosis in writing), but against a real project the learner helped build rather than a fixture manufactured just to be broken. Takeaway: a personal Rust diagnostic playbook compressing the whole arc into one checklist, built *from* that defended diagnosis, not a substitute for it. |

## Build-in-public build log

Published as a dated build-log/journal via GitHub Pages, using the same Astro-on-Pages pipeline `terminal-velocity` built and validated (Content Layer API, `base`-aware links, `deploy-pages` Action) — see `docs/workshop-gremlin-design.md`'s Key Decisions for why this repo reuses that pipeline rather than building a new one.

## What's explicitly out of scope for this design pass

- `relay`'s Features 2-8 (Modules 02-08's real content) — this pass only stands up the shared
  project's skeleton plus Feature 1 (migrated from the original Module 01 work). Coachgremlin's job,
  run later, one module at a time.
- The Iterators & Closures optional extension's actual content (scoped above, not authored).
- The actual Astro site content and first Pages deploy.
- The real Ardan Labs exam attempt itself — a stated intent this session, not yet scheduled or sat.
