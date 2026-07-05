# Spec: `relay`

This is the authoritative spec for `relay`, the one project every module in Borrow Native's arc
builds a real feature onto (Module 01 adds core domain types and `finalize_session`; Module 02 adds
read-only session statistics; Module 03 adds trigger/response modeling; Module 04 adds a pluggable
notifier; Module 05 adds config/IO error handling; Module 06 adds concurrent multi-session tracking;
Module 07 adds async checkpoint waiting; Module 08's capstone diagnoses a real seeded bug in the
accumulated result). Keep this file as the single source of truth: don't restate the product pitch
differently inside a module README.

**This differs from `terminal-velocity/fixtures/receipts/`'s shape on purpose.** That fixture is one
static function engineered five different ways (prompted, curated-around, harnessed, fixed,
sabotaged) because Terminal Velocity's subject (agentic engineering practice) is five lenses on one
problem. `relay` grows one real feature per module instead, because Rust's own concept arc (ownership
→ borrowing → structs/enums → generics/traits/lifetimes → error handling → concurrency → async) is an
additive dependency chain: each concept is genuinely required to build the next feature, not a
different lens on a feature that already exists.

## The product

`relay` is a CLI for **hybrid human-agent team pacing**: bounded work-unit checkpoints (a
pomodoro-style rhythm, sized for a coding-agent session rather than a human's 25 minutes) that each
produce a **restartable-handoff summary** (goal, changes, verification evidence, open risks) and
notify a human that a checkpoint was reached - a direct implementation of the "Restartable Handoff
Loop" pattern named in `terminal-velocity/docs/workshop-design.md`'s own research, this time as a
real, running tool instead of a described pattern.

**Why this, not a multi-agent cockpit:** `agent-mission-control-lab` already exists in this factory
building a full multi-agent session cockpit. `relay` is deliberately narrower: one CLI that answers
"has this session reached a natural checkpoint, and if so, what's the handoff," not a dashboard for
orchestrating many agents at once. Module 06 does add multi-session *tracking* (several concurrent
`relay` instances reporting into one place), but that's in service of the same narrow question asked
across sessions, not a reimplementation of the cockpit's own scope.

## Feature build-out (one module, one feature, in order)

| # | Module | Feature | Status |
|---|---|---|---|
| 01 | Ownership & Move Semantics | Core domain types (`DraftCheckpoint`, `CheckpointRecord`, `Session`) and `finalize_session`, sealing a session's drafts into records | **Exercise authored + dry-run complete** (`runs/2026-07-05-module-01-dry-run/`, `runs/2026-07-05-module-01-relay-dry-run/`). Stub + tests shipped; learner's own implementation intentionally left as the open exercise. |
| 02 | Borrowing & References | Session statistics (average/longest checkpoint gap) computed by borrowing session history | **Exercise authored + dry-run complete** (`runs/2026-07-05-module-02-dry-run/`). Stub + tests shipped; learner's own implementation intentionally left as the open exercise. |
| 03 | Structs, Enums & Pattern Matching | `CheckpointTrigger` (time/tool-count/context-budget) and human-response (`Acknowledged`/`Snoozed`/`Ignored`) enums | **Exercise authored + dry-run complete** (`runs/2026-07-05-module-03-dry-run/`). Stub + tests shipped; learner's own implementation intentionally left as the open exercise. |
| 04 | Generics, Traits & Lifetimes | `Notifier` trait (desktop/terminal-bell/stdout), implemented generically | **Exercise authored + dry-run complete** (`runs/2026-07-05-module-04-dry-run/`). Stub + tests shipped; learner's own implementation intentionally left as the open exercise. |
| 05 | Error Handling | Config parsing and handoff-summary file I/O with real `Result`/custom error types | **Exercise authored + dry-run complete** (`runs/2026-07-05-module-05-dry-run/`). Stub + tests shipped; learner's own implementation intentionally left as the open exercise. |
| 06 | Fearless Concurrency | Tracking multiple concurrent sessions reporting into one tracker | Not started |
| 07 | Async Programming | Checkpoint trigger waits on a timer, a session-log file-watch, and a signal concurrently | Not started |
| 08 | Synthesis capstone | A real, seeded bug in the accumulated project; diagnose the root-cause concept and fix it | Not started |

## Module 01: `finalize_session`

### Signature

```rust
pub struct DraftCheckpoint {
    pub goal: String,
    pub summary: String,
    pub risks: Vec<String>,
    pub elapsed_secs: u64,
}

pub struct CheckpointRecord {
    pub goal: String,
    pub summary: String,
    pub risks: Vec<String>,
    pub elapsed_secs: u64,
}

pub struct Session {
    pub label: String,
    pub checkpoints: Vec<CheckpointRecord>,
}

pub fn finalize_session(label: String, drafts: Vec<DraftCheckpoint>) -> Session {
    todo!()
}
```

- `label`: a human-chosen name for the session (e.g. `"module-01-async-rewrite"`). Owned, moved into
  the returned `Session`.
- `drafts`: every checkpoint drafted during the session, in the order they were reached. Owned,
  consumed: each `DraftCheckpoint` is sealed into a `CheckpointRecord` by moving its fields, not
  cloning them.
- `elapsed_secs`: seconds since the previous checkpoint (0 for the first). Added retroactively when
  Module 02 landed (see that module's section below and `.hekton/governance.yaml`'s ARB trigger for
  this file) - a `u64` is `Copy`, so it carries no move-semantics teaching weight of its own; it just
  needs to be preserved, not dropped, same as `risks`.

### Required edge cases

1. **Empty drafts.** `drafts == []` produces a `Session` with `checkpoints == []`. `label` is still
   set correctly.
2. **Single draft.** One draft produces a one-record session, fields preserved exactly.
3. **Multiple drafts preserve order.** Records appear in `Session.checkpoints` in the same order the
   drafts were given, not reordered.
4. **Empty risks are preserved, not dropped.** A draft with `risks: vec![]` produces a record with
   `risks: vec![]`, not an omitted field or a panic.
5. **Risks with multiple entries are preserved in order**, not deduplicated or reordered.

### The actual point of this exercise

Same lesson as the workshop's first dry run (`runs/2026-07-05-module-01-dry-run/`, when this
exercise was still a standalone `merge_customer_totals` toy, before the pivot to this shared
project): `drafts: Vec<DraftCheckpoint>` arrives owned. Every `DraftCheckpoint` and every field of
every `DraftCheckpoint` in it is yours to move. A solution that clones `goal`/`summary`/each `risks`
entry to sidestep the borrow checker will pass every test above identically to one that doesn't; see
`modules/01-ownership-move-semantics/README.md` for the full rubric, including the two-tier gate this
project's every module uses.

