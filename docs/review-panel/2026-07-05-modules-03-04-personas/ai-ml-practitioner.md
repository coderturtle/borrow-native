# AI/ML Practitioner — Modules 03+04 (2026-07-05)

## Findings (AI/ML Practitioner — technical accuracy lens)

I compiled and ran the actual `relay` fixture with `cargo clippy` (clippy 0.1.96) at default/pedantic/restriction levels against reconstructed correct and naive implementations, rather than trusting the prose. Most claims check out. One claim does not.

**1. Overstated equivalence between `clippy::pedantic`'s actual suggestion and the wildcard anti-pattern (Module 03).** README.md L140-142, `grading.md` L75-78, `retro.md` L33-36, and `enum-modeling-playbook/SKILL.md` L40-46 all state that `match_same_arms`'s suggested fix is "functionally the same collapse the wildcard version already made" / "would undo the point of criterion 5." I ran it: clippy's actual suggested fix is not a `_` wildcard, it's a named or-pattern — `CheckpointTrigger::TimeElapsed(_) | CheckpointTrigger::ToolCallCount(_) => NextAction::Continue`. I verified directly that this or-pattern form is *not* equivalent to a wildcard for the forward-compatibility property criterion 5 cares about: adding a fourth `CheckpointTrigger` variant to an or-pattern match still fails to compile (`error[E0004]: non-exhaustive patterns`), exactly like the two-separate-arms version — only a true `_` arm would silently swallow the new variant. So following pedantic's advice literally does *not* recreate the anti-pattern or sacrifice the compile-time safety net; it just merges two explicitly-named arms into one line. The module's repeated framing ("pedantic actively recommends the wrong direction," "pushes a learner toward the anti-pattern") overstates the actual risk of that specific suggestion. This should be corrected everywhere it's asserted, since it's used as evidence for a fairly strong pedagogical claim.

**Everything else checked out under direct execution**, contrary to my initial suspicion:
- `match_same_arms` is genuinely pedantic-gated (not default/style) in this clippy version — confirmed via `-D warnings` staying clean and pedantic explicitly noting `implied by clippy::pedantic`.
- `clippy::wildcard_enum_match_arm` is genuinely off by default and only fires when named explicitly — confirmed.
- Module 04's elision reasoning (rule 3 for `&self`, no rule for a 3-reference-parameter free function) is correct — confirmed by reproducing the actual `E0106` compiler error when eliding on a two-reference function.
- Pedantic output between the clone-based and lifetime-based `alert_checkpoint` really is identical except for build-timestamp noise — confirmed via `diff`.
- `&dyn Notifier` really does compile and pass all 6 `alert_checkpoint` tests identically to `<N: Notifier>` — confirmed by swapping the signature and rerunning.

## Disposition

The pedantic/or-pattern overclaim was independently re-verified by the operating session (compiled both forms, added a new enum variant, confirmed `E0004` on the or-pattern) and corrected across `modules/03-structs-enums-pattern-matching/README.md`, `runs/2026-07-05-module-03-dry-run/{grading.md,retro.md}`, `.claude/skills/enum-modeling-playbook/SKILL.md`, `runs/2026-07-05-module-03-dry-run/takeaway-validation/README.md`, and `~/hekton/gremlins/coaching/coachgremlin.md`'s Module 03 entry. All other findings confirmed no issue.
