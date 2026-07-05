# Professional Technical Writer — Modules 03+04 (2026-07-05)

Confirmed the wiki-links resolve to real skill files.

## Findings (Professional Technical Writer lens)

**1. Direct contradiction between Module 03 and Module 04 on review status, plus a dangling link.** Module 03's README (line 184-185) states: *"Not yet reviewed by the Workshop Review Panel: Module 03 is the first module of content since the Modules 01+02 batch... not yet due."* Module 04's README (line 240-242), authored the same day, states the opposite as settled fact: *"Reviewed by the Workshop Review Panel as part of the Modules 03+04 batch: `docs/review-panel/2026-07-05-modules-03-04-content.md`."* That file does not exist yet — it's a link to a report this very panel is producing, cited in the past tense as if already filed. Module 04's own `grading.md` (line 145-148) gets the tense right — "due now... the panel run this triggers" (future/anticipatory) — so the README overclaims relative to its own source-of-truth run doc. This is the same species of bug the first batch flagged (contradictory status phrasing across files).

**2. Leaked internal path recurs, uncaught by the batch-1 fix.** `modules/README.md:88` — *"See `docs/workshop-design.md`... and `~/hekton/gremlins/coaching/coachgremlin.md`'s Workflow step 3."* This is the exact class of issue batch 1 found and fixed elsewhere (an internal `~/hekton/...` path in a public-repo file). It's still present in this in-scope file, so either it was missed in the earlier fix pass or reintroduced since.

**3. Rubric-tag vocabulary has silently forked from the "shared vocabulary" table.** `modules/README.md`'s "Gate tiers (every module uses this vocabulary)" section (lines 81-88) defines exactly two tiers: **Deterministic** and **Conceptual**. But Module 03 and 04's rubrics tag criteria with four distinct labels never introduced there: `(gate, deterministic)`, `(gate, anti-gaming)`, `(scored, conceptual)`, `(scored, structural)` (e.g., Module 04 rubric items 3, 5, 6). "Gate" vs "scored" as a cross-cutting axis, and "anti-gaming"/"structural" as tier names, appear only inside the module READMEs and their `grading.md` files — never reconciled with the canonical table that explicitly claims universal usage. This is the same category of defect batch 1 flagged for "deterministic gate" phrasing drift, just recurring in a new spot.

**4. Arc-table summary under-describes module detail.** `modules/README.md`'s arc table gives Module 04 one Coachgremlin check ("confirms no `'static`/`.clone()` used to silence a lifetime error"), but Module 04's own rubric has two independently scored conceptual/structural criteria (5: generic vs. `&dyn`; 6: borrow vs. clone). Not a contradiction, but an incompleteness that could mislead a reader skimming only the summary table.

No em-dash violations and no other broken cross-references found — all `runs/`, `.claude/skills/`, and `docs/next-actions.md` links in the reviewed files resolve to real paths, and the `[[wiki-links]]` inside both SKILL.md files point to skills that exist.

## Disposition

**Finding 1: self-resolving.** README's forward reference becomes true once this synthesis report is filed at the cited path in this same session — no separate fix needed.

**Finding 2: fixed.** `modules/README.md`'s Gate tiers section reworded to describe the file by its Hekton-repo-relative path (`gremlins/coaching/coachgremlin.md` in the operating-model repo) instead of the `~/hekton/...` absolute path.

**Finding 3: fixed.** Extended `modules/README.md`'s "Gate tiers" section with a paragraph naming the gate/scored axis and the anti-gaming/structural sub-tags explicitly, so the table's "every module uses this vocabulary" claim is actually true of what the rubrics use.

**Finding 4: fixed.** Module 04's arc-table cell rewritten to name all three of its scored/conceptual criteria (generic-not-dyn, no clone-to-avoid-lifetime, no over-annotation), not just one.
