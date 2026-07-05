# Trial: `scripts/clone-count-check.sh` against Modules 01 and 02's real dry runs

Not a learner dry run (no exercise, rubric, or human gate involved) — a trial of the Workshop Review
Panel's own suggestion (`docs/review-panel/2026-07-05-modules-01-02-content.md`, deferred in
`docs/next-actions.md`): a scripted clone-count check on the diff might mechanically discriminate
both modules' good/naive attempts, as a cheap first-pass signal ahead of Coachgremlin's conceptual
read (Workflow step 3). Run against the diffs already captured from real dry runs, not invented
fixtures.

## Method

Count `.clone()` occurrences on added lines (`+`, not `+++` headers) in a unified diff, restricted to
`lib.rs`'s hunk, compared against an expected-max baseline. `scripts/clone-count-check.sh
<diff-file> <expected-max> [path-filter]`; exit 0 within baseline, exit 1 if exceeded.

## First pass: expected-max read off the rubric's prose, not the reference diff

Module 02's rubric criterion 5 says "only the one necessary clone exists." Taking that literally,
first pass used `expected-max=1` for Module 02.

```
$ ./scripts/clone-count-check.sh runs/2026-07-05-module-02-dry-run/attempt-good/diff.patch 1 lib.rs
clone-count-check: 2 clone() call(s) on added lines matching 'lib.rs' (expected max: 1)
FLAGGED: clone count exceeds this exercise's expected baseline - route to Coachgremlin's conceptual read before trusting a green cargo test/clippy run.
exit: 1
```

**Wrong on the first try, and worth recording rather than quietly fixing:** the *good* attempt itself
gets flagged. Its running-max tracking pattern (`let mut longest_gap_goal = checkpoints[0].goal.clone();`
at init, then `longest_gap_goal = checkpoint.goal.clone();` inside the loop, only reached on a new
strict max) calls `.clone()` at two call sites for what is conceptually one value ending up cloned
once per execution. "Only the one necessary clone exists" describes the *conceptual* result, not the
number of `.clone()` textual call sites the correct control flow requires to get there — those two
things diverge whenever a loop needs to reassign the same clone at more than one point.

## Corrected baseline: calibrate from the reference implementation's own diff, not the rubric's prose

```
$ ./scripts/clone-count-check.sh runs/2026-07-05-module-01-relay-dry-run/attempt-good/diff.patch 0 lib.rs
clone-count-check: 0 clone() call(s) on added lines matching 'lib.rs' (expected max: 0)
OK: within expected baseline.
exit: 0

$ ./scripts/clone-count-check.sh runs/2026-07-05-module-01-relay-dry-run/attempt-naive-clone/diff.patch 0 lib.rs
clone-count-check: 3 clone() call(s) on added lines matching 'lib.rs' (expected max: 0)
FLAGGED: clone count exceeds this exercise's expected baseline - route to Coachgremlin's conceptual read before trusting a green cargo test/clippy run.
exit: 1

$ ./scripts/clone-count-check.sh runs/2026-07-05-module-02-dry-run/attempt-good/diff.patch 2 lib.rs
clone-count-check: 2 clone() call(s) on added lines matching 'lib.rs' (expected max: 2)
OK: within expected baseline.
exit: 0

$ ./scripts/clone-count-check.sh runs/2026-07-05-module-02-dry-run/attempt-naive-clone-checkpoints/diff.patch 2 lib.rs
clone-count-check: 5 clone() call(s) on added lines matching 'lib.rs' (expected max: 2)
FLAGGED: clone count exceeds this exercise's expected baseline - route to Coachgremlin's conceptual read before trusting a green cargo test/clippy run.
exit: 1
```

With the baseline calibrated against each module's own real good-attempt diff (0 for Module 01, 2 for
Module 02) rather than inferred from rubric wording, the check cleanly discriminates good from naive
in both modules — the same two failure modes that (correctly) required Coachgremlin's conceptual tier
to catch, since neither `cargo test` nor default/`pedantic`/`nursery` clippy discriminate them at all
(`runs/2026-07-05-module-01-relay-dry-run/`, `runs/2026-07-05-module-02-dry-run/grading.md`).

## Conclusion

**Adopted as an optional pre-filter, not a replacement for the conceptual tier**, and not a hard
gate on its own (see Limitations below):

1. It generalizes across two mechanically distinct failure modes (an owned-value clone-instead-of-move,
   a borrowed-collection defensive full-clone) — real, if still thin, breadth evidence for the
   approach, same standard this project applies to its other findings (2 instances, not yet 3+).
2. The expected-max baseline is **not** safely inferable from a rubric's prose description of "how
   many clones are necessary" — it must be measured against a real reference implementation's own
   diff first, the same "checked fresh, not assumed" discipline this workshop already applies to
   whether findings generalize across modules. Recording each module's calibrated baseline as it's
   authored (in the module's own rubric or run record) is now part of using this script, not
   optional.
3. **Limitations, stated plainly, not discovered later:** this is a textual heuristic. It cannot see
   `.to_owned()`, `.to_vec()`, or a manual copy loop achieving the same defensive-clone effect without
   the literal substring `.clone()`. It cannot tell a load-bearing clone from a defensive one by
   itself — only a call-site *count* above a pre-measured baseline, which is why a differently-shaped
   but equally correct solution (e.g. one using `Iterator::max_by_key` differently, or folding instead
   of an explicit loop) could legitimately need a different call-site count and get a false positive.
   It is a prioritization signal for where Coachgremlin's conceptual read should look first, not a
   pass/fail gate that can stand in for that read.

Wired into `~/hekton/gremlins/coaching/coachgremlin.md`'s Workflow step 3 (two-tier grading) as an
optional sub-step, starting with Module 03: run this script against the diff before the conceptual
read, using a baseline calibrated from that module's own known-good attempt once one exists.
