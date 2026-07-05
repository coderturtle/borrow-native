# Skeptical Practitioner / Critic

**Lens:** Hunt for unsupported claims, hedge-free overclaiming, and hype language. "Prove it or cut it."

**Reviewed:** top-level `README.md`, `modules/README.md`, Modules 01+02 READMEs, `fixtures/relay/SPEC.md`, both dry-run `grading.md`s, Module 02's `retro.md`, `borrow-checker-playbook/SKILL.md`.

---

# Critique — Skeptical Practitioner / Critic

## 1. "Confirms the workshop's central bet" contradicts its own hedge three paragraphs later
`retro.md` states: *"Confirms the workshop's central bet generalizes to a second concept, rather than being a one-off property of Module 01's specific exercise."* That's a strong verb ("confirms") for two self-constructed, self-graded instances. The same document later admits: *"whether it counts as an independent data point toward the 3+ run bar (as opposed to the same lesson re-instantiated) is coderturtle's call, flagged in the run record, not self-certified here."* Those two claims can't both be true — either it's confirmed, or the reviewer hasn't decided if it even counts as independent evidence. Cut "confirms" for something like "is consistent with."

## 2. "Zero discriminating signal" is accurate for the one comparison run, but phrased as a settled property
`grading.md` and the SKILL.md both state pedantic gives "*zero* discriminating signal" / "gives no help at all" for Module 02. This is directly evidenced (identical 5 warnings, same lines, quoted) — for the one naive-clone shape tested. It is not shown that no over-cloning variant would ever trip pedantic differently. The "here"/"in this exercise" qualifiers appear in some places but not all (SKILL.md: "unlike... pedantic doesn't even give a noisy partial signal here" — fine; but the bare rubric text "gives no help at all" loses the qualifier). Worth uniform scoping.

## 3. Single-grader limitation is named in retro.md but not carried into learner-facing docs
`retro.md` is explicit: *"this dry run used one session (this one) constructing and grading both attempts. An independent blind pass would be a stronger test."* Module 02's README footer only says "actually run once so far" — true but softer than "the same session wrote and graded both submissions," which is the actual limitation. The top-level README does state this caveat for both modules combined; the per-module pages don't.

## 4. "The one clone that's actually correct" elides a real design choice
Both the README and SPEC assert `longest_gap_goal`'s clone is "the one clone that's actually correct" / "actually necessary." True only because Module 02 predates lifetimes — a lifetime-annotated `SessionStats<'a>` returning `&'a str` would eliminate this clone entirely. That's a legitimate sequencing decision (lifetimes are Module 04), but it's asserted as settled correctness rather than flagged as scoped-to-current-toolset.
