/// Simulates writing a log entry to a human-readable scroll.
use crate::watchtower::logs::log_writer::{write_scroll, write_json};

use crate::shared::schemas::base_log_entry::BaseLogEntry;
use crate::shared::schemas::severity::SeverityBase10;
use crate::shared::schemas::specialized_log_entry::{CriticalLogEntry, DebugLogEntry, SpiritualLogEntry};

/// Trait that allows any log entry to be routed by the Watchtower system.
pub trait RoutableLog {
    fn route(&self);
}

/// Routing logic for base log entries.
impl RoutableLog for BaseLogEntry {
    fn route(&self) {
        let scroll = format!(
            "[{}][{}] from {} — {} (Severity: {:?})",
            self.timestamp, self.level, self.source, self.message, self.severity
        );
        write_scroll(&scroll);
        write_json(&self); // where self: &BaseLogEntry or similar
    }
}

/// Routing logic for critical logs.
impl RoutableLog for CriticalLogEntry {
    fn route(&self) {
        let scroll = format!(
            "[{}][🔥 CRITICAL] from {} — {} (Code: {:?}, Severity: {:?})",
            self.base.timestamp,
            self.base.source,
            self.base.message,
            self.system_error_code,
            SeverityBase10::Fatal
        );
        write_scroll(&scroll);
        write_json(&self); // where self: &BaseLogEntry or similar
    }
}

/// Routing logic for debug logs.
impl RoutableLog for DebugLogEntry {
    fn route(&self) {
        let scroll = format!(
            "[{}][🪵 DEBUG] from {} — {} | Context: {:?} (Severity: {:?})",
            self.base.timestamp,
            self.base.source,
            self.base.message,
            self.context,
            self.base.severity
        );
        write_scroll(&scroll);
        write_json(&self); // where self: &BaseLogEntry or similar
    }
}

/// Routing logic for spiritual logs.
impl RoutableLog for SpiritualLogEntry {
    fn route(&self) {
        let verse = self
            .scripture_reference
            .clone()
            .unwrap_or_else(|| "🕊️ No scripture".to_string());

        let scroll = format!(
            "[{}][🕊️ SPIRITUAL] from {} — {}\n  → Scripture: {}\n  → Prophetic Weight: {}\n  → Severity: {:?}",
            self.base.timestamp,
            self.base.source,
            self.base.message,
            verse,
            self.prophetic_weight,
            self.base.severity
        );
        write_scroll(&scroll);
        write_json(&self); // where self: &BaseLogEntry or similar
    }
}
