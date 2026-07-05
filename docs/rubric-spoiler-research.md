# Rubric-spoiler tension: research and candidate structures

**Status:** decided (2026-07-05, coderturtle) - see the "Decision" section near the bottom for
what was adopted and why. Everything above that section is the research and candidate-generation
pass that fed the decision; left as-is rather than rewritten, so the reasoning that led to the
choice stays visible alongside it.
**Input:** `docs/rubric-spoiler-tension.md` (the problem statement and the research prompt this
answers). **Method:** web research on assessment pedagogy and autograded-course practice
(2026-07-05), synthesized against this workshop's actual shipped modules and constraints.

## What the literature calls this

The tension has a name and a real research history - it is not a quirk of this workshop:

- **"Criteria compliance" (Torrance, 2007; 2012):** the canonical critique of transparent
  assessment criteria - explicit criteria can redirect learners from productive learning toward
  surface strategies that satisfy the stated criterion without the underlying understanding.
  Torrance describes teachers "overspecifying and thereby limiting learning through finely
  explicated criteria." This is precisely what a rubric line like "only the one necessary clone
  exists" invites: write the borrow because the rubric said so, understand nothing.
- **Sadler's indeterminacy argument (2009):** explicit codified criteria cannot fully represent
  multi-criterion quality judgments; recognition of quality *precedes* the articulation of
  criteria, so criteria are always retrospective approximations. The standard response in this
  literature is exemplars and post-hoc qualitative judgment, not more precise pre-statements.
