---
name: agentic-learning-discipline
description: Check whether you actually learned this module's concept or your coding agent just demonstrated it for you. Use at the end of any Borrow Native module, before moving to the next one, especially if your agent produced a working diff quickly or with no real back-and-forth.
---

# Agentic learning discipline

Not a Rust skill - a cross-cutting one, referenced from every module's "Why this is hard" section
rather than owned by any single one. The workshop's premise is a learner working *through* a
coding agent to attempt a real exercise, then reading why their first instinct was wrong. That
premise has a structural weakness none of the per-module rubric fixes solve on their own: a
coding agent that already knows Rust can produce this module's correct answer in one shot, with no
struggle at all, the moment its instructions (or the rubric, or this very skill) are visible to
it. That's not against the rules - the deterministic gate (`cargo test`/`cargo clippy`) doesn't
care how a diff got written, and it never will. But "the gate passed" and "I understand why" are
different claims, and only checking the first one lets the second one quietly go missing.

## The check

Before accepting a module's diff as done - your own attempt or your agent's - answer these three
questions **before rereading the diff**:

1. **Name the concept this exercise was actually testing, without looking at the code.** Not "it
   passed the tests" - the one sentence a reviewer would use to describe what this diff
   demonstrates (e.g. "the alert borrows from the session with a lifetime scoped to just that
   argument, instead of cloning to dodge the question").
2. **Predict whether the diff would pass `cargo clippy -- -D warnings` before running it.** Not
   whether `cargo test` passes - whether the *idiom* would survive a linter's default settings. If
   you can't predict this, you haven't yet internalized what makes this diff's shape correct versus
   merely functional.
3. **Name the specific way a plausible, honest-but-naive attempt would have gotten this wrong -
   and why it would have passed every automated check anyway.** Every module in this workshop has a
   real, documented one (`runs/*/grading.md`, if you want to check your answer). If you can't name
   one, you likely haven't engaged with why this module's exercise is hard, only with why your
   specific diff compiles.

**Three "no"s (or "I'd have to look") is a real signal, not a moral failing.** It means this
module's diff was delegated, not learned - worth a second, slower pass before moving on, ideally
without your agent doing the typing this time. One or two "no"s might just mean you're moving
fast; use judgment. This check is intentionally not scored and not something Coachgremlin grades -
grading a self-reported answer to "did you really understand this" invites the same criteria-
compliance problem this skill exists to catch, one level up. It's a tool for your own honesty, not
a gate.

## Why this exists, and why it isn't a rubric fix

This module was written after this workshop's own Workshop Review Panel flagged that every rubric
criterion, by construction, has to name the target shape to be checkable - which spoils the
"why this is hard" reveal each module builds toward (`docs/rubric-spoiler-tension.md`,
`docs/rubric-spoiler-research.md` for the full research and candidate structures considered). The
rubric-wording fix adopted (property-phrased criteria - stating an observable fact about the
finished artifact, like "it's a compile error for X to outlive Y," instead of naming the technique
that produces it) helps against a human reading the page slowly. It does essentially nothing
against a coding agent, which can derive the technique from a stated property just as fast as from
a stated answer. That's the gap this skill is for: not better rubric wording, but a habit of
checking your own understanding directly, independent of whatever the rubric says or how it's
phrased.

## When to reach for this

At the end of every module, right before opening the Takeaway Skill - especially if your agent
produced a working `cargo test` pass quickly, on the first try, or without you needing to explain
the concept back to it at any point. Not needed mid-attempt, and not a substitute for actually
trying the exercise yourself first; it's a check on what happened after the fact, not a rule about
how to work.
