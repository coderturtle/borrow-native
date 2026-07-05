# Instructional Designer — Modules 03+04 (2026-07-05)

## Top findings (Instructional Designer lens)

**1. Module 03's first learning objective is never gated — it's demonstrated, not exercised.** The README lists as LO1: "Recognize when a `bool`/`Option` flag on a struct is really a hidden enum trying to get out." But the module's own text says the opposite happened: "`fixtures/relay/src/lib.rs` already declares three enums for you... That's this module's real lesson, **already made for you** in the type design." The dry-run grading.md confirms it plainly: "The three enums themselves... are given in the shipped stub, **not part of the learner's own exercise**... The graded exercise is the match logic against them." So the enum-vs-flag recognition — the exact skill LO1 names — is shown to the learner, never produced by them. Rubric criterion 5 and the other LOs cover exhaustive-match discipline, but nothing gates LO1. This is a textbook "read this, then move on" case for that one objective, and Module 03's README (unlike Module 04's) doesn't even flag it with an honesty caveat.

**2. Module 04's rubric silently dropped a validated criterion.** The dry run (`runs/2026-07-05-module-04-dry-run/grading.md`) scores a 7th criterion — "a lifetime is scoped to only the reference actually borrowed... not unified across every reference parameter defensively" — validated against a real failure mode (`extra-check-overannotated-lifetime`, which passes criteria 1–6 while still failing this one). But the shipped README rubric only has 6 criteria; criterion 7 never made it in. LO2 is honestly labeled "Partially covered," but the honest label undersells it — there is no rubric line for over-annotation at all, only a stray sentence in "Valid alternate terminal" prose. Coachgremlin has nothing numbered to point to.

**3. Prerequisite framing (03→04) is defensible, not dishonest.** `alert_checkpoint` alerts on the trigger *before* the human responds, so it genuinely can't depend on `next_action`'s output chronologically — this isn't a missed mechanical-enforcement opportunity like Module 02's was. Minor point only.

## Disposition

**Finding 1: fixed.** Added an honesty caveat to Module 03's LO1, matching the pattern Module 04 already used for its own partially-covered objective: "Not hands-on gated by the core exercise... demonstrated in the type design you read, not produced by a decision you make."

**Finding 2: fixed.** Added rubric criterion 7 to Module 04's README, naming the over-annotation failure mode explicitly and citing the dry run's `extra-check-overannotated-lifetime` evidence, plus updated LO2's annotation from "Partially covered" to "Directly gated" now that a real numbered criterion exists for it.

**Finding 3: no action needed** (confirmed correct as framed).
