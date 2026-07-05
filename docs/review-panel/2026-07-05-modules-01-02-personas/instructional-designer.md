# Instructional Designer

**Lens:** Do stated learning objectives survive contact with the actual exercises? Does the sequence really build? Does every module state a required gate? Is the deterministic tier genuinely primary where a subject has an objective checker? Does each module name its hard prerequisite, or is the sequence asserted?

**Reviewed:** `modules/README.md` (arc table + "Why this order"), Modules 01+02 READMEs, `fixtures/relay/SPEC.md`, both test files, both dry-run `grading.md`s.

---

## Instructional Design Review — Modules 01 & 02

**Top finding: the "hard prerequisite" on Module 01 is asserted, not demonstrated — mechanically, Module 02 doesn't need it.** The arc table claims Module 02's prerequisite is "01 — borrowing is unusable without ownership underneath it... `session_stats` only exists because `finalize_session` already produced a `Session` worth borrowing." But `fixtures/relay/tests/session_stats.rs` never calls `finalize_session`; it constructs `Session { label: ..., checkpoints: vec![record(...)] }` directly via a local helper. A learner could skip Module 01 entirely, land on Module 02's stub with `Session`/`CheckpointRecord` already defined, and complete the exercise with zero contact with `finalize_session`. The prerequisite is real at the *concept* level (you should understand ownership before borrowing) but not at the *exercise* level (nothing forces or even touches Module 01's artifact). That's a sequencing claim resting on narrative continuity of the fixture, not a load-bearing technical dependency.

**Second finding: the "deterministic tier is primary" framing doesn't hold for the actual lesson.** Both grading.md dry runs conclude the deterministic gate (test+clippy) "provably can't catch" the real learning objective (over-cloning), and Module 02's is starker: `clippy::pedantic` gives "*zero* additional signal here." So 100% of each module's actual pedagogical content is scored by Coachgremlin's subjective diff read, not the compiler. Yet a trivial scripted check — grep the diff for `.clone(` and assert count 0 (Module 01) or exactly 1 on `longest_gap_goal` (Module 02) — would deterministically separate `attempt-good` from `attempt-naive-clone` in both dry runs, since that's literally how the grading.md tables score criteria 4/5. The workshop never tried this cheaper, still-mechanical check before routing the whole lesson through subjective judgment.

**Minor drift:** Module 02's third learning objective ("Read a borrow-checker error and identify which of the two core rules it's enforcing") isn't reachable via this exercise — the spec has no aliasing conflict, so no learner ever sees that error here.

**What works:** rubrics match stated LOs elsewhere; each module gates on a real diff, not reading; takeaways are validated against a second problem, not just written.
