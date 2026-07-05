use relay::{next_action, CheckpointTrigger, HumanResponse, NextAction};

#[test]
fn acknowledged_continues_regardless_of_trigger() {
    let triggers = [
        CheckpointTrigger::TimeElapsed(600),
        CheckpointTrigger::ToolCallCount(40),
        CheckpointTrigger::ContextBudget(0.95),
    ];
    for trigger in &triggers {
        assert!(matches!(
            next_action(trigger, &HumanResponse::Acknowledged),
            NextAction::Continue
        ));
    }
}

#[test]
fn snoozed_pauses_for_exactly_the_given_duration_regardless_of_trigger() {
    let triggers = [
        CheckpointTrigger::TimeElapsed(600),
        CheckpointTrigger::ContextBudget(0.95),
    ];
    for trigger in &triggers {
        match next_action(trigger, &HumanResponse::Snoozed(120)) {
            NextAction::PauseFor(secs) => assert_eq!(secs, 120),
            _ => panic!("expected PauseFor(120), got a different NextAction variant"),
        }
    }
}

#[test]
fn ignored_context_budget_ends_the_session() {
    // The safety-critical case: ignoring a blown context budget must not be
    // treated as safe to continue past.
    assert!(matches!(
        next_action(
            &CheckpointTrigger::ContextBudget(0.98),
            &HumanResponse::Ignored
        ),
        NextAction::EndSession
    ));
}

#[test]
fn ignored_time_elapsed_continues() {
    assert!(matches!(
        next_action(&CheckpointTrigger::TimeElapsed(600), &HumanResponse::Ignored),
        NextAction::Continue
    ));
}

#[test]
fn ignored_tool_call_count_continues() {
    assert!(matches!(
        next_action(&CheckpointTrigger::ToolCallCount(40), &HumanResponse::Ignored),
        NextAction::Continue
    ));
}
