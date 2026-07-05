# The rubric-spoils-the-answer tension

**Status:** decided (2026-07-05, coderturtle). Flagged by the Modules 03+04 Workshop Review
Panel batch (2026-07-05), deferred rather than patched - see
`docs/review-panel/2026-07-05-modules-03-04-content.md`'s deferred-item 1 and
`docs/next-actions.md`. This doc states the problem; `docs/rubric-spoiler-research.md` has the
research, the candidate structures considered, and the decision actually made.

> **Research complete, decision made (2026-07-05):** the prompt below was run
> (`docs/rubric-spoiler-research.md` has the output - literature grounding, comparable-system
> practice, four candidate structures, three cross-cutting risks). **Adopted: Candidate A
> (property-phrased criteria), plus all three items from that doc's Proposal section** - the
> workshop-wide meta-skill named in `docs/workshop-design.md`, a reflective closing beat in every
> module, and `.claude/skills/agentic-learning-discipline/SKILL.md`. Modules 01-05's rubrics have
> been rewritten accordingly. See `docs/rubric-spoiler-research.md`'s new "Decision" section for
> the full reasoning, including what this does and doesn't solve.

## The problem, in one line

Every module's rubric states the correct shape as a scored criterion, in writing, before the
learner has attempted the exercise - which spoils the exact "why this is hard" discovery each
module's own "Why this is hard" section spends several paragraphs building toward.

## Where this shows up

Coachgremlin's own Workflow requires the rubric to exist and be shared *before* the learner starts:

> **Set the rubric** - 3-6 observable criteria, scored, before the learner starts. Shared with the
> learner so there's no guessing what's graded.
> - `~/hekton/gremlins/coaching/coachgremlin.md`, Workflow step 2

That's a deliberate, good design choice on its own terms (no learner should be graded against a
secret standard). The collision is that a scored/conceptual criterion, by construction, has to
*name the correct shape* to be checkable at all - and once it's named, the module's later "Why
this is hard" section's entire rhetorical structure (walk the learner through the naive instinct,
let them feel why it's tempting, then reveal what's actually wrong with it) has nothing left to
reveal. The learner already read the answer two sections earlier, wrapped in a parenthetical tag.

Concrete instances, one per module, all currently shipped:

- **Module 01**, rubric criterion 4: *"No `.clone()` on data the function could have moved instead
  (scored, conceptual)."*
- **Module 02**, rubric criterion 5: *"Only the one necessary clone exists (scored, conceptual)."*
- **Module 03**, rubric criterion 5: *"Every `CheckpointTrigger` variant is listed explicitly under
  `Ignored`, not covered by a `_` wildcard (scored, conceptual)."* - flagged directly by the
  Review Panel's End-User/Learner persona: *"That's the entire punchline of the exercise, handed
  over before I've written a line of code - and it directly contradicts the 'Valid alternate
  terminal' framing later, which pretends reaching for `_` first is a legitimate discovery step. It
  isn't anymore; I already know not to."*
- **Module 04**, rubric criterion 6: *"`CheckpointAlert.session_label` borrows `session` with an
  explicit lifetime, rather than owning a clone of it (scored, conceptual)."* Same persona: *"This
  is the exact lesson 'Why this is hard' spends four paragraphs building up to - spoiled two
  screen-scrolls earlier."*
- **Module 05**, rubric criterion 6: *"`HandoffError::Io` preserves the real underlying error
  rather than discarding its structure into a `String` the instant it's caught (scored,
  conceptual)."* Authored after the Review Panel flagged this tension in Modules 03/04 and it was
  deferred, not resolved - the pattern was reproduced again anyway, because no alternative rubric
  shape has been decided yet.

This is not a Module 03/04-specific defect. It is present in every module authored so far
(01 through 05), which is itself evidence that it is structural to how this workshop currently
writes rubrics, not an authoring slip fixable module-by-module.

