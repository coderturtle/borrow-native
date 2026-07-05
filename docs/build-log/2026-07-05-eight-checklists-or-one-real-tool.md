---
title: "Eight Checklists, or One Real Tool"
description: "Module 01 shipped as a working exercise, then got retired the same day, in favor of one growing project every module builds a real feature onto."
pubDate: 2026-07-05T18:00:00
tags: ["build-log", "project-design", "relay"]
draft: false
---

Module 01 was done. Real exercise, real dry run, a genuine finding about clippy and the borrow checker, a packaged and validated takeaway. Then came the obvious next question, the kind that's easy to not ask when the thing you already built works: what does a learner actually have at the end of eight of these? An answer, in this case: a checklist, a playbook, a cheat-sheet, a template, a decision guide, another decision guide, and a diagnostic checklist that compresses the other six. Useful, all of them. Not a thing. Not something you'd point at and say "I built this."

Terminal Velocity had already solved a version of this problem, just not the version I needed. Its `receipts` fixture is one function, engineered five different ways across five modules: prompted precisely, buried in noise, wrapped in a harness, fixed under a deadline, sabotaged on purpose. That works because agentic-engineering practice really is five lenses on one problem. Rust's arc isn't five lenses on one problem. Ownership doesn't give you a different view of borrowing, it's a prerequisite for it. Copying the `receipts` shape here would have been the easy move and the wrong one: five lenses on a static thing, when what this arc actually needed was one thing that grows.

So: one project, `relay`, a restartable-handoff CLI. The pitch is specific on purpose, not "a Rust app," a tool for hybrid human-agent teams that fires a handoff summary (goal, diff, evidence, risks) at pomodoro-style checkpoints during an agent session. That's not a new idea invented for this workshop, it's the "Restartable Handoff Loop" Terminal Velocity's own research already named and never built. Building it for real, in Rust, this time, felt like the kind of callback worth having between two workshops in the same factory rather than two unrelated products.

One thing worth saying plainly, because it would have been easy to skip past: `agent-mission-control-lab` already exists, building a real multi-agent cockpit. I do not need a second one of those, badly, in a different language, as a workshop side-quest. `relay` stays narrower on purpose: one CLI, one question (has this session hit a checkpoint, what's the handoff), not a dashboard. If Module 06's multi-session tracking starts feeling like it wants to become a cockpit, that's the signal to stop, not a feature request.

Retiring Module 01's actual working exercise the same day it shipped felt worse than it turned out to be. The finding survives the move intact: I rebuilt the same lesson (own it, move it, don't clone it) against `relay`'s own real types and reran both attempts, the honest one and the clone-heavy one. Same result, exactly. Clippy stayed silent on both. The dry run that proved the lesson the first time didn't get thrown away either, it's still sitting in `runs/`, still true, just no longer the shipped fixture. Nothing about a good exercise design is wasted by moving it to better material.
