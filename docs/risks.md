# Risks: Borrow Native

## Risk Register

Machine-readable risk state lives in `.hekton/risk-register.yaml`. Keep this
Markdown file as the human-readable explanation of material risks and mitigations.

| ID | Date | Risk | Impact | Likelihood | Mitigation | Status |
|---|---|---|---|---|---|---|
| RISK-0001 | 2026-07-05 | Initial governance baseline needs first human/agent review | Medium | Medium | Run governance preflight and end-session review during the first material session | Open |
| RISK-0002 | 2026-07-05 | `site/` starter dependencies carry 4 inherited vulnerabilities (`npm audit`: 1 high, 3 low, all in the Astro <=7.0.0-alpha.1 / esbuild chain via `@astrojs/mdx`/`@astrojs/tailwind`) | Low - triaged 2026-07-08 (Vulnerability Gremlin), see below | Certain (present on every `npm install`) | **Triaged and accepted, not fixed.** See rationale below. Re-open if `output` mode ever changes from `"static"`. | Closed |
| RISK-0003 | 2026-07-08 | (Vulnerability Gremlin run) `cargo audit` against `fixtures/relay/`'s live Rust dependency tree | N/A - clean scan, no findings | N/A | 0 vulnerabilities across 7 crate dependencies, RustSec advisory DB (1,159 advisories loaded). Recorded as a clean-scan baseline, not an open risk - re-run periodically as `relay`'s dependency tree grows across later modules. | Closed (informational) |

## RISK-0002 triage detail (2026-07-08, Vulnerability Gremlin's second real run)

This is the Vulnerability Gremlin's (`~/hekton/gremlins/red-team/vulnerability-gremlin.md`) second
real run, after `half-life`'s RISK-0002 (2026-07-08) — same finding, same starter, confirmed via
direct lockfile inspection to resolve to **identical versions** (`astro@5.18.2`,
`@astrojs/tailwind@6.0.2`), not merely assumed similar.

**Checked directly against this project's own code, not copied from `half-life`'s conclusion:**
`grep -rn "define:vars\|server:island" site/src/` returns nothing here either.
`site/astro.config.mjs` sets `output: "static"`. `.github/workflows/*.yml` only runs `npm run
build`, never `astro dev`. Same reachability conclusion as `half-life`: none of the 5 Astro/esbuild
advisories (XSS via `define:vars`, reflected XSS via slot name, XSS via spread props, server-island
replay, host-header SSRF, and the Windows-only esbuild dev-server file-read) apply given this
project's actual configuration.

**Did not re-attempt the breaking upgrade** — since the resolved dependency versions are confirmed
identical to `half-life`'s (checked directly, not assumed), `half-life`'s already-verified finding
transfers exactly: `npm audit fix --force` upgrades cleanly to `astro@7.0.6` (0 vulnerabilities), but
`@astrojs/tailwind` fails outright on Astro 7 (`Cannot read properties of undefined (reading
'postcss')`) since Astro dropped its bundled Tailwind integration for its own Vite plugin - a real
migration, not a version bump. Re-deriving this by actually re-running the broken upgrade here would
have been redundant, not more rigorous, given the version match is independently verifiable.

**Decision:** accept the risk as-is, same reasoning as `half-life`'s RISK-0002. Upgrade path stays
known and documented for whenever `site/`'s dependency stack gets touched deliberately (ideally
alongside `half-life`'s, since they'd need the identical migration).

## RISK-0003 detail: `fixtures/relay` cargo audit (2026-07-08)

`cargo audit` against `fixtures/relay/Cargo.lock` (the shared Rust project every module builds a
real feature onto): 7 crate dependencies scanned against 1,159 loaded RustSec advisories, 0
vulnerabilities found. Recorded as a clean baseline rather than left unchecked - the Vulnerability
Gremlin's own Workflow requires auditing every package-manager ecosystem present in a project, not
just the one that happens to have findings. Historical dry-run fixture snapshots under `runs/*/`
(each module's own `Cargo.lock` copies from Coachgremlin's grading attempts) were deliberately not
audited individually - they're point-in-time evidence artifacts, not the live project's dependency
set, and auditing dozens of frozen snapshots would be noise, not signal.
