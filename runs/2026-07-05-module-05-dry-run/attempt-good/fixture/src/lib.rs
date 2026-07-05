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

impl Session {
    /// Borrow this session's label. No lifetime annotation needed here -
    /// elision rule 3 covers it (a `&self` method's elided output lifetime
    /// is assigned `&self`'s own). Contrast with `alert_checkpoint` below:
    /// a free function with no `self` and more than one reference
    /// parameter, where elision has no rule left to apply and the
    /// compiler requires an explicit lifetime the moment the output
    /// borrows from one of them.
    pub fn label(&self) -> &str {
        &self.label
    }
}

/// How `relay` actually delivers a checkpoint notification to a human.
/// Three real channels are provided below; `alert_checkpoint` is written
/// generically over any type implementing this trait (including, in
/// tests, a fourth: a recording test double) rather than hardcoding one.
pub trait Notifier {
    /// Deliver `message`. Returns whether delivery actually succeeded -
    /// real channels can fail (no display server attached, output isn't a
    /// tty); a test double can simulate either outcome deterministically,
    /// which is exactly why `alert_checkpoint` must trust this return
    /// value rather than assuming every notification lands.
    fn notify(&self, message: &str) -> bool;
}

/// Simulates a desktop OS notification (what a real build would hand off
/// to `notify-send`/`osascript`/a toast API). Stubbed as a `println!` on
/// purpose: shelling out to a real platform notifier is an OS-integration
/// concern, not this module's subject.
pub struct DesktopNotifier;

impl Notifier for DesktopNotifier {
    fn notify(&self, message: &str) -> bool {
        println!("[desktop] {message}");
        true
    }
}

/// Rings the terminal bell (`\x07`) ahead of the message - audible even if
/// the learner's terminal is scrolled away or behind another window.
pub struct TerminalBellNotifier;

impl Notifier for TerminalBellNotifier {
    fn notify(&self, message: &str) -> bool {
        println!("\x07[bell] {message}");
        true
    }
}

/// The plainest channel: prints the message with no decoration. Always
/// succeeds - there's no external system for stdout to fail to reach.
pub struct StdoutNotifier;

impl Notifier for StdoutNotifier {
    fn notify(&self, message: &str) -> bool {
        println!("{message}");
        true
    }
}

/// A record of one checkpoint notification: which session it belongs to,
/// what message was sent, and whether the notifier reported success.
/// `session_label` borrows straight from the `Session` that produced this
/// alert rather than cloning it.
pub struct CheckpointAlert<'a> {
    pub session_label: &'a str,
    pub message: String,
    pub sent: bool,
}

/// Build a checkpoint's alert message and hand it to any `Notifier`.
///
/// `'a` is tied only to `session` - the one reference this function's
/// output actually borrows from.
pub fn alert_checkpoint<'a, N: Notifier>(
    notifier: &N,
    session: &'a Session,
    trigger: &CheckpointTrigger,
) -> CheckpointAlert<'a> {
    let message = match trigger {
        CheckpointTrigger::TimeElapsed(secs) => {
            format!("Checkpoint: {secs}s since last checkpoint")
        }
        CheckpointTrigger::ToolCallCount(n) => {
            format!("Checkpoint: {n} tool calls since last checkpoint")
        }
        CheckpointTrigger::ContextBudget(frac) => {
            format!("Checkpoint: {:.0}% of context budget used", frac * 100.0)
        }
    };
    let sent = notifier.notify(&message);
    CheckpointAlert {
        session_label: session.label(),
        message,
        sent,
    }
}

/// A parsed, valid relay configuration.
#[derive(Debug)]
pub struct RelayConfig {
    pub checkpoint_interval_secs: u64,
    pub notifier_kind: NotifierKind,
}

/// Which real `Notifier` (see the Module 04 section above) a parsed config
/// selects. A separate type from `Notifier` itself on purpose: this is data
/// describing a choice, not a channel that can actually deliver a message -
/// something in `main` (not yet wired up) would match on this to pick the
/// real `Notifier` impl to construct.
#[derive(Debug)]
pub enum NotifierKind {
    Desktop,
    TerminalBell,
    Stdout,
}

