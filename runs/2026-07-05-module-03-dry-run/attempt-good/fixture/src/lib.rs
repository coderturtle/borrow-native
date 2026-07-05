//! `relay`: a restartable-handoff CLI for hybrid human-agent team pacing.
//! See `SPEC.md` for the full product spec and the module-by-module build-out.

/// A checkpoint still being assembled during a work session, before it's sealed.
pub struct DraftCheckpoint {
    pub goal: String,
    pub summary: String,
    pub risks: Vec<String>,
    pub elapsed_secs: u64,
}

/// A finalized, sealed checkpoint record.
pub struct CheckpointRecord {
    pub goal: String,
    pub summary: String,
    pub risks: Vec<String>,
    pub elapsed_secs: u64,
}

/// A finished work session: a label plus every checkpoint reached during it, in order.
pub struct Session {
    pub label: String,
    pub checkpoints: Vec<CheckpointRecord>,
}

/// Seal a session's drafted checkpoints into finalized records.
pub fn finalize_session(label: String, drafts: Vec<DraftCheckpoint>) -> Session {
    let checkpoints = drafts
        .into_iter()
        .map(|draft| CheckpointRecord {
            goal: draft.goal,
            summary: draft.summary,
            risks: draft.risks,
            elapsed_secs: draft.elapsed_secs,
        })
        .collect();
    Session { label, checkpoints }
}

/// Summary statistics over a session's checkpoint gaps, computed by borrowing
/// the session's history rather than consuming it.
pub struct SessionStats {
    pub checkpoint_count: usize,
    pub average_gap_secs: f64,
    pub longest_gap_secs: u64,
    pub longest_gap_goal: String,
}

/// Compute gap statistics over a session's checkpoint history without taking
/// ownership of it.
pub fn session_stats(session: &Session) -> SessionStats {
    let checkpoints = &session.checkpoints;

    if checkpoints.is_empty() {
        return SessionStats {
            checkpoint_count: 0,
            average_gap_secs: 0.0,
            longest_gap_secs: 0,
            longest_gap_goal: String::new(),
        };
    }

    let total: u64 = checkpoints.iter().map(|c| c.elapsed_secs).sum();
    let average_gap_secs = total as f64 / checkpoints.len() as f64;

    let mut longest_gap_secs = checkpoints[0].elapsed_secs;
    let mut longest_gap_goal = checkpoints[0].goal.clone();
    for checkpoint in checkpoints.iter().skip(1) {
        if checkpoint.elapsed_secs > longest_gap_secs {
            longest_gap_secs = checkpoint.elapsed_secs;
            longest_gap_goal = checkpoint.goal.clone();
        }
    }

    SessionStats {
        checkpoint_count: checkpoints.len(),
        average_gap_secs,
        longest_gap_secs,
        longest_gap_goal,
    }
}

/// Why a checkpoint fired. Exactly one reason per checkpoint, by construction.
pub enum CheckpointTrigger {
    TimeElapsed(u64),
    ToolCallCount(u32),
    ContextBudget(f64),
}

/// How a human responded to a checkpoint notification.
pub enum HumanResponse {
    Acknowledged,
    Snoozed(u64),
    Ignored,
}

/// What `relay` does next, given why a checkpoint fired and how the human responded.
pub enum NextAction {
    Continue,
    PauseFor(u64),
    EndSession,
}

/// Decide the next action for a checkpoint, given why it fired and how the
/// human responded to it. Every `CheckpointTrigger` variant is listed
/// explicitly under `Ignored`, not covered by a `_` catch-all: `ContextBudget`
/// resolves differently than the other two today, and a new trigger variant
/// added later would fail to compile here until its own `Ignored` outcome was
/// decided, rather than silently inheriting `Continue`.
pub fn next_action(trigger: &CheckpointTrigger, response: &HumanResponse) -> NextAction {
    match response {
        HumanResponse::Acknowledged => NextAction::Continue,
        HumanResponse::Snoozed(secs) => NextAction::PauseFor(*secs),
        HumanResponse::Ignored => match trigger {
            CheckpointTrigger::ContextBudget(_) => NextAction::EndSession,
            CheckpointTrigger::TimeElapsed(_) => NextAction::Continue,
            CheckpointTrigger::ToolCallCount(_) => NextAction::Continue,
        },
    }
}
