use relay::{write_handoff_summary, CheckpointRecord, HandoffError, Session};
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};

/// A fresh path under the OS temp dir, unique per call - avoids a `tempfile`
/// dependency (this workshop's governance requires human approval for any
/// new dependency; a counter-suffixed path in `std::env::temp_dir()` is
/// enough for a test that cleans up after itself).
fn scratch_path(name: &str) -> PathBuf {
    static COUNTER: AtomicU32 = AtomicU32::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("relay-module-05-dry-run-{name}-{n}.txt"))
}

fn two_checkpoint_session() -> Session {
    Session {
        label: "module-05-dry-run".to_string(),
        checkpoints: vec![
            CheckpointRecord {
                goal: "wire up config parsing".to_string(),
                summary: "parse_config reads key=value pairs".to_string(),
                risks: vec!["unknown keys silently ignored".to_string()],
                elapsed_secs: 300,
            },
            CheckpointRecord {
                goal: "write the handoff summary".to_string(),
                summary: "write_handoff_summary renders + writes to disk".to_string(),
                risks: vec![],
                elapsed_secs: 180,
            },
        ],
    }
}

#[test]
fn writes_exact_expected_content_including_empty_risks_as_none() {
    let path = scratch_path("exact-content");
    let session = two_checkpoint_session();

    write_handoff_summary(&path, &session).expect("write should succeed");

    let content = fs::read_to_string(&path).expect("file should exist and be readable");
    let expected = "\
- wire up config parsing: parse_config reads key=value pairs (risks: unknown keys silently ignored)
- write the handoff summary: write_handoff_summary renders + writes to disk (risks: none)
";
    assert_eq!(content, expected);

    let _ = fs::remove_file(&path);
}

#[test]
fn io_failure_comes_back_as_err_not_a_panic() {
    // Parent directory deliberately does not exist.
    let path = std::env::temp_dir()
        .join("relay-module-05-dry-run-nonexistent-dir")
        .join("handoff.txt");
    let session = two_checkpoint_session();

    let result = write_handoff_summary(&path, &session);
    match result {
        Err(HandoffError::Io(_)) => {}
        Ok(()) => panic!("expected Err on a missing parent directory, got Ok(())"),
    }
}