/// Everything that can go wrong parsing a relay config file. A caller can
/// match on which of these it got and react differently - a missing key
/// might be recoverable with a hardcoded default upstream; an unknown
/// notifier name never is. This shape is given; the exercise is
/// `parse_config`'s body, not this enum.
#[derive(Debug)]
pub enum ConfigError {
    MissingKey(String),
    InvalidInterval { key: String, value: String },
    UnknownNotifier(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::MissingKey(key) => write!(f, "missing required config key: {key}"),
            ConfigError::InvalidInterval { key, value } => {
                write!(f, "invalid value for {key}: {value:?} is not a valid number of seconds")
            }
            ConfigError::UnknownNotifier(value) => {
                write!(f, "unknown notifier: {value:?} (expected \"desktop\", \"bell\", or \"stdout\")")
            }
        }
    }
}

impl std::error::Error for ConfigError {}

/// Parse a relay config file: newline-separated `key=value` lines (blank
/// lines ignored, unrecognized keys ignored). Required keys:
/// `checkpoint_interval_secs` (a `u64`) and `notifier` (one of `"desktop"`,
/// `"bell"`, or `"stdout"`). See `SPEC.md`'s Module 05 section for the full
/// edge-case table this must satisfy - in particular, every failure this
/// function can hit must come back as an `Err(ConfigError::..)`, never a
/// panic, and never lose which specific thing went wrong.
fn parse_interval(value: &str) -> Result<u64, ConfigError> {
    value.parse::<u64>().map_err(|_| ConfigError::InvalidInterval {
        key: "checkpoint_interval_secs".to_string(),
        value: value.to_string(),
    })
}

fn parse_notifier_kind(value: &str) -> Result<NotifierKind, ConfigError> {
    match value {
        "desktop" => Ok(NotifierKind::Desktop),
        "bell" => Ok(NotifierKind::TerminalBell),
        "stdout" => Ok(NotifierKind::Stdout),
        other => Err(ConfigError::UnknownNotifier(other.to_string())),
    }
}

pub fn parse_config(input: &str) -> Result<RelayConfig, ConfigError> {
    let mut interval_raw: Option<&str> = None;
    let mut notifier_raw: Option<&str> = None;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        match key {
            "checkpoint_interval_secs" => interval_raw = Some(value),
            "notifier" => notifier_raw = Some(value),
            _ => {} // unrecognized keys are ignored, forward-compatibly
        }
    }

    let interval_raw = interval_raw
        .ok_or_else(|| ConfigError::MissingKey("checkpoint_interval_secs".to_string()))?;
    let notifier_raw =
        notifier_raw.ok_or_else(|| ConfigError::MissingKey("notifier".to_string()))?;

    Ok(RelayConfig {
        checkpoint_interval_secs: parse_interval(interval_raw)?,
        notifier_kind: parse_notifier_kind(notifier_raw)?,
    })
}

/// Everything that can go wrong writing a handoff summary to disk. Wraps the
/// real `std::io::Error` rather than stringifying it early, so a caller (or
/// this type's own `source()`) still has the underlying error to inspect -
/// see the module's README for why the shipped `Io(String)` stub's
/// early-stringify shape gives that up for no behavioral difference today.
#[derive(Debug)]
pub enum HandoffError {
    Io(std::io::Error),
}

impl std::fmt::Display for HandoffError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandoffError::Io(inner) => write!(f, "failed to write handoff summary: {inner}"),
        }
    }
}

impl std::error::Error for HandoffError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            HandoffError::Io(inner) => Some(inner),
        }
    }
}

impl From<std::io::Error> for HandoffError {
    fn from(inner: std::io::Error) -> Self {
        HandoffError::Io(inner)
    }
}

fn render_handoff_summary(session: &Session) -> String {
    let mut out = String::new();
    for checkpoint in &session.checkpoints {
        let risks = if checkpoint.risks.is_empty() {
            "none".to_string()
        } else {
            checkpoint.risks.join(", ")
        };
        out.push_str(&format!(
            "- {}: {} (risks: {risks})\n",
            checkpoint.goal, checkpoint.summary
        ));
    }
    out
}

/// Render a session's checkpoints into a restartable-handoff summary and
/// write it to `path`. One line per checkpoint: goal, summary, and risks
/// (comma-joined, or `none` if empty), in checkpoint order. See `SPEC.md`'s
/// Module 05 section for the exact format and required edge cases -
/// including a real I/O failure (e.g. `path`'s parent directory doesn't
/// exist), which must come back as an `Err`, not a panic.
pub fn write_handoff_summary(path: &std::path::Path, session: &Session) -> Result<(), HandoffError> {
    let content = render_handoff_summary(session);
    std::fs::write(path, content)?;
    Ok(())
}
