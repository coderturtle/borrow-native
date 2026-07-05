# Takeaway validation: `ownership-move-checklist`

Per the bar `terminal-velocity` used ("drop the packaged artifact into a different task and
confirm it helps"): the Skill packaged from Module 01's dry run (`.claude/skills/
ownership-move-checklist/SKILL.md`) is applied here to a genuinely different problem, nothing to
do with `Order`/`HashMap`, to check the checklist generalizes rather than just describing the one
exercise it came from.

## The unrelated problem

`first_matching_line(lines: Vec<String>, prefix: &str) -> Option<String>`: given an owned list of
lines and a prefix, return the first line starting with that prefix, or `None`.

`src/lib.rs` starts with a naive implementation (see `diff.patch` for the fix applied):

```rust
pub fn first_matching_line(lines: Vec<String>, prefix: &str) -> Option<String> {
    for line in &lines {
        if line.starts_with(prefix) {
            return Some(line.clone());
        }
    }
    None
}
```

## Applying the checklist

**Step 1 (do I already own this?):** `lines: Vec<String>` arrives by value. Owned outright.

**Step 2 (why am I borrowing it, then?):** `for line in &lines` borrows a `Vec` that's never used
again after the loop (no second pass, no reference kept). No reason found.

**Step 3 (rewrite the borrow to a move):** changed `for line in &lines` to `for line in lines`,
and `Some(line.clone())` to `Some(line)` (`transcript.txt` has the real command output).

**Result: it compiles, and `cargo test` still passes.** The clone was never load-bearing, exactly
what the checklist predicts for an owned input that's never reused.

**Honest wrinkle, not smoothed over:** that first fix (`transcript.txt`'s "AFTER" section)
compiles and passes tests, but `cargo clippy -- -D warnings` then flags something new and
unrelated to ownership entirely: `manual_find`, an idiom lint suggesting the manual loop become
`lines.into_iter().find(|line| line.starts_with(prefix))`. Not a checklist failure, a real
reminder that fixing one thing (an unnecessary clone) doesn't mean clippy has nothing left to say
about idiom. Applied clippy's own suggestion (`transcript.txt`'s "AFTER-2" section); both
`cargo test` and `cargo clippy -- -D warnings` clean at that point, 3/3 tests, zero clippy output.

**Step 4 (don't trust green tests/clippy as proof):** confirmed here too, same as Module 01's own
finding — both the naive and fixed versions pass `cargo test` and default `cargo clippy -- -D
warnings` identically. The checklist's step 4 warning holds on a second, unrelated example, not
just the one it was extracted from.

## Verdict

The checklist correctly diagnosed and fixed a genuinely different shape of the same root problem
(early-return/`Option` pattern, not a `HashMap`-aggregation pattern) without modification. Counts
as validated per the bar this takeaway needed to clear before being called done.
