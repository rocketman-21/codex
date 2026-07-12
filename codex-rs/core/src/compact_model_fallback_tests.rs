use super::*;

#[test]
fn specific_fallback_error_replaces_previous_model_rejection() {
    let selected = select_model_fallback_error(
        CodexErr::InvalidRequest("previous model rejected compaction".to_string()),
        CodexErr::QuotaExceeded,
    );

    assert!(matches!(selected, CodexErr::QuotaExceeded));
}

#[test]
fn cancellation_replaces_previous_model_rejection() {
    let selected = select_model_fallback_error(
        CodexErr::InvalidRequest("previous model rejected compaction".to_string()),
        CodexErr::TurnAborted,
    );

    assert!(matches!(selected, CodexErr::TurnAborted));
}

#[test]
fn generic_fallback_error_preserves_previous_model_rejection() {
    let selected = select_model_fallback_error(
        CodexErr::InvalidRequest("previous model rejected compaction".to_string()),
        CodexErr::InvalidRequest("fallback model rejected compaction".to_string()),
    );

    match selected {
        CodexErr::InvalidRequest(message) => {
            assert_eq!(message, "previous model rejected compaction");
        }
        other => panic!("expected the previous-model rejection, got {other:?}"),
    }
}
