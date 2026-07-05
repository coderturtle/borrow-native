# Decisions: Borrow Native

| Date | Decision | Rationale |
|---|---|---|
| 2026-07-05 | Project scaffolded as factory-output | Initial setup |
| 2026-07-05 | Naming pass complete: final name is Borrow Native (slug `borrow-native`) | Chosen by coderturtle from naming-agent candidates; GitHub repo, local dir, mirror, and vault card all renamed to match. |
| 2026-07-05 | Second real run of the Workshop Gremlin, first on a non-agentic-engineering subject | Surfaced a deterministic-gate augmentation (compiler/test/lint as an objective tier) written back into the canonical Gremlin definitions. |
| 2026-07-05 | 6-module + capstone arc anchored to the Rust Book/Rustlings/Exercism | Real concept-dependency evidence from fetched sources, not an invented sequence. |
| 2026-07-05 | Ran the Workshop Review Panel against naming + design docs before the module skeleton | Caught two citation errors and an undefined capstone gate; fixed same-session. |
| 2026-07-05 | Built module skeleton, brand layer, README rework, and Astro-on-Pages site (no custom domain) | Reused `terminal-velocity`'s site verbatim where generic; locally validated build + typecheck clean. |
| 2026-07-05 | Corrected certification-companion target from the Rust Foundation to Ardan Labs | Rust Foundation accredits training providers, not learners; Ardan Labs has a real proctored exam individuals can sit. |
| 2026-07-05 | Added Module 07 (Async Programming); capstone renumbered 07 to 08 | Checking the arc against Ardan's exam topics found the one gap the original curriculum research missed. |
| 2026-07-05 | coderturtle committed to sitting the real Ardan exam once module content exists | This workshop's own dogfooding evidence; stated intent, not yet done. |
| 2026-07-05 | Installed Rust toolchain via Homebrew; added install script wired into check-prereqs.sh | Needed to actually run Module 01's deterministic gate for a real dry run. |
| 2026-07-05 | Built Module 01's real exercise and ran Coachgremlin's first real dry run | Finding: default clippy can't distinguish a correct move-based solution from a naive clone-heavy one; only Coachgremlin's conceptual tier can. Fed back into coachgremlin.md. |
| 2026-07-05 | Pivoted to one shared throughline project (fixtures/relay/, a human-agent pacing CLI), retiring the per-module toy fixture | Modules should build one real, usable tool. Migrated Module 01's lesson in; same finding reproduced exactly. |
| 2026-07-05 | Designed the full module-to-feature mapping for relay (Modules 01-08) | Rust's additive concept arc maps to one growing project, not five lenses on one static fixture (terminal-velocity's shape). |
| 2026-07-05 | coderturtle reviewed and confirmed both Module 01 dry runs | Both runs flipped to human_confirmed: true. |
| 2026-07-05 | Added a scoped ARB trigger for relay's shared files, plus a Coachgremlin workflow step | ARB is a mechanical governance.yaml file-pattern trigger, not a persona panel; catches a later module silently breaking an earlier module's shipped gate. |
| 2026-07-05 | Confirmed Coachgremlin lacked the persona panel too; added a batch-review cadence (every 2-3 modules) | ARB catches structural regression, the panel catches content quality; neither substitutes for the other. First batch due after Module 02. |
| 2026-07-05 | Found `scripts/arb-trigger-check.sh` was claimed complete but didn't exist; wrote it for real | Discovered while actually running Coachgremlin's Workflow step 0 for Module 02. Verified against a clean baseline and a real touch to `lib.rs`. |
| 2026-07-05 | Extended relay's shared types (`elapsed_secs` on `DraftCheckpoint`/`CheckpointRecord`) for Module 02's gap statistics | An ARB-triggering change, resolved by updating Module 01's already-shipped test file and re-confirming it still compiles. |
| 2026-07-05 | Built Module 02's real exercise (`session_stats`) and ran Coachgremlin's dry run | Finding: default clippy (and even `clippy::pedantic`, unlike Module 01) can't distinguish borrowing correctly from defensively cloning the whole borrowed collection first. Fed back into `coachgremlin.md`. |
| 2026-07-05 | coderturtle reviewed and confirmed Module 02's dry run (go) | `runs/run-20260705-RW-003.yaml` flipped to human_confirmed: true. |
| 2026-07-05 | Resolved Coachgremlin's open Review Trigger question | Module 02's dry run counts as depth evidence within one workshop, not new breadth toward the "3+ runs/2+ workshops" bar - same distinction the Review Panel draws for its own bar. Run count stays at 2. |
| 2026-07-05 | Ran the first content-level Workshop Review Panel batch (Modules 01+02) | 7 personas, 2 cross-persona agreements, 11 single-persona findings, including a real bug (`scripts/arb-trigger-check.sh` always exited 0). Nine findings fixed same-pass, three deferred. Full report: `docs/review-panel/2026-07-05-modules-01-02-content.md`. |
