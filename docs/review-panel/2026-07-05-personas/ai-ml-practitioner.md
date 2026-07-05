# Technical Accuracy Critique — AI/ML Practitioner Lens

## Finding 1 (most severe): Rustlings exercise count is wrong
`workshop-design.md` states Rustlings has "**23 numbered exercises**." This is off by roughly 4x. Rustlings organizes exercises into ~20-25 *topic directories* (variables, move_semantics, structs, enums, generics, traits, lifetimes, threads, etc.), each containing multiple numbered files — the actual exercise count is closer to 90+. Conflating "topics" with "exercises" undercuts the doc's own claim to have "fetched and read directly" the source material rather than working from memory.

## Finding 2: Exercism exercise names look fabricated
The claim that Exercism's Rust track `config.json` shows `entry-api requires [functions, option]` and `vec-stack requires [functions, integers, option, enums]` is presented as verified, fetched evidence ("Real, current evidence... this workshop invented"). But Exercism exercises are drawn from the shared cross-language `problem-specifications` repo and use whimsical/thematic canonical names (`high-scores`, `robot-name`, `clock`, `diamond`, `luhn`, etc.) — they are not literally named after the API or data structure under test. `entry-api` and `vec-stack` are not names I recognize from that track, and the naming *convention itself* is inconsistent with how Exercism names exercises. This reads as invented detail dressed as a citation.

## Finding 3: Concurrency's dependency on Module 04 is asserted, not demonstrated
The reasoning for Module 06 needing Module 04 is "`Send`/`Sync` are traits." But `Send`/`Sync` are zero-method marker traits taught *within* ch16 itself (the Book's own final section of Fearless Concurrency covers them directly) — they don't require the trait-bound/lifetime-annotation machinery that Module 04 is actually about (`T: PartialOrd`, `&'a`). This prerequisite link is weaker than claimed.

## Finding 4: Silent reversal of Book's canonical chapter order
Module 05 (Error Handling, Book ch9) is placed *after* Module 04 (Generics/Traits/Lifetimes, ch10) — reversing the Book's own sequence — justified by "`?` across generic error types." That's true for idiomatic `Box<dyn Error>`/custom-error work, but the Book itself teaches ch9 before ch10 using concrete error types. The doc claims to "anchor" to the Book's order while quietly inverting two chapters; this deserves explicit acknowledgment rather than being buried in a table.

No other outright false statements found — the ch4→ch20 chapter renumbering (including Async at ch17) and the `largest<T: PartialOrd>` elided-lifetime example are both correct.
