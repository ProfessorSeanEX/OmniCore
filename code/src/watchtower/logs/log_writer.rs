use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use std::path::Path;
use chrono::Utc;
use serde::Serialize;

// 📁 Log directory constants — inside mounted /app/logs
const SCROLL_DIR: &str = "/app/logs/scrolls/";
const JSON_DIR: &str = "/app/logs/json/";

/// 🌀 Write a plain-text scroll entry to a .log file
pub fn write_scroll(text: &str) {
    let date = Utc::now().format("%Y-%m-%d").to_string();
    let path_str = format!("{}{}.log", SCROLL_DIR, date);
    let path = Path::new(&path_str); // 🔒 path_str lives long enough

    // 🛡 Ensure parent directory exists
    if let Some(parent) = path.parent() {
        if let Err(e) = create_dir_all(parent) {
            eprintln!("⚠️ Failed to create scroll log directory: {}", e);
            return;
        }
    }

    // ✍️ Append to log file
    match OpenOptions::new().create(true).append(true).open(path) {
        Ok(mut file) => {
            let entry = format!("{}\n", text);
            if let Err(e) = file.write_all(entry.as_bytes()) {
                eprintln!("⚠️ Failed to write to scroll log file: {}", e);
            }
        }
        Err(e) => {
            eprintln!("⚠️ Failed to open scroll log file: {}", e);
        }
    }
}

/// 📜 Write a serialized JSON entry to a .json file
pub fn write_json<T: Serialize>(log: &T) {
    let date = Utc::now().format("%Y-%m-%d").to_string();
    let path_str = format!("{}{}.json", JSON_DIR, date);
    let path = Path::new(&path_str); // 🔒 path_str lives long enough

    // 🛡 Ensure parent directory exists
    if let Some(parent) = path.parent() {
        if let Err(e) = create_dir_all(parent) {
            eprintln!("⚠️ Failed to create JSON log directory: {}", e);
            return;
        }
    }

    // ✍️ Append serialized JSON
    match OpenOptions::new().create(true).append(true).open(path) {
        Ok(mut file) => {
            match serde_json::to_string(log) {
                Ok(serialized) => {
                    if let Err(e) = writeln!(file, "{}", serialized) {
                        eprintln!("⚠️ Failed to write to JSON log file: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("⚠️ Failed to serialize log entry: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("⚠️ Failed to open JSON log file: {}", e);
        }
    }
}