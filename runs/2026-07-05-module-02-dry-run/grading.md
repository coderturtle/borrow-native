# Coachgremlin's dry run: Module 02 (Borrowing & References) on `relay`

First real content-authoring pass for Module 02, per Coachgremlin's Workflow. Same discipline as
Module 01's dry runs: a correct attempt and a deliberately naive, honest (non-adversarial) attempt,
both implementing `session_stats(session: &Session) -> SessionStats`. Per `docs/next-actions.md`'s
own instruction not to assume Module 01's finding generalizes without checking, this dry run treats
the question as open, not settled.

## Step 0: ARB trigger check

`fixtures/relay/src/lib.rs` and `SPEC.md` both needed real changes before this exercise could exist
at all: `session_stats` needs *some* timing data to compute a "gap" over, and neither struct had any.
Added `elapsed_secs: u64` to `DraftCheckpoint` and `CheckpointRecord` (seconds since the previous
checkpoint, supplied by the caller - `relay` never reads the system clock directly, keeping every
module's tests deterministic). `scripts/arb-trigger-check.sh` (written this session - see below)
confirmed this fires the `fixtures/relay/src/lib.rs`/`SPEC.md` triggers in `.hekton/governance.yaml`.
Resolution: `tests/finalize_session.rs` (Module 01's already-shipped test file) was updated to
include `elapsed_secs` in every `DraftCheckpoint` literal, plus one propagation assertion. Re-ran
Module 01's own test suite after the change - still 0 compile errors, same `todo!()`-panic behavior
as before (Module 01's implementation itself is still an unsolved stub, so there was nothing to
re-grade there, only compilation to protect). `u64` is `Copy`, so this addition carries none of
Module 01's move-semantics teaching weight - it's plumbing, not new exercise content for that module.

**Real gap found while doing this:** `docs/decisions.md` and `.hekton/governance.yaml` both claimed
`scripts/arb-trigger-check.sh` already existed and had been "verified firing correctly via a real
test touch." It didn't exist anywhere in the repo or git history. Written for real this session,
verified against both a clean baseline (no trigger) and a real touch to `lib.rs` (fires correctly),
then reverted and re-verified clean. Flagged in `docs/decisions.md` rather than quietly patched over.

## Step 1-2: Frame the exercise + rubric

Task: implement `session_stats`, computing `checkpoint_count`, `average_gap_secs`,
`longest_gap_secs`, and `longest_gap_goal` by reading `session.checkpoints` through the `&Session`
borrow, never consuming `session`. Six edge cases in `fixtures/relay/tests/session_stats.rs`: empty
session, single checkpoint, average-vs-last-vs-running-total, longest-gap-identifies-the-right-
checkpoint (not just the right number), a tie resolving to first occurrence, and `session` still
usable after the call. Rubric: see `modules/02-borrowing-references/README.md`.

## Step 3: Observe the attempts

**`attempt-good/`** (`diff.patch`): reads `&session.checkpoints` directly, no clone of the
collection. Tracks the running max with a manual loop using strict `>` (not `Iterator::max_by_key`,
which returns the *last* of several equal maxima - the wrong tie-break for this spec). Clones
`checkpoint.goal` exactly once, into `longest_gap_goal`, because that value must outlive the borrow
of `session` - the one clone in this exercise that's actually correct, not a habit to unlearn.
`cargo test`: 11/11 pass (5 from Module 01's suite, 6 new). `cargo clippy -- -D warnings`: clean.

**`attempt-naive-clone-checkpoints/`** (`diff.patch`): before doing anything else, builds an owned
`Vec<CheckpointRecord>` by cloning every field of every borrowed checkpoint (`goal.clone()`,
`summary.clone()`, `risks.clone()`, `elapsed_secs` copied), then computes stats from that owned copy
instead of the original borrow - "not sure what I can do with a `&Session`, so make my own copy
first," the exact same instinct as Module 01's naive attempt, now aimed at a borrow instead of an
owned `Vec`. `cargo test`: **11/11 pass, identical to `attempt-good`.** `cargo clippy -- -D
warnings`: **also clean, identical to `attempt-good`.**

## Step 4: Score against the rubric

| # | Criterion | attempt-good | attempt-naive-clone-checkpoints |
|---|---|---|---|
| 1 | `cargo test` green (gate, deterministic) | Pass. 11/11. | Pass. 11/11. |
| 2 | `cargo clippy -- -D warnings` clean (gate, deterministic) | Pass. | **Pass - deterministic gate cannot distinguish them.** |
| 3 | Diff touches only the implementation file (gate, anti-gaming) | Pass. | Pass. |
| 4 | `session` is read through the borrow, not copied into an owned local first (scored, conceptual) | Pass. | **Fails.** Clones the entire checkpoint history before reading it, discarding the whole point of taking `&Session`. |
| 5 | Only the one necessary clone exists - `longest_gap_goal`'s owned `String` (scored, conceptual) | Pass, exactly one clone, load-bearing. | **Fails on count, not on the concept**: still produces the one necessary clone, but buried inside 3 unnecessary per-field clones times every checkpoint in the session. |

**Result: same shape of finding as Module 01, new failure mode.** Module 01's naive attempt over-
*cloned fields* to avoid a move it didn't need to avoid. Module 02's over-*clones the whole
collection* to avoid engaging with a borrow it didn't need to avoid. Different concept, same root
symptom (defensive cloning as a stand-in for actually reasoning about ownership/borrowing), and the
deterministic gate misses both for the same underlying reason: cloning is never a compile error or a
test failure, only ever a design smell.

**Checked, not assumed, whether a stricter lint closes the gap - and this time it's a starker
result than Module 01's.** `clippy::pedantic` produces the *exact same 5 warnings, on the same
lines*, for both attempts (`must_use_candidate` x2, `cast_precision_loss` x2,
`assigning_clones` on the legitimate `longest_gap_goal` reassignment). Module 01 at least got a
partial, if noisy, signal from pedantic (`needless_pass_by_value`). Here, pedantic gives literally
zero discriminating signal - it doesn't even flag the wasteful clone under a different name, and its
one clone-related warning (`assigning_clones`) fires identically on both attempts' *correct* clone,
suggesting a micro-optimization (`clone_from`) instead of ever questioning the collection-level clone
that's actually the problem. `clippy::nursery` (checked because `redundant_clone` lives there) is
clean on both, unsurprising in hindsight: that lint targets cloning an owned local you could have
moved instead, not cloning data reached through a shared reference, which is structurally what
`redundant_clone` isn't designed to catch. Full transcripts: `attempt-good/fixture/transcript.txt`,
`attempt-naive-clone-checkpoints/fixture/transcript.txt`.

## Step 5: Confirm or loop

- **attempt-good:** rubric met.
- **attempt-naive-clone-checkpoints:** rubric not met on the conceptual tier.

## Step 6: Takeaway

A borrow-checker diagnostic playbook, `.claude/skills/borrow-checker-playbook/SKILL.md`, built
directly from this finding (see that file) and validated against a second, unrelated borrowing
problem before being called done (`takeaway-validation/` in this same run directory).

## Step 7: Batch-review cadence

Modules 01+02 now both have real authored content - this completes the first 2-module batch since
the workshop's only prior panel run (`docs/review-panel/2026-07-05-initial-design.md`, a design-doc-
only pass before any exercise existed). Per Coachgremlin's Workflow step 7, the full 7-persona
Workshop Review Panel is due now, not deferred further. Recorded as a pending next action
(`docs/next-actions.md`) rather than run automatically in the same pass that authored the content,
since it's a real-cost action (~7 parallel subagents) - left for an explicit decision.

## Human Gate

Recommendation only. `human_confirmed: false` in the run record, per Coachgremlin's Human Gate.
