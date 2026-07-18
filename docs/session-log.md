# Session Log: Borrow Native

## 2026-07-05 - Initial scaffold

Project scaffolded as **factory-output**. Purpose: Learn Rust the agent-native way: agent-literate practitioners work through harness-driven exercises with the compiler and test suite as a deterministic gate, and Coachgremlin grading the conceptual/idiomatic layer on top.

### Decisions Made

- Classification: factory-output
- Owner: coderturtle
- Vault mutation: not allowed by default
- Promotion target: none

### Next Actions

- Define brief and first phase plan
- Add first implementation
- Record initial decisions

## 2026-07-05 - Workshop Gremlin full run: scaffold through build-log site

Full Workshop Gremlin pipeline run in one session, this Gremlin's second end-to-end run and its first on a subject other than agentic engineering.

### What happened

- Scaffolded as `rust-workshop`, then renamed to **Borrow Native** (`borrow-native`) after the naming pass, chosen by coderturtle from candidates. GitHub repo, local dir, git remote, repo-local mind-palace mirror, and vault card all renamed to match (vault mutation explicitly authorised for this specific move).
- Researched Rust's canonical teaching material (the Book, Rustlings, Exercism, all fetched directly) and designed a 6-module + capstone arc anchored to it, each module naming an explicit hard prerequisite. Wrote `docs/workshop-design.md` and `docs/workshop-gremlin-design.md`.
- Ran the Workshop Review Panel (7 personas, parallel) against the naming + design docs. All seven returned distinct findings; two cross-persona agreements (overstated curriculum-anchor citations; unreconciled Module 04 prerequisite reasoning) and five single-persona findings, including an undefined capstone gate. Cheap design-doc-text fixes applied same-session; the rest deferred on the record. Full report: `docs/review-panel/2026-07-05-initial-design.md`.
- Built the module skeleton (7 modules, 8-part template each) and brand layer (`docs/brand.md`, with a new Rust-specific hard rule against modeling `unsafe`/`.clone()`-to-silence-the-compiler as reasonable). Split internal framing into `docs/maintainers.md`; rewrote the learner-facing README to lead with the deterministic-gate hook and explain Coachgremlin in plain terms.
- Adapted `terminal-velocity`'s Astro-on-Pages site (Content Layer API, base-aware links under `/borrow-native/`, no custom domain for this workshop) and wrote one deliberate build-log entry. Locally validated: `npm run build` and `astro check` both clean; base-path links confirmed correct in built HTML. `npm audit` found 4 inherited vulnerabilities from the starter (same as `terminal-velocity`'s), logged as RISK-0002, not yet triaged.
- **Fed back into the canonical Gremlin definitions** (`~/hekton/gremlins/`): a deterministic-gate tier (Rust's compiler/test/lint as an objective check, layered under Coachgremlin's subjective conceptual tier), a concept-dependency-arc requirement, a canonical-curriculum-anchor research step, and a subject/method decoupling note. Written into `workshop-gremlin.md` (new "Variant: Tech/Language Workshops" section, Design Principle 2 extension, Second Run entry), `coachgremlin.md` (two-tier grading in Workflow step 3), and `workshop-review-panel.md` (Instructional Designer check) — a minor version step, not a new roster item.

### Decisions Made

- See `docs/decisions.md` for the full ADR log this session added (naming, Gremlin augmentation, module count/scoping).

### Assumptions

