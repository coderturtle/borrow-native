---
name: custom-error-type-template
description: Design a Rust error type a caller can actually match on and recover a real underlying cause from, and propagate it with `?` instead of hand-rolled match boilerplate or an early-stringified catch-all. Use before writing `enum FooError`, before converting a lower-level error to a message with `.to_string()` the moment it's caught, or when reviewing a diff that adds a `Result`-returning function.
---

# Custom-error-type template

The Module 05 takeaway: the reasoning from a real exercise
(`runs/2026-07-05-module-05-dry-run/`, `parse_config`/`write_handoff_summary`), made loadable
instead of one-off. The exercise's real finding: an error type that converts a lower-level error
(`std::io::Error`) to a `String` the instant it's caught compiles, passes every test, and lints
identically to one that wraps the real error and exposes it through `source()` - the deterministic
gate can't see which one a caller could actually recover structured information from afterward. A
second, lighter check found the opposite result for a related mistake: skipping `?` for a
same-type `Result -> Result` manual match *is* caught by default `clippy::question_mark`, unlike
every one of Modules 01-04's own naive shapes. **Uses `thiserror`** (added to `relay`'s `Cargo.toml`
same-session, coderturtle's explicit approval - this project's `dependency_changes: human_required`
governance gate is why it wasn't added by default) rather than a hand-written `impl Error`/`Display`;
the finding was re-verified fresh against the `thiserror`-derived code, not assumed to carry over,
and turned out *starker* than the original manual-`impl` version - see step 3.

## Playbook, in order

1. **Design the error type as an enum with one variant per distinct thing that can go wrong, not one
   catch-all wrapping a message.** `ConfigError::MissingKey(String)` /
   `InvalidInterval { key, value }` / `UnknownNotifier(String)` let a caller `match` and react
   differently per cause (retry with a default, refuse to start, log and continue) - a single
   `ConfigError(String)` variant would still compile and still produce a correct-looking `Display`
   message, but forces every caller into the same generic handling regardless of what actually
   failed.
2. **When a variant wraps a lower-level error, wrap the real error type - don't stringify it at the
   point you catch it.** `HandoffError::Io(std::io::Error)`, not `Io(String)` built via
   `e.to_string()`. The string-early version reads as "handled" (there's still an `Err`, the message
   is still correct) while quietly discarding the original error's structure (its `ErrorKind`, its
   own `source()` chain) - a decision that looks identical in every test this exercise's own suite
   runs, and only shows up the moment something downstream needs more than a message (matching on
   `ErrorKind`, retrying only on a specific failure, logging a full chain).
3. **Mark the wrapping field `#[from]` so `thiserror` derives both `From<LowerError>` and a working
   `source()` in one attribute, rather than writing either by hand.** `Io(#[from] std::io::Error)`
   on a `#[derive(thiserror::Error)]` enum gets you `?`-compatible conversion (`std::fs::write(path,
   content)?` compiles unchanged) *and* `source()` returning the wrapped error - less ceremony than
   a manual `impl From` plus a manual `source()` override, and the natural reason the "keep the real
   error" path is worth taking once `thiserror` is available: it's now the *cheaper* correct answer,
   not merely the more thorough one. (No `thiserror`? A manual `impl std::error::Error for
   HandoffError { fn source(&self) -> Option<&(dyn Error + 'static)> { match self { HandoffError::Io(
   inner) => Some(inner) } } }` plus `impl From<std::io::Error> for HandoffError` gets the same
   result, just with more lines - real cons of not using the dependency: more boilerplate to write
   and keep in sync by hand, and a shape less immediately recognizable to a reviewer familiar with
   `thiserror`'s conventions.)
4. **Checked directly, not assumed to carry over: after migrating from a manual `impl` to
   `#[derive(thiserror::Error)]`, re-run the full `clippy::restriction` group rather than assuming
   the earlier finding still holds.** The manual-`impl` version of this exercise found one weak,
   generically-aimed partial signal - `clippy::missing_trait_methods` flagging a `source()`-less
   `impl Error` as "missing" a default method. That signal **disappears entirely** once the `Error`
   impl is derived instead of hand-written (0 occurrences on either attempt, re-verified with
   `--message-format=json`, not eyeballed) - `thiserror`'s derive macro doesn't trigger this lint at
   all. A dependency migration can silently remove a lint signal you were relying on, not just add
   or preserve one; check, don't assume, after any such migration.
5. **Checked directly, not assumed: default `cargo clippy -- -D warnings`, `clippy::pedantic`, and
   `clippy::nursery` give zero signal on whether an error variant preserves a real underlying error
   or stringifies it early - identical output on both shapes, before and after the `thiserror`
   migration.** This now matches [[trait-lifetime-cheatsheet]]'s finding exactly rather than
   differing in degree: after the migration, *nothing* in `clippy::restriction` rescues this finding
   either (see step 4) - no lint at any level checked is a substitute for actually asking "can a
   caller recover the original cause from this error, structurally, not just read a message."
6. **A hand-rolled `match { Ok(v) => v, Err(e) => return Err(e) }` with no error transformation *is*
   caught by default clippy (`clippy::question_mark`, part of the "style" group - no opt-in
   needed) - but only when the match passes the same error type straight through.** Checked
   directly: this fires on a same-type `Result -> Result` passthrough, but stays silent on the
   mechanically similar `Option -> Result` case (`None => return Err(..)`), and - checked again on a
   second, unrelated problem during this Skill's own takeaway validation
   (`runs/2026-07-05-module-05-dry-run/takeaway-validation/`) - stays silent on a manual match that
   *converts* one error type into another (e.g. `ParseIntError -> RetryPolicyError`) too, since
   that's no longer the same-type passthrough shape this lint targets. Unlike every naive shape
   Modules 01-04 found, part of this module's `?`-avoidance mistake genuinely does not need
   Coachgremlin's conceptual tier to catch it - but check which exact shape of manual match you're
   looking at (same-type passthrough vs. type-converting) before assuming the default lint set has
   it covered.

## When to reach for this

Any time you're about to write `enum FooError` for a function that can fail in more than one way
(check step 1 before defaulting to a single `String`-wrapping variant), any time you're about to
write `.map_err(|e| MyError::Whatever(e.to_string()))` (check step 2 - would a `#[from]`-wrapped
variant work instead, per steps 2-3), any time a dependency you rely on for a lint signal gets
added/removed/upgraded (check step 4 - re-verify, don't assume the signal still holds), or any time
you're reviewing a diff that adds a `Result`-returning function and want to know whether a manual
`match` is doing real work or could just be `?` (check step 6 - default clippy may already tell you,
for at least the same-type-passthrough case).
