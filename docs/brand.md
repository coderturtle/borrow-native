# Brand / Style Layer: Borrow Native

> The only place this workshop's personality lives. `README.md` and, once built, `site/`'s layout and `astro.config.mjs` all read from this file — they don't redefine voice, banned language, or visual identity independently. Adapted from `terminal-velocity/docs/brand.md`, itself adapted from blog-factory-lab's `templates/brand-style-layer-template.md`.

## Site identity

**Name:** Borrow Native
**Tagline:** Learn Rust the way you already work, with your agent, and let the compiler, not an opinion, be the first gate.
**Parent brand:** Hekton
**Slug:** `borrow-native`

The tagline leads with the deterministic-gate hook rather than the spec-sentence pitch ("agent-literate practitioners work through harness-driven exercises...") — per this workshop's own Workshop Review Panel Developer Evangelist finding (`docs/review-panel/2026-07-05-initial-design.md`), the spec sentence buries the actual ten-second hook one paragraph below itself.

## Tone and voice

**Core voice:** A competent peer, not an instructor. Specific, dryly funny, anti-hype by default. Treats the reader as someone who already ships code daily in some language and already drives a coding agent — because they do; the only thing assumed unfamiliar is Rust itself.

**Tone rules:**
- Prefer plain verbs and concrete nouns; show the mechanism (a specific compiler error, a specific fix), don't just assert the result.
- Any claim about what the teaching method achieves ("teaches better," "the deterministic gate makes grading more trustworthy") must be marked as a hypothesis unless there's actual evidence behind it — this workshop's own design docs got caught overclaiming by its own review panel; the fix is a permanent voice rule, not a one-time edit.
- First person for build-log entries. System/instructional language for module content and workshop structure.
- Admit uncertainty directly rather than smoothing over it.
- Never imply a technique is magic; name the mechanism (the compiler said X, here's why).

## Hard rules

- **No em dash characters.** Use period, colon, semicolon, comma, parenthesis, or a plain hyphen instead. (Applies to all published workshop content — README, module READMEs, build-log entries, the site. Design/planning docs under `docs/` are working documents and are exempt.)
- No AI-slop openers ("In today's fast-paced world...", "It's important to note...").
- No unqualified efficacy superlatives ("game-changing," "revolutionary," "10x," "unlock your potential") — direct fix for this workshop's own Skeptical Critic findings against the design docs.
- No engagement bait, fake scarcity, or "one weird trick" framing.
- **Rust-specific:** never imply `unsafe`, `.clone()`-to-silence-the-compiler, or `unsafe impl Send`/`Sync` is a reasonable default fix in any published example, walkthrough, or build-log entry — even in passing. This workshop's whole bet is that the compiler's complaints are worth understanding, not routing around; content that models the opposite, even casually, undercuts the pitch.

## Banned phrases

Reused from the wider Hekton house style, plus workshop-specific additions:

- delve, tapestry, unlock, seamless, game-changing, revolutionize, transform your workflow, supercharge, effortlessly, cutting-edge, thought leader
- "in today's fast-paced world," "it's important to note," "at scale" (unless the content proves the scale)
- Workshop-specific: "master the art of," "in this comprehensive guide," "unlock your potential," "10x your skills," "fight the borrow checker" (frames the compiler as an adversary rather than a collaborator, the opposite of what this workshop teaches)

## Visual identity

Inherit `terminal-velocity`'s Astro starter tokens rather than invent a new palette, once the site is built: `--accent`, `ink`/`paper` Tailwind tokens, the `.post-body` typography rhythm, "no section dividers, whitespace only."

| Element | Direction |
|---|---|
| Overall mood | Clean technical workshop notebook. Not a marketing landing page. |
| Colour approach | Dark-on-light default; restrained palette; dark mode optional later |
| Typography | Crisp, generous whitespace, readable code blocks (Rust syntax highlighting matters more here than it did for `terminal-velocity`) |
| Imagery | Artifact-led: compiler output, diffs, terminal sessions — not stock photos or decorative AI art |
| Decoration | No neon AI aesthetic, no hero banners, no gradient-mesh backgrounds |

## Gremlin and factory language rules

- Coachgremlin and the Workshop Gremlin are real, documented agents with concrete responsibilities (`~/hekton/gremlins/`) — reference them plainly when explaining how the workshop works, don't decorate every heading with gremlin language, and don't assume a learner already knows what a "Gremlin" is without a one-line explanation the first time the term appears in learner-facing copy.
- A module README is a production artifact: plain. A build-log entry can be funny where the actual events were funny.

## Anti-goals

- Not an AI-hype funnel or a marketing page for Hekton.
- Not a certification mill — no claim that completing this workshop credentials anything.
- Not a place to publish unverified efficacy claims — every claim about what the teaching method achieves gets the hypothesis treatment above until there's real evidence.
- Not overrun with gremlin language to the point of reading childish.
- Not a Rustlings/Book/Exercism knockoff — it exists alongside them, not instead of them; never imply those resources are inadequate, only that this workshop adds something they don't.

## Application map

| Artifact | Reads |
|---|---|
| `README.md` | Title + tagline |
| `site/` (once built) | Tone, hard rules, banned phrases, visual identity |
| Module READMEs | Tone, hard rules, banned phrases, Rust-specific escape-hatch rule |
| Build-log entries | Tone and voice rules (first person, tension, no hype) |

## [TBD]: items for later

- [ ] Exact accent colour token (once site is built)
- [ ] Favicon / wordmark treatment
- [ ] Dark mode colour tokens
- [ ] Rust-syntax-highlighting theme choice for code blocks
