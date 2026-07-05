//! `relay`: a restartable-handoff CLI for hybrid human-agent team pacing.
//! See `SPEC.md` for the full product spec and the module-by-module build-out.

/// A checkpoint still being assembled during a work session, before it's sealed.
pub struct DraftCheckpoint {
    pub goal: String,
    pub summary: String,
    pub risks: Vec<String>,
    /// Seconds since the previous checkpoint was reached (0 for a session's
    /// first checkpoint). Supplied by whatever clock is driving the session -
    /// `relay` never reads the system clock directly, which keeps every
    /// module's tests deterministic. Added in Module 02 for `session_stats`;
    /// see `SPEC.md`'s Module 02 section.
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
///
/// Edge cases this must handle (see `SPEC.md` for the full list): empty drafts,
/// a single draft, multiple drafts preserving order, and risks (including an
/// empty list) preserved exactly as given.
pub fn finalize_session(label: String, drafts: Vec<DraftCheckpoint>) -> Session {
    todo!("implement per SPEC.md")
}

/// Summary statistics over a session's checkpoint gaps, computed by borrowing
/// the session's history rather than consuming it - the learner never needs
/// to give up `session` to compute these.
pub struct SessionStats {
    pub checkpoint_count: usize,
    pub average_gap_secs: f64,
    pub longest_gap_secs: u64,
    /// The goal of the checkpoint that produced the longest gap. Owned, on
    /// purpose: `SessionStats` has no lifetime parameter (lifetimes arrive in
    /// Module 04), so this value must be owned to leave the function - given
    /// that shape, cloning it here once isn't a habit to unlearn, it's the
    /// correct move.
    pub longest_gap_goal: String,
}

/// Compute gap statistics over a session's checkpoint history without taking
/// ownership of it.
///
/// Edge cases this must handle (see `SPEC.md` for the full list): an empty
/// session (no checkpoints), a single checkpoint, and multiple checkpoints
/// with a tie for the longest gap (first occurrence wins).
pub fn session_stats(session: &Session) -> SessionStats {
    todo!("implement per SPEC.md")
}

/// Why a checkpoint fired. Exactly one reason per checkpoint, by construction -
/// an enum, not three `bool`/`Option` fields on a struct, because a checkpoint
/// firing for more than one reason at once, or for none, isn't a state `relay`
/// should be able to represent, let alone have to handle.
pub enum CheckpointTrigger {
    /// Seconds since the last checkpoint exceeded the configured interval.
    TimeElapsed(u64),
    /// Tool calls since the last checkpoint exceeded the configured count.
    ToolCallCount(u32),
    /// Fraction of the context budget used (0.0-1.0) exceeded the configured threshold.
    ContextBudget(f64),
}

/// How a human responded to a checkpoint notification. Also exactly one of
/// these per response, never a combination - see `CheckpointTrigger`'s doc
/// comment for why that's modeled as an enum rather than flags.
pub enum HumanResponse {
    Acknowledged,
    /// Snooze for this many seconds before the next checkpoint check.
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
/// human responded to it.
///
/// Edge cases this must handle (see `SPEC.md` for the full rule table):
/// `Acknowledged` and `Snoozed` both resolve the same way regardless of which
/// `CheckpointTrigger` fired - only `Ignored`'s outcome depends on the trigger,
/// and only for one specific trigger.
pub fn next_action(trigger: &CheckpointTrigger, response: &HumanResponse) -> NextAction {
    todo!("implement per SPEC.md")
}
