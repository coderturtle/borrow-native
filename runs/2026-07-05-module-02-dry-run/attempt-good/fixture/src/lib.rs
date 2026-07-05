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
/// the session's history rather than consuming it - the learner never needs
/// to give up `session` to compute these.
pub struct SessionStats {
    pub checkpoint_count: usize,
    pub average_gap_secs: f64,
    pub longest_gap_secs: u64,
    /// The goal of the checkpoint that produced the longest gap. Owned, on
    /// purpose: this value must outlive the borrow of `session` that produced
    /// it, so cloning it here isn't a habit to unlearn - it's the actual fix.
    pub longest_gap_goal: String,
}

/// Compute gap statistics over a session's checkpoint history without taking
/// ownership of it.
///
/// Edge cases this must handle (see `SPEC.md` for the full list): an empty
/// session (no checkpoints), a single checkpoint, and multiple checkpoints
/// with a tie for the longest gap (first occurrence wins).
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

    // `Iterator::max_by_key` returns the *last* of several equally-maximum
    // elements, but the spec wants the *first* - so this walks manually with
    // a strict `>` instead, which only updates on a new strict maximum.
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