- Audience is agent-literate practitioners new to Rust specifically, not true beginners (user-confirmed).
- Custom domain (matching `terminal-velocity.coderturtle.io`'s pattern) explicitly out of scope this session; default GitHub Pages project hosting used instead.

### Risks

- RISK-0002: 4 inherited `npm audit` vulnerabilities in the Astro starter chain, not yet triaged, must be addressed before the first real Pages deploy.
- Coachgremlin's two-tier grading extension is designed but has never graded a real learner attempt, in this or any context.

### Next Actions

- See `docs/next-actions.md`. Immediate: human-triggered first Pages deploy; Coachgremlin content-building starting with Module 01.

### Validation Status

- `npm run build` and `astro check` (site): clean.
- `scripts/check-brand-lint.sh` and `scripts/check-mirror-drift.sh`: clean.
- Workshop Gremlin Completion Condition: scaffold/name/review-panel/module-skeleton/brand/site all done; content-building (Coachgremlin) and first live deploy explicitly out of scope, per the Gremlin's own stop condition.

### Mind-palace updated

Yes — vault card renamed to match (`borrow-native`), repo-local mirror kept in sync (`check-mirror-drift.sh` clean).

## 2026-07-05 - Certification-target correction, Async module, Coachgremlin's first real dry run

### What happened

- Corrected the certification-companion target from "the Rust Foundation certification" (doesn't exist for individuals — RFTT accredits training providers only) to Ardan Labs' real proctored exam, per research against six external links. Checking this arc against Ardan's topics found a gap: added Module 07 (Async Programming), capstone renumbered 07 → 08. coderturtle committed to personally sitting the real exam once module content exists.
- Installed the Rust toolchain (`brew install rust`), added `scripts/install-rust-toolchain.sh` (scoped to this project, portable enough to promote to `~/hekton/bootstrap/` later), wired into `scripts/check-prereqs.sh`.
- Built Module 01's real exercise (`merge_customer_totals`, a cargo crate under `modules/01-ownership-move-semantics/exercise/`) and ran Coachgremlin's first real dry run for this workshop: a correct move-based attempt and a deliberately naive, honest (non-adversarial) clone-heavy attempt.
- **Load-bearing finding:** both attempts pass `cargo test` (6/6) and default `cargo clippy -- -D warnings` (zero output) identically. The deterministic gate cannot tell them apart; only Coachgremlin's conceptual tier does. Checked whether `clippy::pedantic` closes the gap: it flags the naive attempt, but via a different, more general lint (`needless_pass_by_value`), noisily, and isn't a default most real projects enable — kept this module's stated gate at default clippy. Full evidence: `runs/2026-07-05-module-01-dry-run/`.
- Packaged the takeaway (`.claude/skills/ownership-move-checklist/SKILL.md`) and validated it against a second, unrelated ownership problem (an early-return `Option` pattern) before calling it done — including an honest, unplanned wrinkle (the move-based fix itself tripped a different, unrelated clippy lint, `manual_find`, fixed by applying clippy's own suggestion).
- Fed this real evidence back into `~/hekton/gremlins/coaching/coachgremlin.md`: Status/Version updated, Review Trigger run count now 2 of the 3+ needed for v0 (alongside `terminal-velocity`'s Module 04 dry run — two different workshops, two different failure modes caught).
- Rewrote Module 01's README from skeleton to real authored content; updated `modules/README.md` and the top-level README's content-status banners to reflect Module 01 is real, Modules 02-08 are still skeleton.

### Decisions Made

- See `docs/decisions.md` for the full ADR log this session added.

### Risks

- This dry run graded its own constructed attempts, not an independent learner's — same limitation `terminal-velocity`'s own retro named for its Module 04 dry run.
- `runs/run-20260705-RW-001.yaml` has `human_confirmed: false` — this run's recommendation (go on authoring Modules 02-08 the same way) has not yet been reviewed by coderturtle.

### Next Actions

- See `docs/next-actions.md`. Immediate: coderturtle to review this dry run's recommendation; then author Modules 02-08 with the same dry-run discipline.

### Validation Status

- `cargo test` and `cargo clippy -- -D warnings`: run for real against both attempts and the takeaway-validation crate, transcripts captured, not narrated.
- `scripts/check-brand-lint.sh` and `scripts/check-mirror-drift.sh`: clean.

## 2026-07-05 - Pivot to one shared throughline project (`relay`)

### What happened

- coderturtle proposed building one real, usable project across all 8 modules instead of independent per-module toy fixtures, plus a specific angle: applying pomodoro-style pacing to hybrid human-agent teams. Explored the idea, corrected an early tagline risk ("Claude crack" reads as drug-adjacent wordplay), and checked for overlap with `agent-mission-control-lab` (an existing multi-agent cockpit in this factory).
- Resolved via three decisions: one throughline project (Terminal Velocity's shared-fixture pattern, reshaped for Rust's additive concept arc rather than copied as-is); product shape is a restartable-handoff CLI + hooks, narrower than the existing cockpit lab; tagline settled as "hybrid human-agent team pacing."
- Designed the full module-to-feature mapping (Module 01: core types + `finalize_session`; 02: borrowing-based session stats; 03: trigger/response enums; 04: `Notifier` trait; 05: config/IO error handling; 06: concurrent session tracking; 07: async checkpoint waiting; 08: a real seeded bug in the accumulated project as the capstone).
- Scaffolded `fixtures/relay/` (working name), wrote `SPEC.md` as the project's single source of truth, retired `modules/01-ownership-move-semantics/exercise/`, migrated its lesson into `relay`'s first feature.
- Re-ran the dry run (good + naive-honest attempts) against the migrated feature: **the original finding reproduces exactly** — both pass `cargo test` (5/5) and default `cargo clippy -- -D warnings` identically. Confirms the pivot didn't lose or weaken Module 01's content. This re-run is explicitly *not* counted as new evidence toward Coachgremlin's 3-run bar (same lesson, not a new failure mode) — that count stays at 2.
- Rewrote Module 01's README, `modules/README.md` (new "One shared project" section with the full feature table), and `docs/workshop-design.md` (new "The shared project: relay" section) to reflect the pivot. Added a build-log entry.

### Decisions Made

- See `docs/decisions.md` for the full ADR log this session added.

### Risks

- `runs/run-20260705-RW-002.yaml` has `human_confirmed: false` — not yet reviewed by coderturtle.
- Modules 02-08's features are still unbuilt; the module-to-feature mapping is a design, not yet evidenced per-feature the way Module 01's was.

### Next Actions

- See `docs/next-actions.md`. Immediate: coderturtle to review both Module 01 dry runs; then author Module 02's feature (session statistics) against `fixtures/relay/`.

### Validation Status

- `cargo test` and `cargo clippy -- -D warnings`: run for real against both the migrated good and naive attempts, transcripts captured.
- `scripts/check-brand-lint.sh` and `scripts/check-mirror-drift.sh`: clean.

### Mind-palace updated

Pending — see next commit's mirror sync.

## 2026-07-05 - Module 02 (Borrowing & References) authored, ARB trigger check written for real

### What happened

- Ran Coachgremlin Workflow step 0 for real before touching `fixtures/relay/`'s already-shipped shared files, and discovered `scripts/arb-trigger-check.sh` — claimed complete in `docs/decisions.md` and referenced in `.hekton/governance.yaml` as "verified firing correctly" — did not actually exist anywhere in the repo or git history. Wrote it for real (portable bash, matches the existing `scripts/*.sh` logging convention), verified it against a clean baseline and a real touch to `fixtures/relay/src/lib.rs` (fires correctly, reverts cleanly). Flagged the discrepancy in `docs/decisions.md` rather than quietly patching over it.
- Extended `relay`'s shared types for Module 02: added `elapsed_secs: u64` to `DraftCheckpoint`/`CheckpointRecord` (seconds since the previous checkpoint; `relay` never reads the system clock directly, keeping tests deterministic). This is an ARB-triggering change — confirmed the trigger fires, then resolved it by updating Module 01's already-shipped `tests/finalize_session.rs` to include the new field and re-confirming it still compiles and exhibits the same `todo!()`-stub behavior as before.
- Authored Module 02's real exercise: `session_stats(session: &Session) -> SessionStats`, computing average/longest checkpoint-gap statistics by borrowing `relay`'s session history rather than consuming it. Added `fixtures/relay/tests/session_stats.rs` (6 edge cases: empty session, single checkpoint, average-vs-last-vs-running-total, longest-gap-identifies-the-right-checkpoint, tied-longest-gap resolves to first occurrence, session still usable afterward).
- Ran Coachgremlin's first real dry run for Module 02: a correct attempt reading through the borrow (one legitimate clone, `longest_gap_goal`, since it must outlive the borrow) and a deliberately naive, honest attempt that clones the entire checkpoint collection into an owned copy before reading it.
- **Load-bearing finding, checked fresh rather than assumed to generalize from Module 01:** both attempts pass `cargo test` (11/11) and default `cargo clippy -- -D warnings` (zero output) identically. Checked `clippy::pedantic` (identical output on both attempts — zero discriminating signal, starker than Module 01, where pedantic at least caught a noisy proxy lint) and `clippy::nursery`'s `redundant_clone` specifically (clean on both — that lint targets owned-value clones, not clones of data reached through a reference). Full evidence: `runs/2026-07-05-module-02-dry-run/`.
- Packaged the takeaway (`.claude/skills/borrow-checker-playbook/SKILL.md`) and validated it against a second, unrelated borrowing problem (`total_word_count`, a pure read-only word-count aggregation with zero legitimate clones — a useful contrast case, since `session_stats` has exactly one).
- Rewrote Module 02's README from skeleton to real authored content; updated `modules/README.md`, top-level `README.md`, and `fixtures/relay/SPEC.md`'s status table to reflect Module 02 is real.

### Decisions Made

- See `docs/decisions.md` for the full ADR log this session added.

### Risks

- This dry run graded its own constructed attempts, not an independent learner's — same limitation named in every prior dry run's retro.
- `runs/run-20260705-RW-003.yaml` has `human_confirmed: false` — not yet reviewed by coderturtle.
- The first content-level Workshop Review Panel batch (Modules 01+02) is now due per Coachgremlin's own batch-review cadence, but was deliberately not run in this same pass (real-cost, ~7 parallel subagents) — left as an explicit next action rather than bundled in.

### Next Actions

- See `docs/next-actions.md`. Immediate: coderturtle to review this dry run's recommendation; run the first content-level Workshop Review Panel batch; then author Module 03 with the same dry-run discipline.

### Validation Status

- `cargo test` and `cargo clippy -- -D warnings`: run for real against both attempts and the takeaway-validation crate, transcripts captured, not narrated.
- `cargo clippy -- -W clippy::pedantic` and `-W clippy::nursery`: run for real for comparison, not assumed from Module 01's result.
- `scripts/arb-trigger-check.sh`: run for real against a clean baseline and a real touch to `fixtures/relay/src/lib.rs`.

### Mind-palace updated

Pending — see next commit's mirror sync.

## 2026-07-05 - Module 02 dry run confirmed (go); Review Trigger question resolved; first content-level Review Panel batch run

### What happened

- coderturtle reviewed Module 02's dry run and confirmed (go) — `runs/run-20260705-RW-003.yaml` flipped to `human_confirmed: true`.
- Resolved the open question of whether Module 02's dry run counts as a new data point toward Coachgremlin's "3+ runs across 2+ workshops" Review Trigger: decided it counts as **depth** evidence for the two-tier grading extension within one workshop, not new **breadth** toward the cross-workshop bar — the same distinction the Workshop Review Panel's own maturity note already draws for an identical-shaped bar. Run count toward that bar stays at 2 (`terminal-velocity`; `borrow-native`, counted once regardless of module count). Recorded in `~/hekton/gremlins/coaching/coachgremlin.md`'s Review Triggers section.
- Ran the first content-level Workshop Review Panel batch (Modules 01+02): 7 personas, real parallel subagents, no cross-visibility. All seven returned distinct findings. Two cross-persona agreements (a spoiler-placement issue in "Why this is hard," independently caught by End-User/Learner and Technical Writer; the "one correct clone" framing overstating what's true only pre-lifetimes, independently caught by AI/ML Practitioner and Skeptical Critic) plus 11 single-persona findings.
- Most significant single finding: Security-Conscious Reviewer caught a real bug in `scripts/arb-trigger-check.sh` (written earlier this session) — it always exited 0, even when a trigger fired, meaning it could never actually block anything gated on its exit code.
- Fixed nine findings in the same pass: the script's exit code and `git status --porcelain` parsing; added an explicit spoiler guard above "Why this is hard" in both modules (mirroring the existing Takeaway guard); softened the lifetime-scoped clone framing across Module 02's README/SPEC.md/lib.rs/SKILL.md; added the missing `cd fixtures/relay` instruction to both modules; softened `retro.md`'s "confirms" overclaim; fixed an em dash in `SPEC.md`; removed an internal `~/hekton/...` path from both modules' public-facing status blockquotes; reworded Module 02's third learning objective to match what the exercise actually exercises.
- Deferred three findings on the record (`docs/next-actions.md`): whether Module 02's stated Module-01 prerequisite should be mechanically enforced or reworded as conceptual-only (the panel found `session_stats.rs`'s tests never call `finalize_session`); trying a cheap deterministic clone-count check before Coachgremlin's subjective read, starting Module 03; harmonizing "gate"/"tier" terminology across README.md/modules/README.md/rubrics.

### Decisions Made

- See `docs/decisions.md` for the full ADR log this session added.

### Risks

- The three deferred panel findings are real, not hypothetical — most notably, Module 02's prerequisite claim currently overstates what the exercise itself enforces.
- Panel findings were triaged and fixed by the same agent that authored the content being reviewed, not an independent human editor — same limitation named in the panel's own Risks section for any single run.

### Next Actions

- See `docs/next-actions.md`. Immediate: author Module 03 (Structs, Enums & Pattern Matching) with the same dry-run discipline, factoring in the deferred panel findings (especially trying a cheap deterministic check before assuming Coachgremlin's subjective read is the only option).

### Validation Status

- `scripts/arb-trigger-check.sh`: fix verified against both a clean baseline and a real touch to `fixtures/relay/src/lib.rs` (fires, exits 1; clean case exits 0).
- `cargo check --tests` (fixtures/relay): clean after all doc-comment/wording edits.
- `scripts/check-brand-lint.sh`: clean after the em-dash fix.

### Mind-palace updated

Pending — see next commit's mirror sync.

## 2026-07-05 - Module 02 prerequisite resolved, clone-count-check trialed, Module 03 authored + dry run

### What happened

- **Resolved the deferred Module 02 prerequisite-enforcement decision: mechanical, not conceptual-only.** `fixtures/relay/tests/session_stats.rs` rewritten to build every `Session` via `finalize_session(label, drafts)` instead of a `Session { .. }` literal. Verified both directions: with `finalize_session` left at its unsolved `todo!()` stub, all 6 Module 02 tests now panic before `session_stats` runs; with a correct implementation temporarily restored, all 6 pass unchanged (11/11 total, clippy clean). Module 02's README status blockquote updated.
- **Built and trialed `scripts/clone-count-check.sh`**, the other deferred item: counts `.clone()` calls on added diff lines against a per-module expected-max baseline. First pass used Module 02's rubric prose to set that baseline to 1 — wrong: the good attempt itself has two call sites for one conceptual value (a running-max loop). Recalibrated against each module's own reference-implementation diff (0 for Module 01, 2 for Module 02), it then cleanly discriminated good from naive in both. Adopted as an optional pre-filter, not a gate substitute — full trial and captured output in `runs/2026-07-05-clone-count-check-trial/trial.md`. Fed back into `~/hekton/gremlins/coaching/coachgremlin.md`'s Workflow step 3.
- **Authored Module 03 (Structs, Enums & Pattern Matching):** `relay`'s `CheckpointTrigger`/`HumanResponse`/`NextAction` enums and `next_action`, an ARB-triggering but purely additive change to `src/lib.rs`/`SPEC.md` (resolved by confirming Modules 01-02's own tests were unaffected). Ran Coachgremlin's first real dry run: a correct attempt listing every `CheckpointTrigger` variant explicitly under `Ignored`, and a naive, honest attempt folding two of them into a `_` wildcard. Both pass `cargo test` (16/16) and default `cargo clippy -- -D warnings` identically — the deterministic gate can't tell them apart, a genuinely different mechanism from Modules 01-02's clone-shaped finding (no cloning involved at all). Sharper result than either prior module: `clippy::pedantic` doesn't just fail to help, it recommends the wrong direction (flags the good attempt's explicit arms via `match_same_arms`, suggesting the same collapse the naive attempt already made); the lint that actually catches the risk, `clippy::wildcard_enum_match_arm`, is off by default in clippy's `restriction` group. Independently reproduced on a second, unrelated order/refund-status example (`runs/2026-07-05-module-03-dry-run/takeaway-validation/`). The new clone-count-check pre-filter was also run here and correctly reported a true negative (this failure mode isn't clone-shaped).
- Takeaway packaged: `.claude/skills/enum-modeling-playbook/SKILL.md`, validated against the unrelated order/refund example before being called done.
- Module 03's README rewritten from skeleton to real content; `modules/README.md`'s content-status blockquote and takeaway table updated.

### Decisions Made

- See `docs/decisions.md` for the full ADR log this session added (Module 02 prerequisite enforcement, clone-count-check adoption, Module 03's finding).

### Risks

- Module 03's dry run used one session (this one) constructing and grading both attempts, same single-grader limitation named in Modules 01-02's retros.
- The `clippy::pedantic`-recommends-the-anti-pattern finding is one data point (now two, counting the unrelated validation example) — whether it generalizes beyond this specific "identical-body arms kept separate on purpose" shape isn't yet checked further.

### Next Actions

- See `docs/next-actions.md`. Immediate: get coderturtle's review of Module 03's dry run (`runs/run-20260705-RW-004.yaml`, currently `human_confirmed: false`), then author Module 04 (Generics, Traits & Lifetimes) with the same dry-run discipline.

### Validation Status

- `fixtures/relay`: `cargo test`/`cargo clippy -- -D warnings` run for real at every step (public stub, both Module 03 dry-run attempts at four lint levels, the takeaway-validation crate before and after its fix) — see `runs/2026-07-05-module-03-dry-run/grading.md` and `takeaway-validation/README.md` for full transcripts.
- `scripts/arb-trigger-check.sh --dry-run`: clean baseline confirmed before the Module 03 `lib.rs`/`SPEC.md` change, correct fire confirmed after.
- `scripts/clone-count-check.sh`: run against Modules 01-03's real dry-run diffs; output captured in `runs/2026-07-05-clone-count-check-trial/trial.md` and `runs/2026-07-05-module-03-dry-run/grading.md`.

### Mind-palace updated

Pending — see next commit's mirror sync.

## 2026-07-05 - Module 04 authored + dry run, second content-level Review Panel batch, a real correction to Module 03

### What happened

- **Authored Module 04 (Generics, Traits & Lifetimes):** added `Notifier` (trait + `DesktopNotifier`/`TerminalBellNotifier`/`StdoutNotifier`), `Session::label()` (an elision-eligible contrast case), `CheckpointAlert`, and `alert_checkpoint` to `fixtures/relay/src/lib.rs`/`SPEC.md` - an ARB-triggering but purely additive change, resolved by confirming Modules 01-03's own test suites still fail identically to before (5+6+5 tests, all still panicking on their own unsolved `todo!()`s).
- **Ran Coachgremlin's first real dry run for Module 04**, the first module to exercise explicit lifetime annotation: a correct attempt (`CheckpointAlert<'a>` borrowing `session_label: &'a str`, tying `'a` only to `session`), a deliberately naive, honest attempt (owned `String`, built via `.clone()` to sidestep the lifetime question), and a lighter third check for the module's other learning objective (unifying every reference parameter under one lifetime instead of scoping it to `session` alone). All three pass `cargo test` (22/22) and `cargo clippy -- -D warnings` identically. Checked further than any prior module: `clippy::pedantic`'s output is byte-for-byte identical between the correct and clone-avoidance attempts; the full `clippy::restriction` group (67 vs. 64 warnings) was also run and diffed by warning type, and its one difference (`single_char_lifetime_names`) flags the *correct* attempt, not the naive one. `scripts/clone-count-check.sh`, calibrated against the reference diff, correctly flagged the naive attempt - the first true positive this pre-filter has produced inside a module's own graded dry-run scoring. Takeaway packaged (`.claude/skills/trait-lifetime-cheatsheet/SKILL.md`) and validated against an unrelated generic config-store lookup problem.
- **Ran the second content-level Workshop Review Panel batch** (Modules 03+04, 7 real parallel-subagent personas), due once Module 04 completed the 2-3-module window. Most significant finding of any batch so far: the AI/ML Practitioner persona, by actually compiling the claim, found Module 03's "clippy::pedantic actively recommends the anti-pattern" finding was factually overstated - the suggested or-pattern merge preserves exhaustiveness (verified by adding a fourth enum variant and confirming both forms fail to compile identically). Corrected across `modules/03-structs-enums-pattern-matching/README.md`, both `runs/2026-07-05-module-03-dry-run/{grading.md,retro.md}`, `.claude/skills/enum-modeling-playbook/SKILL.md`, its `takeaway-validation/README.md`, `~/hekton/gremlins/coaching/coachgremlin.md`'s Module 03 entry, and appended (not silently rewritten) to the already-human-confirmed `runs/run-20260705-RW-004.yaml`.
- Eight more findings fixed same-pass: Module 04's "no lint anywhere"/"first true positive" claims re-verified (not just softened) by actually running the full `clippy::restriction` group and reconciling against the pre-adoption trial record; Module 03's LO1 given an honesty caveat; Module 04's dry-run-validated 7th rubric criterion (lifetime over-annotation) added to the shipped rubric; a leaked `~/hekton/...` path and the gate/scored/anti-gaming/structural vocabulary gap both fixed in `modules/README.md`; the arc table's Module 04 cell rewritten to name all three scored criteria; `scripts/arb-trigger-check.sh`'s silent-fail-open parser risk documented via comment; a "don't modify" note added to Module 04's given code.
- Two findings deferred, recorded in `docs/next-actions.md`: the rubric spoiling its own answer before the learner attempts it (a real tension against Coachgremlin's own rubric-sharing requirement); and the workshop's pages reading as audit reports rather than pitches (a workshop-wide structural question).
- Module 04's README rewritten from skeleton to real content; `fixtures/relay/SPEC.md`'s status table and `modules/README.md`'s content-status blockquote/arc table updated.

### Decisions Made

- See `docs/decisions.md` for the full ADR log this session added (Module 04's finding, the panel batch and its most significant correction).

### Risks

- Module 04's dry run used one session (this one) constructing and grading all three variants (good, naive-clone, naive-overannotated) - same single-grader limitation named in every prior module's retro.
- The rubric-spoils-the-answer tension (deferred this pass) means Module 03/04's "Valid alternate terminal" framing currently promises a discovery experience the Rubric section's own wording undercuts - flagged, not yet resolved.

### Next Actions

- See `docs/next-actions.md`. Immediate: get coderturtle's review of Module 04's dry run (`runs/run-20260705-RW-005.yaml`, currently `human_confirmed: false`), then author Module 05 (Error Handling) with the same dry-run discipline.

### Validation Status

- `fixtures/relay`: `cargo test`/`cargo clippy` run for real at every step, including all three Module 04 attempts across default/`pedantic`/`nursery`/full-`restriction` lint levels - see `runs/2026-07-05-module-04-dry-run/grading.md` for full transcripts.
- The Module 03 pedantic correction was independently re-verified, not just re-worded: compiled both the suggested or-pattern and the explicit-arms version after adding a new `CheckpointTrigger` variant, confirmed both fail identically (`E0004`).
- `scripts/arb-trigger-check.sh --dry-run`: clean baseline confirmed before the Module 04 `lib.rs`/`SPEC.md` change, correct fire confirmed after; still fires correctly after the comment-only limitation note was added.
- Shipped stub final state confirmed: `cargo build` clean, all four modules' test suites (6+5+5+6) fail identically to their pre-session state (still panicking on their own unsolved `todo!()`s).

### Mind-palace updated

Pending — see next commit's mirror sync.

## 2026-07-05 - Module 04 dry run confirmed; scoped the Certification Alignment Retrofit phase

### What happened

- **coderturtle reviewed and confirmed Module 04's dry run (go)** - `runs/run-20260705-RW-005.yaml` flipped to `human_confirmed: true`; `.hekton/change-log.yaml`'s CHG-0004 (Module 04 authoring) and CHG-0005 (Modules 03+04 Review Panel batch) approvals flipped to `approved`. Committed.
- **Scoped a deferred Certification Alignment Retrofit phase**, at coderturtle's direction, ahead of authoring Module 05: map each module to the real Ardan Labs exam's structure, and give a learner's harness an interactive way to verify and tick off progress - but not built now, since the only Ardan topic information available today is a four-category marketing-page list, not the exam's actual structure. Added a full-reasoning section to `docs/workshop-design.md`, expanded `docs/next-actions.md`'s existing "sit the exam" item into an explicit two-step phase (Step 1: sit the exam, record a retrospective - topic coverage/confidence, format notes, explicitly not verbatim exam content; Step 2: once that retrospective exists and all 8 modules + capstone are authored, build the real mapping and tracker), and logged the decision in `docs/decisions.md`.
- Created two skeleton files, decided shape but no real content, mirroring the same idiom Modules 05-08 already use for themselves: `docs/certification-mapping.md` (the mapping doc, with today's four placeholder categories explicitly marked pre-exam/unverified) and `.claude/skills/certification-tracker/SKILL.md` (decided mechanism: re-run a module's own deterministic gate before ticking off its mapped topics in a learner-local progress file, never a self-reported checkbox). Added `.cert-progress.local.yaml` to `.gitignore`.

### Decisions Made

- See `docs/decisions.md`'s newest row for the full reasoning.

### Risks

- None new - this session added no real content claims, only a scoped, honestly-labeled skeleton for a phase gated on real future evidence (the exam retrospective).

### Next Actions

- See `docs/next-actions.md`. Immediate: author Module 05 (Error Handling) with the same dry-run discipline. The certification retrofit phase's Step 1 (sitting the exam) can happen independently, whenever scheduled - it isn't blocking Module 05.

### Validation Status

- No code changed this session (docs/skeletons only); `fixtures/relay`'s state is unchanged from the prior session's confirmed-clean build.

### Mind-palace updated

Pending — see next commit's mirror sync.

## 2026-07-05 - Module 05 (Error Handling) authored + dry run

### What happened

- **Ran Coachgremlin's Workflow step 0 (ARB check)** before touching `fixtures/relay/src/lib.rs`/`SPEC.md`: clean baseline confirmed, correct fire after adding Module 05's given code and stubs, resolved by confirming `cargo build` succeeds and Modules 01-04's own tests fail identically to before the change.
- **Authored Module 05's real exercise**: `parse_config(input: &str) -> Result<RelayConfig, ConfigError>` and `write_handoff_summary(path: &Path, session: &Session) -> Result<(), HandoffError>`, added to `fixtures/relay/src/lib.rs`/`SPEC.md` with 10 new tests (`tests/parse_config.rs`, `tests/write_handoff_summary.rs`). Real requirement surfaced immediately: `std::error::Error: Debug + Display` meant `ConfigError`/`HandoffError`/`RelayConfig`/`NotifierKind` all needed `#[derive(Debug)]`, not obvious from the given code alone until `cargo build` actually failed on it.
- **Ran the first real dry run for Module 05** (`runs/2026-07-05-module-05-dry-run/`): a correct attempt (`HandoffError::Io` wrapping the real `std::io::Error`, `From`/`source()` added), a deliberately naive attempt (keeps the shipped `Io(String)`, stringifies the I/O error the instant it's caught), and a lighter second check (manual `match` instead of `?` in `parse_config`). Both primary attempts pass `cargo test` (32/32) and `cargo clippy -- -D warnings` identically - checked at `pedantic`/`nursery`/full `restriction` (diffed programmatically via `--message-format=json`), finding one weak generic partial signal and one wrong-direction lint, same pattern as Module 04. The lighter check found the opposite: default `clippy::question_mark` genuinely fails a same-type `Result -> Result` manual match, the first time default clippy alone has rescued part of a naive shape in this workshop.
- **Packaged and validated the takeaway**: `.claude/skills/custom-error-type-template/SKILL.md`, validated against a second, unrelated retry-policy-config-parsing problem (`runs/2026-07-05-module-05-dry-run/takeaway-validation/`) - the validation refined the Skill's own `question_mark` claim (also silent on a type-converting manual match, not just same-type passthroughs) rather than just confirming it.
- **Rewrote `modules/05-error-handling/README.md`** from skeleton to real authored content: rubric, learning objectives, and a "why this is hard" section reflecting this dry run's actual evidence.
- **Decided against adding `thiserror` as a dependency** for the error types - `.hekton/governance.yaml` gates dependency changes as human-required, and this didn't warrant an unscoped mid-session addition. Manual `impl Error`/`Display` used instead; recorded in `docs/decisions.md`.
- Recorded `runs/run-20260705-RW-006.yaml` (`human_confirmed: false`), `.hekton/change-log.yaml`'s CHG-0006, and a new `.hekton/agent-run-log.yaml` entry (RUN-0006).

### Decisions Made

- See `docs/decisions.md`'s two newest rows (the `thiserror`-vs-manual choice, and the full Module 05 finding).

### Risks

- Same single-grader limitation as every prior module's dry run: one session constructed and graded both attempts plus the extra check. Recorded in `retro.md`, not new here.
- Recommendation only - `human_confirmed: false`, awaiting coderturtle's review per Coachgremlin's Human Gate.

### Next Actions

- See `docs/next-actions.md`. Immediate: get coderturtle's review on Module 05's dry run, then author Module 06 (Fearless Concurrency) with the same discipline. Batch-review cadence not due yet (1 of 2-3 modules since the Modules 03+04 batch).

### Validation Status

- `cargo build`/`cargo test`/`cargo clippy -- -D warnings` all run for real, output captured in `runs/2026-07-05-module-05-dry-run/*/fixture/transcript.txt`, not narrated.
- Modules 01-04's own test suites re-run before and after the `src/lib.rs` addition, identical still-failing-on-`todo!()` output confirmed (no regression).

### Mind-palace updated

Pending — see next commit's mirror sync.

## 2026-07-05 - Module 05 dry run confirmed; `thiserror` authorized and migrated; CLAUDE.md dependency-flagging rule added

### What happened

- **coderturtle reviewed and confirmed Module 05's dry run (go)** - `runs/run-20260705-RW-006.yaml` flipped to `human_confirmed: true`.
- **coderturtle authorized adding `thiserror`** as a real Cargo dependency, reversing the prior session's manual-`impl` choice (which was gated on approval, not a design objection). Ran `cargo add thiserror` in `fixtures/relay/`; migrated `ConfigError`/`HandoffError` from a manual `impl Error`/`Display` to `#[derive(thiserror::Error)]` with `#[error("...")]` attributes; the correct-attempt reference shape for `HandoffError::Io` simplified to `Io(#[from] std::io::Error)`, one attribute replacing a manual `impl From` plus `source()` override.
- **Re-verified the module's finding against the migrated code, not assumed to carry over**: rebuilt both the correct and naive attempts against the `thiserror`-derived given code (in a scratch copy, then reverted `fixtures/relay/src/lib.rs` exactly back to its shipped `todo!()` stub afterward, diffed to confirm). Primary finding held (`cargo test` 32/32, `cargo clippy -- -D warnings` identical on both). One secondary claim did **not** survive unchanged: the original dry run's `clippy::restriction`-group finding (`clippy::missing_trait_methods` giving one weak partial signal) disappears entirely once the `Error` impl is derived rather than hand-written (0 occurrences on both attempts, re-checked via `--message-format=json`). Corrected across `modules/05-error-handling/README.md`, `.claude/skills/custom-error-type-template/SKILL.md`, and addenda appended to `runs/2026-07-05-module-05-dry-run/{grading.md,retro.md}` (not silently rewritten) - the finding is now stated as *starker* (no lint at any level rescues it) than originally recorded.
- **Added a "Dependency changes" rule to this repo's `CLAUDE.md`**: future sessions must flag dependency questions to the user explicitly, naming the cons of *not* adding the dependency, rather than deciding unilaterally around a governance gate.
- `fixtures/relay/SPEC.md` and `modules/05-error-handling/README.md`'s given-code blocks updated to show the `thiserror`-derived shape.

### Decisions Made

- See `docs/decisions.md`'s newest row.

### Risks

- None new. The re-verification found a claim that needed correcting (a lint signal that no longer applies), which was corrected rather than left stale - the same discipline this workshop has applied to every prior tooling-behavior claim.

### Next Actions

- Author Module 06 (Fearless Concurrency) with the same dry-run discipline.

### Validation Status

- `cargo build`/`cargo test`/`cargo clippy -- -D warnings` all re-run for real against the `thiserror`-migrated shipped stub; 32/32 tests still correctly fail on their own unsolved `todo!()`s (no regression from the dependency migration).
- The scratch verification (both attempts rebuilt, tested, linted, then reverted) confirmed `fixtures/relay/src/lib.rs` was restored byte-for-byte to its shipped stub state (`diff` empty) before this session ended.

### Mind-palace updated

Yes — repo-local `mind-palace/20-projects/factory-output/borrow-native/{decisions,next-actions,session-log}.md` synced this commit, covering every session back to the last sync point (Modules 01+02 Review Panel batch). Condensed, not verbatim - see `docs/decisions.md`/`docs/next-actions.md`/`docs/session-log.md` in this repo for full per-decision detail. Live vault untouched (`vault_mutation_allowed: false`).

## 2026-07-05 - Rubric-spoiler tension decided and implemented; agentic-learning-discipline Skill added

### What happened

- **Decided the rubric-spoiler tension**, following up on the same-day research
  (`docs/rubric-spoiler-research.md`): adopted Candidate A (property-phrased rubric criteria -
  state an observable fact about the finished artifact, not the technique that produces it)
  plus all three items from that doc's Proposal section. Reasoning against B/C/D recorded in
  the research doc's new "Decision" section and `docs/decisions.md`'s newest row.
- **Rewrote every scored/conceptual rubric criterion in Modules 01-05** to property-phrased
  form - e.g. Module 04 criterion 6 went from "borrows `session` with an explicit lifetime,
  rather than owning a clone" to "it is a compile error for a `CheckpointAlert` to be used after
  the `Session` it describes has been dropped." Also reworded each module's "Required to
  advance" line to match, since it named the same technique directly. Left "Valid alternate
  terminal" sections and "Why this is hard" as-is - those are explicitly framed as the
  after-a-first-attempt reveal, not pre-attempt spoilers, and this decision didn't touch that
  framing.
- **Added a short, ungated reflective closing beat to every module** ("if you'd handed this to
  your agent with no attempt first, it would've one-shotted the answer - what did *you* learn
  versus what did your agent demonstrate?"), right after each module's "Why this is hard"
  section. Not scored, not gated - a habit, not a criterion.
- **Created `.claude/skills/agentic-learning-discipline/SKILL.md`**: a cross-cutting,
  non-scored self-check (name the concept without looking at the code; predict whether the diff
  passes `cargo clippy` before running it; name how a plausible naive attempt would have gotten
  it wrong) - referenced from every module's reflective beat and from `modules/README.md`'s
  takeaway table, not owned by any single module.
- **Added a new subsection to `docs/workshop-design.md`** ("A second, named skill running
  underneath the Rust arc") naming this as an explicit, workshop-wide second thing the workshop
  teaches, not just a rubric patch.
- Historical `runs/*/grading.md`/`retro.md` files were **not** rewritten - the underlying
  pass/fail distinction each dry run already tested doesn't change when the rubric's wording
  does; only what a future learner reads before attempting is different.

### Decisions Made

- See `docs/decisions.md`'s newest row for the full reasoning.

### Risks

- The property-phrased criteria are a first attempt at this shape across 5 already-shipped
  modules - not independently reviewed yet. Added to `docs/next-actions.md`: get the third
  content-level Review Panel batch (due once Module 06 or 07 completes the window) to
  sanity-check the new phrasing, same as any other new pattern this workshop has adopted.
- This decision explicitly does not claim the agentic-short-circuit problem is solved - see the
  research doc's "Decision" section's own "What this does not claim" paragraph. It's a real,
  bounded improvement (more honest rubric wording, an explicit acknowledgment of what wording
  alone can't fix), not a closed case.

### Next Actions

- Apply the same property-phrasing + reflective-beat pattern to Modules 06-08 as they're
  authored, rather than letting the old spoiler-shaped pattern creep back in.
- Author Module 06 (Fearless Concurrency) next, same dry-run discipline.
- Get the third Review Panel batch's read on the new rubric pattern once it's due.

### Validation Status

- No code changed this session (rubric/doc wording and a new Skill file only);
  `fixtures/relay`'s state is unchanged, no `cargo test`/`clippy` re-run needed.
- `scripts/check-brand-lint.sh` and `scripts/check-mirror-drift.sh` both run clean after all
  edits.

### Mind-palace updated

Pending — see next commit's mirror sync.
---

## Session: Module 05 authored + thiserror migration + rubric-spoiler tension decided

**Date:** 2026-07-05 21:06

### What Changed

Authored Module 05 (parse_config/write_handoff_summary, custom error types), authorized and migrated to thiserror, then decided the rubric-spoiler tension: adopted property-phrased rubric criteria plus a workshop-wide reflective closing beat and agentic-learning-discipline Skill across Modules 01-05.

### Decisions

Adopted Candidate A (property-phrased criteria) over B/C/D; full reasoning in docs/rubric-spoiler-research.md's Decision section. Explicitly does not claim the agentic-short-circuit problem is fully solved.

### Assumptions

*(none)*

### Risks

Property-phrased criteria are a first attempt at this shape across 5 already-shipped modules, not yet independently reviewed - flagged for the next Workshop Review Panel batch.

### Next Actions

- [ ] Author Module 06 with the new rubric pattern
- [ ] Get the third Review Panel batch's read on the new phrasing once due

## 2026-07-08 - Vulnerability Gremlin's second real run (RISK-0002/RISK-0003)

Ran the new Vulnerability Gremlin (`~/hekton/gremlins/red-team/vulnerability-gremlin.md`, born
earlier today from `half-life`'s RISK-0002 triage) for real against this project - its second run,
and first on the cargo/Rust ecosystem, per that Gremlin's own Follow-Up Actions requiring a
second-ecosystem run before it could move past `draft` status.

Installed `cargo-audit` via Homebrew. Ran it against `fixtures/relay/Cargo.lock`: 0 vulnerabilities
across 7 crates, 1,159 RustSec advisories loaded - a clean baseline, recorded as RISK-0003.

Ran `npm audit` against `site/`'s dependencies: same 4 findings `half-life` already triaged.
Discovered RISK-0002 had been flagged during this project's own scaffolding (2026-07-05) but never
closed, and `.hekton/risk-register.yaml` had silently drifted from `docs/risks.md` - missing the
entry entirely. Checked reachability directly against this project's own code (not copied from
`half-life`'s conclusion): no `define:vars`/server-island usage, `output: "static"`, CI only runs
`npm run build`. Confirmed via direct lockfile inspection that both projects resolve to identical
dependency versions (`astro@5.18.2`, `@astrojs/tailwind@6.0.2`), so `half-life`'s already-verified
upgrade-attempt finding (breaks on `@astrojs/tailwind`/Astro 7) transfers exactly - did not
re-attempt the broken upgrade here, since re-deriving an already-verified fact from an identical
lockfile isn't more rigorous, just redundant. Closed RISK-0002 as accepted risk; fixed the
registry-vs-markdown drift in the same pass.

### Decisions Made

- See `docs/decisions.md`'s 2026-07-08 entry.

### Risks

- RISK-0002: closed, accepted (not fixed) - see `docs/risks.md` for full reachability reasoning.
- RISK-0003: closed, informational - clean `cargo audit` baseline for `fixtures/relay/`.

### Next Actions

- Author Module 06 (Fearless Concurrency) next, per the existing plan - unaffected by this session.
- Re-run `cargo audit` against `fixtures/relay/` as its dependency tree grows across later modules.

---

## 2026-07-18 - Fixed a fully-broken custom domain, found while building coderturtle.io's Workshops page

**Agent:** Claude

### What changed

- Building a Workshops listing page on `coderturtle.io` surfaced a real, live bug: `curl` against
  `borrow-native.coderturtle.io` returned 404 on every path (not a cert issue — checked over plain
  HTTP too), and the `coderturtle.github.io/borrow-native/` fallback just 301-redirected back into
  the same 404. No working URL existed for this workshop at all.
- Root cause: DNS (Route53 CNAME + TXT) and GitHub Pages' own `cname`/domain-verification were
  already live and `"verified"` (from `agentic-infra-lab`'s `github-pages-dns` onboarding), but
  `site/astro.config.mjs` still had `site: "https://coderturtle.github.io"` / `base:
  "/borrow-native/"` and no `site/public/CNAME` existed — the repo-side half of the cutover
  `terminal-velocity`/`closed-book` already went through was never done here.
- Fixed on branch `agent/claude/custom-domain-cutover`: `astro.config.mjs` (`site`/`base` to the
  domain root), new `site/public/CNAME`, `.github/workflows/deploy-pages.yml`'s stale "no custom
  domain" comment corrected. `npm run build` reconfirmed clean, `dist/CNAME` present.

### Decisions Made

See `docs/decisions.md`'s 2026-07-18 entry.

### Assumptions

None new.

### Risks

No new RISK entry — a real, currently-broken deploy fixed same session, not a residual risk.

### Next Actions

Review/merge the cutover PR, then trigger the first real deploy under the corrected config and
confirm the site is actually live (not just that the workflow reports success).

### Validation

- Real `curl` checks (custom domain, plain HTTP, insecure-TLS, and the project-page redirect
  target) confirmed the break before fixing, not assumed from DNS/Pages state alone.
- `npm run build` clean; `dist/CNAME` = `borrow-native.coderturtle.io`.

### Mind-palace updated

No — not yet authorised this session.
---

## Session: Fixed a live, fully-broken custom domain and first-ever deploy

**Date:** 2026-07-18 20:58

### What Changed

borrow-native.coderturtle.io was 404ing on every path (DNS/Pages config live, site-side cutover never done) - found while building coderturtle.io's Workshops page. Fixed astro.config.mjs/public/CNAME, triggered and confirmed the first real deploy, enabled the push trigger

### Decisions

See docs/decisions.md's 2026-07-18 entry

### Assumptions

None new

### Risks

None new - a real, currently-broken deploy fixed same session

### Next Actions

- [ ] Author Module 06 (Fearless Concurrency)
- [ ] continuing the existing content plan - unaffected by this session's infra fix
