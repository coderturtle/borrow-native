//! `relay`: a restartable-handoff CLI for hybrid human-agent team pacing.
//! See `SPEC.md` for the full product spec and the module-by-module build-out.

/// A checkpoint still being assembled during a work session, before it's sealed.
pub struct DraftCheckpoint {
    pub goal: String,
    pub summary: String,
    pub risks: Vec<String>,
}

/// A finalized, sealed checkpoint record.
pub struct CheckpointRecord {
    pub goal: String,
    pub summary: String,
    pub risks: Vec<String>,
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
        })
        .collect();
    Session { label, checkpoints }
}
