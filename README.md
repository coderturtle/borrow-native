# Borrow Native

Learn Rust the way you already work, with your agent, and let the compiler, not an opinion, be the first gate.

## What this is

A self-paced workshop that teaches Rust (ownership, borrowing, structs and enums, generics and traits and lifetimes, error handling, concurrency) to people who already drive a coding agent daily. Every exercise runs through your own harness: you write the Rust, your agent is the tool you're already using to do it, and two things check your work. First, a **deterministic gate**: `cargo test` and `cargo clippy` either pass or they don't, no opinion involved. Second, a **conceptual check** from Coachgremlin, this workshop's teaching agent (a role you run yourself, inside your own harness, not a hosted service): did you actually work with the borrow checker, or did you reach for `.clone()`/`unsafe` to make it stop complaining? A green compiler is necessary but never sufficient on its own.

The name is the pitch: you're not aiming to eventually get past the borrow checker. You're aiming to become native to it.

**Who it's for:** agent-literate practitioners, people comfortable with git, the CLI, and reading a diff, who already use a coding agent daily for real work, but haven't written real Rust yet. Not a true-beginner programming workshop (general programming fluency is assumed) and not an intro-to-agents workshop (harness fluency is assumed).

## Prerequisites

- Comfortable with git, the CLI, and reading a diff.
- General programming fluency in at least one other language.
- Already using at least one coding-agent harness regularly, with one installed on your machine.
- Rust and `cargo` installed (`rustup`).

## How to start

```bash
git clone git@github.com:coderturtle/borrow-native.git
cd borrow-native
cat modules/README.md
```

Then work through `modules/` in order. Each module states a hard prerequisite on an earlier one; skipping ahead means hitting compiler errors the workshop hasn't equipped you to read yet.

> **Current status: skeleton only.** The module arc, gate shape, and takeaway shape are decided for all seven modules (see [`modules/README.md`](modules/README.md)), but the actual exercises, fixtures, and rubrics aren't authored yet. There's nothing to run through your harness today. Watch `docs/build-log/` (once published) for progress, or [open an issue](https://github.com/coderturtle/borrow-native/issues) to ask.

## How the modules connect

Ownership has no prerequisite; everything else in Rust is a consequence of it. Borrowing depends on ownership. Structs and enums depend on borrowing (methods take `&self`/`&mut self`). Generics, traits, and lifetimes depend on having real types to implement them on. Error handling depends on both enums and generic error propagation. Concurrency depends on ownership and borrowing specifically, and pays off everything before it. A synthesis capstone closes the arc: given a broken or non-idiomatic program, diagnose which concept is actually the root cause. Full arc, gate tiers, and the curriculum research behind it: [`modules/README.md`](modules/README.md).

## What you keep

Every module leaves you with something, not just a passed check: a move-error diagnostic checklist, a borrow-checker playbook, an idiomatic-modeling checklist, a trait-bound/lifetime cheat-sheet, a custom-error-type template, a concurrency decision guide, and a personal Rust diagnostic playbook tying it all together. See [`modules/README.md`](modules/README.md#what-you-keep) for the full list.

## The teaching method

Our working hypothesis, not a settled finding: a deterministic first gate (the compiler, not a rubric) makes Coachgremlin's conceptual feedback easier to trust than it could be for a subject without one. This hasn't been tested against a real learner attempt yet. See [`docs/workshop-design.md`](docs/workshop-design.md) for the full reasoning, including where this is a bet, not proven pedagogy, and how this workshop's arc is anchored to (not a replacement for) Rust's own existing teaching material: the official Book, Rustlings, and Exercism.

## Build in public

This workshop's own build will be published as a dated journal, the maintainer's record of building the workshop and its reusable Gremlin tooling at the same time, once the build-log site is live (not yet, see status above).

## Something wrong?

This is early and imperfect by design. If a module reduces to "read this, then move on" instead of a real gate, or a link here is broken, [open an issue](https://github.com/coderturtle/borrow-native/issues).

## Key docs

- [Workshop Design](docs/workshop-design.md): audience, format, deterministic-gate teaching method, full module arc
- [Maintainers](docs/maintainers.md): internal/agent-facing docs, classification, documentation contract
