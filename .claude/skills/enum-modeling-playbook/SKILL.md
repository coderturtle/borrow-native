---
name: enum-modeling-playbook
description: Diagnose whether Rust data is modeled so illegal states are unrepresentable, and whether a match against an enum is exhaustive on purpose rather than by accident. Use before adding a bool/Option flag to a struct where only one combination of flags is ever valid, before writing a `_` wildcard arm in a match over an enum you own, or when reviewing a diff that adds a new enum variant.
---

# Enum-modeling & exhaustive-match playbook

The Module 03 takeaway: the reasoning from a real exercise (`runs/2026-07-05-module-03-dry-run/`,
`next_action`), made loadable instead of one-off. The exercise's real finding: a `match` arm that
uses `_` to cover several enum variants that currently resolve the same way passes `cargo test` and
default `cargo clippy` exactly as cleanly as one that lists every variant explicitly - and escalating
to `clippy::pedantic` doesn't just fail to help here, it actively suggests collapsing the explicit
version into the wildcard one. Neither check catches the actual risk; you have to ask the question
yourself.

## Playbook, in order

1. **Before adding a `bool` or `Option<T>` field to a struct, ask: can more than one of these ever be
   true at once, or none of them?** If the honest answer is "no, exactly one of these is always true,"
   that's an enum trying to get out from inside a struct. `relay`'s `CheckpointTrigger` is one enum
   with three variants, not three `Option<u64>`/`Option<u32>`/`Option<f64>` fields on a struct - a
   checkpoint fires for exactly one reason, and the type should make any other combination
   unrepresentable, not just conventionally-avoided.
2. **A struct-plus-flags version and an enum version can behave identically on every test you happen
   to write.** Tests exercise valid inputs; a flags-based struct's actual weakness is the *invalid*
   combinations it can still construct (both flags true, or none), which a test suite built only
   around intended usage may never construct either. The absence of a failing test is not evidence
   the modeling choice was safe - see [[borrow-checker-playbook]] for the identically-shaped lesson
   applied to cloning instead of modeling.
3. **Before writing a `_` wildcard arm in a match over an enum you own, ask: am I covering these
   variants because they're conceptually the same case, or because it's less typing?** `next_action`'s
   naive attempt wrote `_ => NextAction::Continue` for `TimeElapsed`/`ToolCallCount` under `Ignored` -
   correct for both variants today, but indistinguishable in the compiled behavior from "I didn't
   think about whether a new variant should land here too." Listing `TimeElapsed(_) => Continue` and
   `ToolCallCount(_) => Continue` as separate arms costs a few extra characters and buys a compile
   error the moment a new trigger variant is added anywhere in the codebase, forcing a deliberate
   decision about that arm instead of a silent default.
4. **Don't trust `clippy::pedantic` to praise this decision - checked directly, it flags it as
   duplication, though not as dangerously as it first looks.** `next_action`'s good attempt (explicit
   arms) triggers pedantic's `match_same_arms` lint, which recommends merging the identical-body arms
   into an or-pattern. Verified by compiling both forms after adding a new enum variant: the
   or-pattern still names every variant explicitly, so it still fails to compile on the new one,
   exactly like the two-separate-arms version - following pedantic's suggestion here does not
   reintroduce the wildcard's exhaustiveness gap, only its per-variant readability. The lint that
   actually catches the wildcard risk, `clippy::wildcard_enum_match_arm`, lives in clippy's
   `restriction` group: off by default, not part of `pedantic` or `nursery`, and meant to be enabled
   individually rather than as a bundle (clippy's own docs note restriction lints can conflict with
   idiomatic style). If a workshop or codebase cares about this property, that specific lint needs to
   be turned on by name - inheriting
   it from a broader lint group won't happen.
5. **A green `cargo test` and a clean `cargo clippy -- -D warnings` prove today's behavior is right,
   never that the modeling will still be right after the next change.** This is the same caution
   [[borrow-checker-playbook]] and [[ownership-move-checklist]] each name for their own failure mode,
   applied here to a third mechanism: exhaustiveness erosion instead of defensive cloning. All three
   are invisible to the deterministic gate for the same underlying reason - they're properties of the
   code's shape, not of any input/output pair a test observes.

## When to reach for this

Any time you're modeling a value that has a small number of genuinely mutually-exclusive states
(reach for an enum, not flags), or writing a `match` over an enum you control and reaching for `_` to
cover more than one variant (ask whether that's a real "these are the same case" or just fewer
keystrokes). Not needed when a wildcard genuinely means "any variant, handled the same way, including
ones that don't exist yet and reasonably shouldn't need a new decision" - that's a legitimate use of
`_`, not the habit this playbook exists to catch.
