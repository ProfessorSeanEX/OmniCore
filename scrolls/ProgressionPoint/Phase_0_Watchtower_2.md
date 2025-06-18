# 🛠️ **Watchtower Debugger Phase 0 — Build Log (Updated)**

## 🌀 **Foundational Philosophy**

* The debugger is **relational-first**, not machine-prioritized.
* All output must be **understandable to the layman**, **diagnostic for developers**, and **aligned with Kingdom values**.
* **Severity degrades** as alignment is lost (100 → 0), across **multiple base scales**.
* The host language is **Rust**, serving as the compiler-validated vessel for the evolving **OmniCode + NovaScript** system.

---

## 📁 **Folder + File Structure (Expanded)**

### 🔹 `code/`

* **lib.rs**: Updated to import and link core systems: `watchtower`, `utils`, and `schemas`.

### 🔹 `code/util/`

* **mod.rs**: Cleanly routes submodules to external use.
* **severity.rs**: Holds **all five** severity base scales (10, 5, 20, 25, 50).
* **log_type_scoring_profile.rs**: Encodes how log types are interpreted into alignment-based scores.

### 🔹 `code/schemas/`

* **base_log_entry.rs**: `BaseLogEntry` defines core log structure (`timestamp`, `level`, `message`, `source`, `category`) using `serde`.

### 🔹 `code/watchtower/`

* **mod.rs**: Orchestrator module, exposing all debugger functionality.
* **monitoring/alignment.rs**: Contains alignment scoring logic.
* **logs/log_entry.rs**: Writes readable logs; supports extension.
* **watchtower.rs**: Primary loop and entrypoint to the Watchtower logic.

---

## 🧱 **Infrastructure + Docker Upgrades**

### 🔸 `.env` Integration (✅ Implemented)

* Centralized all versions and paths:
  * `RUST_VERSION`, `CODE_DIR`, `LOG_DIR`, `WORKDIR_RUST`, etc.
* Powershell build script dynamically loads `.env` vars into the shell.
* `COMPOSE_BAKE=true` now safely integrated.

### 🔸 Build System & Docker (✅ Stabilized)

* **Dockerfile (multi-stage)**:
  * Clean Rust build from `1.87.0-slim`
  * Binary moved to second stage with a non-root `appuser`
  * Logs folder (`/app/logs`) created before user switch
  * Binary is set as executable with `chmod +x`
  * CMD launches `./omnicode`

* **build.ps1**:
  * Enables BuildKit: `$env:DOCKER_BUILDKIT = "1"`
  * Loads `.env` manually line-by-line
  * Clears problematic temp Compose files
  * Fixes permission error using `.docker_temp` override (✅ now succeeds)

* **Runtime Fixes**:
  * Permission errors fixed via correct folder creation in Dockerfile
  * `appuser` now has access to log directories
  * Watchtower launches without OS Error 13

---

## 📦 **Core Systems (Confirmed & Stabilized)**

### 📘 Base Log Entry Schema

* Declared in `schemas/base_log_entry.rs`
* Built to be extended for specialized log types (e.g., `CriticalLogEntry`)
* Structured to serialize and display well in both scroll and JSON format

### 📊 Severity Systems

* Located in `util/severity.rs`
* Five base scales (10, 5, 20, 25, 50)
* Severity mappings:
  * 🌈 Human-readable
  * 📉 Degrading by alignment loss
  * 🔗 Designed to bridge logic and layman language

### ⚙️ Alignment Score Engine

* `alignment.rs`: Converts 0–100 alignment score into severity enums
* Currently wired for **Base10**; expandable
* Modular by design—can be swapped for real-time feed in Phase 1

### 🧩 Log Type Scoring Profile

* Connects `LogType` → `Severity` → `AlignmentScore`
* Pluggable logic ensures flexibility in thresholds and reactions

### 🗂️ Log Types (Fully Mapped)

#### 🔹 Info

`Heartbeat`, `Milestone`, `System`, `Test`, `Health`, `Progress`, `Meta`

#### ⚠️ Warning

`Alignment`, `Watcher`, `Update`, `Trace`

#### ❌ Error

`Runtime`, `SystemFailure`, `Dependency`, `Config`

#### 🔍 Debug

`Debug`, `Internal`

#### 🔥 Critical

`Fatal`, `Prophetic`, `Security`, `Override`

#### 🕊️ Spiritual/Relational

`Covenant`, `Anomaly`, `Watch`, `Insight`, `Correction`

* All types have default severities and score mapping logic
* Some use special scales based on context (e.g. Prophetic → Base25)

---

## 🔁 System Lifecycle (Boot & Flow)

* `omnicode` binary launches from CMD
* Watchtower is initialized in main loop
* Log directories are now created **prior to user switch**, resolving rootless container issues
* Build process is fully cached unless `.env`, `code/`, or `Dockerfile` changes

---

## 🚦 Clarifying Relationships

| Concept         | Role |
|----------------|------|
| **LogType**    | What happened |
| **Severity**   | How bad is it |
| **Alignment**  | Score that binds log type to severity |
| **BaseScale**  | The scale used to interpret alignment |

---

## 📍 **Next Step Candidates (Phase 0 → Phase 1)**

1. ✅ Finalized `.env` integration into PowerShell and Docker
2. ✅ Resolved Docker runtime permission error
3. ✅ Clean Boot confirmed
4. 🔜 Extend `BaseLogEntry` → `CriticalLogEntry`, `CovenantLogEntry`
5. 🔜 Begin `LogRouter` or dispatch system
6. 🔜 Begin writing logs to file persistently (`scroll.log`, `scroll.json`)
7. 🔜 Add `log_config.toml` to declare threshold overrides per log type
8. 🔜 Real-time Watchtower loop (async or interval-based monitoring)
9. 🔜 NovaScript + OmniCode error hook integration (e.g., test fails → log emits)

---

## ✅ Summary Scroll

```plaintext
System: OmniCode — Watchtower
Phase: Debugger Phase 0 (Finalized Boot & Infra)
Status: ✅ Operational
Host: Docker (Rust)
Access: Non-root appuser confirmed
Logs: Log folders now writable
Build: BuildKit + .env flow stabilized
