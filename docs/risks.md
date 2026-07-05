# Risks: Borrow Native

## Risk Register

Machine-readable risk state lives in `.hekton/risk-register.yaml`. Keep this
Markdown file as the human-readable explanation of material risks and mitigations.

| ID | Date | Risk | Impact | Likelihood | Mitigation | Status |
|---|---|---|---|---|---|---|
| RISK-0001 | 2026-07-05 | Initial governance baseline needs first human/agent review | Medium | Medium | Run governance preflight and end-session review during the first material session | Open |
| RISK-0002 | 2026-07-05 | `site/` starter dependencies carry 4 inherited vulnerabilities (`npm audit`: 1 high, 3 low, all in the Astro <=7.0.0-alpha.1 / esbuild chain via `@astrojs/mdx`/`@astrojs/tailwind`) | Medium | Low (dev/build-time surface, not runtime-served code) | Inherited from `terminal-velocity`'s same starter, not introduced here. Fix available (`npm audit fix --force`) but is a breaking Astro 7 upgrade, out of scope for this scaffolding pass. Must be triaged (upgrade or explicitly accept) before the first real Pages deploy, per the Workshop Gremlin's own Risks note. | Open |
