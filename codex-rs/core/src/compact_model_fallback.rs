use codex_analytics::CompactionImplementation;
use codex_analytics::CompactionReason;
use codex_otel::SessionTelemetry;
use codex_protocol::error::CodexErr;
use codex_protocol::protocol::CodexErrorInfo;
use tracing::warn;

pub(crate) fn record_model_fallback(
    session_telemetry: &SessionTelemetry,
    previous_model: &str,
    current_model: &str,
    reason: CompactionReason,
    implementation: CompactionImplementation,
    fallback_error: Option<&CodexErr>,
) {
    let reason_tag = match reason {
        CompactionReason::UserRequested => "user_requested",
        CompactionReason::ContextLimit => "context_limit",
        CompactionReason::ModelDownshift => "model_downshift",
        CompactionReason::CompHashChanged => "comp_hash_changed",
    };
    let implementation_tag = match implementation {
        CompactionImplementation::Responses => "responses",
        CompactionImplementation::ResponsesCompactionV2 => "responses_compaction_v2",
        CompactionImplementation::ResponsesCompact => "responses_compact",
    };
    let outcome = if fallback_error.is_none() {
        "succeeded"
    } else {
        "failed"
    };
    session_telemetry.counter(
        "codex.compaction.model_fallback",
        /*inc*/ 1,
        &[
            ("reason", reason_tag),
            ("implementation", implementation_tag),
            ("outcome", outcome),
        ],
    );
    warn!(
        previous_model,
        current_model,
        ?reason,
        ?implementation,
        outcome,
        ?fallback_error,
        "previous-model compaction failed; retried with current model"
    );
}

/// Preserve the previous-model rejection unless the fallback exposes a
/// specific client-visible error or cancellation.
pub(crate) fn select_model_fallback_error(
    original_error: CodexErr,
    fallback_error: CodexErr,
) -> CodexErr {
    if matches!(&fallback_error, CodexErr::TurnAborted)
        || fallback_error.to_codex_protocol_error() != CodexErrorInfo::Other
    {
        fallback_error
    } else {
        original_error
    }
}

#[cfg(test)]
#[path = "compact_model_fallback_tests.rs"]
mod tests;
