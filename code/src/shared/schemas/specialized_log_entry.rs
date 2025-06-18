use crate::shared::schemas::base_log_entry::BaseLogEntry;
use crate::shared::schemas::severity::SeverityBase10;
use crate::shared::schemas::log_type::LogType;
use chrono::{Utc};
use std::fmt;
use serde::{Serialize, Deserialize};


/// Specialized log entry for fatal system errors or unrecoverable faults.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriticalLogEntry {
    pub base: BaseLogEntry,
    pub system_error_code: Option<String>,
}

impl CriticalLogEntry {
    pub fn new(message: &str, source: &str, category: Option<String>, error_code: Option<String>, score: u8) -> Self {
        let base = BaseLogEntry::new(
            Utc::now().to_rfc3339(),
            "ERROR".into(),
            message.into(),
            source.into(),
            category,
            score
        );

        CriticalLogEntry {
            base,
            system_error_code: error_code,
        }
    }

    pub fn escalate(&self) -> SeverityBase10 {
        SeverityBase10::Fatal
    }
}


/// Specialized log entry for deep debugging and internal traces.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugLogEntry {
    pub base: BaseLogEntry,
    pub context: Option<String>,
}

impl DebugLogEntry {
    pub fn new(message: &str, source: &str, category: Option<String>, context: Option<String>, score: u8) -> Self {
        let base = BaseLogEntry::new(
            Utc::now().to_rfc3339(),
            "DEBUG".into(),
            message.into(),
            source.into(),
            category,
            score
        );

        DebugLogEntry {
            base,
            context,
        }
    }

    pub fn verbosity(&self) -> &'static str {
        "low"
    }
}


/// Specialized log entry for spiritually significant logs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiritualLogEntry {
    pub base: BaseLogEntry,
    pub scripture_reference: Option<String>,
    pub prophetic_weight: u8, // 1–10
}

impl SpiritualLogEntry {
    pub fn new(message: &str, source: &str, category: Option<String>, verse: Option<String>, weight: u8, score: u8) -> Self {
        let base = BaseLogEntry::new(
            Utc::now().to_rfc3339(),
            "INFO".into(),
            message.into(),
            source.into(),
            category,
            score
        );

        SpiritualLogEntry {
            base,
            scripture_reference: verse,
            prophetic_weight: weight,
        }
    }

    pub fn quote(&self) -> String {
        self.scripture_reference.clone().unwrap_or_else(|| "🕊️ No scripture provided.".to_string())
    }

    pub fn is_highly_weighted(&self) -> bool {
        self.prophetic_weight >= 8
    }
}
