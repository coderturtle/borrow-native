# Critique — End-User / Target Learner Lens

**Verdict: I'd bounce, because there's nothing here to do yet.**

**1. Zero actual content — this is a repo about a workshop, not a workshop.** The README says "Scaffolded 2026-07-05 — initial setup in progress," the Quick Start block is literally `# Add project-specific commands here`, and workshop-design.md's own scope-out list admits: "Exact exercise specs, fixtures, and rubrics per module... not yet authored," "The actual Astro site content and first Pages deploy" also missing. As the advertised learner clicking in today, I find design docs about *how a workshop will be designed*, not a Module 01 I can open my agent against. That's the single biggest gap between promise and delivery.

**2. The core interaction mechanic — the actual thing I'd do all day — is never specified.** "Every module's core exercise runs through the learner's own harness" is repeated three times across the docs but never shows *what I type*, *what the agent does*, or where the line is between "agent writes the Rust for me" (which defeats learning ownership/borrowing) and "I write it, agent critiques." For an audience that already drives Claude Code daily, this is the one design question I'd need answered before trusting the pitch, and it's absent.

**3. Insider jargon leaks into learner-facing framing.** "Coachgremlin grading the conceptual/idiomatic layer" (README line 8) assumes I know what a Gremlin is. I'm agent-literate, not a Hekton-factory insider — this reads like internal tooling vocabulary bolted onto a public README.

**4. The good part, buried.** The prerequisite reasoning in the module table (borrowing needs ownership underneath it; structs wait on borrowing because methods need `&self`) is legible, respects my existing dependency-graph thinking, and doesn't condescend on general programming concepts. If the exercises matched this rigor, I'd stay. But I can't evaluate exercises that don't exist, and the doc-to-meta-process ratio (Gremlin versioning, Review Panel personas) so far outweighs any worked example that it reads as effort spent on infrastructure rather than the thing I'd actually experience.
