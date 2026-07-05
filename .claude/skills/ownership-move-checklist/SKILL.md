---
name: ownership-move-checklist
description: Diagnose whether Rust code actually needs to clone a value, or is cloning out of habit because it borrowed when it could have owned. Use before adding a .clone() call, when the borrow checker complains and .clone() looks like the fastest fix, or when reviewing a diff that clones something that came from an owned input.
---

# Ownership / move checklist

The Module 01 takeaway: the reasoning from a real exercise (`runs/2026-07-05-module-01-dry-run/`,
`merge_customer_totals`), made loadable instead of one-off. The exercise's real finding: a
solution that clones defensively passes `cargo test` and default `cargo clippy` exactly as cleanly
as one that doesn't. Neither check catches this; you have to ask the question yourself.

## Checklist, in order

1. **Before reaching for `.clone()`, ask: do I already own this?** Trace the value back to where
   it entered the current scope. If it arrived by value (a function parameter that's `T`, not
   `&T`; a `for x in collection` loop over an owned `collection`, not `&collection`), you already
   own it and everything inside it. Ownership doesn't stop at the top level: if you own a `struct`,
   you own its fields too, and can move them out individually.
2. **If you own it, ask why you're borrowing it instead.** The `merge_customer_totals` exercise's
   naive attempt iterated `for order in &orders` purely out of habit, even though `orders: Vec<Order>`
   was owned outright and never used again after the loop. Borrowing something you own, when
   nothing later needs it back, is the single most common source of an "unnecessary" clone: the
   borrow forces the clone, and the borrow wasn't needed in the first place.
3. **Rewrite the borrow to a move and see what happens.** Concretely: change `for x in &collection`
   to `for x in collection`, or change a `&T` parameter to `T`, and delete the `.clone()` that was
   compensating for it. Two outcomes, both informative:
   - **It compiles.** The clone was never load-bearing; you were cloning to route around a
     self-imposed borrow, not because two things genuinely needed the same value at once.
   - **It doesn't compile.** Read the actual error. If it says the value is used again later in a
     way that requires the original to still exist, that's a real, necessary clone (or a sign the
     function's shape needs a genuine rethink, e.g. returning borrowed data instead). Don't
     conclude "the compiler is wrong"; it's never wrong about this.
4. **Don't trust a green `cargo test` or a clean `cargo clippy` as proof the clone was necessary.**
   Confirmed directly in the exercise this Skill comes from: a version with a genuinely
   unnecessary clone passed both, identically to the version without one. Neither tool asks "did
   you need to borrow in the first place," they only check "does this compile and does it pass."
   That question is yours to ask.
5. **`clippy::pedantic`'s `needless_pass_by_value` is a real but partial signal, not a substitute
   for step 1-3.** It fires when a by-value parameter is never consumed by the function body, a
   proxy for the same root problem this checklist targets, but bundled with unrelated stylistic
   lints and not run by most projects' default CI. Useful as a second opinion; not something to
   rely on instead of tracing ownership yourself.

## When to reach for this

Any time you're about to write `.clone()` on a value that arrived in the current scope by value
(a function parameter, a loop variable, a struct field you already own). Not needed when the value
genuinely arrived by reference and you need an owned copy to outlive the borrow, that's a real
clone, not the habit this checklist exists to catch.
