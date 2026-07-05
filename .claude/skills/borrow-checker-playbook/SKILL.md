---
name: borrow-checker-playbook
description: Diagnose whether Rust code actually needs to clone or take ownership of a value, or is doing so out of habit because it's not sure what a shared reference already lets it do. Use before adding a .clone() call on data reached through a &T, before widening a &T parameter to &mut T or an owned T, or when reviewing a diff that clones a whole collection just to read it.
---

# Borrow-checker diagnostic playbook

The Module 02 takeaway: the reasoning from a real exercise (`runs/2026-07-05-module-02-dry-run/`,
`session_stats`), made loadable instead of one-off. The exercise's real finding: a solution that
clones an entire borrowed collection before reading it passes `cargo test` and default `cargo
clippy` exactly as cleanly as one that reads through the reference directly - and escalating to
`clippy::pedantic` gives *zero* additional signal here (unlike the companion finding in
[[ownership-move-checklist]], where pedantic at least caught a proxy lint). Neither check catches
this; you have to ask the question yourself.

## Playbook, in order

1. **Before writing `&mut T`, ask: am I actually going to mutate this, or just read it?** `&T` is
   the default; reach for `&mut T` only once you have a concrete line of code that assigns through
   it or calls a `&mut self` method. A function that only ever reads its input doesn't need
   exclusive access, and asking for it needlessly blocks every other reader while your borrow is
   alive.
2. **Before cloning something you were only given a reference to, ask: can I compute what I need
   directly through the reference?** The `session_stats` exercise's naive attempt built an owned
   `Vec<CheckpointRecord>` by cloning every field of every checkpoint in `&session.checkpoints`,
   then worked from that copy - never asking whether `session.checkpoints.iter()` could already do
   everything it needed. It could: sums, maxima, and comparisons all work fine over `&T` items: you
   only need to own something once you have to return it or store it somewhere that outlives the
   borrow.
3. **If you do need an owned value out of a borrow, that's a real clone, not a bug.** `session_stats`
   returns `longest_gap_goal: String`; its containing type has no lifetime parameter, so the value
   must be owned to leave the function - cloning `checkpoint.goal` there is correct given that
   shape (a lifetime-annotated return type would remove even this clone, a later-module concern).
   The lesson isn't "never clone under a borrow," it's "clone only the specific piece you need to
   keep, not the whole borrowed structure as a defensive first step."
4. **Read a borrow-checker error for which of the two core rules it's actually enforcing**: at any
   point, either one `&mut` reference, or any number of `&` references - never both at once, and
   never a `&mut` while a `&` you're still using exists. Most "fights with the borrow checker" are
   one of these two rules, not a bug in the checker.
5. **Fix a real conflict by restructuring scope or splitting the borrow, not by cloning to sidestep
   it.** Shorten a borrow's lifetime by ending it before the next one starts (an inner block, or
   dropping a variable early), or borrow disjoint fields separately (`&mut a.x` and `&a.y` can
   coexist; `&mut a` and `&a.y` cannot) before reaching for `.clone()` to make the checker stop
   complaining.
6. **Don't trust a green `cargo test` or a clean `cargo clippy` as proof a clone or an upgraded
   borrow was necessary.** Confirmed directly in this exercise: a version that clones an entire
   collection defensively passes both, identically to a version that reads through the reference
   directly - and unlike [[ownership-move-checklist]]'s finding, `clippy::pedantic` doesn't even
   give a noisy partial signal here. That question is yours to ask, every time, regardless of lint
   level.

## When to reach for this

Any time you're about to clone data you only have a `&`/`&mut` reference to, or about to widen a
function's parameter from `&T` to `&mut T` or owned `T` because a borrow "isn't working." Not
needed when you already know a value must be stored or returned beyond the borrow's lifetime - that
clone is real, not the habit this playbook exists to catch.
