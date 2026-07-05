# Takeaway validation: `enum-modeling-playbook`

Per the bar Modules 01-02 both used ("drop the packaged artifact into a different task and confirm
it helps"): the Skill packaged from Module 03's dry run
(`.claude/skills/enum-modeling-playbook/SKILL.md`) is applied here to a genuinely different problem,
nothing to do with `relay` or checkpoint triggers, to check the playbook generalizes rather than just
describing the one exercise it came from.

## The unrelated problem

`refund_policy(status: &OrderStatus) -> RefundPolicy`: given an order's status, decide its refund
policy. Four `OrderStatus` variants (`Pending`, `Shipped`, `Delivered`, `Cancelled`); three
`RefundPolicy` variants (`FullRefund`, `PartialRefund`, `NoRefund`). Rule: `Pending` and `Cancelled`
both get a full refund, `Shipped` gets a partial refund, `Delivered` gets none.

`src/lib.rs` started with a naive implementation (see `diff.patch` for the fix applied):

```rust
pub fn refund_policy(status: &OrderStatus) -> RefundPolicy {
    match status {
        OrderStatus::Shipped => RefundPolicy::PartialRefund,
        OrderStatus::Delivered => RefundPolicy::NoRefund,
        _ => RefundPolicy::FullRefund,
    }
}
```

## Applying the playbook

**Step 1 (is this a hidden-enum-in-a-struct situation?):** N/A here - `OrderStatus` and
`RefundPolicy` were already modeled as enums, not flags on a struct. Nothing to fix at this step;
included to confirm the playbook correctly recognizes when there's nothing to do.

**Step 2 (can a struct-plus-flags version hide an invalid combination a test wouldn't catch?):**
N/A for the same reason - no flags-based alternative exists here to compare against. Noted, not
skipped silently.

**Step 3 (is the `_` arm a real "these are the same case," or just fewer keystrokes?):** `_ =>
RefundPolicy::FullRefund` covers `Pending` and `Cancelled` - correct today, but it would also
silently cover any new `OrderStatus` variant added later (e.g. a future `Returned` or `Disputed`
status), defaulting it to a full refund without anyone deciding that was the right call for the new
case. Real, load-bearing finding, checked directly on this unrelated example - not assumed to carry
over from `next_action` just because the shape looks similar.

**Fix applied:** replaced `_ => RefundPolicy::FullRefund` with explicit `OrderStatus::Pending =>
RefundPolicy::FullRefund` and `OrderStatus::Cancelled => RefundPolicy::FullRefund` arms (`diff.patch`).

**Step 4 (does `clippy::pedantic` help or hurt here?):** Checked fresh on this second example, not
assumed from `next_action` - and it reproduces the same suggestion, though not the same risk.
`cargo clippy -- -W clippy::pedantic` on the *fixed* version fires `match_same_arms` on the two
explicit `FullRefund` arms and suggests merging them into `OrderStatus::Pending |
OrderStatus::Cancelled => RefundPolicy::FullRefund`. **Corrected 2026-07-05** (Modules 03+04
Workshop Review Panel batch, AI/ML Practitioner persona): this or-pattern is not "the same collapse
as the original `_` wildcard" - it still names both variants, so it still fails to compile if a fifth
`OrderStatus` variant is added, unlike the wildcard. Pedantic's suggestion here costs the
per-variant readability signal, not the exhaustiveness guarantee. `cargo clippy -- -W
clippy::wildcard_enum_match_arm` on the *naive* version correctly flags the `_` arm by name; on the
fixed version, silent. Full output, both versions, all three lint checks: `transcript.txt`.

**Step 5 (does a green test/clippy run prove the modeling holds up under change?):** Confirmed
directly: both the naive and fixed versions compile and (if a test suite existed here) would pass it
identically for the four `OrderStatus` variants that exist today. The difference only shows up if a
fifth variant is ever added - which is exactly the playbook's point: today's green run is not
evidence the modeling will still be correct after that change.

## Conclusion

The playbook generalized: applied cold to an order/refund domain with zero relationship to `relay` or
checkpoints, it correctly identified the same class of `_`-wildcard risk, and independently
reproduced the `clippy::pedantic`-flags-explicit-arms-as-duplicative finding on a second, unrelated
example - strengthening confidence that finding is a property of the lint's own logic (identical-body
arms get flagged for merging, regardless of *why* they were kept separate), not a one-off quirk of
`relay`'s
specific enums.
