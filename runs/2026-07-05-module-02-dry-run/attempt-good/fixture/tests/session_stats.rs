use relay::{session_stats, CheckpointRecord, Session};

fn record(goal: &str, elapsed_secs: u64) -> CheckpointRecord {
    CheckpointRecord {
        goal: goal.to_string(),
        summary: format!("{goal} summary"),
        risks: vec![],
        elapsed_secs,
    }
}

#[test]
fn empty_session_produces_zeroed_stats_not_nan() {
    let session = Session {
        label: "empty".to_string(),
        checkpoints: vec![],
    };
    let stats = session_stats(&session);
    assert_eq!(stats.checkpoint_count, 0);
    assert_eq!(stats.average_gap_secs, 0.0);
    assert_eq!(stats.longest_gap_secs, 0);
    assert_eq!(stats.longest_gap_goal, "");
}

#[test]
fn single_checkpoint_gap_is_its_own_elapsed() {
    let session = Session {
        label: "solo".to_string(),
        checkpoints: vec![record("reproduce the bug", 45)],
    };
    let stats = session_stats(&session);
    assert_eq!(stats.checkpoint_count, 1);
    assert_eq!(stats.average_gap_secs, 45.0);
    assert_eq!(stats.longest_gap_secs, 45);
    assert_eq!(stats.longest_gap_goal, "reproduce the bug");
}

#[test]
fn average_gap_is_the_mean_not_the_last_or_a_running_total() {
    let session = Session {
        label: "ordered".to_string(),
        checkpoints: vec![record("first", 0), record("second", 30), record("third", 90)],
    };
    let stats = session_stats(&session);
    assert_eq!(stats.checkpoint_count, 3);
    // (0 + 30 + 90) / 3 = 40.0 - not 90.0 (last) and not 120.0 (running total).
    assert_eq!(stats.average_gap_secs, 40.0);
}

#[test]
fn longest_gap_identifies_the_right_checkpoint_not_just_the_number() {
    let session = Session {
        label: "ordered".to_string(),
        checkpoints: vec![record("first", 0), record("second", 90), record("third", 30)],
    };
    let stats = session_stats(&session);
    assert_eq!(stats.longest_gap_secs, 90);
    // The longest gap is the *second* checkpoint, not the last one - a stats
    // function that only ever looks at `.last()` passes the average-gap test
    // above but fails this one.
    assert_eq!(stats.longest_gap_goal, "second");
}

#[test]
fn tied_longest_gap_resolves_to_first_occurrence() {
    let session = Session {
        label: "tied".to_string(),
        checkpoints: vec![record("first", 60), record("second", 60), record("third", 10)],
    };
    let stats = session_stats(&session);
    assert_eq!(stats.longest_gap_secs, 60);
    assert_eq!(stats.longest_gap_goal, "first");
}

#[test]
fn session_is_still_usable_after_computing_stats() {
    let session = Session {
        label: "reused".to_string(),
        checkpoints: vec![record("first", 10), record("second", 20)],
    };
    let stats = session_stats(&session);
    assert_eq!(stats.checkpoint_count, 2);
    // If `session_stats` had taken `session` by value instead of `&Session`,
    // this line wouldn't compile at all - the borrow is what makes this
    // still-usable-afterward check possible in the first place.
    assert_eq!(session.label, "reused");
    assert_eq!(session.checkpoints.len(), 2);
}
