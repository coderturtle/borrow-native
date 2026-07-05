# Workshop Review Panel — Modules 01+02 Content (2026-07-05)

**Reviewed:** `modules/01-ownership-move-semantics/README.md`, `modules/02-borrowing-references/README.md`, `fixtures/relay/SPEC.md`, `fixtures/relay/src/lib.rs`, `fixtures/relay/tests/{finalize_session,session_stats}.rs`, both dry runs' `grading.md`/`retro.md`, both takeaway Skills, and (Security-Conscious Reviewer only) `scripts/arb-trigger-check.sh`.
**Timing:** first content-level batch, per Coachgremlin's Workflow step 7 (every 2-3 modules of real content) — Modules 01 and 02 are the only two authored so far.
**Panel:** 7 personas, independent parallel passes (each a separate general-purpose subagent, no cross-visibility). Raw critiques: `docs/review-panel/2026-07-05-modules-01-02-personas/`.

## Outcome

All seven personas returned genuinely distinct, non-generic findings — no persona came back empty, none converged into duplicate feedback. Two cross-persona agreements; the rest single-persona, each real. This is the panel's first run against actual authored exercise content in this workshop (the only prior run was design-doc-only, `docs/review-panel/2026-07-05-initial-design.md`), and the first time in `borrow-native` that findings required touching shipped fixture code and a script, not just prose.

## Cross-persona agreements (highest confidence)

1. **The exercise spoils its own diagnosis, and the spoiler isn't where the existing guard protects.** Independently flagged by the **End-User/Learner** ("the pages spoil their own exercise, not just the Skill... this same answer is baked into `fixtures/relay/src/lib.rs` itself") and the **Professional Technical Writer** ("the spoiler isn't in 'Takeaway,' it's one section earlier... the Takeaway guard is well-written but structurally misplaced — protecting the wrong section"). Both point at the same root cause from different angles: "Why this is hard" states the naive attempt's exact code and the fix, in prose, before a learner has attempted anything, while the only "don't read this yet" warning sits on the Takeaway Skill link below it.
2. **The "one clone that's actually correct" framing overstates what's true only because lifetimes haven't been taught yet.** Independently flagged by the **AI/ML Practitioner** (technical: "the more precise reason is that `SessionStats` carries no lifetime parameter — it's a type-design choice, not a borrow-timing constraint") and the **Skeptical Critic** (overclaiming: "asserted as settled correctness rather than flagged as scoped-to-current-toolset"). Same underlying fact, caught on two different grounds.

## Single-persona findings (each real, not noise)

- **Instructional Designer**: Module 02's "hard prerequisite" on Module 01 is a conceptual claim, not a mechanically enforced one — `session_stats.rs`'s own tests build a `Session` directly, never calling `finalize_session`, so a learner could skip Module 01 entirely and still complete Module 02.
- **Instructional Designer**: the deterministic tier isn't actually primary for either module's real lesson (both dry runs conclude it "can't catch" the target concept) — but a cheap, still-mechanical check (grep the diff for clone count) would have discriminated the two attempts in both dry runs, and wasn't tried before routing the whole lesson through Coachgremlin's subjective read. Also: Module 02's third learning objective (reading a real borrow-checker error) isn't reachable by this exercise — no aliasing conflict exists here.
- **Security-Conscious Reviewer**: `scripts/arb-trigger-check.sh` (written this session) always exited 0, even when a trigger fired — a governance check that can never block anything wired to its exit code. Also flagged fragile `git status --porcelain` parsing (renames, quoted paths).
- **End-User/Learner**: neither module README says `cd fixtures/relay` before `cargo test` — there's no top-level `Cargo.toml`, so running the command from repo root fails outright, and the correct path only appears one link away in `SPEC.md`.
- **Skeptical Critic**: Module 02's `retro.md` says a self-graded, self-constructed pair of attempts "confirms the workshop's central bet generalizes," then two paragraphs later says whether it counts as independent evidence is "not self-certified here" — the two statements can't both be true as worded.
- **Skeptical Critic**: the single-grader limitation (this session wrote and graded both dry-run attempts) is named honestly in `retro.md` but not carried into the module-level README footers, which say only "run once so far."
- **Developer Evangelist**: the top-level README's pitch lands in the first few lines, then buries the actual "clone and run" moment under three successive hedges (cert-not-yet-attempted, 6-of-8-skeleton status, "working hypothesis not settled finding") before a reader reaches "How to start."
- **Developer Evangelist**: `relay`'s "you keep a real, usable tool" promise is credible as a roadmap but not yet demonstrable — at 2/8 modules, nothing in `main.rs` is runnable yet.
- **Professional Technical Writer**: three different vocabularies for the same two-tier concept across README.md / modules/README.md / the per-module rubrics ("deterministic gate" vs. "deterministic tier" vs. "(gate, deterministic)"), despite `modules/README.md` explicitly claiming "every module uses this vocabulary."
- **Professional Technical Writer**: both module READMEs' status blockquotes cited an internal `~/hekton/...` path meaningless to anyone outside the Hekton system, in a repo marked `privacy boundary: public`.
- **Professional Technical Writer**: a literal em dash in `fixtures/relay/SPEC.md`, violating the brand lint rule the rest of the repo passes — SPEC.md's exemption status (design doc vs. published content) was undefined.
- **AI/ML Practitioner**: `average_gap_secs` is the mean over *all* checkpoints including the first one's mandatory `elapsed_secs: 0`, not the mean of the actual inter-checkpoint gaps — deterministic and test-correct, but "average gap between checkpoints" slightly misdescribes what's computed.

