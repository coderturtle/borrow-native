# End-User / Target Learner — Modules 03+04 (2026-07-05)

## Findings — End-User / Target Learner Lens

**1. Spoiler bug reappears in a new location: the Rubric section, not "Why this is hard."** The batch-1 fix moved the "don't read before attempting" guard onto the "Why this is hard" heading itself (present in both M03 line 123 and M04 line 156 — correctly fixed there). But neither module guards the **Rubric** section, which sits immediately after the exercise instructions — the section any learner reads next to know what "done" looks like, before attempting anything.

- M03 Rubric criterion 5 states outright: *"Every `CheckpointTrigger` variant is listed explicitly under `Ignored`, not covered by a `_` wildcard (scored, conceptual)."* That's the entire punchline of the exercise, handed over before I've written a line of code — and it directly contradicts the "Valid alternate terminal" framing later, which pretends reaching for `_` first is a legitimate discovery step. It isn't anymore; I already know not to.
- M04 Rubric criterion 6 is the same problem: *"`CheckpointAlert.session_label` borrows `session` with an explicit lifetime, rather than owning a clone of it (scored, conceptual)."* This is the exact lesson "Why this is hard" spends four paragraphs building up to — spoiled two screen-scrolls earlier.
- Learning Objectives compounds it in both modules, e.g. M03: *"Recognize that a `_` wildcard arm... is not free... clippy::pedantic recommends collapsing the explicit forward-safe arms back into a wildcard, which would undo the point of criterion 5"* — this is un-guarded and appears before "Why this is hard."

So the fix generalized narrowly (one heading) rather than to the underlying problem (any pre-attempt text that names the answer). As a learner reading top-to-bottom, I never get the "valid alternate terminal" experience the module claims to offer.

**2. Path to `cargo test` is clean.** Both modules explicitly give `cd fixtures/relay && cargo test`, name the single file to edit (`src/lib.rs`), and state the test file is off-limits. No missing steps here — this part of the batch-1 issue class is resolved.

**3. Minor:** the given `Notifier`/`DesktopNotifier`/etc. code in M04 lives in the same file I'm allowed to touch, with no explicit "don't modify this" — low risk since the prose calls it out as "not this module's teaching subject," but not gated like the test-file restriction is.

## Disposition

**Finding 1 (rubric spoils the answer): real, but not a quick fix — recorded as a genuine, deferred design tension, not actioned this pass.** The rubric must be shared before the learner starts (Coachgremlin's own Workflow step 2: "no guessing what's graded") — a scored/conceptual criterion, by construction, states the observable correct shape, which is close to stating the concept itself for an exercise this small. There isn't an obvious rewording that keeps the rubric checkable while hiding the answer; likely candidates (reordering Rubric after "Why this is hard," or accepting the spoiler and dropping "discovery" framing from "Valid alternate terminal") are workshop-wide structural calls, not Module 03/04-specific text fixes, and this tension likely predates this batch (probably present since Module 01/02's own scored criteria). Added to `docs/next-actions.md` for a real decision.

**Finding 2: no action needed** (already correct).

**Finding 3: fixed.** Added "don't modify them" to Module 04's exercise blockquote, next to the existing "not this module's teaching subject" note.
