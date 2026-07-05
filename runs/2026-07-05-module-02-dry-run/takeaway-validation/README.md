# Takeaway validation: `borrow-checker-playbook`

Per the bar `terminal-velocity`/Module 01 both used ("drop the packaged artifact into a different
task and confirm it helps"): the Skill packaged from Module 02's dry run
(`.claude/skills/borrow-checker-playbook/SKILL.md`) is applied here to a genuinely different
problem, nothing to do with `relay` or session/checkpoint statistics, to check the playbook
generalizes rather than just describing the one exercise it came from.

## The unrelated problem

`total_word_count(paragraphs: &[String]) -> usize`: given a borrowed slice of paragraphs, return
the total word count across all of them.

`src/lib.rs` started with a naive implementation (see `diff.patch` for the fix applied):

```rust
pub fn total_word_count(paragraphs: &[String]) -> usize {
    let paragraphs = paragraphs.to_vec();
    paragraphs.iter().map(|p| p.split_whitespace().count()).sum()
}
```

## Applying the playbook

**Step 1 (do I actually need `&mut`?):** N/A here - the function only ever reads, and correctly
took `&[String]`, not `&mut [String]`. Nothing to fix at this step.

**Step 2 (can I compute what I need directly through the reference?):** `paragraphs: &[String]`
arrives borrowed. The naive body's first line, `let paragraphs = paragraphs.to_vec();`, clones
every `String` in the slice into a new owned `Vec` before doing anything else - purely so the rest
of the function could iterate "its own" copy. Nothing downstream needs an owned copy: the function
returns a `usize` (`Copy`, no borrow to outlive), so every read can happen straight through the
original `&[String]`.

**Step 3 (is there a real, load-bearing clone here?):** No - unlike `session_stats`'s
`longest_gap_goal`, nothing in this function's return value needs to outlive the borrow. This is a
useful contrast case: the playbook doesn't say "always keep one clone," it says "clone only what
actually needs to outlive the borrow," and here that's nothing at all.

**Fix applied:** deleted the `.to_vec()` line and its rebinding; the rest of the function body
(`paragraphs.iter().map(...).sum()`) worked unchanged directly over the original reference
(`diff.patch`).

**Result: identical outcome, no clone.** `cargo test` (3/3) and `cargo clippy -- -D warnings`
(clean) both pass identically before and after the fix (`transcript.txt`'s BEFORE/AFTER sections) -
confirming, on a second, unrelated problem, that the deterministic gate genuinely cannot tell a
defensively-cloned version from a directly-borrowed one, exactly as `session_stats`'s own dry run
found.

**Step 6 (don't trust green tests/clippy as proof):** confirmed again here, on a shape of problem
(counting, not statistics) the playbook wasn't written from. Both versions pass every mechanical
check identically; only reading the body for an avoidable clone catches the difference.

## Verdict

The playbook correctly diagnosed and fixed a genuinely different shape of the same root problem
(a pure read-only aggregation with zero legitimate clones, versus `session_stats`'s one legitimate
clone) without modification. Counts as validated per the bar this takeaway needed to clear before
being called done.
