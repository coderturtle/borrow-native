use relay::{alert_checkpoint, CheckpointTrigger, Notifier, Session};
use std::cell::RefCell;

/// A test double that records every message it's asked to send instead of
/// touching a real terminal or OS notifier. Configurable to simulate a
/// failed delivery, so `alert_checkpoint`'s `sent` field can be checked
/// against both outcomes, not just the always-succeeds case every real
/// `Notifier` in `src/lib.rs` happens to be.
struct RecordingNotifier {
    calls: RefCell<Vec<String>>,
    succeeds: bool,
}

impl RecordingNotifier {
    fn new(succeeds: bool) -> Self {
        RecordingNotifier {
            calls: RefCell::new(Vec::new()),
            succeeds,
        }
    }
}

impl Notifier for RecordingNotifier {
    fn notify(&self, message: &str) -> bool {
        self.calls.borrow_mut().push(message.to_string());
        self.succeeds
    }
}

fn session_named(label: &str) -> Session {
    Session {
        label: label.to_string(),
        checkpoints: vec![],
    }
}

#[test]
fn time_elapsed_message_format_is_exact() {
    let notifier = RecordingNotifier::new(true);
    let session = session_named("module-04-dry-run");
    let alert = alert_checkpoint(&notifier, &session, &CheckpointTrigger::TimeElapsed(600));
    assert_eq!(alert.message, "Checkpoint: 600s since last checkpoint");
}

#[test]
fn tool_call_count_message_format_is_exact() {
    let notifier = RecordingNotifier::new(true);
    let session = session_named("module-04-dry-run");
    let alert = alert_checkpoint(&notifier, &session, &CheckpointTrigger::ToolCallCount(40));
    assert_eq!(alert.message, "Checkpoint: 40 tool calls since last checkpoint");
}

#[test]
fn context_budget_message_rounds_to_nearest_whole_percent() {
    let notifier = RecordingNotifier::new(true);
    let session = session_named("module-04-dry-run");
    let alert = alert_checkpoint(&notifier, &session, &CheckpointTrigger::ContextBudget(0.983));
    assert_eq!(alert.message, "Checkpoint: 98% of context budget used");
}

#[test]
fn sent_reflects_notifier_failure_not_hardcoded_true() {
    let notifier = RecordingNotifier::new(false);
    let session = session_named("module-04-dry-run");
    let alert = alert_checkpoint(&notifier, &session, &CheckpointTrigger::TimeElapsed(600));
    assert!(!alert.sent, "sent must reflect the notifier's real return value");
}

#[test]
fn session_label_matches_the_session_actually_passed_in() {
    let notifier = RecordingNotifier::new(true);

    let session_a = session_named("session-a");
    let alert_a = alert_checkpoint(&notifier, &session_a, &CheckpointTrigger::TimeElapsed(60));
    assert_eq!(alert_a.session_label, "session-a");

    let session_b = session_named("session-b");
    let alert_b = alert_checkpoint(&notifier, &session_b, &CheckpointTrigger::TimeElapsed(60));
    assert_eq!(alert_b.session_label, "session-b");
}

#[test]
fn notifier_is_invoked_exactly_once_with_the_built_message() {
    let notifier = RecordingNotifier::new(true);
    let session = session_named("module-04-dry-run");
    let alert = alert_checkpoint(&notifier, &session, &CheckpointTrigger::ToolCallCount(5));

    let calls = notifier.calls.borrow();
    assert_eq!(calls.len(), 1, "notify must be called exactly once");
    assert_eq!(calls[0], alert.message);
}
