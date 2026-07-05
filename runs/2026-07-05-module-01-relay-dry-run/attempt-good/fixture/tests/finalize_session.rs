use relay::{finalize_session, DraftCheckpoint};

#[test]
fn empty_drafts_produce_empty_session() {
    let session = finalize_session("empty-session".to_string(), vec![]);
    assert_eq!(session.label, "empty-session");
    assert!(session.checkpoints.is_empty());
}

#[test]
fn single_draft_produces_one_record() {
    let drafts = vec![DraftCheckpoint {
        goal: "reproduce the bug".to_string(),
        summary: "confirmed failing test".to_string(),
        risks: vec![],
    }];
    let session = finalize_session("solo".to_string(), drafts);
    assert_eq!(session.checkpoints.len(), 1);
    assert_eq!(session.checkpoints[0].goal, "reproduce the bug");
    assert_eq!(session.checkpoints[0].summary, "confirmed failing test");
    assert!(session.checkpoints[0].risks.is_empty());
}

#[test]
fn multiple_drafts_preserve_order() {
    let drafts = vec![
        DraftCheckpoint {
            goal: "first".to_string(),
            summary: "first summary".to_string(),
            risks: vec![],
        },
        DraftCheckpoint {
            goal: "second".to_string(),
            summary: "second summary".to_string(),
            risks: vec![],
        },
        DraftCheckpoint {
            goal: "third".to_string(),
            summary: "third summary".to_string(),
            risks: vec![],
        },
    ];
    let session = finalize_session("ordered".to_string(), drafts);
    assert_eq!(session.checkpoints.len(), 3);
    assert_eq!(session.checkpoints[0].goal, "first");
    assert_eq!(session.checkpoints[1].goal, "second");
    assert_eq!(session.checkpoints[2].goal, "third");
}

#[test]
fn empty_risks_are_preserved_not_dropped() {
    let drafts = vec![DraftCheckpoint {
        goal: "goal".to_string(),
        summary: "summary".to_string(),
        risks: vec![],
    }];
    let session = finalize_session("risks-empty".to_string(), drafts);
    assert_eq!(session.checkpoints[0].risks, Vec::<String>::new());
}

#[test]
fn multiple_risks_preserved_in_order() {
    let drafts = vec![DraftCheckpoint {
        goal: "goal".to_string(),
        summary: "summary".to_string(),
        risks: vec![
            "flaky test".to_string(),
            "needs a second reviewer".to_string(),
        ],
    }];
    let session = finalize_session("risks-many".to_string(), drafts);
    assert_eq!(
        session.checkpoints[0].risks,
        vec!["flaky test".to_string(), "needs a second reviewer".to_string()]
    );
}
