# Workshop Review Panel — Initial Design (2026-07-05)

**Reviewed:** `README.md` (still the generic scaffold template — expected at this checkpoint), `docs/workshop-design.md`, `docs/workshop-gremlin-design.md`.
**Timing:** after naming + design docs, before Deliverables & branding (module skeleton, brand layer, site) — the checkpoint `terminal-velocity` validated as cheapest-to-fix.
**Panel:** 7 personas, independent parallel passes (each a separate general-purpose subagent, no cross-visibility). Raw critiques: `docs/review-panel/2026-07-05-personas/`.

## Outcome

All seven personas returned genuinely distinct, non-generic findings — no persona came back empty, none converged into duplicate feedback. Two cross-persona agreements; five single-persona findings. This is itself a second data point (after `terminal-velocity`'s six runs) that the panel's fan-out design produces differentiated signal on a second, differently-shaped workshop, not just its founding one.

## Cross-persona agreements (highest confidence)

1. **The deterministic-gate/curriculum-anchor claims outrun what's actually been verified**, independently flagged by the **AI/ML Practitioner** (two factual errors in the curriculum-anchor citations: Rustlings' "23 numbered exercises" conflates topic directories with actual exercise files — real count is ~94; and Exercism's `entry-api`/`vec-stack` are concept tags, not exercise names, cited as if they were) and the **Skeptical Critic** (the same section's "necessary, not a nice-to-have" and "validates that shape works" language claims more than the fetched artifacts support, and the canonical-Gremlin-file line "a strict upgrade for any subject that has one, not a Rust-only special case" is written as settled doctrine one paragraph away from admitting the mechanism has zero real runs). Same root cause both times: citing real, fetched evidence, then overstating what it proves.
2. **The module arc's prerequisite reasoning is solid for 5 of 6 modules but unreconciled for Module 04**, independently flagged by the **Instructional Designer** (Module 04's dependency on Module 03 is asserted, never argued — the Book's own cited trait-bound example needs no struct) and the **AI/ML Practitioner** (Module 06's dependency on Module 04 is weaker than claimed — `Send`/`Sync` are taught directly within ch16, not derived from Module 04's machinery).

## Single-persona findings (each real, not noise)

- **Instructional Designer**: the capstone (Module 07)'s stated takeaway is a checklist, not a coding artifact — its deterministic gate is literally undefined, the one module most exposed to becoming "Module 06 with extra steps."
- **Security-Conscious Reviewer**: the design names the exact test-tampering failure mode `terminal-velocity`'s own Module 04 dry run found, but doesn't specify a mitigation (fixture/test-integrity check) for a self-paced, unwatched repo — and the `unsafe`-as-escape-hatch warning isn't reinforced in the two modules (04, 06) where it's most tempting.
- **Technical Writer**: a dead cross-reference (workshop-design.md points to workshop-gremlin-design.md for Astro detail that isn't there), an unexplained duplicate curriculum-anchor entry (`06_move_semantics` anchors both Module 01 and 02), plus a grammar slip and a numbering-style inconsistency.
- **Developer Evangelist**: the strongest hook sentence is buried one paragraph below the actual headline pitch, and the "Coachgremlin" name is used in the README before a learner has any reason to know what that is.
- **End-User/Learner**: the core interaction mechanic (what exactly the learner types vs. what the agent does, and where the line is so the agent doesn't just write the Rust for them) is never specified anywhere in either doc.

## Prioritized action list

1. **Fix the two curriculum-anchor factual errors** (Rustlings count; Exercism concept-tag-vs-exercise-name conflation) — cheap, design-doc-text, fixed in this same pass.
2. **Soften the "strict upgrade... not a special case" line** in the canonical Gremlin docs and this workshop's docs to match its actual evidence status (designed, not run) — fixed in this same pass.
3. **Add explicit "why this depends on that" reasoning for Module 04→03**, and correct the overstated Module 06→04 justification to its real basis — fixed in this same pass.
4. **Define the capstone's actual gate** (a program, not just a checklist) — fixed in this same pass (skeleton-level fix; full exercise spec is still Coachgremlin's later job).
5. **Add a hedge to the "Why this order" section** acknowledging it's this workshop's own editorial synthesis, not an external finding — mirrors `terminal-velocity`'s own honesty pattern for its harness/loop split — fixed in this same pass.
6. **Fix the dead cross-reference and duplicate-anchor note, and the grammar/numbering nits** — fixed in this same pass.
7. **Deferred to the Deliverables & branding step** (next, not this pass): rewrite the README's headline pitch to lead with the compelling sentence, strip "Coachgremlin" jargon from learner-facing copy, specify the learner/agent interaction mechanic explicitly, add per-module Gate column, add fixture-integrity-check language to the two-tier grading section, reinforce the `unsafe`-escape-hatch warning in Modules 04 and 06's own rows. These need the module skeleton to exist to land properly, rather than being design-doc word-fixes.
8. **Not actioned, on the record:** the "nothing to do yet" and "doc-to-worked-example ratio" findings from the End-User/Learner and Developer Evangelist personas are accurate but describe this checkpoint's expected state (module content is explicitly Coachgremlin's later job, per the Gremlin's own Completion Condition) — re-run the panel once real module content exists, same as `terminal-velocity` did, rather than pulling content-building forward into this pass.