## Module 02: `session_stats`

### Signature

```rust
pub struct SessionStats {
    pub checkpoint_count: usize,
    pub average_gap_secs: f64,
    pub longest_gap_secs: u64,
    pub longest_gap_goal: String,
}

pub fn session_stats(session: &Session) -> SessionStats {
    todo!()
}
```

- `session`: borrowed, not consumed. Every module before and after this one still needs to be able to
  use the same `Session` value; `session_stats` only ever needs to read it.
- `average_gap_secs`/`longest_gap_secs`: computed directly over each `CheckpointRecord.elapsed_secs`
  in `session.checkpoints` - no wall-clock reads, so results are deterministic and testable.
- `longest_gap_goal`: the `goal` of the checkpoint with the longest gap. Returned as an owned
  `String` on purpose - `SessionStats` has no lifetime parameter (lifetimes arrive in Module 04), so
  this value must be owned to leave the function. Given that type shape, cloning it here once is
  the correct move, not a habit to unlearn - a `SessionStats<'a>` borrowing instead would remove
  even this clone, out of scope for this module.

### Required edge cases

1. **Empty session.** No checkpoints produces `checkpoint_count: 0`, `average_gap_secs: 0.0`,
   `longest_gap_secs: 0`, `longest_gap_goal: ""` - not a panic and not `NaN` from a `0.0 / 0.0`.
