# Technical Writer Review — Borrow Native

## Findings (most significant first)

**1. Cross-reference that doesn't resolve.** `workshop-design.md`'s build-log section says: "Published as a dated build-log/journal via GitHub Pages (**Astro, see `docs/workshop-gremlin-design.md`**)." Reading `workshop-gremlin-design.md` in full — it never mentions Astro. It only says a "build-log site" is a remaining Completion Condition item. The citation promises detail that isn't at the destination; a reader following it will feel misdirected. Either add the Astro detail to the gremlin doc or drop the pointer.

**2. Duplicate curriculum anchor, unexplained.** In the module table, `06_move_semantics` is listed as a Rustlings anchor for **both** Module 01 (Ownership & Move Semantics) and Module 02 (Borrowing & References). Nothing in the surrounding prose explains why the same exercise anchors two different modules — reads as a copy-paste residue rather than a deliberate choice.

**3. Overloaded run-on sentence hurts the section it's supposed to clarify.** The "Why this order" paragraph is one continuous chain of clauses (Ownership → Borrowing → Structs → Generics → Error handling → Concurrency) with no sentence breaks doing structural work. The worst offender: "...splitting them the way this workshop split harness/loop engineering in `terminal-velocity` would misrepresent how tightly coupled they actually are in Rust, unlike that split, which *was* real." This is doing real reasoning but the syntax buries it — a reader has to parse a negated analogy inside a dash-clause inside an already-long sentence.

**4. Grammar slip.** Row 03 of the takeaway table: "**A** idiomatic-modeling checklist" — should be "An."

**5. Inconsistent module-numbering style.** The tables use zero-padded IDs (`01`, `02`...), but the "Why this order" prose reverts to unpadded references ("module 1-2," "module 4"). Minor, but it's the kind of small inconsistency a style pass should catch.

**No contradictions found** between the two docs on module count, naming status, gate-tier terminology (Coachgremlin vs. Workshop Gremlin used consistently and distinctly throughout), or completion status — those actually line up well.