## Why a quick edit doesn't resolve it

A first instinct - "just move the Rubric section after 'Why this is hard'" - doesn't actually
hide anything; it only changes reading order, not what's readable before an attempt (a learner can
scroll ahead, and Coachgremlin's own workflow still requires the rubric shared *before* the
attempt, not merely *after some other section*). A second instinct - "drop the 'discovery'
framing from 'Valid alternate terminal' language, and just present the correct answer as a
worked example" - abandons the module's own stated hands-on-by-design principle (every module's
gate is "an artifact you produce," not "read this, then move on") for the specific criteria that
happen to be conceptual rather than deterministic.

Both of these were already named as candidate directions by the Review Panel's End-User/Learner
persona and left undecided:

> There isn't an obvious rewording that keeps the rubric checkable while hiding the answer; likely
> candidates (reordering Rubric after "Why this is hard," or accepting the spoiler and dropping
> "discovery" framing from "Valid alternate terminal") are workshop-wide structural calls, not
> Module 03/04-specific text fixes.

Neither has been evaluated for its actual tradeoffs (what a learner experiences under each, whether
either one is even a real fix or just a cosmetic rearrangement), and no other candidate structure
has been generated at all. That's the gap this doc is trying to get outside research on.

## Constraints any proposed fix has to satisfy

1. **The rubric must still be shared with the learner before they attempt the exercise** -
   Coachgremlin's Workflow step 2 is not up for revision here; a fix that requires hiding the
   rubric until after the attempt is out of scope.
2. **A scored/conceptual criterion must remain independently checkable by Coachgremlin** - it can't
   become vaguer just to avoid naming the answer, or grading becomes subjective in a way this
   workshop's own two-tier model (deterministic gate + conceptual read) is designed to avoid.
3. **The fix should generalize across all 8 modules**, not be re-decided per module - this is a
   workshop-wide structural question per the Review Panel's own framing, not a per-module edit.
4. **Whatever the fix is, it needs to preserve (or replace with something equally real) the
   "hands-on by design" principle** - every module's gate is a produced artifact, not something
   read passively; a fix that trades discovery for "here's the worked example, now go implement
   it" needs to be weighed against that principle explicitly, not silently drift away from it.

## What's NOT being asked here

This doc is not asking whether the finding is real (it is, verified directly against five modules'
shipped rubrics above) or whether it matters (the Review Panel already judged it worth a "needs a
real decision, not a quick edit" flag, via three independent personas touching adjacent parts of
it). It's asking for a **path forward**: what do other rubric-driven, hands-on technical curricula
(bootcamps, autograded university courses, competitive-programming judges, technical certification
prep) actually do about this exact tension, and does any of that translate to a workshop whose
rubric is read by an AI grader (Coachgremlin) rather than only a human one.

---

## Research prompt (for Fable)

The following is intended to be handed to Claude Fable 5 as a standalone research prompt. It
restates the necessary context so it doesn't depend on this doc's surrounding narration.

