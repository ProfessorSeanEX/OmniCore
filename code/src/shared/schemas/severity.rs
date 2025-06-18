use serde::{Serialize, Deserialize};

/// ==============================
/// 📊 Severity Scale Definitions
/// ==============================
///
/// These enums define all available severity scoring systems for log entries.
/// Each scale is **degrading**, starting from 100 and decreasing based on alignment loss.
/// These are used to calculate severity based on an `alignment_score`, which ranges from 0 to 100.
///
/// 📌 NOTE:
/// - These are not mutually exclusive. A single log entry may utilize multiple scales
///   depending on its type and diagnostic context.
/// - Trigger thresholds may be customized later to allow fine-tuned behavior.

/// ------------------------------
/// Base 10 — Standard Scale (10 levels)
/// ------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SeverityBase10 {
    Perfect,      // 100–91
    Excellent,    // 90–81
    Good,         // 80–71
    Fair,         // 70–61
    Caution,      // 60–51
    Risky,        // 50–41
    Degraded,     // 40–31
    Failing,      // 30–21
    Critical,     // 20–11
    Fatal,        // 10–0
}

/// ------------------------------
/// Base 5 — Precision Scale (20 levels)
/// ------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SeverityBase5 {
    Perfect,      // 100–96
    NearPerfect,  // 95–91
    Excellent,    // 90–86
    Strong,       // 85–81
    Stable,       // 80–76
    Balanced,     // 75–71
    Watchful,     // 70–66
    Drifting,     // 65–61
    Wavering,     // 60–56
    Exposed,      // 55–51
    Warning,      // 50–46
    Tense,        // 45–41
    Unstable,     // 40–36
    Fragile,      // 35–31
    Slipping,     // 30–26
    Dangerous,    // 25–21
    Severe,       // 20–16
    Collapsing,   // 15–11
    Critical,     // 10–6
    Fatal,        // 5–0
}

/// ------------------------------
/// Base 20 — Milestone Scale (5 levels)
/// ------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SeverityBase20 {
    Perfect,   // 100–81
    Stable,    // 80–61
    Unstable,  // 60–41
    Critical,  // 40–21
    Fatal,     // 20–0
}

/// ------------------------------
/// Base 25 — Anchor Scale (4 levels)
/// ------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SeverityBase25 {
    AnchorPerfect,  // 100–76
    AnchorStable,   // 75–51
    AnchorFailing,  // 50–26
    AnchorFatal,    // 25–0
}

/// ------------------------------
/// Base 50 — Binary Pass/Fail Scale (3 levels)
/// ------------------------------
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SeverityBase50 {
    Pass,     // 100–51
    Warning,  // 50–1
    Fail,     // 0
}

impl SeverityBase10 {
    pub fn derive_from(score: u8) -> Self {
        match score {
            91..=100 => Self::Perfect,
            81..=90 => Self::Excellent,
            71..=80 => Self::Good,
            61..=70 => Self::Fair,
            51..=60 => Self::Caution,
            41..=50 => Self::Risky,
            31..=40 => Self::Degraded,
            21..=30 => Self::Failing,
            11..=20 => Self::Critical,
            _ => Self::Fatal,
        }
    }
}

impl SeverityBase5 {
    pub fn derive_from(score: u8) -> Self {
        match score {
            96..=100 => Self::Perfect,
            91..=95 => Self::NearPerfect,
            86..=90 => Self::Excellent,
            81..=85 => Self::Strong,
            76..=80 => Self::Stable,
            71..=75 => Self::Balanced,
            66..=70 => Self::Watchful,
            61..=65 => Self::Drifting,
            56..=60 => Self::Wavering,
            51..=55 => Self::Exposed,
            46..=50 => Self::Warning,
            41..=45 => Self::Tense,
            36..=40 => Self::Unstable,
            31..=35 => Self::Fragile,
            26..=30 => Self::Slipping,
            21..=25 => Self::Dangerous,
            16..=20 => Self::Severe,
            11..=15 => Self::Collapsing,
            6..=10 => Self::Critical,
            _ => Self::Fatal,
        }
    }
}

impl SeverityBase20 {
    pub fn derive_from(score: u8) -> Self {
        match score {
            81..=100 => Self::Perfect,
            61..=80 => Self::Stable,
            41..=60 => Self::Unstable,
            21..=40 => Self::Critical,
            _ => Self::Fatal,
        }
    }
}

impl SeverityBase25 {
    pub fn derive_from(score: u8) -> Self {
        match score {
            76..=100 => Self::AnchorPerfect,
            51..=75 => Self::AnchorStable,
            26..=50 => Self::AnchorFailing,
            _ => Self::AnchorFatal,
        }
    }
}

impl SeverityBase50 {
    pub fn derive_from(score: u8) -> Self {
        match score {
            51..=100 => Self::Pass,
            1..=50 => Self::Warning,
            _ => Self::Fail,
        }
    }
}
