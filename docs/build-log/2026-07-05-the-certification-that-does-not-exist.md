---
title: "The Certification That Doesn't Exist"
description: "This workshop was going to be a companion to a Rust Foundation certification. Turns out there isn't one. Fixing that found a real gap in the arc anyway."
pubDate: 2026-07-05T14:00:00
tags: ["build-log", "certification", "curriculum", "async"]
draft: false
---

The original idea, floated before any design doc existed, was to build this workshop as a companion to the Rust Foundation's certification, something a learner could work through here and then go get officially certified against. Reasonable premise, one problem: it doesn't exist. The Rust Foundation's Trusted Training program, launched this June, accredits training providers, five founding organizations, a published rubric, a two-year renewal cycle, not individual learners. There's no exam. There's nothing for a person to pass. I'd been about to design an entire external-validation story around a credential that isn't there to validate against.

Caught before any content got built, which is the only reason this is a build-log entry and not a retraction. The fix wasn't to drop the idea, it was to find the real version of it: Ardan Labs runs an actual proctored exam, a hundred multiple-choice questions, eighty percent to pass, ninety-nine dollars, retake as many times as it takes. Not as prestigious as an official foundation credential would have been. Real, though, in the way that matters here: a bar a person can actually clear or not clear.

Checking this workshop's arc against Ardan's own topic list turned up something I didn't expect. Six modules, each one anchored to the Rust Book and Rustlings and cross-checked against Exercism's own prerequisite graphs, and somehow none of them touched async. Not an oversight I caught by rereading my own design doc harder. It took an external list of what a real exam actually tests to surface a gap that three separate pieces of internal curriculum research had all walked past. Module 07 exists because of that specific check, not because I sat down and thought "this arc needs async." Worth being honest about: the research I did first wasn't wrong, exactly, it was just incomplete in a way I couldn't have caught by reading the same three sources more carefully.

The part I'm actually on the hook for now: sitting that exam myself, for real, once there's content here worth taking it after. Terminal Velocity's Coachgremlin proved its grading loop by dry-running it against real attempts, good and adversarial both, and writing down what actually happened. Same standard applies here, just external this time: does this workshop prepare someone for a test nobody on this project controls the grading of. I don't know yet. Nobody's taken it.