## Prioritized action list

1. **Fixed `scripts/arb-trigger-check.sh`'s exit code and porcelain parsing** — real bug in a governance mechanism, cheap and unambiguous. Verified: still fires correctly on a real touch to `lib.rs`, now exits 1; clean baseline still exits 0.
2. **Added an explicit "don't read this section before your first attempt either" guard directly above "Why this is hard" in both Module 01 and 02** — mirrors the existing Takeaway guard, protects the section that actually needs it.
3. **Softened the "one clone that's actually correct" framing** in Module 02's README, `SPEC.md`, `src/lib.rs`'s doc comment, and `borrow-checker-playbook/SKILL.md` — now explicit that it's correct *given `SessionStats` has no lifetime parameter yet*, not a borrow-timing law, with a forward-pointer to Module 04.
4. **Added the missing `cd fixtures/relay` instruction** directly into both module READMEs' exercise blockquotes.
5. **Softened Module 02's `retro.md`** — "confirms the workshop's central bet generalizes" changed to "is consistent with," removing the self-contradiction with its own later hedge.
6. **Fixed the em dash in `SPEC.md`** to match the rest of the repo's brand rule.
7. **Removed the internal `~/hekton/...` path** from both module READMEs' status blockquotes, replaced with a description that doesn't assume Hekton-system access.
8. **Reworded Module 02's third learning objective** to match what this exercise actually exercises (predicting borrow legality up front, not reading a real conflict error), with an honest forward-pointer to where a real conflict does appear.
9. **Deferred, on the record, not actioned this pass:**
   - The mechanical, not-just-conceptual Module 01→02 prerequisite gap (Instructional Designer's top finding) — fixing this properly means deciding whether `session_stats.rs` should require `finalize_session` to be already implemented (which would make Module 02 ungradable until Module 01 is solved, a real design tradeoff) or whether the prerequisite claim should just be reworded to "conceptual, not enforced." Needs a decision, not a quick edit — added to `docs/next-actions.md`.
   - The "a cheap clone-count check would have worked" finding — real, and worth trying for Module 03 onward before assuming Coachgremlin's subjective read is the only option, but retrofitting it onto Modules 01-02's already-reviewed rubrics is lower priority than getting it right going forward.
   - Terminology drift across "gate"/"tier" vocabulary (Technical Writer) — a cross-file harmonization pass (README.md, modules/README.md, per-module rubrics), bigger than a same-pass text fix.
   - The top-level README's hook-vs-hedge ordering (Developer Evangelist) and relay's "real tool" promise being ahead of its current runnable state — both are about the *whole workshop's* framing, not Module 01/02-specific, better addressed once more modules land.
   - `average_gap_secs` including the first checkpoint's mandatory `0` (AI/ML Practitioner) — accurate to spec and tests as shipped; renaming or reshaping this would change test expectations, a design call for a future session, not a wording fix.
   - Single-grader limitation not repeated in per-module README footers (Skeptical Critic) — real, low urgency; the top-level README already states it workshop-wide.
