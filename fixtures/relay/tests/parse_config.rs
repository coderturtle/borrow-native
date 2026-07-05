use relay::{parse_config, ConfigError, NotifierKind};

#[test]
fn valid_config_with_desktop_notifier_parses_exactly() {
    let config = parse_config("checkpoint_interval_secs=300\nnotifier=desktop\n").unwrap();
    assert_eq!(config.checkpoint_interval_secs, 300);
    assert!(matches!(config.notifier_kind, NotifierKind::Desktop));
}

#[test]
fn valid_config_with_bell_notifier_parses_exactly() {
    let config = parse_config("notifier=bell\ncheckpoint_interval_secs=45\n").unwrap();
    assert_eq!(config.checkpoint_interval_secs, 45);
    assert!(matches!(config.notifier_kind, NotifierKind::TerminalBell));
}

#[test]
fn missing_checkpoint_interval_secs_is_named_precisely() {
    let err = parse_config("notifier=stdout\n").unwrap_err();
    match err {
        ConfigError::MissingKey(key) => assert_eq!(key, "checkpoint_interval_secs"),
        other => panic!("expected MissingKey(\"checkpoint_interval_secs\"), got {other:?}"),
    }
}

#[test]
fn missing_notifier_is_named_precisely() {
    let err = parse_config("checkpoint_interval_secs=60\n").unwrap_err();
    match err {
        ConfigError::MissingKey(key) => assert_eq!(key, "notifier"),
        other => panic!("expected MissingKey(\"notifier\"), got {other:?}"),
    }
}

#[test]
fn non_numeric_interval_is_reported_with_the_exact_key_and_value() {
    let err =
        parse_config("checkpoint_interval_secs=soon\nnotifier=desktop\n").unwrap_err();
    match err {
        ConfigError::InvalidInterval { key, value } => {
            assert_eq!(key, "checkpoint_interval_secs");
            assert_eq!(value, "soon");
        }
        other => panic!("expected InvalidInterval, got {other:?}"),
    }
}

#[test]
fn unrecognized_notifier_value_is_reported_exactly() {
    let err =
        parse_config("checkpoint_interval_secs=60\nnotifier=carrier-pigeon\n").unwrap_err();
    match err {
        ConfigError::UnknownNotifier(value) => assert_eq!(value, "carrier-pigeon"),
        other => panic!("expected UnknownNotifier(\"carrier-pigeon\"), got {other:?}"),
    }
}

#[test]
fn unrecognized_extra_key_is_ignored_not_an_error() {
    let config = parse_config(
        "checkpoint_interval_secs=120\nnotifier=stdout\nfuture_feature=on\n",
    )
    .unwrap();
    assert_eq!(config.checkpoint_interval_secs, 120);
    assert!(matches!(config.notifier_kind, NotifierKind::Stdout));
}

#[test]
fn blank_lines_are_tolerated() {
    let config = parse_config("\ncheckpoint_interval_secs=90\n\nnotifier=bell\n\n").unwrap();
    assert_eq!(config.checkpoint_interval_secs, 90);
    assert!(matches!(config.notifier_kind, NotifierKind::TerminalBell));
}
