# Developer Evangelist — the hook

**Lens:** Would you share this? Does the pitch land in the first ten seconds? Ignored technical depth — assumed the Rust content is correct, asked only whether it's compelling.

**Reviewed:** top-level `README.md`, `modules/README.md`, Modules 01+02 READMEs, `fixtures/relay/SPEC.md`.

---

**Top-level pitch — lands, mostly.** Line 3 ("let the compiler, not an opinion, be the first gate") and line 9 ("you're not aiming to eventually get past the borrow checker. You're aiming to become native to it") are genuinely good hooks — sharp, ownable taglines a dev would screenshot. The two-tier gate is explained crisply in one paragraph (README.md:7). That's a strong 10-second pitch.

**But the README undercuts its own momentum before "How to start."** By the time a reader hits the actual clone command (line 24), they've already read: an Ardan Labs cert caveat with "no exam has been attempted yet" (line 11), a status callout that 6 of 8 modules are skeleton (line 32), and a "working hypothesis, not a settled finding" methodology paragraph citing dry-run grading files (line 44). This is honest, not dishonest — but for a hook, it reads like a thesis defense before the demo. A Rust-curious dev skimming GitHub wants "clone this, run this, feel this" in the first screen; instead they get three successive hedges. Cut or move most of this below "How to start."

**Relay's pitch is real but currently thin.** The "you keep a real, usable tool" promise (README.md:38-40) is credible in design — the build-out table in SPEC.md is coherent and the module-to-feature mapping makes sense — but at 2/8 modules, what exists is two private functions (`finalize_session`, `session_stats`) with no `main.rs` behavior yet ("cargo run # once a module has wired up something", SPEC.md:160). Nothing is runnable as a tool today. That's fine as a roadmap, but the "keep a real tool" language is aspirational, not yet demonstrable — a skeptical reader will notice the gap between promise and current artifact.

**Friction once inside a module: low, and that's the good news.** Module 01/02 READMEs get to the actual task fast (the `>` blockquote exercise, ~10 lines in). The "Why this is hard" / dry-run evidence sections are long but sit *after* the task statement, so they don't block starting — a reader who wants to just type code can skip straight to the rubric.

**Would I share it today?** Yes, but framed as "watch this build," not "go do this workshop" — the meta-honesty is a feature for a build-in-public audience but a drag on a cold pitch to someone just hunting for a fun Rust exercise.