2. **Single checkpoint.** One checkpoint produces `checkpoint_count: 1`, `average_gap_secs` and
   `longest_gap_secs` both equal to that checkpoint's own `elapsed_secs`, `longest_gap_goal` equal to
   its `goal`.
3. **Multiple checkpoints, average computed correctly.** The mean of every checkpoint's
   `elapsed_secs`, not just the last one or a running total.
4. **Longest gap identifies the right checkpoint**, not just the right number - `longest_gap_goal`
   must match the checkpoint whose `elapsed_secs` is the max, not e.g. the last checkpoint.
5. **A tie for longest gap resolves to the first occurrence**, not the last - deterministic, stated
   up front rather than left to whatever a given implementation happens to do.
6. **`session` is usable again after the call** - since it's borrowed, not consumed, a test may call
   `session_stats(&session)` and then still read `session.label` or `session.checkpoints` afterward.

### The actual point of this exercise

See `runs/2026-07-05-module-02-dry-run/` for the real dry run and its finding, and
`modules/02-borrowing-references/README.md` for the full rubric.

## Module 03: `next_action`

### Signature

```rust
pub enum CheckpointTrigger {
    TimeElapsed(u64),
    ToolCallCount(u32),
    ContextBudget(f64),
}

pub enum HumanResponse {
    Acknowledged,
    Snoozed(u64),
    Ignored,
}

pub enum NextAction {
    Continue,
    PauseFor(u64),
    EndSession,
}

pub fn next_action(trigger: &CheckpointTrigger, response: &HumanResponse) -> NextAction {
    todo!()
}
```

