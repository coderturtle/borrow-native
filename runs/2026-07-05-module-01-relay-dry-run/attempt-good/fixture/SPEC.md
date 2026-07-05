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
notify a human that a checkpoint was reached — a direct implementation of the "Restartable Handoff
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
| 01 | Ownership & Move Semantics | Core domain types (`DraftCheckpoint`, `CheckpointRecord`, `Session`) and `finalize_session`, sealing a session's drafts into records | **This pass: stub + tests shipped, unstarted (see below)** |
| 02 | Borrowing & References | Session statistics (average/longest checkpoint gap) computed by borrowing session history | Not started |
| 03 | Structs, Enums & Pattern Matching | `CheckpointTrigger` (time/tool-count/context-budget) and human-response (`Acknowledged`/`Snoozed`/`Ignored`) enums | Not started |
| 04 | Generics, Traits & Lifetimes | `Notifier` trait (desktop/terminal-bell/stdout), implemented generically | Not started |
| 05 | Error Handling | Config parsing and handoff-summary file I/O with real `Result`/custom error types | Not started |
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
}

pub struct CheckpointRecord {
    pub goal: String,
    pub summary: String,
    pub risks: Vec<String>,
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

## Running it

```bash
cd fixtures/relay
cargo test
cargo clippy -- -D warnings
cargo run   # once a module has wired up something main.rs actually does
```