- **The counter-position (Panadero & colleagues' critical reviews; the Frontiers "Transparency in
  Assessment" collection):** transparency supports self-regulation and is defensible even for
  complex tasks - the field's resolution is generally *not* to hide criteria but to change what
  kind of thing a criterion names (a quality dimension, not a surface feature).
- **Productive failure (Kapur; Sinha & Kapur's 2021 review, 53 studies):** attempting a problem
  *before* receiving canonical instruction measurably improves conceptual learning - specifically
  for learners with enough prior knowledge to generate their own (suboptimal) solutions first.
  That is exactly Borrow Native's audience (experienced developers, Rust-novice) and exactly the
  current "attempt, then read Why this is hard" structure. Two load-bearing details: (1) PF
  designs do **not** give learners the target solution before the attempt - the benefit comes from
  generating your own wrong answer and then comparing; (2) PF *requires* the canonical explanation
  afterward - the post-attempt reveal is not optional garnish, it is where the learning lands. So
  the current module structure is PF-shaped and evidence-backed; the rubric line is the one element
  breaking it.
- **Expertise reversal effect (Kalyuga et al.):** worked examples beat discovery for novices, and
  the effect reverses for experienced learners, who learn more from problem-solving-first.
  Relevant both ways: it supports keeping discovery for this audience, and it warns that if the
  audience were true beginners, "just show the answer" (candidate C below) would be the *right*
  call, not a concession.

## What comparable systems actually do

- **Autograded university courses (Gradescope et al.):** the standard pattern is *visible
  criterion names and weights, hidden test details* - students see "edge-case handling: 20pts"
  before submitting but not the specific hidden inputs. Disclosure of the axis, not the instance.
- **CS50 (Harvard):** work is graded on published axes (correctness / design / style) with a
  public style guide, but per-assignment *design* expectations are never enumerated in advance -
  design is assessed qualitatively after submission, against course-wide values the student
  already knows. Nobody considers the style guide a spoiler, because it is cross-cutting: knowing
  the course's ten design values does not tell you which one this particular problem set is
  secretly about.
- **Competitive programming judges:** the problem statement discloses *constraints and observable
  properties* (input size, time limit) that fully determine what is graded, without ever naming
  the intended algorithm. Deriving the technique from the disclosed property is the exercise.

These three patterns map directly onto candidates D, B, and A below, respectively.

---

## Candidate A: property-phrased conceptual criteria

**Structure.** Keep the rubric exactly where it is, fully shared pre-attempt - but rewrite each
conceptual criterion to name an *observable property of the finished artifact* instead of the
*technique that produces it*. The competitive-judge move: disclose the constraint, not the
algorithm. Deriving the technique from the property becomes part of the exercise rather than a
leak.

**Module 04 walkthrough.** Current criterion 6 reads: *"`CheckpointAlert.session_label` borrows
`session` with an explicit lifetime, rather than owning a clone of it."* Property-phrased:

> 6'. **It is a compile error - not merely unlikely - for a `CheckpointAlert` to outlive the
> `Session` it describes (scored, structural).**
> 7'. **A caller may pass a `notifier` and `trigger` with strictly shorter lifetimes than
> `session` (scored, structural).**

The learner reads both before attempting. Their first instinct (clone) passes every test, and
then fails their own self-check against 6' - "can my alert outlive the session? Yes, it's owned" -
which *forces* the question the exercise wants asked (does this data need to outlive its source?)
without stating the answer (borrow + `'a` tied to `session` alone). 7' catches the over-annotation
shape the same way. "Why this is hard" still has its reveal: elision rules, why the compiler
can't infer this, what the clone actually costs.

**Coachgremlin changes.** Grading arguably gets *easier* - properties are closer to mechanically
checkable than "is this idiomatic" (6' is checkable by reading whether the type system enforces
the relationship; 7' by reading the signature). Each module's dry run already produces exactly the
evidence needed to validate that a property discriminates the naive attempts.

**Cons, honestly.** (1) Authoring is genuinely harder, and quality will vary - some concepts
resist: Module 03's exhaustiveness property ("adding a new `CheckpointTrigger` variant must break
compilation at every decision point that inspects it") hints heavily at "don't use `_`" even if it
never says it. (2) Risk of criteria reading as riddles - a frustrated learner experiences an
obliquely-worded criterion as *worse* than a spoiler. (3) It does not survive the agentic
short-circuit (see cross-cutting risks): paste the property into any coding agent and it derives
the technique instantly. (4) For very small exercises, property and technique converge - it
relocates the spoiler distance-wise without eliminating it in the worst cases.

## Candidate B: workshop-wide idiom charter (the CS50/style-guide model)

**Structure.** Hoist every conceptual criterion out of the per-module rubrics into one published,
workshop-level **Idiom Charter** (~8-10 entries: "own only what must outlive its source," "scope
lifetime parameters to only the references borrowed into the output," "preserve a wrapped error's
structure; stringify only at display time," "exhaustive matches on domain enums," etc.),
published in full at Module 01. Each module's rubric keeps its deterministic criteria verbatim and
replaces its conceptual lines with one: *"No Idiom Charter violations in the diff (scored,
conceptual - the charter is published in full; which entries this module's exercise stresses is
not disclosed until grading)."*

**Module 04 walkthrough.** The learner read the charter back at Module 01 - they know the
workshop grades ownership discipline and lifetime scoping *somewhere*. Module 04's page names
no specific trap. They attempt, clone, pass the gate, then self-review the diff against the
charter (a real skill - it is exactly what reviewing your own PR against a team style guide is),
find the ownership entry, and refactor. The discovery ("*this* is the module where that entry
bites, and here's why it's tempting here") survives, diluted but real.

**Coachgremlin changes.** Grading gains a charter cross-reference step; the per-module
`grading.md` still holds the instantiated criteria internally (as the grader's application of the
charter - CS50 precedent says this is normal practice, not hidden grading).

**Cons, honestly.** (1) A diligent learner is spoiled *once, globally* at Module 01 - ten idioms
at once is far more diffuse than one named answer per module, but it is not zero. (2) The charter
must be substantially complete up front, and Modules 06-08's landmines have deliberately *not*
been decided yet - this workshop's own discipline is that each module's failure mode is
discovered by dry run, not assumed. Pre-committing charter entries for unauthored modules
contradicts the evidence-first ethos; adding entries per module re-spoils incrementally.
(3) The per-module "no guessing what's graded" guarantee becomes indirect - defensible (the
charter *is* the full disclosure) but a learner could reasonably say the module rubric got vaguer.

## Candidate C: accept the spoiler; reframe around the agentic reality

**Structure.** The informally-proposed direction (b), evaluated honestly - with one addition the
original framing missed. Present the target idiom as a known spec from the start; convert "Why
this is hard" into un-guarded pre-reading ("why this criterion exists"); delete the "Valid
alternate terminal" discovery fiction; and add one new criterion shape per module: a short written
**defense** (*"in ≤5 sentences, explain why `'a` is tied only to `session` - graded on the
explanation, not just the diff (scored, conceptual)"*). The exercise stops pretending the learner
will rediscover Rust idioms and starts grading what the workshop's premise actually implies:
recognizing, demanding, and *defending* idiomatic Rust produced in collaboration with an agent.

**Module 04 walkthrough.** The page says up front: the shipped stub's owned `String` is the
naive shape; your job is the borrowed `CheckpointAlert<'a>` with `'a` scoped to `session` alone,
plus a five-sentence defense of that scoping. The learner (or realistically their agent) produces
the borrow directly - no discovery - and the graded conceptual artifact becomes the defense,
which cannot be satisfied by copying the rubric line back.

**Coachgremlin changes.** Significant: the conceptual tier shifts from diff-reading to
explanation-grading. Precedent exists *inside this workshop already* - Module 07's planned gate
("learner can explain why a non-`Send` value can't cross an `.await` point") is already
explanation-shaped, so this would generalize an existing arc decision rather than invent one.

**Cons, honestly.** (1) It abandons productive failure's measurable conceptual-learning benefit
for exactly the audience PF works best on - that is a real pedagogical cost, not a style change.
(2) The workshop's distinctive identity ("the deterministic gate can't see this - feel why")
partially dissolves into spec-execution; the dry-run findings become documentation rather than the
learner's own arc. (3) Explanations can be agent-drafted too - grading a defense the learner's
agent wrote is the deep unsolved problem of agentic assessment, and this candidate walks straight
into it rather than around it. (4) It quietly rewrites the workshop's learning objectives; that is
an editorial decision about what Borrow Native *is*, above this tension's pay grade.

## Candidate D: axis disclosed pre-attempt, instance disclosed post-gate

**Structure.** The Gradescope pattern: each conceptual criterion is published pre-attempt at the
**axis** level with full weight ("criterion 6: ownership and allocation discipline in the new code
- the grader reads whether anything is owned that could be borrowed (scored, conceptual)"), and
the **instantiated** criterion (the exact field and technique, plus the dry-run evidence) unlocks
after the deterministic gate passes. **Flag: this bends constraint 1's strictest reading** ("fully
shared... not partial"). It is presented anyway because it is the *single most common resolution
in real autograded courses* - visible criterion names and weights, deferred detail - and the
decision-maker should get to decide whether "the axis, its weight, and what the grader will read"
constitutes full sharing. If constraint 1 is truly absolute, strike this candidate.

**Module 04 walkthrough.** Pre-attempt the learner sees the axis line above. They attempt; the
clone version passes the gate; the unlock reveals criterion 6 verbatim as shipped today plus the
"Why this is hard" narrative. Discovery is preserved almost perfectly - the axis says *where* to
look without saying *what* is wrong - and the post-gate reveal is exactly PF's
instruction-after-attempt.

**Coachgremlin changes.** Minimal for grading; some mechanism (a section gated in the module page,
or Coachgremlin sharing the full criterion at grading time) must implement the unlock - in
practice probably the same honor-system heading guard the modules already use, which is also its
weakness.

**Cons, honestly.** (1) The constraint-1 tension is real, not cosmetic. (2) Enforced by honor
system exactly like the current "don't read this yet" guards - it relocates the spoiler behind a
politeness barrier rather than removing it. (3) Axis phrasing that is informative enough to be
fair converges toward candidate A's property phrasing anyway - A may dominate D if authoring
effort is being spent either way.

---

## Ruled out

**Reordering the Rubric section after "Why this is hard" (informal candidate (a)):** cosmetic.
It changes page order, not information availability - the rubric remains readable pre-attempt (as
required), so the spoiler is one scroll further away and nothing else changes. No researched
system treats section order as the mechanism; every real resolution changes *what the criterion
says* (A), *where it lives* (B), *what is graded* (C), or *when detail unlocks* (D).

## Cross-cutting risks the authors may not have fully priced in

1. **The agentic short-circuit is the bigger problem, and no page structure fixes it.** Every
   candidate above except C's defense-criterion assumes the learner reads the rubric and then
   personally reasons. But this workshop's premise is harness-driven: the learner's coding agent
   already knows every idiom in this arc, and any pre-attempt text - spoiler-y or property-phrased
   - can be pasted into it verbatim. The discovery framing is already fragile *independent of the
   rubric wording*. This cuts two ways: it strengthens candidate C's honesty argument, and it
   suggests the durable fix for the *conceptual* tier is explanation/defense-shaped criteria
   (which the arc already commits to for Module 07) regardless of which candidate wins.
2. **Criteria compliance is currently untested.** The dry runs construct naive attempts *without*
   simulating a learner who read the rubric first - so the workshop has never actually observed
   whether its rubric produces Torrance-style compliance (correct diff, zero understanding).
   Whatever candidate is chosen, one dry-run variant worth adding: an attempt made *by following
   the rubric text alone*, graded to see whether Coachgremlin can tell it from an understood one.
3. **PF's post-attempt instruction requirement protects "Why this is hard."** If candidate C is
   chosen, the PF literature says the retrospective explanation still matters - do not let
   "accept the spoiler" quietly shrink the narrative sections; they are where the conceptual
   learning consolidates under every candidate.

## Proposal: make the agentic short-circuit an explicit, recurring workshop thread

Not decided - a candidate addition, orthogonal to picking A/B/C/D above, surfaced by connecting
this research to the workshop's actual premise (a learner working *through* a coding agent) more
directly than the four candidates do individually. Cross-cutting risk 1 above already names the
core observation: no rubric-wording fix removes the ability to paste a module's text into an
agent that already knows the idiom. That observation is bigger than a rubric-formatting problem -
it is close to the actual subject of a workshop whose learners are, by design, agent-literate
developers using a coding agent to learn a language. Right now the workshop treats that fact as a
delivery mechanism (the learner happens to use an agent to run `cargo test`). It could instead
treat it as part of the curriculum.

**Concretely, three additions, independent of which rubric candidate is chosen:**

1. **Name the skill explicitly, once, in `docs/workshop-design.md`/the top-level README, alongside
   the Rust concept arc.** Something like: *"This workshop teaches two things at once: a Rust
   concept, and the discipline of noticing when you're about to let your own agent do your
   understanding for you instead of your typing. The second skill doesn't have its own module -
   it's tested every time a rubric criterion, a hint, or a `cargo clippy` suggestion is one paste
   away from a finished answer."* This makes the tension a stated feature rather than an
   unstated vulnerability - the same reframe candidate C attempts locally, generalized to the
   whole workshop rather than smuggled into rubric wording module-by-module.
2. **A closing beat in every module's "Why this is hard" section**, after the reveal, before the
   Takeaway: a short, direct question, not scored, not gated - e.g. Module 04's version might
   read: *"If you'd pasted this exercise's prompt into your agent with no attempt of your own
   first, it would have produced the correctly-scoped `<'a>` in one shot, no clone anywhere in
   sight. That's not cheating - the deterministic gate doesn't care how the diff got written. But
   if that's what happened here, what did you actually learn versus what did your agent
   demonstrate? That question doesn't have a rubric line, on purpose - it's yours to answer
   honestly, not Coachgremlin's to grade."* This is cheap (one paragraph per module, no rubric
   mechanics to redesign), directly addresses this session's actual question ("how do we not
   outsource the learning"), and needs no decision on A/B/C/D to ship - it could go in now,
   independent of and before whichever candidate is chosen.
3. **A cross-cutting takeaway, not a per-module one**: an "agentic learning discipline" Skill
   (parallel to the per-module playbooks, but workshop-wide, maybe factory-wide - Coachgremlin's
   own `~/hekton/gremlins/` home suggests this could be reusable beyond `borrow-native`, the same
   way the Gremlin model itself is) that gives a learner a concrete self-check: *before* accepting
   an agent-produced diff for a module's own exercise, can you (a) name which concept it
   demonstrates without looking at the code, (b) predict whether it would pass the deterministic
   gate before running it, (c) explain the one thing about it a naive attempt would have gotten
   wrong instead. Three "no"s is a real signal the module was delegated, not learned - a Version
   of criteria-compliance detection the learner runs on themselves, which sidesteps the problem of
   Coachgremlin (or anyone) grading whether a *written defense* was itself agent-drafted
   (candidate C's own unresolved cost, cross-cutting risk 3).

**Why propose this now rather than folding it into one candidate:** addition 2 costs almost
nothing and can ship regardless of the A/B/C/D decision timeline; addition 3 turns the
"Struggle Is the Point" blog post's open ending into a real one - if this ships, that post can
report "the workshop's actual answer" rather than leaving the reader with an open question. Addition
1 is the only one that requires committing to a specific framing workshop-wide, and is the most
worth coderturtle's explicit sign-off before it goes in any public-facing doc.

**What this does not do:** it does not replace a decision on A/B/C/D - the rubric line itself
still spoils the answer today regardless of whether a learner is later asked to reflect on
whether they outsourced their understanding. It is a complementary thread, not a substitute fix.

## Decision (2026-07-05, coderturtle)

**Adopted: Candidate A (property-phrased criteria), plus all three items in the Proposal section
above.** Not B, C, or D - reasoning below.

**Why A over the others.** B (idiom charter) requires pre-committing entries for Modules 06-08
before their own dry runs exist, against this workshop's evidence-first ethos (every prior module's
finding was checked fresh, never assumed to generalize from an earlier one - see Modules 02, 03, 04,
05's own retros each saying so explicitly). C (accept the spoiler, add defense-shaped criteria)
trades away the productive-failure benefit this workshop's attempt-then-reveal structure already has
real backing for, specifically for the audience (developers with prior knowledge) this workshop
targets - too large a cost to accept just to solve a rubric-wording problem, when cross-cutting risk
1 already says wording alone won't solve the deeper problem anyway. D (axis pre-attempt, instance
post-gate) bends constraint 1 (rubric fully shared before the attempt) and is enforced by the same
honor-system guard the modules' existing spoiler-avoidance headers already rely on - not a
structural improvement, just a differently-shaped version of the status quo. A is the only
candidate that keeps every constraint from `rubric-spoiler-tension.md` intact (rubric-before-attempt,
independently checkable, generalizes across all 8 modules, doesn't touch the hands-on-by-design
principle) while still being a real change, not a relabeling.

**Why the Proposal's three additions too, not A alone.** Cross-cutting risk 1 is the load-bearing
finding of this whole research pass: no rubric-wording fix, including A, removes a coding agent's
ability to derive a technique from a stated property in one shot. A is worth doing anyway (it helps
a human reading slowly, and it's the more honest phrasing regardless of the agentic question), but
adopting it alone without acknowledging what it doesn't fix would be the same kind of overclaim this
workshop's own Review Panel exists to catch. The reflective closing beat and the
`agentic-learning-discipline` Skill are the honest acknowledgment: they don't close the gap
mechanically (nothing scored, nothing gated - see the Skill's own "why this isn't a rubric fix"
section), they give the learner a real tool for the actual problem A can't solve.

**What this does not claim.** It does not claim the agentic short-circuit is solved. It claims the
workshop now (a) states its rubric criteria more honestly than before, and (b) treats the deeper
problem as a named, explicit part of what it teaches rather than an unstated risk - which is a
meaningfully different, more honest position than either "the rubric is fine" (false, per the
Review Panel's own finding) or "nothing can be done" (also false, per this research). A third,
differently-shaped workshop or a real independent-learner test (both already-named open items for
this workshop's own maturity bar - see `docs/next-actions.md`) would be the actual test of whether
any of this holds up outside one team's own editorial judgment.

**Implementation, same session:** Modules 01-05's rubric criteria rewritten to property-phrased
form (`modules/*/README.md`); a reflective closing beat added to each module's "Why this is hard"
section; `docs/workshop-design.md` names the meta-skill; `.claude/skills/agentic-learning-discipline/SKILL.md`
created and cross-referenced from `modules/README.md`'s "What you keep" table. Historical
`runs/*/grading.md`/`retro.md` evidence was not rewritten - the underlying pass/fail distinction
each dry run tested is unchanged by a rubric-wording edit, only what a future learner reads before
attempting is different.

## Sources

- [Torrance's criteria-compliance critique - Frontiers editorial, Transparency in Assessment](https://www.frontiersin.org/journals/education/articles/10.3389/feduc.2018.00119/full)
- [Panadero et al., A critical review of the arguments against the use of rubrics](https://www.sciencedirect.com/science/article/abs/pii/S1747938X19303732)
- [Sadler, Indeterminacy in the use of preset criteria for assessment and grading](https://www.tandfonline.com/doi/full/10.1080/02602930801956059)
- [Sinha & Kapur, When Problem Solving Followed by Instruction Works: Evidence for Productive Failure (2021 review)](https://journals.sagepub.com/doi/10.3102/00346543211019105)
- [Kapur, Productive Failure in Learning Math (Cognitive Science)](https://onlinelibrary.wiley.com/doi/10.1111/cogs.12107)
- [Kalyuga, Expertise Reversal Effect and Its Implications](https://www.uky.edu/~gmswan3/EDC608/Kalyuga2007_Article_ExpertiseReversalEffectAndItsI.pdf)
- [Expertise reversal effect - overview](https://en.wikipedia.org/wiki/Expertise_reversal_effect)
- [Gradescope autograder specifications (test visibility settings)](https://gradescope-autograders.readthedocs.io/en/latest/specs/)
- [CS50 grading axes (correctness / design / style)](https://cs50.tf/ap/2024/grading/)
- [CS50 syllabus (axis weighting)](https://cs50.harvard.edu/x/syllabus/)
- [Cult of Pedagogy, Meet the Single Point Rubric](https://www.cultofpedagogy.com/single-point-rubric/)
