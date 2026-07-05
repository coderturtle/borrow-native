# Developer Evangelist — Modules 03+04 (2026-07-05)

## Top finding: the `cd fixtures/relay` fix from batch 1 held — no regression
Both modules correctly say "Run everything from `fixtures/relay/` (`cd fixtures/relay && cargo test`)." Good, that specific friction point is gone. No new variant of it either — both modules assume you're already inside a working `fixtures/relay` (built up over 01→04), which is fine and consistent.

## Bigger problem: these read as audit reports, not workshop pages
The hook lands ("How do I model this data so illegal states are unrepresentable?" — genuinely good, quotable line, Module 03 line 5), but it's immediately buried under an avalanche of meta-commentary: `runs/2026-07-05-module-03-dry-run/grading.md`, "checked fresh, not assumed," "byte-for-byte identical," clippy::pedantic escalation logs. This is valuable *rigor evidence* but it's dev-internal furniture sitting in the reader-facing page. If I clicked in from a shared link, I'd hit paragraph after paragraph of self-referential grading proof before reaching "here's the thing you build." That's a hook-killer — it reads like a lab notebook, not a pitch.

## The payoff is buried at the bottom
The actual shareable hook — "you walk away with a reusable Claude Code Skill" (the Takeaway section) — is the literal last thing on both pages, after ~150+ lines of rubric, "why this is hard," and dry-run evidence. That's backwards for evangelism: the "what you keep" promise should be near the top, not the epilogue.

## Module 04's hook is weaker than 03's
Module 03: crisp, punchy, one clause. Module 04: "How do I write one function that works across types, safely, without the compiler losing track of how long references live?" — a run-on that trails off into a hedge ("losing track of how long references live" is fuzzy compared to 03's sharp "illegal states are unrepresentable"). Minor, but noticeable back-to-back.

## Net
Mechanical friction (cd fixtures/relay) is fixed. Remaining friction is tonal/structural: the pages are optimized for rigor-proof, not for "would I share this" — the actual hook and payoff get diluted by dry-run bookkeeping placed in the reader's path.

## Disposition

Deferred, not actioned this pass. Same category as batch 1's deferred "top-level README hook-vs-hedge ordering" item — this is about the whole workshop's page structure/tone (dry-run evidence sitting in the reader's path, Takeaway placed last), not a Module 03/04-specific defect, and reworking it risks an inconsistent structure against Modules 01-02's already-shipped pages if done piecemeal. The Module 04 hook-phrasing note is lower priority still: that question is the module's already-decided core framing (present since the skeleton stage, reused verbatim in `modules/README.md`'s arc table), so rewording it here would diverge from the canonical phrasing rather than fix an authoring-pass-introduced problem. Added to `docs/next-actions.md` as a workshop-wide "page structure: rigor evidence vs. reader-facing pitch" item to address once more modules exist to judge the right general pattern from, rather than one-off per module.
