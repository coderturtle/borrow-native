# Coachgremlin's first real dry run: Module 01 core, graded

Per `docs/coachgremlin-implementation-plan.md`-equivalent discipline for this workshop (no
separate plan doc written yet; this run follows `~/hekton/gremlins/coaching/coachgremlin.md`'s
Workflow directly). Two attempts against the same exercise (`modules/01-ownership-move-semantics/
exercise/`, spec in that directory's `SPEC.md`): a deliberately correct, move-based one
(`attempt-good/`) and a deliberately naive, clone-heavy one (`attempt-naive-clone/`) that is
**not** a rubric-gaming attempt in the Module 04 sense (it doesn't touch the test file or try to
defeat the check) — it's architecturally naive Rust that a learner unfamiliar with ownership might
write in good faith, exactly the failure mode this module's stated gate exists to catch.

## Step 3: Observe the attempt

**`attempt-good/`** (`transcript.txt`, `diff.patch`): one-file diff, touches only `src/lib.rs`.
`for order in orders` consumes the `Vec<Order>` by value; `order.customer` is moved directly into
`HashMap::entry()` as the key. No `.clone()` anywhere. `cargo test`: 6/6 pass. `cargo clippy -- -D
warnings`: zero output, clean.

**`attempt-naive-clone/`** (`transcript.txt`, `diff.patch`): also a one-file diff, touches only
`src/lib.rs`. `for order in &orders` iterates by reference (habit, not necessity — `orders` is
owned and never used again after the loop), forcing `order.customer.clone()` to get an owned
`String` for the map key out of a borrowed `&Order`. `cargo test`: 6/6 pass, identical to
`attempt-good`. **`cargo clippy -- -D warnings`: also zero output, clean.**

## Step 4: Score against the rubric

| # | Criterion | attempt-good | attempt-naive-clone |
|---|---|---|---|
| 1 | `cargo test` green (gate, deterministic) | Pass. 6/6. | Pass. 6/6. |
| 2 | `cargo clippy -- -D warnings` clean (gate, deterministic) | Pass. | **Pass — and this is the finding.** Default clippy does not flag this. |
| 3 | Diff touches only the implementation file, not the test file (gate, anti-gaming) | Pass. | Pass. Neither attempt touched `tests/`. |
| 4 | No `.clone()` on data the function could have moved instead (scored, conceptual) | Pass. Zero clones. | **Fails.** `order.customer.clone()` clones a `String` the function owns outright and never needs back; a by-value loop would have avoided it entirely. |
| 5 | Ownership shape (borrow vs. own) matches what the function body actually needs (scored, conceptual) | Pass. Takes `Vec<Order>` by value and consumes it, matching the by-value loop. | **Fails.** Takes `Vec<Order>` by value but only ever borrows it (`&orders`) — the by-value parameter is never justified by the body. |

**Rubric result: `attempt-good` meets the rubric (all gates pass, all scored criteria pass).
`attempt-naive-clone` passes every deterministic gate (1-3) and fails both conceptual criteria
(4-5).**

## A real finding, not just a pass/fail

**This is exactly the evidence this workshop's whole two-tier-gate thesis needed and didn't have
before this run.** Criteria 1-3, the deterministic tier as currently defined for this module, do
**not** distinguish these two attempts at all: both pass every test, both produce a clean default
`cargo clippy -- -D warnings`. A grading pass that stopped at "is the deterministic tier green"
would score `attempt-naive-clone` a full pass, silently teaching that defensive cloning is fine as
long as the compiler doesn't complain, which is precisely the escape hatch this module's own
design doc warns Coachgremlin's conceptual tier must catch.

**Checked, not assumed:** re-ran `attempt-naive-clone` against `cargo clippy -- -W clippy::pedantic
-D warnings` (a stricter, opt-in lint group most real Rust projects don't enable by default,
`pedantic-clippy-transcript.txt`). It **does** flag the file, but via `needless_pass_by_value`
("this argument is passed by value, but not consumed in the function body") — a different, more
general lint about the function signature, not one that names the clone directly — bundled
alongside an unrelated stylistic nit (`must_use_candidate`) in the same run. So: default clippy
(this module's actual stated gate) misses the over-cloning completely; the stricter pedantic group
catches a *symptom* of it through a different lint, noisily, and isn't what most real-world CI
gates run. Coachgremlin's conceptual tier is the only check in this pipeline that names the actual
problem (a clone that was never necessary) directly and specifically.

## Step 5: Confirm or loop

- **attempt-good:** rubric met. Concept (ownership/move semantics — moving a value into a
  collection versus cloning it out of habit) marked taught for this dry run.
- **attempt-naive-clone:** rubric not met on the conceptual tier. Feedback below; not a repeat of
  a different exercise, the next try is the same one, since the underlying concept was never
  actually demonstrated.

## Step 4 (feedback) and Step 6 (takeaway), in order given

### Feedback to attempt-good

What worked: the loop consumes `orders` by value from the start (`for order in orders`, not `for
order in &orders`), so every `Order` and every field of every `Order` was already owned at the
point it's used. Moving `order.customer` into `HashMap::entry()` isn't a trick, it's just letting
the compiler do what ownership already allowed.

What to watch next time: this exercise's function signature (`Vec<Order>` in, by value) all but
invites the by-value loop. A harder version of this same lesson is a function that's handed a
`&Vec<Order>` or `&[Order]` and still needs an owned `String` somewhere in the result, that
case genuinely needs a real clone (or a restructure to return borrowed data instead), and the
skill is telling those two cases apart, not "never clone." Worth a follow-up exercise, not
this one.

No solution was handed over: this feedback describes what the diff shows, not a fix Coachgremlin
supplied. The implementation in `attempt-good/` is the one being described.

### Feedback to attempt-naive-clone

Both gates are green. `cargo test` passes, `cargo clippy` is silent. That's exactly what a
solution that works but doesn't demonstrate this module's actual concept looks like from the
compiler's side. Look at the diff: `for order in &orders` borrows, then `.clone()`s a field to
get around that borrow. Nothing later in the function uses `orders` again. What changes if the
loop is `for order in orders` instead, no `&`, and the `.clone()` is deleted? Try it, and see what
happens to the clone (it should stop being necessary and can just be removed).

One concrete next try: rewrite `merge_customer_totals` with the loop iterating over `orders` by
value instead of `&orders`, and remove the `.clone()`. If the compiler is unhappy about it, that's
real signal, read what it says; if it isn't, that's real signal too.

No solution handed over: the actual fix (the by-value loop) is described above as a question to
try, not applied to `attempt-naive-clone/`'s files.

### Takeaway packaging (attempt-good only, since attempt-naive-clone's rubric was not met)

See `../../modules/01-ownership-move-semantics/takeaway/` for the packaged "who owns this"
diagnostic checklist, generalized from the actual reasoning used to write `attempt-good`'s
implementation (own the input? own every field of it. never `.clone()` before checking whether
the loop even needs to borrow at all). Validated per the same bar `terminal-velocity` used: applied
to a second, unrelated ownership problem, not just written and left alone — see
`takeaway-validation/`.

## Human Gate

This grading is Coachgremlin's recommendation, not a certified completion. `human_confirmed: false`
in `runs/run-20260705-RW-001.yaml`, per Coachgremlin's Human Gate: it never certifies completion
with external consequence without a human confirming the rubric result.
