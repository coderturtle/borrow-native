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
- [ ] Author Module 02 (Borrowing & References) next: `relay`'s session-statistics feature, same dry-run discipline (construct at least one honest-naive attempt per module, don't assume Module 01's finding generalizes without checking).
- [ ] Re-run the Workshop Review Panel once several modules have real content (design-doc-only findings can't check exercise design or grader trust)
- [ ] Build the capstone's real seeded bug into `relay` once Modules 02-07 have added enough features for a genuine multi-concept diagnosis to be possible

## Later

- [ ] **coderturtle to sit the real Ardan Labs Rust certification exam** ($99, 100 MCQ, 90 min, 80% to pass) once module content exists, as this workshop's own dogfooding evidence — record the real result (pass/fail, which topics felt under-covered) in the build log regardless of outcome
- [ ] Register this workshop and its Workshop-Gremlin/Coachgremlin augmentation in the mind-palace Gremlin Registry once vault mutation is explicitly authorised for that purpose
- [ ] Decide whether a custom domain (matching `terminal-velocity.coderturtle.io`'s pattern) is wanted for this workshop too, once it has real content to point a domain at
- [ ] A third, differently-shaped-again workshop is still needed before the Workshop Gremlin, the Review Panel, or Coachgremlin's two-tier grading extension can claim v1 stability (see `~/hekton/gremlins/workshop/workshop-gremlin.md`)
