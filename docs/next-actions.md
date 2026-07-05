# Next Actions: Borrow Native

## Immediate

- [x] Naming pass complete: **Borrow Native**
- [x] Workshop Review Panel first run complete against naming + design docs
- [x] Module skeleton (8 modules + index, after adding Module 07 Async Programming), brand layer, README rework
- [x] Astro build-log/Pages site built and locally validated (`npm run build` + `astro check` clean, base-path links confirmed in built HTML)
- [x] Certification-companion target corrected from the Rust Foundation (no individual credential exists) to Ardan Labs' real proctored exam; arc checked against its topic list, which found the Module 07 (Async) gap
- [ ] Get a human to enable GitHub Pages (Settings > Pages > Source: GitHub Actions) and trigger the first real `workflow_dispatch` deploy, then confirm the Actions run actually succeeds (not just the local build)
- [ ] Triage RISK-0002 (4 inherited `npm audit` vulnerabilities in the Astro starter chain) before that first real deploy

## This Week

- [x] Coachgremlin content-building pass: author Module 01 (Ownership & Move Semantics), real exercise + fixture + rubric + dry run, matching `terminal-velocity`'s own first-module dry-run discipline. Real finding: default clippy can't distinguish a correct move-based solution from a naive clone-heavy one; only Coachgremlin's conceptual tier can. See `runs/2026-07-05-module-01-dry-run/`.
- [x] Pivoted to one shared throughline project (`fixtures/relay/`, a restartable-handoff CLI for hybrid human-agent team pacing) instead of eight independent toy fixtures. Retired `merge_customer_totals`, migrated its lesson into `relay`'s first feature (`finalize_session`), re-confirmed the same finding transfers. See `docs/workshop-design.md`'s "The shared project: relay" section and `fixtures/relay/SPEC.md`.
- [x] coderturtle reviewed both Module 01 dry runs and confirmed (go) — `runs/run-20260705-RW-001.yaml` and `runs/run-20260705-RW-002.yaml` both `human_confirmed: true`, 2026-07-05.
- [x] Added a scoped ARB trigger for `fixtures/relay/src/lib.rs`/`SPEC.md` (`.hekton/governance.yaml`, `review_type: governance`) and a matching Coachgremlin Workflow step 0 — check for ARB triggers before touching the shared project's already-shipped files. Verified firing correctly.
- [x] Confirmed Coachgremlin didn't reuse the Workshop Review Panel (persona panel) at all — only had the mechanical ARB check. Added Workflow step 7: full 7-persona panel every 2-3 modules of real content, batched for cost reasons per the panel's own Risks section. Updated `workshop-review-panel.md` (Coachgremlin as a second invoker) and `workshop-lifecycle.md` (gives the previously-abstract "quality gate both phases can invoke" a concrete trigger).
- [x] Author Module 02 (Borrowing & References): `relay`'s session-statistics feature (`session_stats`, borrowing session history rather than consuming it). Added `elapsed_secs` to `DraftCheckpoint`/`CheckpointRecord` (an ARB-triggering change to relay's shared types, resolved per Coachgremlin Workflow step 0). Real finding, checked fresh rather than assumed from Module 01: default clippy can't distinguish a correct borrowing-based solution from one that defensively clones the whole borrowed checkpoint collection first — and `clippy::pedantic` gives *zero* help here, unlike Module 01. See `runs/2026-07-05-module-02-dry-run/`.
- [x] Found and fixed a real gap while doing the above: `scripts/arb-trigger-check.sh` was claimed to exist and to have been verified, but wasn't in the repo. Written for real this session; `docs/decisions.md` corrected.
- [x] coderturtle reviewed Module 02's dry run and confirmed (go) — `runs/run-20260705-RW-003.yaml` flipped to `human_confirmed: true`, 2026-07-05.
- [x] Resolved the open Review Trigger question: Module 02's dry run is depth evidence for the two-tier grading extension (within one workshop), not a new breadth data point toward Coachgremlin's "3+ runs across 2+ workshops" bar — same distinction the Workshop Review Panel's own maturity note already draws. Run count toward that bar stays at 2. See `docs/decisions.md` and `~/hekton/gremlins/coaching/coachgremlin.md`'s Review Triggers section.
- [x] **First content-level Workshop Review Panel batch, complete** (Modules 01+02) — `docs/review-panel/2026-07-05-modules-01-02-content.md`. Two cross-persona agreements (a spoiler-placement issue; the "one correct clone" framing overstating what's true only pre-lifetimes) and 11 single-persona findings, including a real bug in `scripts/arb-trigger-check.sh` (always exited 0, even on a fire). Nine findings fixed same-pass (script exit code + parsing, spoiler guards, softened clone framing, missing `cd fixtures/relay` instruction, retro.md overclaim, em dash, internal-path leak, Module 02 LO3 wording); three deferred below.
- [ ] **Decide Module 02's Module-01 prerequisite: mechanical or conceptual-only?** Panel found `fixtures/relay/tests/session_stats.rs` never calls `finalize_session` — a learner can complete Module 02 having never touched Module 01's artifact, despite the stated "hard prerequisite." Either make `session_stats.rs` depend on a working `finalize_session` (real tradeoff: Module 02 becomes ungradable until Module 01 is solved) or reword the prerequisite claim to be honest that it's conceptual, not enforced.
- [ ] **Try a cheap deterministic check before Coachgremlin's subjective read, starting Module 03.** Panel noted a scripted clone-count check on the diff would have mechanically discriminated both dry runs' good/naive attempts — worth attempting as a first-pass deterministic signal before assuming only the conceptual tier can catch it.
- [ ] **Harmonize "gate"/"tier" vocabulary** across `README.md`, `modules/README.md`, and per-module rubrics — three different phrasings currently, despite `modules/README.md` claiming one shared vocabulary.
- [ ] Author Module 03 (Structs, Enums & Pattern Matching) next: `relay`'s `CheckpointTrigger`/human-response enums, same dry-run discipline (construct at least one honest-naive attempt, don't assume Modules 01-02's finding generalizes without checking).
- [ ] Build the capstone's real seeded bug into `relay` once Modules 03-07 have added enough features for a genuine multi-concept diagnosis to be possible

## Later

- [ ] **coderturtle to sit the real Ardan Labs Rust certification exam** ($99, 100 MCQ, 90 min, 80% to pass) once module content exists, as this workshop's own dogfooding evidence — record the real result (pass/fail, which topics felt under-covered) in the build log regardless of outcome
- [ ] Register this workshop and its Workshop-Gremlin/Coachgremlin augmentation in the mind-palace Gremlin Registry once vault mutation is explicitly authorised for that purpose
- [ ] Decide whether a custom domain (matching `terminal-velocity.coderturtle.io`'s pattern) is wanted for this workshop too, once it has real content to point a domain at
- [ ] A third, differently-shaped-again workshop is still needed before the Workshop Gremlin, the Review Panel, or Coachgremlin's two-tier grading extension can claim v1 stability (see `~/hekton/gremlins/workshop/workshop-gremlin.md`)
