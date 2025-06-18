/// ==============================
/// 📏 Alignment Severity Evaluator
/// ==============================
///
/// This module provides logic for interpreting alignment scores (0–100)
/// into defined severity levels using scales declared in `schema::severity`.
/// This is the "calculator" for Watchtower — translating numerical alignment
/// into qualitative meaning for debugging and spiritual insight.
///
/// Alignment scores degrade from 100 to 0.
/// Severity increases as the score drops, based on which base scale is used.
///
/// ==============================

use crate::shared::schemas::severity::{SeverityBase10, SeverityBase5, SeverityBase20, SeverityBase25, SeverityBase50};

/// Evaluates alignment score into Base10 severity (standard scale).
pub fn evaluate_base10(score: u8) -> SeverityBase10 {
    match score {
        91..=100 => SeverityBase10::Perfect,
        81..=90 => SeverityBase10::Excellent,
        71..=80 => SeverityBase10::Good,
        61..=70 => SeverityBase10::Fair,
        51..=60 => SeverityBase10::Caution,
        41..=50 => SeverityBase10::Risky,
        31..=40 => SeverityBase10::Degraded,
        21..=30 => SeverityBase10::Failing,
        11..=20 => SeverityBase10::Critical,
        _ => SeverityBase10::Fatal,
    }
}

/// Evaluates alignment score into Base5 severity (precision scale).
pub fn evaluate_base5(score: u8) -> SeverityBase5 {
    match score {
        96..=100 => SeverityBase5::Perfect,
        91..=95 => SeverityBase5::NearPerfect,
        86..=90 => SeverityBase5::Excellent,
        81..=85 => SeverityBase5::Strong,
        76..=80 => SeverityBase5::Stable,
        71..=75 => SeverityBase5::Balanced,
        66..=70 => SeverityBase5::Watchful,
        61..=65 => SeverityBase5::Drifting,
        56..=60 => SeverityBase5::Wavering,
        51..=55 => SeverityBase5::Exposed,
        46..=50 => SeverityBase5::Warning,
        41..=45 => SeverityBase5::Tense,
        36..=40 => SeverityBase5::Unstable,
        31..=35 => SeverityBase5::Fragile,
        26..=30 => SeverityBase5::Slipping,
        21..=25 => SeverityBase5::Dangerous,
        16..=20 => SeverityBase5::Severe,
        11..=15 => SeverityBase5::Collapsing,
        6..=10 => SeverityBase5::Critical,
        _ => SeverityBase5::Fatal,
    }
}

/// Evaluates alignment score into Base20 severity (milestone scale).
pub fn evaluate_base20(score: u8) -> SeverityBase20 {
    match score {
        81..=100 => SeverityBase20::Perfect,
        61..=80 => SeverityBase20::Stable,
        41..=60 => SeverityBase20::Unstable,
        21..=40 => SeverityBase20::Critical,
        _ => SeverityBase20::Fatal,
    }
}

/// Evaluates alignment score into Base25 severity (anchor scale).
pub fn evaluate_base25(score: u8) -> SeverityBase25 {
    match score {
        76..=100 => SeverityBase25::AnchorPerfect,
        51..=75 => SeverityBase25::AnchorStable,
        26..=50 => SeverityBase25::AnchorFailing,
        _ => SeverityBase25::AnchorFatal,
    }
}

/// Evaluates alignment score into Base50 severity (binary pass/fail).
pub fn evaluate_base50(score: u8) -> SeverityBase50 {
    match score {
        51..=100 => SeverityBase50::Pass,
        _ => SeverityBase50::Fail,
    }
}
