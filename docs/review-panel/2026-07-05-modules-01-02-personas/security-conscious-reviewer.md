# Security-Conscious Reviewer

**Lens:** Lighter-weight than the other personas. Does any exercise/case study/guidance model an unsafe practice (blind auto-merge, secrets handling, unreviewed destructive actions, `unsafe`/interior-mutability used to silence a real complaint) that a learner might copy uncritically? Scoped narrowly, not a full security audit.

**Reviewed:** Modules 01+02 READMEs, `fixtures/relay/SPEC.md`, `src/lib.rs`, both Skill files, `scripts/arb-trigger-check.sh` (new this session).

---

## Security-Conscious Review — Modules 01 & 02

**Top finding: `scripts/arb-trigger-check.sh` always exits 0, even when a trigger fires.**

The script's whole purpose is to catch changes to governed files (`fixtures/relay/src/lib.rs`, `SPEC.md`) before a learner works on them further. It correctly detects and logs a match:

```bash
log "ARB TRIGGER FIRED: $f matches '$pattern' (review_type: ${review_types[$i]}) - ${reasons[$i]}"
fired=1
```

but the tail of the script is:

```bash
if [ "$fired" -eq 0 ]; then
  log "No ARB triggers fired against current pending changes."
fi
exit 0
```

There is no `exit 1` (or any nonzero code) in the fired branch. If anyone wires this into a pre-commit hook, CI step, or has an agent gate on `$?`, it will silently "pass" every time regardless of outcome — a check that logs a warning but can never block is exactly the anti-pattern this reviewer is watching for: a governance script that looks like a gate but isn't one. Worth fixing before it's held up as a template for future governance scripts.

**Secondary: fragile `git status --porcelain` parsing.** `awk '{print $2}'` breaks on renames (`R  old -> new` yields `$2=old`, so a rename *into* a protected path, e.g. something renamed to `fixtures/relay/src/lib.rs`, is never detected) and on any path containing a space (porcelain quotes such paths, so `$2` grabs a fragment, not the real filename, and the exact-match comparison silently fails). Today's two trigger patterns are simple space-free literal paths, so it happens to work now — but it's a bad general pattern to copy.

**On the unsafe/`RefCell` question: the claim holds.** Neither README, neither Skill, nor `relay`'s code ever reaches for `unsafe`, `RefCell`, `Rc<RefCell<_>>`, or interior mutability as a workaround. Both skills explicitly instruct restructuring ownership/scope instead of "sidestepping" the checker (e.g. borrow-checker-playbook step 5: "Fix a real conflict by restructuring scope... not by cloning to sidestep it").

**On harness-agnostic framing:** no auto-merge modeling. Both modules require "a working implementation Coachgremlin has actually reviewed against the rubric," so a review step is built in, not skipped.

**Resolution (same session):** both findings fixed directly — the script now `exit 1`s on a fire, and porcelain parsing strips the status prefix, unquotes quoted paths, and resolves `old -> new` rename lines to the `new` path.
