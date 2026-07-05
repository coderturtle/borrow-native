# Takeaway validation: `custom-error-type-template`

Per the bar Modules 01-04 each used ("drop the packaged artifact into a different task and confirm
it helps"): the Skill packaged from Module 05's dry run
(`.claude/skills/custom-error-type-template/SKILL.md`) is applied here to a genuinely different
problem, nothing to do with `relay` or checkpoints, to check it generalizes rather than just
describing the one exercise it came from.

## The unrelated problem

`parse_retry_policy(input: &str) -> Result<RetryPolicy, RetryPolicyError>`: parse a
`"max_attempts,backoff_secs"` string (e.g. `"3,5"`) into a `RetryPolicy`. Same shape as
`parse_config`/`write_handoff_summary`: a function that can fail more than one way, propagating via
`Result`, where a lower-level error (`std::num::ParseIntError`) needs converting into the function's
own error type.

`src/lib.rs` started with the naive version (see `diff.patch` for the fix applied):

```rust
#[derive(Debug)]
pub enum RetryPolicyError {
    Malformed(String),
}
// Display/Error impls omitted here - see src/lib.rs's git history via diff.patch

pub fn parse_retry_policy(input: &str) -> Result<RetryPolicy, RetryPolicyError> {
    let mut parts = input.split(',');
    let max_attempts_raw = parts.next().ok_or_else(|| RetryPolicyError::Malformed(input.to_string()))?;
    let backoff_raw = parts.next().ok_or_else(|| RetryPolicyError::Malformed(input.to_string()))?;

    let max_attempts = match max_attempts_raw.parse::<u32>() {
        Ok(v) => v,
        Err(e) => return Err(RetryPolicyError::Malformed(e.to_string())),
    };
    // ... same shape for backoff_secs
}
```

## Applying the template

**Step 1 (one variant per distinct cause, not a single catch-all)?** No - already the finding.
The naive version's single `Malformed(String)` variant forces every caller into identical handling
whether the field was missing or the number failed to parse, exactly the smell step 1 names.

**Step 2 (does a variant wrapping a lower-level error keep the real error, or stringify it
early)?** No in the naive version: `RetryPolicyError::Malformed(e.to_string())` discards
`std::num::ParseIntError`'s own structure the instant it's caught, identical in kind to
`HandoffError::Io(e.to_string())` from the original exercise - a different lower-level error type
(`ParseIntError` vs `io::Error`), same mechanism.

**Step 3 (does `source()` actually return the wrapped error)?** No in the naive version -
`RetryPolicyError` has no field left holding a real error to return once it's been stringified.

**Step 4 (`impl From` instead of hand-rolled `.map_err`/manual match)?** Checked directly, not
assumed to transfer unchanged: the naive version's manual matches convert `ParseIntError` to
`RetryPolicyError` (a *different* error type), not a same-type passthrough - so this is a case
`impl From` plus `?` would help write more cleanly, but not the `clippy::question_mark`-catchable
shape from Module 05's own extra-check (that lint targets a match that returns the *same* error
type unchanged, not a converting one).

**Step 5-6 (does any clippy level, including default, distinguish the two versions)?** Checked
directly. `cargo build`, `cargo clippy -- -D warnings` (clean on both, identical), and
`clippy::pedantic` (3 warnings on both, same lines, same text, byte-identical - `transcript.txt`)
all agree between the naive and fixed versions. **This refines the original exercise's step 6
finding rather than just repeating it**: `clippy::question_mark` (which *did* catch Module 05's
own `?`-avoidance extra-check) does **not** fire here, because this naive version's manual matches
convert the error type (`ParseIntError -> RetryPolicyError`) rather than passing the same type
through unchanged - confirming the template's step 6 caveat ("for at least the
same-type-passthrough case") is load-bearing, not a throwaway qualifier, on a case the original
exercise itself didn't test.

## Conclusion

The template generalized, and sharpened one of its own caveats in the process: applied cold to a
retry-policy domain with zero relationship to `relay` or checkpoints, it correctly predicted that a
stringified-catch-all error type would compile and lint identically to a properly-structured one at
every level tried (default, pedantic) - the same near-total deterministic-gate blindness Module 05's
own dry run found. It also correctly predicted, once actually checked rather than assumed, that this
naive version's manual matches would *not* trip `clippy::question_mark` the way Module 05's own
extra-check did, because converting-type matches and same-type-passthrough matches are mechanically
different cases for that lint - a real, checked refinement, not a re-run of the same result.
