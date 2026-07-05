# Takeaway validation: `trait-lifetime-cheatsheet`

Per the bar Modules 01-03 each used ("drop the packaged artifact into a different task and confirm
it helps"): the Skill packaged from Module 04's dry run
(`.claude/skills/trait-lifetime-cheatsheet/SKILL.md`) is applied here to a genuinely different
problem, nothing to do with `relay` or checkpoint notifications, to check it generalizes rather than
just describing the one exercise it came from.

## The unrelated problem

`lookup<S: Store>(store: &S, key: &str) -> LookupResult`: given any config-store backend and a key,
look the value up and report back both the key asked for and the value found (if any). Same shape as
`alert_checkpoint`: a function generic over a trait bound, returning a struct that could either own
or borrow one of its fields.

`src/lib.rs` started with the naive version (see `diff.patch` for the fix applied):

```rust
pub struct LookupResult {
    pub key: String,
    pub value: Option<String>,
}

pub fn lookup<S: Store>(store: &S, key: &str) -> LookupResult {
    LookupResult {
        key: key.to_string(),
        value: store.get(key),
    }
}
```

## Applying the cheat-sheet

**Step 1 (is `S: Store` a real generic trait bound, not `&dyn Store` or a concrete type?):** Yes,
already correct in the naive version - nothing to fix at this step. Included to confirm the
cheat-sheet correctly recognizes when there's nothing to do, same discipline the enum-modeling
playbook's validation used for its own N/A steps.

**Step 2 (does an elision rule already cover this, or is an explicit lifetime genuinely required?):**
`lookup` is a free function (no `self`) with two reference-shaped inputs (`&S`, `&str`) and an output
that would need to borrow one of them - elision's rule 3 (assign `&self`'s lifetime) doesn't apply
(no `self`), and rule 2 (assign the one input lifetime) doesn't apply either (two candidates, not
one). An explicit lifetime is genuinely required the moment `LookupResult` borrows anything, not a
defensive habit.

**Step 3 (is the naive fix "clone to make the question go away," and does it pass everything
identically to the correct version?):** Yes on both counts, checked directly rather than assumed to
carry over from `alert_checkpoint`. `key: key.to_string()` sidesteps the lifetime question entirely;
`cargo build` and `cargo clippy -- -D warnings` are clean on both the naive and fixed versions, and
`clippy::pedantic` gives zero discriminating signal between them either - full output for both,
`transcript.txt`.

**Step 4 (if a lifetime is added, is it the minimal one, or over-applied defensively?):** The fixed
version ties `'a` only to `key`, not to `store` as well - `store` doesn't need to outlive the
returned `LookupResult`, so unifying its lifetime with `key`'s would only over-constrain a caller
whose store and key don't happen to share a lifetime already. Checked: a variant with `store: &'a S`
too was not written up as a separate transcript here (Module 04's own dry run already confirmed this
shape compiles and passes identically - see `runs/2026-07-05-module-04-dry-run/grading.md`'s
over-annotation check); noting the same minimal-lifetime judgment call applies here rather than
re-deriving it from scratch.

## Conclusion

The cheat-sheet generalized: applied cold to a config-store domain with zero relationship to `relay`
or checkpoints, it correctly identified that an explicit lifetime was genuinely required (not
optional, not defensive), and correctly predicted that both the clone-to-avoid-it naive version and
the correctly-annotated version would compile and lint identically at every level tried - the same
gap Module 04's own dry run found, independently reproduced here rather than assumed to transfer.