- `CheckpointTrigger`/`HumanResponse`/`NextAction` are all given (declared in `src/lib.rs` already) -
  this module's exercise is the match logic in `next_action`, not the type design. The type design
  itself is worth reading before starting: each is an enum, not a struct with `bool`/`Option` flags,
  because at most one variant can ever be true at a time for each of these - a checkpoint fires for
  exactly one reason, a human gives exactly one response, `relay` takes exactly one next action.
  Modeling any of these three as flags would let the type represent a state that can never actually
  happen (e.g. a response that's simultaneously `Acknowledged` and `Snoozed`), which the compiler
  then has no way to rule out for you.
- `trigger`/`response`: both borrowed, not consumed - `next_action` only needs to read which variant
  each is (and, for `TimeElapsed`/`ToolCallCount`/`Snoozed`, the value it carries), never to own
  either. Reuses Module 02's lesson: read through the reference, don't clone to feel safer about it.

### Rule table (the actual spec for `next_action`)

1. `Acknowledged` → always `Continue`, regardless of which `CheckpointTrigger` fired. The human has
   handled it; keep working.
2. `Snoozed(secs)` → always `PauseFor(secs)`, regardless of trigger. The human asked for a break of
   that length; honor it.
3. `Ignored` + `ContextBudget(_)` → `EndSession`. Ignoring a blown context budget isn't safe to just
   continue past - the session's context will keep degrading. `relay` must not let that ride on a
   human's silence.
4. `Ignored` + `TimeElapsed(_)` → `Continue`. Ignoring a time-based nudge is fine; soft reminder, no
   real risk in proceeding.
5. `Ignored` + `ToolCallCount(_)` → `Continue`. Same reasoning as (4) - a tool-count nudge is a
   pacing suggestion, not a safety boundary.

Only rule (3) singles out one specific trigger under `Ignored`; rules (4) and (5) resolve the same
way. Whether that's written as two explicit arms or one `_` catch-all arm under `Ignored` is exactly
this module's real exercise - see `modules/03-structs-enums-pattern-matching/README.md`'s rubric and
`runs/2026-07-05-module-03-dry-run/` for why it matters even though both produce an identical
`next_action` for every input rules (1)-(5) describe.

### Required edge cases

1. **`Acknowledged` continues regardless of trigger** - checked against at least two different
   trigger variants, not just one, to confirm trigger truly has no effect on this arm.
2. **`Snoozed(secs)` pauses for exactly that duration, regardless of trigger.**
3. **`Ignored` + `ContextBudget` ends the session** - the safety-critical case; a wrong default here
   is the one behavioral bug this exercise can actually catch deterministically.
4. **`Ignored` + `TimeElapsed` continues.**
5. **`Ignored` + `ToolCallCount` continues.**

### The actual point of this exercise

See `runs/2026-07-05-module-03-dry-run/` for the real dry run and its finding, and
`modules/03-structs-enums-pattern-matching/README.md` for the full rubric.

## Module 04: `alert_checkpoint`

### Signature

```rust
pub trait Notifier {
    fn notify(&self, message: &str) -> bool;
}

pub struct DesktopNotifier;
pub struct TerminalBellNotifier;
pub struct StdoutNotifier;
// all three implement Notifier already - see src/lib.rs

impl Session {
    pub fn label(&self) -> &str {
        &self.label
    }
}

pub struct CheckpointAlert {
    pub session_label: String,
    pub message: String,
    pub sent: bool,
}

pub fn alert_checkpoint<N: Notifier>(
    notifier: &N,
    session: &Session,
    trigger: &CheckpointTrigger,
) -> CheckpointAlert {
    todo!()
}
```

- `Notifier`, its three implementations, and `Session::label()` are all given (declared in
  `src/lib.rs` already) - this module's exercise is `alert_checkpoint`'s message-building logic and,
  specifically, whatever shape you give `CheckpointAlert`'s `session_label` field. The stub ships it
  as an owned `String` because that's the shape that compiles today with no lifetime parameter
  anywhere in sight - whether that's the shape you keep is exactly this module's real exercise. See
  `modules/04-generics-traits-lifetimes/README.md`'s rubric for what changing it actually buys you
  and what the compiler will and won't force on you either way.
- `notifier`: generic (`N: Notifier`), not `&dyn Notifier` or a concrete type - `alert_checkpoint`
  must work unchanged whether it's called with `DesktopNotifier`, `TerminalBellNotifier`,
  `StdoutNotifier`, or (in tests) a fourth, recording test double that never touches a real terminal.
- `session`: borrowed, not consumed - same discipline as Module 02/03's functions. Only `session`'s
  `label` is ever read.
- `trigger`: borrowed, not consumed. Determines the message text per the format table below.

### Message format (the actual spec for `alert_checkpoint`)

1. `TimeElapsed(secs)` → `format!("Checkpoint: {secs}s since last checkpoint")`.
2. `ToolCallCount(n)` → `format!("Checkpoint: {n} tool calls since last checkpoint")`.
3. `ContextBudget(frac)` → `format!("Checkpoint: {:.0}% of context budget used", frac * 100.0)` -
   rounded to the nearest whole percent (e.g. `0.983` → `"98%"`).

`alert_checkpoint` must call `notifier.notify(&message)` exactly once with that exact string, and
report back whatever `notify` actually returned in `CheckpointAlert.sent` - not an assumed `true`.
`session_label` must equal the `session` argument's own `label`, not a different session's.

### Required edge cases

1. **`TimeElapsed` message format is exact.**
2. **`ToolCallCount` message format is exact.**
3. **`ContextBudget` message format rounds to the nearest whole percent.**
4. **`sent` reflects the notifier's real return value**, including `false` - checked with a notifier
   configured to fail, not just one that always succeeds.
5. **`session_label` matches the session actually passed in** - checked against two different
   sessions, not one, to confirm it isn't hardcoded or read from the wrong value.
6. **The notifier is actually invoked, exactly once, with the built message** - checked via a
   recording test double, so a solution that fabricates a `CheckpointAlert` without ever calling
   `notify` fails this even though it might otherwise look plausible.

### The actual point of this exercise

See `runs/2026-07-05-module-04-dry-run/` for the real dry run and its finding, and
`modules/04-generics-traits-lifetimes/README.md` for the full rubric.

## Module 05: `parse_config` and `write_handoff_summary`

### Signature

```rust
pub struct RelayConfig {
    pub checkpoint_interval_secs: u64,
    pub notifier_kind: NotifierKind,
}

pub enum NotifierKind {
    Desktop,
    TerminalBell,
    Stdout,
}

#[derive(Debug)]
pub enum ConfigError {
    MissingKey(String),
    InvalidInterval { key: String, value: String },
    UnknownNotifier(String),
}
// Display + std::error::Error already implemented - see src/lib.rs

pub fn parse_config(input: &str) -> Result<RelayConfig, ConfigError> {
    todo!()
}

#[derive(Debug)]
pub enum HandoffError {
    Io(String), // shipped shape - whether you keep it is the exercise
}
// Display + std::error::Error already implemented for the shipped shape

pub fn write_handoff_summary(path: &std::path::Path, session: &Session) -> Result<(), HandoffError> {
    todo!()
}
```

- `RelayConfig`, `NotifierKind`, `ConfigError`'s three variants (with their `Display`/`Error` impls
  already written), and `HandoffError`'s `Io` variant name (with its `Display`/`Error` impls already
  written for whatever shape `Io` ends up holding) are all given - this module's exercise is the two
  functions' bodies, and, specifically, whatever inner type `HandoffError::Io` ends up holding. The
  stub ships `Io(String)` because that's the shape that compiles with the least ceremony
  (`.to_string()` the underlying `std::io::Error` immediately, at the point it's caught). Whether
  that's the shape you keep is exactly this module's real exercise - see
  `modules/05-error-handling/README.md`'s rubric for what changing it actually buys you.

### `parse_config` format (the actual spec)

Input: newline-separated `key=value` lines. Blank lines are skipped. Unrecognized keys are ignored
(forward-compatible: a config with an extra key not yet in `RelayConfig` isn't an error). Two
required keys:

1. `checkpoint_interval_secs` - must parse as a `u64`. Missing → `ConfigError::MissingKey`. Present
   but not a valid `u64` (e.g. `"soon"`, `"-5"`, empty string) → `ConfigError::InvalidInterval { key,
   value }` with `value` the exact string that failed to parse.
2. `notifier` - must be exactly one of `"desktop"`, `"bell"`, or `"stdout"` (case-sensitive),
   mapping to `NotifierKind::Desktop`/`TerminalBell`/`Stdout` respectively. Missing →
   `ConfigError::MissingKey`. Present but none of the three → `ConfigError::UnknownNotifier(value)`
   with `value` the exact string given.

`parse_config` must never panic on malformed input - every failure comes back as an `Err` naming
which specific thing was wrong, not a generic catch-all.

### `write_handoff_summary` format (the actual spec)

Renders `session`'s checkpoints into a plain-text handoff summary and writes it to `path` via
`std::fs::write`. One line per checkpoint, in checkpoint order:

```
- {goal}: {summary} (risks: {risks joined with ", ", or "none" if empty})
```

A real I/O failure (`path`'s parent directory doesn't exist, or any other `std::fs::write` error)
must come back as `Err(HandoffError::Io(..))`, never a panic and never a silently-swallowed success.

### Required edge cases

1. **A valid config with both keys parses to the exact expected `RelayConfig`** - both possible
   correct field combinations checked, not just one.
2. **Missing `checkpoint_interval_secs`** → `ConfigError::MissingKey("checkpoint_interval_secs")`.
3. **Missing `notifier`** → `ConfigError::MissingKey("notifier")`.
4. **Non-numeric `checkpoint_interval_secs` value** → `ConfigError::InvalidInterval` naming the
   exact key and the exact bad value given.
5. **Unrecognized `notifier` value** → `ConfigError::UnknownNotifier` naming the exact bad value
   given.
6. **An unrecognized extra key is ignored, not an error**, when the two required keys are otherwise
   present and valid.
7. **`write_handoff_summary` writes the exact expected content**, verified by reading the file back,
   across a session with multiple checkpoints (including one with empty `risks`, rendering `none`).
8. **A real I/O failure comes back as `Err`, not a panic** - checked by pointing `path` at a
   directory that doesn't exist.

### The actual point of this exercise

See `runs/2026-07-05-module-05-dry-run/` for the real dry run and its finding, and
`modules/05-error-handling/README.md` for the full rubric.

## Running it

```bash
cd fixtures/relay
cargo test
cargo clippy -- -D warnings
cargo run   # once a module has wired up something main.rs actually does
```
