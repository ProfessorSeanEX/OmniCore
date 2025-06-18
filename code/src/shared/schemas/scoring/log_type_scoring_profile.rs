/// ==============================
/// 📊 LogType Scoring Profile — Watchtower Logging System
/// ==============================
///
/// This struct defines the scoring profile for each log type within the **Watchtower** system.
/// It provides a mechanism for associating each log type with a severity level, alignment score, 
/// and ensures the correct handling of logs based on their classification.
///
/// The `LogTypeScoringProfile` is used to determine how logs are categorized, scored, 
/// and associated with appropriate severity levels. It works in conjunction with the 
/// `BaseLogEntry` struct and the severity scales (Base 10, Base 5, etc.).

use serde::{Serialize, Deserialize};

/// ==============================
/// Log Type Scoring Profile
/// ==============================
///
/// This struct ties a **LogType** to its severity, alignment, and scoring profile.
/// It dictates how the system interprets the log’s severity, type, and alignment score
/// to track the status of the system more effectively.

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogTypeScoringProfile {
    pub log_type: String,        // The type of the log (e.g., "INFO", "ERROR", "Critical")
    pub severity_level: String,  // Severity level as defined by the severity scales (Base 5, Base 10, etc.)
    pub alignment_score: u8,     // Alignment score in a scale from 0 to 100
    pub base_score: u8,          // Base score based on the severity level and alignment score
}

impl LogTypeScoringProfile {
    /// Creates a new `LogTypeScoringProfile` with the provided parameters.
    ///
    /// # Arguments
    /// * `log_type` - The type of log (e.g., "INFO", "ERROR", etc.).
    /// * `severity_level` - Severity level based on the predefined scales.
    /// * `alignment_score` - The alignment score ranging from 0 to 100.
    ///
    /// # Returns
    /// * Returns a new `LogTypeScoringProfile` struct.
    pub fn new(log_type: String, severity_level: String, alignment_score: u8) -> Self {
        let base_score = Self::calculate_base_score(&severity_level, alignment_score); // Calculates base score

        LogTypeScoringProfile {
            log_type,
            severity_level,
            alignment_score,
            base_score,
        }
    }

    /// Calculates the base score for a log entry based on its severity level and alignment score.
    ///
    /// # Arguments
    /// * `severity_level` - The severity level of the log.
    /// * `alignment_score` - The current alignment score of the log.
    ///
    /// # Returns
    /// * Returns the calculated base score.
    fn calculate_base_score(severity_level: &str, alignment_score: u8) -> u8 {
        // Logic to map severity level to a specific base score (this could use a predefined mapping)
        let severity_multiplier = match severity_level {
            "Perfect" => 100,
            "Excellent" => 90,
            "Good" => 80,
            "Fair" => 70,
            "Caution" => 60,
            "Risky" => 50,
            "Degraded" => 40,
            "Failing" => 30,
            "Critical" => 20,
            "Fatal" => 10,
            _ => 0,
        };

        // Combine severity multiplier with alignment score to generate base score
        severity_multiplier * alignment_score / 100
    }
}
