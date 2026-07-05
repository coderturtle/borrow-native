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
