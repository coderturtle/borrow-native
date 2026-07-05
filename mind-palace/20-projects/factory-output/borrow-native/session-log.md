# Session Log: Borrow Native

## 2026-07-05 - Initial scaffold

Project scaffolded as **factory-output**. Learn Rust the agent-native way: agent-literate practitioners work through harness-driven exercises with the compiler and test suite as a deterministic gate, and Coachgremlin grading the conceptual/idiomatic layer on top.

## 2026-07-05 - Workshop Gremlin full run through build-log site

Naming pass (**Borrow Native**), Workshop Review Panel run against naming/design docs, 8-module skeleton (Module 07 Async added after checking the arc against Ardan Labs' real cert exam topics), brand layer, and an Astro-on-GitHub-Pages site, locally validated. Certification-companion target corrected from the Rust Foundation (no individual credential) to Ardan Labs' real proctored exam.

## 2026-07-05 - Pivot to `relay`, Module 01 authored and confirmed

Retired the per-module toy-fixture plan in favor of one shared throughline project, `fixtures/relay/` (a restartable-handoff CLI). Designed the full Module 01-08 feature build-out. Authored Module 01's real exercise (`finalize_session`) and ran Coachgremlin's first real dry run: default clippy can't distinguish a correct move-based solution from a naive clone-heavy one. coderturtle reviewed and confirmed both Module 01 dry runs (go).

## 2026-07-05 - Module 02 authored, ARB mechanism added, first Review Panel batch

Added a scoped ARB trigger (`scripts/arb-trigger-check.sh`, written for real after discovering it had been claimed complete but didn't exist) plus a Coachgremlin workflow step to catch a later module silently breaking an earlier one's shipped gate. Authored Module 02 (`session_stats`): default clippy, even `clippy::pedantic`, can't distinguish borrowing correctly from defensively cloning the whole collection first. coderturtle confirmed the dry run. Ran the workshop's first content-level Review Panel batch (Modules 01+02, 7 personas) - found and fixed a real bug (the ARB script always exited 0).

## 2026-07-05 - Module 02 prerequisite resolved, clone-count pre-filter adopted, Module 03 authored

Made Module 02's Module-01 prerequisite mechanical (tests now build `Session` via `finalize_session`, not a literal). Trialed and adopted `scripts/clone-count-check.sh` as a cheap deterministic pre-filter ahead of Coachgremlin's conceptual read. Authored Module 03 (`next_action`, enums/pattern-matching): default clippy can't tell an exhaustive match from a wildcard-folded one, and `clippy::pedantic` actively points the wrong way. Reviewed and confirmed by coderturtle.

## 2026-07-05 - Module 04 authored, second Review Panel batch, a real correction to Module 03

Authored Module 04 (`alert_checkpoint`, `Notifier` trait) - the first module to exercise explicit lifetimes. Finding: no clippy lint at any level, including the full `restriction` group, rescues either of two naive shapes. Reviewed and confirmed by coderturtle. Ran the second content-level Review Panel batch (Modules 03+04): most significant finding was a real technical overclaim in Module 03's pedantic finding, caught by actually compiling the claim. Corrected across five files. Two findings deferred (rubric-spoils-the-answer tension; module pages read as audit reports, not pitches).

## 2026-07-05 - Certification Alignment Retrofit scoped (not built)

Scoped a deferred phase mapping each module to Ardan Labs' real exam structure, gated on coderturtle sitting the real exam and all 8 modules being authored - not built now, since only a four-category marketing-page list of Ardan's topics exists today.

## 2026-07-05 - Module 05 authored, dry run, confirmed; `thiserror` authorized and migrated

Authored Module 05 (`parse_config`/`write_handoff_summary`, custom error types). Finding: the deterministic gate is near-blind to the error-type-design half (stringify-early vs preserve-structure via `source()`), but genuinely partially sighted on the `?`-avoidance half - default `clippy::question_mark` catches a same-type manual-match passthrough. First mixed-blindness result in the workshop. coderturtle reviewed and confirmed the dry run, then authorized adding `thiserror` as a real dependency (previously avoided only pending that approval). `ConfigError`/`HandoffError` migrated to `#[derive(thiserror::Error)]`; the module's finding was re-verified against the migrated code rather than assumed to carry over, and turned out starker (the one weak partial lint signal disappears with a derived `Error` impl). Added a CLAUDE.md rule: flag dependency questions to the user with the cons of *not* using one, rather than deciding unilaterally.
