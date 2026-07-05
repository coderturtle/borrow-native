---
title: "A Second Workshop Tests the Machinery"
description: "Terminal Velocity proved the Workshop Gremlin once. Building a Rust workshop instead of another agentic-engineering one is what actually tested whether it generalizes."
pubDate: 2026-07-05T09:00:00
tags: ["build-log", "workshop-gremlin", "coachgremlin", "review-panel"]
draft: false
---

The Workshop Gremlin had run exactly once before today, on its own founding subject. Everything in it, the naming step, the review panel, the module-README template, was designed and validated against Terminal Velocity, a workshop about agentic engineering built by an operator who was, at the time, also building the tool doing the building. That's a real risk: a piece of machinery that has only ever been tested on the one problem it was invented to solve hasn't been tested at all, it's just been used.

So the question for this session wasn't "can we scaffold a Rust workshop." It was "does the Workshop Gremlin actually generalize, or does it just work for the one thing it already knows how to do." The honest way to answer that was to build something genuinely different, not another workshop about how agents work, but a workshop about a subject with its own hard rules that don't care what an agent thinks: a compiler.

That difference showed up immediately. Terminal Velocity's gates are entirely subjective, Coachgremlin's rubric is the only arbiter of whether an exercise passed. Rust has an objective one already built in: `cargo test` either passes or it doesn't, `cargo clippy` either complains or it doesn't. Writing that down as a real design principle, not just an implementation detail of this one workshop, felt like the actual point of running a second workshop at all. It went into the Workshop Gremlin's own definition as a new variant, not a Rust-specific footnote, because the next workshop after this one might be Python or infrastructure-as-code or something else entirely with its own version of a compiler.

The review panel earned its keep again, differently this time. Two of the seven findings were factual: I'd cited Rustlings as having "23 numbered exercises" when that's actually the topic-directory count, the real exercise count is closer to ninety, and I'd cited Exercism concept tags as if they were exercise names. Both were real research, fetched and read, not invented, but stated more precisely than the source actually supported. Catching that in a design doc is cheap. Catching it after publishing curriculum-anchor claims to actual learners would not have been.

The other finding stung more because it was structural, not factual. The capstone module's takeaway was a checklist, no coding artifact at all, which meant its own deterministic gate, the thing every other module in this workshop is built around, was simply undefined. I'd written six modules with real bite and then let the seventh coast on vibes. Fixed now: a broken or non-idiomatic program, a required fix, and a written diagnosis Coachgremlin checks before the fix counts. Whether that capstone actually holds up against a real learner is still unproven. Nothing in this workshop has been attempted by anyone yet.

What's actually here today: a scaffolded public repo, a chosen name, two rounds of review with real findings applied, seven module skeletons with a decided question, gate, and takeaway each, and no content. That last part isn't a gap I'm hiding, it's the honest stopping point the Workshop Gremlin's own contract sets for itself. Coachgremlin's two-tier grading, deterministic first, conceptual second, is designed and has never graded a real attempt at anything, Rust or otherwise. That's the next real test, and it isn't this session's to run.
