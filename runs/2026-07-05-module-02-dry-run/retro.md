# Retro: Coachgremlin's Module 02 dry run (Borrowing & References)

Go/no-go on Module 02's real content, and a direct test of `docs/next-actions.md`'s own instruction:
don't assume Module 01's finding (deterministic gate can't distinguish defensive over-cloning)
generalizes to a new concept without checking it again.

## Checklist, checked against evidence

**Coachgremlin can frame Module 02's exercise from the module README's stated gate and takeaway,
without inventing an unrelated scenario.**
Yes. `session_stats` (`fixtures/relay/SPEC.md`'s Module 02 section) is a direct instance of the
module's own stated question ("when do I need a reference instead of ownership, and what can the
borrow checker actually see?") - reading `Session`'s own history by reference, not a substitute
scenario.

**Given a deliberately good attempt and a deliberately naive (not gaming) attempt, the
deterministic tier alone does not discriminate, and Coachgremlin's conceptual tier does - checked
fresh, not assumed from Module 01.**
Yes, and it's the same shape of finding with a genuinely different mechanism. `attempt-good` and
`attempt-naive-clone-checkpoints` both pass `cargo test` (11/11) and both pass `cargo clippy -- -D
warnings` (zero output) - see `grading.md`. The naive attempt's actual mistake (cloning the entire
borrowed checkpoint history into an owned local before reading it, instead of reading through the
`&Session` it was already given) is a *borrowing*-flavored version of Module 01's *ownership*-flavored
mistake, not a copy-paste of the same code pattern. **Is consistent with** the workshop's central bet
generalizing to a second concept, rather than being a one-off property of Module 01's specific
exercise - "confirms" would overstate what two self-graded instances can establish; see this same
retro's "What's still open" section and the Skeptical Critic finding in
`docs/review-panel/2026-07-05-modules-01-02-content.md` for why that wording was corrected.

**Checked whether a stricter deterministic tier would close the gap, rather than assuming
Module 01's `clippy::pedantic` result carries over unchanged.**
Yes, and it didn't carry over the way I'd have guessed. `clippy::pedantic` gives *zero* discriminating
signal here (identical 5 warnings on identical lines for both attempts), starker than Module 01
(where pedantic at least fired a different, real lint on the naive attempt). `clippy::nursery` was
also checked, specifically for `redundant_clone` - clean on both, because that lint targets cloning
an owned value you could have moved, not cloning data read through a shared reference. Decision:
same as Module 01, this module's stated deterministic gate stays at default `cargo clippy`; the
conceptual tier is doing all the real discriminating work here, more starkly than in Module 01.

**Coachgremlin's feedback references the actual diff, gives one concrete next try, and never hands
the fix over.**
Yes. `grading.md`'s scoring table points at the specific pattern (`session.checkpoints.iter().map(|c|
CheckpointRecord { ... c.goal.clone() ... })` building a whole owned copy) and names what it costs
(discards the entire point of taking `&Session`), without handing over the fixed code.

**The terminal state (test + clippy output) is observed running, not asserted.**
Yes. `transcript.txt` in each `attempt-*/fixture/` directory is real, captured `cargo test`/`cargo
clippy` output (including the `clippy::pedantic` comparison run), not narrated.

**Step 6 produces a takeaway that actually helps on a second, different borrowing problem, not just
a file that was written.**
Yes. See `.claude/skills/borrow-checker-playbook/SKILL.md` and `takeaway-validation/` in this run
directory - the packaged playbook was applied to an unrelated borrowing scenario before being called
done.

**The Human Gate holds: completion is a recommendation with `human_confirmed: false` in a `runs/`
entry, not a self-certified "complete."**
Yes - see the new `run-*.yaml` entry for this run.

**Step 0 (ARB check) and the batch-review cadence (step 7) were both actually exercised, not just
referenced.**
Yes, and step 0 surfaced a real, previously-undetected gap: `scripts/arb-trigger-check.sh` was
claimed to exist (`docs/decisions.md`, `.hekton/governance.yaml`) but wasn't in the repo. Written for
real this session and verified against both a clean baseline and a real touch to `lib.rs`. Step 7
(batch-review cadence) is due now that Modules 01+02 both have real content - recorded as a pending
next action, not run automatically in the same pass, since it's a real-cost multi-agent action
that deserves its own explicit go-ahead.

## Go/no-go

**Go.** Module 02's exercise, rubric, and finding are real, evidenced, and distinct in mechanism
from Module 01's - not a restatement of the same lesson in new clothes. Proceeding with the same
per-module dry-run discipline for Modules 03-08, continuing to check each one fresh rather than
assuming any prior module's finding generalizes.

## Revision fed back

1. **`~/hekton/gremlins/coaching/coachgremlin.md`**: this is Coachgremlin's third real run and
   second workshop-internal one for `borrow-native` - update its Version/Status line to cite this
   run's finding (a second, mechanically distinct instance of "default clippy can't see a defensive
   clone," this time with pedantic giving *zero* signal rather than a noisy partial one).
2. **`modules/02-borrowing-references/README.md`**: rewritten from skeleton to real authored content
   reflecting this dry run's actual evidence.
3. **Takeaway packaged and validated**: `.claude/skills/borrow-checker-playbook/SKILL.md` plus
   `takeaway-validation/` in this run's own directory.
4. **`scripts/arb-trigger-check.sh` written for real** (was previously only claimed to exist) and
   `docs/decisions.md` corrected to reflect what actually happened.

## Toward Coachgremlin's Review Trigger

This run's finding is a genuine second instance of the same class of result Module 01 produced (not
a new failure mode in the way Module 01's was relative to `terminal-velocity`'s Module 04) - it
strengthens confidence in the two-tier model generalizing across concepts within one workshop, but
per the same honesty standard applied throughout: whether it counts as an *independent* data point
toward the 3+ run bar (as opposed to the same lesson re-instantiated) is coderturtle's call, flagged
in the run record, not self-certified here.

## What's still open

- Modules 03-08 still need the same treatment.
- The Workshop Review Panel's first content-level batch (Modules 01+02) is due now, not yet run -
  a real-cost decision left for explicit go-ahead rather than bundled into this session.
- Same single-grader limitation as Module 01's retro named: this dry run used one session (this one)
  constructing and grading both attempts. An independent blind pass would be a stronger test.
