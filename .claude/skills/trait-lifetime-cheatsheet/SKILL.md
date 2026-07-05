---
name: trait-lifetime-cheatsheet
description: Decide whether a Rust function needs an explicit lifetime annotation, or is already covered by elision - and whether a generic function's trait bound is doing real work or hiding a dyn/concrete-type shortcut. Use before adding `'a` anywhere, before reaching for `.clone()`/`.to_string()`/`'static` to make a lifetime error disappear, or when reviewing a diff that adds a generic function returning a reference-shaped type.
---

# Trait-bound/lifetime-annotation cheat-sheet

The Module 04 takeaway: the reasoning from a real exercise (`runs/2026-07-05-module-04-dry-run/`,
`alert_checkpoint`), made loadable instead of one-off. The exercise's real finding: a struct field
that clones data to avoid ever declaring a lifetime, and a struct/function that declares a lifetime
but applies it more broadly than the data actually requires, both compile and pass `cargo test` and
`cargo clippy` (default, `pedantic`, and `nursery`) identically to the correctly-scoped borrowed
version. Unlike Module 03's wildcard-match finding, no off-by-default `restriction` lint closes this
gap either - checked directly, not assumed. Only the module's own clone-count pre-filter, once
calibrated against the reference implementation's own diff, happened to catch this particular
instance, because this mistake is clone-shaped; a differently-shaped avoidance (e.g. `'static`, or
leaking data through a global) would not be.

## Playbook, in order

1. **Before annotating a lifetime anywhere, check whether elision already covers it.** Elision has
   exactly three rules: every elided input reference gets its own lifetime; if there's *exactly one*
   input reference, the output borrows from it; if one of the inputs is `&self`/`&mut self`, the
   output borrows from that. `Session::label(&self) -> &str` needs nothing explicit - rule 3 covers
   it. A free function with no `self` and more than one reference-shaped input, whose output borrows
   from one of them, has no elision rule left to apply - that, and only that, is when the compiler
   actually requires you to write one.
2. **When an explicit lifetime is genuinely required, tie it to only the reference the output
   actually borrows from - not to every reference parameter "to be safe."** `alert_checkpoint`'s
   correct signature is `fn alert_checkpoint<'a, N: Notifier>(notifier: &N, session: &'a Session,
   trigger: &CheckpointTrigger) -> CheckpointAlert<'a>` - `'a` connects only `session` to the output,
   because only `session.label` is borrowed into it. Unifying `notifier` and `trigger` under the same
   `'a` compiles and passes every test in this exercise identically (checked directly,
   `runs/2026-07-05-module-04-dry-run/grading.md`'s over-annotation check) - and still over-constrains
   any real caller whose three arguments don't happen to share a lifetime already. Passing today's
   tests is not evidence the broader annotation was harmless, the same caution
   [[enum-modeling-playbook]] names for a wildcard match arm that happens to be correct for today's
   variants.
3. **Before cloning or converting a borrowed value to owned specifically to dodge a lifetime
   annotation, ask whether the value actually needs to outlive the reference it came from.** If it
   doesn't - if the struct holding it is dropped, read, and discarded within the same scope the
   original reference is still valid - an explicit lifetime tying the two together is not defensive
   over-engineering, it's the correct, zero-copy shape. `CheckpointAlert`'s naive shape
   (`session_label: String`, built via `session.label.clone()`) and its correct shape
   (`session_label: &'a str`, built via `session.label()`) behave identically for every test in this
   exercise - the clone isn't caught by behavior, only by asking this question directly.
4. **Checked directly, not assumed: no clippy lint at any level (default, `pedantic`, `nursery`, or a
   manually-enabled `restriction` lint search) flags a struct that owns/clones data it could instead
   borrow with a lifetime.** This differs from [[enum-modeling-playbook]]'s finding, where an
   off-by-default `restriction` lint (`clippy::wildcard_enum_match_arm`) at least existed to catch the
   analogous smell once enabled by name. Here, whether a field should borrow or own is an API-design
   question about a type's own shape - outside what a per-function, local-dataflow lint pass
   evaluates at any strictness level tried.
5. **A generic function's trait bound (`<N: Notifier>`) and a `&dyn Notifier` parameter can pass the
   exact same test suite.** Both compile, both dispatch correctly to a test double, and nothing in
   `cargo test`/`cargo clippy` distinguishes "generic, statically dispatched, monomorphized per
   caller" from "trait object, dynamically dispatched, one shared vtable" for a test file that only
   ever calls the function with one concrete type per call site. The distinction (dispatch cost,
   whether multiple concrete types can be mixed in one collection, whether the function can be object-
   safe at all) is a design choice checked by reading the signature, not by a green test run.
6. **A green `cargo test` and a clean `cargo clippy -- -D warnings` (even escalated to `pedantic` and
   `nursery`) prove today's shape compiles and behaves correctly - never that it's the shape a
   reviewer would actually want.** Same caution [[ownership-move-checklist]] and
   [[enum-modeling-playbook]] each name for their own mechanism (defensive cloning, exhaustiveness
   erosion), applied here to a fourth: lifetime avoidance and lifetime over-application are both
   invisible to every deterministic tier tried, for the same underlying reason - they're properties of
   a type's own shape and a caller's future flexibility, not of any input/output pair a test observes.

## When to reach for this

Any time you're about to write `<'a>` anywhere (check whether elision already covers it first, per
step 1), any time a lifetime error makes `.clone()`/`.to_string()`/`'static` look like the fastest way
to make it go away (check step 3 before taking that exit), or any time you're reviewing a generic
function's signature and want to know whether its trait bound and lifetime parameters are doing real
work or copy-pasted defensively. Not needed for a method with `&self`/`&mut self` and a single
borrowed output - elision already has that covered, and adding an explicit lifetime there is noise,
not rigor.