```
You're researching a specific, narrow instructional-design tension for a hands-on Rust workshop
called Borrow Native. Read the background below, then research how other rubric-driven or
autograded technical curricula handle this exact problem, and propose 2-4 concrete candidate
structures this workshop could adopt - with honest tradeoffs for each, not just a recommendation.

## Background

Borrow Native teaches Rust through real, harness-driven exercises (the learner uses their own
coding agent - Claude Code, Codex, etc. - against a real cargo crate). Each module has:

- A required exercise (a real function to implement against a real test suite).
- A **deterministic gate**: `cargo test` green, `cargo clippy` clean - mechanical, no judgment call.
- A **rubric**: 3-7 scored criteria, some deterministic (checkable by the gate) and some
  "(scored, conceptual)" - checked by an AI grading agent ("Coachgremlin") reading the learner's
  diff, because the deterministic gate provably can't distinguish a correct solution's *idiom* from
  a naive one that happens to produce the same passing test output (e.g., a solution that clones
  data defensively instead of borrowing it passes every test identically to one that borrows
  correctly - only a conceptual read catches the difference).
- A **"Why this is hard" section**: narrative prose that walks the learner through why an
  experienced developer's first instinct is often wrong for this specific concept, revealing the
  actual lesson only after they've had a chance to attempt it.

The rubric must be shared with the learner BEFORE they attempt the exercise (a firm design rule:
"no guessing what's graded"). But a scored/conceptual rubric criterion, to be checkable at all, has
to name the correct shape directly - for example, one module's actual shipped rubric line reads:
"Only the one necessary clone exists (scored, conceptual)." Reading that line before attempting the
exercise tells the learner exactly what "wrong" and "right" look like, which defeats the "Why this
is hard" section's entire narrative structure two sections later - there's nothing left to
discover. This pattern is present in every module authored so far (5 of 8), not a one-off mistake.

Two candidate directions have been informally proposed and NOT evaluated:
(a) reordering the Rubric section to appear after "Why this is hard" instead of before it
    (though the rubric must still be readable before the attempt itself, per the sharing rule
    above - so this only changes section order on the page, not information availability)
(b) abandoning "discovery" framing in the rubric/exercise language altogether, presenting the
    correct approach as a known target from the start, and moving what's currently discovery-based
    narrative into a purely retrospective "here's why this was worth doing" explanation instead

## Constraints any answer must respect

1. The rubric must remain fully shared with the learner before they attempt the exercise - not
   hidden, not partial, not gated behind a completed attempt.
2. Each conceptual criterion must remain objectively checkable by an AI grader reading a diff/
   transcript - no vaguer wording just to avoid saying the answer.
3. Whatever structure is proposed should generalize across all 8 modules in the workshop's arc
   (ownership, borrowing, structs/enums, generics/traits/lifetimes, error handling, concurrency,
   async, a synthesis capstone) - not be a per-module special case.
4. The workshop has a stated "hands-on by design" principle: every module's required gate must be
   a produced artifact (something the learner does), never "read this, then move on." Any proposed
   fix should either preserve genuine hands-on discovery for the conceptual criteria, or explicitly
   argue why abandoning discovery-as-a-goal for just the conceptual layer is an acceptable trade
   (with a real trade named, not just "this is simpler").

## What to research

- How autograded university CS courses (algorithms, systems courses with hidden vs. visible test
  suites) handle publishing a rubric/grading criteria without giving away the solution approach.
- How competitive programming judges and technical interview platforms (if relevant precedent
  exists) separate "what's graded" from "how to get there."
- Any existing pedagogical literature or named technique for "criterion transparency vs. discovery
  learning" tension - is there a standard term for this, and a standard resolution?
- Whether any curricula intentionally use vaguer, capability-level rubric language (e.g., "handles
  the edge case correctly" rather than "uses X specific technique") paired with worked examples
  disclosed only after a first attempt, and whether that pattern could translate here - specifically
  for the *conceptual* criteria only (deterministic criteria like "cargo test passes" don't have
  this problem at all, since they don't reveal an approach, only an outcome).
- Any risk this research surfaces that the workshop's current authors (not you) might not have
  already considered - flag it rather than omit it for being off-brief.

## What to deliver

2-4 concrete candidate rubric/page structures, each with:
- A one-paragraph description of the structure.
- What a learner experiences under it, concretely (walk through Module 04's real "clone vs.
  borrow" example under the new structure, not just abstractly).
- What Coachgremlin's grading workflow would need to change, if anything.
- Honest cons, not just pros - including whether it actually solves the spoiler problem or just
  relocates it.

Do not pick a winner and present it as decided - this feeds a human decision, not an autonomous
change. If your research turns up a clearly superior approach, say so and why, but still present
the alternatives you ruled out and why, so the decision-maker can weigh in rather than rubber-stamp.
```
