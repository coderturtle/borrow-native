# End-User / Target Learner

**Lens:** The advertised audience — an advanced practitioner who already uses coding agents daily, new to Rust specifically. Does the promise match delivery? Would you actually do these exercises, or bounce?

**Reviewed:** top-level `README.md`, `modules/README.md`, Modules 01+02 READMEs, `fixtures/relay/SPEC.md`, `src/lib.rs`, both test files.

---

## Critique: End-User / Target Learner Lens

**Top finding — the pages spoil their own exercise, not just the Skill.** The instruction is "Get `cargo test` green... then check it against the rubric below," but the rubric sits on the same page, unhidden, and gives the answer before you've had a chance to fail on your own. Module 01's rubric item 5 states outright: "a `Vec<DraftCheckpoint>` parameter that's never consumed is a signal you borrowed when you could have owned" — that's the entire "aha" of the exercise, spelled out. Module 02 is worse: rubric item 5 names the exact field to clone ("`longest_gap_goal`... cloning *that one field, once* is correct"). Worse still, this same answer is baked into `fixtures/relay/src/lib.rs` itself, the file you're editing — the `SessionStats` doc comment reads "cloning it here isn't a habit to unlearn — it's the actual fix." You cannot avoid seeing this; it's not an optional Skill you were warned off of, it's inline in your editor. The "don't open the Skill first" framing implies the diagnosis is hidden, but functionally it isn't.

**Second finding — a real "you'd get stuck here" gap.** Neither module README tells you to `cd fixtures/relay` before running `cargo test`. There's no top-level `Cargo.toml` (confirmed), so running `cargo test` from repo root fails outright. The command only appears in `SPEC.md`'s "Running it" section, one link away, never inline on the exercise page itself. For someone who's never touched a multi-crate Rust layout, this is exactly the kind of "which directory am I even in" stumble the workshop should pre-empt on the page you're actually working from.

**What actually works well:** the harness line ("Agnostic... whatever harness... you already use daily") correctly assumes I don't need agent basics explained. The "Valid alternate terminal" framing (cloning first is a legitimate path, not failure) respects me as capable of iterating rather than needing to get it right first try. The stub file, signatures, and edge-case list are concrete enough to start immediately — if the spoilers above were fixed, I'd genuinely attempt this rather than bounce.

**Minor:** the "Why this is hard" sections read like internal dry-run reports (citing `runs/.../grading.md` paths) rather than learner-facing prose — informative but tonally mismatched, like reading a lab notebook instead of a lesson.
