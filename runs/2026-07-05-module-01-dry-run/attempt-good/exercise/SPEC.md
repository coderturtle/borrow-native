# Exercise: `merge_customer_totals`

## Task

Implement `merge_customer_totals` in `src/lib.rs`. It takes ownership of a `Vec<Order>` and
returns a `HashMap<String, u64>` mapping each distinct customer name to the sum of their
order totals (in cents).

```rust
pub struct Order {
    pub customer: String,
    pub total_cents: u64,
}

pub fn merge_customer_totals(orders: Vec<Order>) -> HashMap<String, u64> {
    todo!()
}
```

## Edge cases the tests check

1. **Empty input** — an empty `Vec<Order>` produces an empty map.
2. **Single order** — one order produces a one-entry map.
3. **Duplicate customer, same case** — two orders from `"Alice"` sum into one entry.
4. **Multiple distinct customers** — each customer gets their own entry; totals don't leak
   across customers.
5. **Case sensitivity** — `"Alice"` and `"alice"` are different customers, not the same one
   normalized. No accidental case-folding.
6. **Zero-cent order** — an order with `total_cents: 0` still creates (or contributes zero to)
   an entry for that customer; it isn't silently dropped.

## The actual point of this exercise

This is Module 01. The test suite above checks correctness. It does **not** check *how* you got
there, and that's deliberate: a solution that clones every `customer` string defensively will
pass every test here. The real gate for this module is stated in
[`../README.md`](../README.md): `cargo test` green is the deterministic tier, but Coachgremlin's
conceptual tier checks whether you actually moved the input's `String`s into the result, or
cloned them because you weren't sure the compiler would let you move them.

Concretely: `orders: Vec<Order>` is passed by value. You own it. Nothing in this function
signature needs it back afterward. That means every `Order` in it, and every field of every
`Order`, is yours to move, not just borrow. If your solution reaches for `.clone()` anywhere on
`order.customer`, ask first: is there a version of this loop where I never needed a reference to
begin with?
