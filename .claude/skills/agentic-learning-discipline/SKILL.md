---
name: agentic-learning-discipline
description: A short self-check before moving to the next module, if your agent produced a working diff quickly or with little back-and-forth.
---

# Understanding check

A coding agent that already knows Rust can produce a module's correct answer in one shot, with no
struggle, the moment your instructions make the target shape legible. The deterministic gate
(`cargo test`/`cargo clippy`) doesn't care how a diff got written, and it never will - "the gate
passed" and "I understand why" are different claims.

Before moving to the next module, close the diff - yours or your agent's - and, without rereading
it, write two or three sentences: what did it actually change, and why does that make it correct
rather than merely passing.

If that's uncomfortable to do, that's worth noticing on its own - it usually means the diff was
produced rather than understood, and a slower second pass through the module is worth more than
moving on. Not scored, not something Coachgremlin checks. It's for your own use.
