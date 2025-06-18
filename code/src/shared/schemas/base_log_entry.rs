/// ==============================
/// 📜 BaseLogEntry Schema — Watchtower Logging Core
/// ==============================
///
/// This struct defines the **core structure** for all log entries in the Watchtower system.
/// It provides the minimal, universal format that other specialized logs will extend or adapt.
///
/// Every log—unless explicitly specialized—should conform to this base schema.
///
/// ==============================
/// Fields:
/// - `timestamp`: UTC timestamp (ISO 8601 format or epoch).
/// - `level`: Importance or severity of the log (INFO, WARN, ERROR, etc.).
/// - `message`: The human-readable message or summary of what occurred.
/// - `source`: The originator of the event (e.g. "CLI", "Watchtower", "TestRunner").
/// - `category`: Optional. Helps classify the log type (e.g. "system", "build", "auth", etc.).
/// - `severity`: A calculated severity based on the alignment score of the event.
/// ==============================

use serde::{Serialize, Deserialize};
use crate::shared::schemas::severity::{SeverityBase10};  // Adjust according to the severity.rs logic

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseLogEntry {
    pub timestamp: String,          // UTC timestamp in ISO 8601 format
    pub level: String,              // Log level: "INFO", "WARN", "ERROR", etc.
    pub message: String,            // Human-readable log message
    pub source: String,             // Subsystem that generated the log
    pub category: Option<String>,   // Optional: system-specific grouping for filtering
    pub severity: Option<SeverityBase10>, // Severity of the event based on its alignment score
}

/// ==============================
/// Logic: Assign Severity to Log
/// ==============================
/// This function assigns the severity level based on the log's alignment score.
impl BaseLogEntry {
    pub fn new(timestamp: String, level: String, message: String, source: String, category: Option<String>, alignment_score: u8) -> Self {
        let severity = SeverityBase10::derive_from(alignment_score);

        BaseLogEntry {
            timestamp,
            level,
            message,
            source,
            category,
            severity: Some(severity),
        }
    }
}
