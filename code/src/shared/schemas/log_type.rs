/// ==============================
/// 📦 Enum: LogType
/// ==============================
///
/// Represents the *type* of log being recorded.
/// These types define the **context** and **intent** of each log entry,
/// and are used to direct how logs are interpreted, stored, and displayed.
///
/// 🔸 Note: While all variants live in this enum, they are grouped below
/// under logical categories for clarity (Info, Warning, Error, Debug, Critical).
///
/// 📌 Future Features:
/// - Categorization could evolve into a separate `LogCategory` enum.
/// - Additional log types (e.g., Covenant, Prophetic, Insight) may be added as system matures.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogType {
    // ==============================
    // 📘 Info — Expected Flow
    // ==============================
    Heartbeat,     // Regular ping or keep-alive
    Milestone,     // System or project goal reached
    System,        // Core system identity/config logs
    Test,          // Build/test logs
    Health,        // System health check-ins
    Progress,      // Active step-by-step progress reports
    Meta,          // Logging internal logging behavior

    // ==============================
    // ⚠️ Warning — Drift or Instability
    // ==============================
    Alignment,     // Detected deviation from standard
    Watcher,       // Manual or automated flagged observation
    Update,        // Change applied that may cause drift
    Trace,         // Chain-following warning or weak signal

    // ==============================
    // ❌ Error — Break or Halt Needed
    // ==============================
    Runtime,       // Code failed at execution time
    SystemFailure, // OS or external failure
    Dependency,    // Missing or unsatisfied requirement
    Config,        // Malformed configuration or missing setup

    // ==============================
    // 🔍 Debug — Developer-Level Detail
    // ==============================
    Debug,         // Debug-only verbosity
    Internal,      // Watchtower system introspection or reflex

    // ==============================
    // 🔥 Critical — Immediate Escalation
    // ==============================
    Fatal,         // Irrecoverable state, crash imminent
    Prophetic,     // Spirit-led alert, divine signal
    Security,      // Breach, intrusion, or policy violation
    Override,      // Manual override, system bypass

    // ==============================
    // 🕊️ Spiritual/Relational Expansion
    // ==============================
    Covenant,      // Logs related to alignment, identity, or trust
    Anomaly,       // Undefined, unclassified behavior
    Watch,         // Watchtower-specific real-time monitor flag
    Insight,       // Wisdom, reflection, revelation logs
    Correction,    // Grace-driven intervention or realignment
}

