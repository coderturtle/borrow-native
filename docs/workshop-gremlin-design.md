# Workshop Gremlin — Design (Track A)

> Canonical definitions live in `~/hekton/gremlins/workshop/workshop-gremlin.md`, `~/hekton/gremlins/coaching/coachgremlin.md`, and `~/hekton/gremlins/workshop/workshop-review-panel.md`. This doc records what changed in those canonical files during *this* run — the Gremlin's second, and its first on a subject other than agentic engineering.

## Why this run matters to the Gremlin, not just this workshop

`terminal-velocity`'s own Follow-Up Actions flagged that the Workshop Gremlin and Coachgremlin should be "watched across 2-3 more workshops before either claims v1 stability," precisely because both were built and run only once, on their own founding subject. A Rust workshop is a genuinely different shape: a language with an objective checker (the compiler, `cargo test`, `cargo clippy`) and an already-mature body of external teaching material (the Book, Rustlings, Exercism) — neither of which `terminal-velocity`'s subject (agentic engineering itself) had. This run exists to find out which parts of the Gremlin generalize and which were quietly specific to its first subject.

## What was found and written back (2026-07-05)

1. **Deterministic-gate tier.** `terminal-velocity`'s gates are entirely subjective — Coachgremlin's rubric is the only arbiter. Rust's compiler/test/lint output is objectively pass/fail. Written into `workshop-gremlin.md`'s Design Principle 2 (a subject with an objective checker states that check as the gate's *primary* tier) and `coachgremlin.md`'s Workflow step 3 (deterministic tier checked first and mechanically; the rubric grades only what the checker can't — idiom, approach). This is a strict upgrade for any subject that has one, not a Rust-only special case.
2. **Concept-dependency arc.** Rust has a real prerequisite graph (confirmed independently by Exercism's own `config.json`, which encodes exactly this as machine-readable `prerequisites` arrays per exercise). Added as an explicit input to the Deliverables & branding agent in `workshop-gremlin.md`.
3. **Canonical-curriculum anchor.** Rather than design this workshop's arc from scratch, it's anchored to the Book's chapter structure and Rustlings' exercise-to-chapter mapping (both fetched and read directly this run, not assumed from memory) — see `docs/workshop-design.md`. Added as the other Deliverables-agent input.
4. **Subject/method decoupling.** `terminal-velocity` named its subject and method as the same thing ("the harness is the classroom"). This workshop's subject (Rust) and method (agent-native delivery) are named separately in `docs/workshop-design.md`, per the new note in `workshop-gremlin.md`.

All four are documented in `~/hekton/gremlins/workshop/workshop-gremlin.md`'s new **"Variant: Tech/Language Workshops"** section — a minor version step (contracts extended within the existing roster), not a new roster item, per `gremlin-model.md`'s versioning rules. The Workshop Review Panel's Instructional Designer persona also gained a matching check (deterministic-gate-vs-subjective-rubric) in `workshop-review-panel.md`.

## Key decisions this run

| Decision | Choice | Why |
|---|---|---|
| Reuse the existing Gremlin roster as-is for scaffolding | Yes — no new roster step | Scaffolding, naming, review panel, deliverables/branding, and the build-log publisher are all subject-agnostic already; only the deliverables agent's *inputs* needed extending, not the pipeline shape. |
| Where the augmentation lives | Canonical Gremlin definitions (`~/hekton/gremlins/`), not this repo only | Explicit coderturtle direction this session: future tech/language workshops should inherit this without re-deriving it. |
| Audience | Agent-literate practitioners, new to Rust specifically (not true beginners) | Keeps Coachgremlin's existing "agent-literate practitioner" assumption valid unchanged — the augmentation needed was about the *subject* (Rust) having a deterministic checker, not about the *audience* needing a different Coachgremlin. |
| Deterministic gate is additive, not a replacement for Coachgremlin | Coachgremlin still grades the conceptual tier on top of a green compiler/test run | A green terminal state was already flagged as insufficient evidence on its own in `coachgremlin.md` (from `terminal-velocity`'s Module 04 dry run) — a subject with a real checker strengthens the first gate, it doesn't remove the need for the second. |
| Module count | 6 core modules + capstone (vs. `terminal-velocity`'s 5 + capstone) | Anchored to real dependency structure found in the Book/Rustlings/Exercism research, not picked to match the prior workshop's count. Iterators/Closures deliberately folded into Module 04 as an optional extension rather than given a standalone module, reusing `terminal-velocity`'s validated "required core + optional extensions" shape instead of inventing a new one. |

## Human gates (same as `terminal-velocity`, unchanged)

- The human picks the final workshop name from the naming agent's candidates — no auto-selection. (Done: **Borrow Native**, from candidates Borrow Native / Fearless Loop / Rust, Verified / Compile or Concede, all GitHub-slug-availability-checked first.)
- First push of the public repo, and the first GitHub Pages deploy, are human-confirmed actions. (First push: confirmed and done this run. Pages deploy: still pending, out of scope for this run.)
- Coachgremlin never certifies a learner "complete" for anything with external consequence without human confirmation of the rubric result — unchanged by the two-tier grading extension; the deterministic tier narrows *what* the rubric needs to judge, it doesn't remove the human-confirmation requirement.

## Status and what's left

**Workshop Gremlin, the Workshop Review Panel, and Coachgremlin are all still v0/draft** — this run adds a second data point toward v1 (3+ runs) for the Workshop Gremlin and Review Panel, but a *third*, ideally differently-shaped again, workshop is still needed before either claims stability. Coachgremlin's two-tier grading extension is designed but not yet run against a real learner attempt in this or any Rust context — same "designed, not yet evidenced" status `terminal-velocity`'s own agent-native-interaction plan was left in. This workshop (`borrow-native`) has, at time of writing: scaffolding, naming, this design doc, done; the Workshop Review Panel pass, module skeleton, brand layer, and build-log site are the remaining Completion Condition items — see `docs/next-actions.md`.
