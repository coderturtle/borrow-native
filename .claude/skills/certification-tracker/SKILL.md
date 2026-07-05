---
name: certification-tracker
description: "SKELETON, NOT YET FUNCTIONAL - decided shape only. Once docs/certification-mapping.md is authored for real (gated on coderturtle sitting the Ardan Labs exam, see docs/workshop-design.md's Certification Alignment Retrofit section), this Skill will let a learner's harness verify a module's deterministic gate actually passes, then tick off the cert topics that module maps to in a learner-local progress file."
---

# Certification tracker (skeleton)

**This Skill does not do anything yet.** Its mechanism is decided; it has nothing real to read
until `docs/certification-mapping.md` is authored, which is gated on coderturtle actually sitting
the Ardan Labs exam and recording a retrospective - see `docs/workshop-design.md`'s "Certification
Alignment Retrofit" section for the full reasoning. Documented now, in this shape, so the mechanism
doesn't need to be redesigned once the real mapping exists - matching this workshop's own idiom of
skeleton-with-decided-shape for Modules 05-08.

## Decided mechanism (once `docs/certification-mapping.md` is real)

1. **Read `docs/certification-mapping.md`** for the current module's row: which real Ardan topic
   area(s) it maps to.
2. **Re-run that module's own deterministic gate** (`cd fixtures/relay && cargo test`, `cargo clippy
   -- -D warnings`, scoped to that module's tests) - never trust a learner's self-report that a
   module is "done." This is the same discipline every module's own rubric already applies to
   itself: the deterministic tier is checked mechanically, not asserted.
3. **On a genuine pass**, tick off the mapped topic(s) in `.cert-progress.local.yaml` (gitignored,
   per-learner, not shipped workshop content - see `.gitignore`). On a failure, report which test(s)
   are still red and change nothing in the progress file.
4. **On request, summarize progress**: which of the exam's real topic areas (per the authored
   mapping, not the pre-exam placeholder categories) are fully covered by modules the learner has
   actually passed, which are partially covered, and which this arc doesn't touch at all - the last
   category should be empty by design (a gap here is what the retrofit's retrospective is supposed
   to catch and close), but the Skill reports it plainly if a real gap is ever found, rather than
   hiding it.

## Not decided yet

- The exact schema of `.cert-progress.local.yaml` - depends on `docs/certification-mapping.md`'s
  real row/column shape, not yet finalized either.
- Whether this Skill also needs to handle a module gaining a follow-up exercise (per that file's
  "Not decided yet" section) mid-workshop, after some learners have already ticked off the
  module's original mapping.

## Human Gate

Once functional, this Skill reports a learner's own progress toward topics a real, external,
paid credential tests. It should never claim "you will pass the Ardan exam" - only "you have
passed the deterministic gate for the modules mapped to these topics," which is a narrower, honest
claim consistent with `docs/workshop-design.md`'s own repeated caution that arc-to-exam alignment is
this workshop's bet, checked once, not a guarantee.
