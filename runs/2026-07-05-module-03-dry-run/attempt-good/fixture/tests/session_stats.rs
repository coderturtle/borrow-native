use relay::{finalize_session, session_stats, DraftCheckpoint, Session};

// Every session in this file is built through `finalize_session` (Module 01's
// exercise), not a `Session { .. }` literal - Module 02's stated prerequisite
// on Module 01 is enforced mechanically, not just claimed. If `finalize_session`
// is still its unsolved `todo!()` stub, every test below panics before
// `session_stats` is ever reached. See docs/decisions.md, 2026-07-05, "Module 02
// prerequisite enforced mechanically."
fn session(label: &str, drafts: Vec<DraftCheckpoint>) -> Session {
    finalize_session(label.to_string(), drafts)
}

fn draft(goal: &str, elapsed_secs: u64) -> DraftCheckpoint {
    DraftCheckpoint {
        goal: goal.to_string(),
        summary: format!("{goal} summary"),
        risks: vec![],
        elapsed_secs,
    }
}

#[test]
fn empty_session_produces_zeroed_stats_not_nan() {
    let s = session("empty", vec![]);
    let stats = session_stats(&s);
    assert_eq!(stats.checkpoint_count, 0);
    assert_eq!(stats.average_gap_secs, 0.0);
    assert_eq!(stats.longest_gap_secs, 0);
    assert_eq!(stats.longest_gap_goal, "");
}

#[test]
fn single_checkpoint_gap_is_its_own_elapsed() {
    let s = session("solo", vec![draft("reproduce the bug", 45)]);
    let stats = session_stats(&s);
    assert_eq!(stats.checkpoint_count, 1);
    assert_eq!(stats.average_gap_secs, 45.0);
    assert_eq!(stats.longest_gap_secs, 45);
    assert_eq!(stats.longest_gap_goal, "reproduce the bug");
}

#[test]
fn average_gap_is_the_mean_not_the_last_or_a_running_total() {
    let s = session(
        "ordered",
        vec![draft("first", 0), draft("second", 30), draft("third", 90)],
    );
    let stats = session_stats(&s);
    assert_eq!(stats.checkpoint_count, 3);
    // (0 + 30 + 90) / 3 = 40.0 - not 90.0 (last) and not 120.0 (running total).
    assert_eq!(stats.average_gap_secs, 40.0);
}

#[test]
fn longest_gap_identifies_the_right_checkpoint_not_just_the_number() {
    let s = session(
        "ordered",
        vec![draft("first", 0), draft("second", 90), draft("third", 30)],
    );
    let stats = session_stats(&s);
    assert_eq!(stats.longest_gap_secs, 90);
    // The longest gap is the *second* checkpoint, not the last one - a stats
    // function that only ever looks at `.last()` passes the average-gap test
    // above but fails this one.
    assert_eq!(stats.longest_gap_goal, "second");
}

#[test]
fn tied_longest_gap_resolves_to_first_occurrence() {
    let s = session(
        "tied",
        vec![draft("first", 60), draft("second", 60), draft("third", 10)],
    );
    let stats = session_stats(&s);
    assert_eq!(stats.longest_gap_secs, 60);
    assert_eq!(stats.longest_gap_goal, "first");
}

#[test]
fn session_is_still_usable_after_computing_stats() {
    let s = session("reused", vec![draft("first", 10), draft("second", 20)]);
    let stats = session_stats(&s);
    assert_eq!(stats.checkpoint_count, 2);
    // If `session_stats` had taken `session` by value instead of `&Session`,
    // this line wouldn't compile at all - the borrow is what makes this
    // still-usable-afterward check possible in the first place.
    assert_eq!(s.label, "reused");
    assert_eq!(s.checkpoints.len(), 2);
}
