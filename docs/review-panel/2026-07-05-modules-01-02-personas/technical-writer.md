# Professional Technical Writer — clarity, structure, consistency

**Lens:** Clarity, structure, terminology consistency, internal consistency (cross-references resolve, no contradictions). Ignored whether the underlying ideas/pedagogy are good — only whether they're said well.

**Reviewed:** top-level `README.md`, `modules/README.md`, Modules 01+02 READMEs, `fixtures/relay/SPEC.md`, last ~10 rows of `docs/decisions.md`.

---

# Technical Writing Review — Modules 01 & 02 (Borrow Native)

**Cross-references:** All checked paths resolve — `.claude/skills/ownership-move-checklist/SKILL.md`, `.claude/skills/borrow-checker-playbook/SKILL.md`, all cited `runs/2026-07-05-module-0{1,2}-*-dry-run/` paths, `modules/03-.../README.md`, `docs/next-actions.md`, `docs/workshop-design.md`. No broken links found.

**Top finding — the spoiler isn't in "Takeaway," it's one section earlier.** Both modules dress their Takeaway section with an identical, careful guard: "Don't open this Skill before your first attempt: it names the diagnosis directly." But that guard is undermined by "Why this is hard, and what actually turned out to matter," which sits *before* it and gives away the exact answer. Module 01 states outright: "the clone-heavy one iterates `for draft in &drafts`... The move-based one just... doesn't borrow." Module 02 does the same, quoting the naive solution's literal code (`session.checkpoints.iter().map(|c| CheckpointRecord { goal: c.goal.clone(), ... })`). A learner who reads top-to-bottom (the natural order) hits the diagnosis in prose before ever attempting the exercise. The Takeaway guard is well-written but structurally misplaced — protecting the wrong section.

**Terminology drift, three ways to say the same thing.** Top-level `README.md` calls the two checks a "deterministic gate" and a "conceptual check" (never "tier"). `modules/README.md` calls them "deterministic tier"/"conceptual tier" and headlines a table "Gate tiers (**every module uses this vocabulary**)" — a claim the top README itself doesn't honor. Module 01/02 rubric items use yet a third form: tags like "(gate, deterministic)" and "(scored, conceptual)."

**Internal paths leaking into public learner content.** `docs/maintainers.md` states module READMEs are learner-facing and follow brand voice. Yet both modules end with a status blockquote citing `~/hekton/gremlins/coaching/coachgremlin.md` — a path meaningless (and inaccessible) to anyone outside the Hekton system, in a repo marked `privacy boundary: public`.

**Brand rule violation:** `fixtures/relay/SPEC.md:24` contains a literal em dash ("reached — a direct implementation"), violating `docs/brand.md`'s "no em dash characters" rule. SPEC.md is directly required reading from both module READMEs, so its exemption status (design doc vs. published content) is undefined and should be.

Module 01 and 02 otherwise share identical section order, heading names, and voice — good internal consistency between the two; the drift is in how they relate to the meta-docs above them.
