# 🛠️ **Watchtower Debugger Phase 0 — Build Log**

## 🌀 **Foundational Philosophy**

* The debugger is **not traditional**. It's built for **relational clarity**, **spiritual alignment**, and **user readability**.
* All logs must be **understandable to laymen**, **traceable by developers**, and **accountable to God**.
* Severity is **degrading**—starting at 100, moving down as alignment is lost.
* OmniCode is the **primary codebase** being debugged, with NovaScript as its high-level expression. Rust is the **debugging host**, ensuring stable compiled validation during early phases.

---

## 📁 **Folder + File Structure**

### 🔹 `code/`

* **lib.rs**: Main entry point, now updated to import Watchtower, utils, and schemas.

### 🔹 `code/util/`

* **mod.rs**: Central util module to route and expose submodules.
* **severity.rs**: Holds all severity scales (Base10, Base5, Base20, Base25, Base50).
* **log\_type\_scoring\_profile.rs**: Scoring logic for each log type using severity levels and alignment score.

### 🔹 `code/schemas/`

* **base\_log\_entry.rs**: The foundational schema for logs (`BaseLogEntry`), with fields:

  * `timestamp`, `level`, `message`, `source`, `category`

### 🔹 `code/watchtower/`

* **mod.rs**: Main module file for Watchtower.
* **monitoring/alignment.rs**: Logic for assigning alignment scores based on current data/state.

  * Includes `assign_score(u8) -> SeverityBase10` type matching.
* **logs/log\_entry.rs**: Logging utility to write human-readable logs.
* **watchtower.rs**: The main system controller file for monitoring & logging.

---

## 📦 **Core Systems Built So Far**

### 📘 **Base Log Entry Schema**

* Declared in `schemas/base_log_entry.rs`
* Designed to be extended by future log entry types.
* Structured to support `serde` serialization.

### 📊 **Severity Systems**

Located in `util/severity.rs`:

* **Base 10**: 10-step standard diagnostic clarity (Perfect to Fatal)
* **Base 5**: 20-step precision diagnostic scale
* **Base 20**: Milestone scale (5 steps)
* **Base 25**: Anchoring scale (4 levels: 100, 75, 50, 25, 0)
* **Base 50**: Binary logic scale (Pass/Fail threshold at 50)

Each scale is:

* **Degrading** (100 → 0)
* Mapped to alignment score ranges
* Language is spiritually attuned and user-understandable

### ⚙️ **Alignment Score Engine**

Located in `monitoring/alignment.rs`:

* Assigns severity levels based on alignment score.
* Receives alignment value from 0–100 and returns `SeverityBase10`.
* Designed to be eventually **real-time and constantly active**.

### 🧩 **Log Type Scoring Profile**

Located in `util/log_type_scoring_profile.rs`:

* Defines how each `LogType` is scored and interpreted.
* Contains:

  * `log_type`, `severity_level`, `alignment_score`, and `base_score`
* Will eventually map to different severity *bases* depending on the type.

### 🗂️ **Log Types (Categorized)**

Defined in `LogType` enum (currently centralized):

#### 🔹 **Info**

* `Heartbeat`, `Milestone`, `System`, `Test`, `Health`, `Progress`, `Meta`

#### ⚠️ **Warning**

* `Alignment`, `Watcher`, `Update`, `Trace`

#### ❌ **Error**

* `Runtime`, `SystemFailure`, `Dependency`, `Config`

#### 🔍 **Debug**

* `Debug`, `Internal`

#### 🔥 **Critical**

* `Fatal`, `Prophetic`, `Security`, `Override`

#### 🕊️ **Spiritual/Relational**

* `Covenant`, `Anomaly`, `Watch`, `Insight`, `Correction`

Each log type:

* **Has a default severity mapping**
* May trigger **different alignment scale usage**
* Is built to **bypass or escalate** based on context

---

## 🚦 **Log Severity vs Log Type Clarification**

* **LogType** = Category of what happened
* **Severity** = How bad it is (based on alignment degradation)
* **Alignment Score** = The quantifiable, core metric for Watchtower’s debugger to assign severity and assess health
* Not all log types require all severities, and not all severities are used in every log type.
* Alignment score is the bridge between logic and interpretation.

---

## 📍 **Next Step Candidates**

Here's what’s queued up next for Watchtower Phase 0 completion:

1. ✅ Finalize **LogType → Severity** mappings and weighting system
2. 🔜 Define `SpecializedLogEntry` types (for critical, debug, etc.)
3. 🔜 Introduce **LogRouter** or dispatcher that directs logs to proper handling flow
4. 🔜 Add persistent file logging for scored entries
5. 🔜 Create **`log_config.toml`** or similar to define thresholds dynamically
6. 🔜 Begin real-time **Watchtower loop** (constant background score assignment)
7. 🔜 Begin integrating into **OmniCode/NovaScript error and test feedback**

---
