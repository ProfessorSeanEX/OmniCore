/// ==============================
/// 🚀 Entry Point — OmniCode Runtime (main.rs)
/// ==============================
///
/// This is the binary entry for the OmniCode system.
/// It currently serves as the launcher for Watchtower's monitor logic,
/// acting as a test of wiring, module linkage, and CLI routing.
///
/// As more subsystems come online (Gate, Tablet, etc),
/// this file will orchestrate runtime delegation.
///
/// ==============================

mod watchtower;
mod shared;

use watchtower::watchtower::monitor_and_log;

fn main() {
    println!("🌀 OmniCode System Booting...");
    monitor_and_log();
    println!("✅ Watchtower initialized and monitoring.");
}
