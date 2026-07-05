# Critique — Skeptical Practitioner / Critic Lens

**Overall**: The docs are unusually disciplined about hedging in places ("Working hypothesis, not a proven finding," "designed but not yet run against a real learner attempt") — better than most workshop pitches. But the discipline is inconsistent, and several claims outrun their evidence.

## Findings

**1. Self-contradicting confidence on the deterministic-gate claim.** `workshop-gremlin-design.md` states as settled doctrine: *"This is a strict upgrade for any subject that has one, not a Rust-only special case"* — a universal law, written into canonical Gremlin files. Yet the same document's Status section admits the mechanism is *"designed but not yet run against a real learner attempt in this or any Rust context."* You cannot generalize a "strict upgrade" claim to *all future subjects* from zero completed runs on this one. Pick one register — hypothesis or finding — not both.

**2. Dependency ordering presented as discovered fact, not asserted design.** The "Why this order" section in `workshop-design.md` uses proof-language ("has no prerequisite *because*," "unusable without," "mutually entangled") for a sequence that is this workshop's own editorial call. Unlike the curriculum-anchor section (which cites the Book/Rustlings/Exercism directly), no external source is checked against this specific 6-module split or its "Hard prerequisite" column — it's an assertion dressed as derivation.

**3. Borrowed validation, wrong target.** *"Rustlings already validates that shape works for Rust specifically"* — true only for the compiler-driven, one-exercise-per-concept shape, which this workshop didn't invent. It's then used to lend credibility to the actual differentiator (agent-native delivery + Coachgremlin), which Rustlings says nothing about and which remains untested.

**4. Overclaimed "necessity."** Exercism's `config.json` having a `prerequisites` array is evidence Exercism *models* dependencies, not "real, current evidence" that the field treats this as *"necessary, not a nice-to-have."* That's a stronger claim than the artifact supports.

**5. README's opening line** — *"Learn Rust the agent-native way"* — brands an unbuilt, unrun method as an established teaching approach before a single module or learner session exists. Minor, but it's the most public-facing sentence in the repo and carries none of the hedging found deeper in.

No findings on hype in the "agent-native" label itself — it's defined with actual mechanism (harness + two-tier grading), not empty branding.
