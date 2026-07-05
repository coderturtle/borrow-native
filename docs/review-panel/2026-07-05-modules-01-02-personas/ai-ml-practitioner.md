# AI/ML Practitioner — technical accuracy

**Lens:** Is the Rust taxonomy and technical content sound, current, and correctly attributed? Ignored prose style, structure, hook, and pedagogy.

**Reviewed:** Modules 01+02 READMEs, `fixtures/relay/SPEC.md`, `src/lib.rs`, both test files, both dry-run `grading.md`s, both Skill files.

---

## Technical Accuracy Review — Modules 01 & 02 (Borrow Native)

**Overall: the Rust content is sound.** I found no incorrect claims about ownership, moves, `Copy`, or borrowing, and the one claim I was asked to fact-check hardest turned out correct.

**1. `max_by_key` tie-break claim — verified correct.** Module 02's grading.md states attempt-good "track[s] the running max with a manual loop using strict `>` (not `Iterator::max_by_key`, which returns the *last* of several equal maxima — the wrong tie-break for this spec)." This matches documented `Iterator::max_by_key` behavior ("If several elements are equally maximum, the last element is returned"). The spec requires first-occurrence-wins (`tied_longest_gap_resolves_to_first_occurrence`), so correctly avoiding `max_by_key` in favor of strict `>` is the right call, not a misunderstanding.

**2. Clippy lint characterizations — all accurate**, no mischaracterizations found: `needless_pass_by_value` (pedantic; fires on unconsumed by-value params — accurately described as "a different, more general lint" than the clone itself); `redundant_clone` (nursery, MIR-based usage analysis; correctly noted it targets cloning an owned local you could've moved, not data reached through a `&`); `assigning_clones` (pedantic; suggests `clone_from` on *reassignment*, correctly distinguished from an initial `let` binding); `must_use_candidate` and `cast_precision_loss` (both standard pedantic lints, correctly applied to pure pub fns and `u64→f64` casts respectively).

**3. Minor oversimplification worth flagging:** SPEC.md/lib.rs frame `longest_gap_goal`'s clone as necessary because the value "must outlive the borrow of `session`." The more precise reason is that `SessionStats` carries no lifetime parameter — it's a type-design choice, not a borrow-timing constraint (an alternative `SessionStats<'a> { longest_gap_goal: &'a str, .. }` would eliminate the clone entirely). Reasonable to defer to Module 04 (lifetimes), but as worded a learner could conclude the clone is forced by borrow-checker mechanics rather than by the chosen return type's shape.

**4. Small semantic looseness, not a bug:** `average_gap_secs` is the mean over *all* `checkpoints.len()` values of `elapsed_secs`, including the first checkpoint's mandatory `0`, not the mean of the `n-1` actual inter-checkpoint gaps. Deterministic and matches its own tests, but "average gap between checkpoints" is a slight misnomer given the first entry isn't a gap at all.

No issues found with the `Copy`/move claims for `u64`, `String`, or `Vec<T>` field-level ownership reasoning in either module.
