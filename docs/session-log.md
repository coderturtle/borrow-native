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

### Mind-palace updated

Pending — see next commit's mirror sync.
