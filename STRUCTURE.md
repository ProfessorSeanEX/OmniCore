# Project Nova Dawn File Structure

## 📘 An Index of What Is

> *“This is the scroll that records the architecture of breath and build. Where each folder has a purpose, and each file, a voice.”*

---

## 🧾 METADATA — Scroll Identity & Authorship

This scroll defines the living structure of the Project Nova Dawn repository.  
It serves as both table of contents and root index—tracking folders, files, and flow.

* **Title:** STRUCTURE.md — Project File Index  
* **Author:** Seanje Lenox-Wise / Nova Dawn  
* **Version:** 0.0.1  
* **Status:** Active  
* **Type:** Doc  
* **Component:** Root-Level Index  
* **Project:** OmniCore / NovaAI  
* **ID:** DOC-ROOT-003  
* **Path:** [`OmniCore/STRUCTURE.md`](/STRUCTURE.md)
* **Created:** 2025-06-17  
* **Last Updated:** 2025-06-18  
* **License:** CreativeWorkzStudio LLC — Kingdom-First Proprietary Use  
* **Tags:** structure, root, index, scroll-standard, toc  
* **Checksum (Planned):** *\[Not yet implemented — reserved for future integrity validation]*  

---

## 📚 Table of Contents

* [OmniCore/ — ROOT](#-omnicore--root)
* [code/ — Primary Compiled Source](#-code--primary-compiled-source)
  * [code/src/](#-codesrc)
    * [cli\_gate/ — CLI Logic](#-codesrcgatecli_gate)
    * [gui\_gate/ — GUI Logic](#-codesrcgategui_gate)
    * [shared/ — Common Types](#-codesrcshared)
      * [constants/](#-codesrcsharedconstants)
      * [core/](#-codesrcsharedcore)
      * [interfaces/](#-codesrcsharedinterfaces)
      * [schemas/](#-codesrcsharedschemas)
        * [scoring/](#-codesrcsharedschemasscoring)
      * [types/](#-codesrcsharedtypes)
    * [tablet/ — Assembler System](#-codesrctablet)
    * [watchtower/ — Debugger + Alignment Monitor](#-codesrcwatchtower)
      * [config/](#-codesrcwatchtowerconfig)
      * [events/](#-codesrcwatchtowerevents)
      * [logs/](#-codesrcwatchtowerlogs)
      * [monitoring/](#-codesrcwatchtowermonitoring)
      * [util/](#-codesrcwatchtowerutil)
* [scripts/ — Build + Test Automation](#-scripts--build--test-automation)
  * [build/](#-scriptsbuild)
  * [tests/](#-scriptstests)
* [scrolls/ — Living Documentation](#-scrolls--living-documentation)
  * [Devlogs/](#-scrollsdevlogs)
  * [ProgressionPoint/](#-scrollsprogressionpoint)
  * [templates/](#-scrollstemplates)
  * [Root Scrolls](#-root-scrolls)
* [Closing Summary — Backmatter & Scroll Ledger](#-closing-summary--backmatter--scroll-ledger)
* [Scroll Integrity Protocol](#-scroll-integrity-protocol)
* [Reference Scrolls](#-reference-scrolls)
* [SEAL — This scroll is aligned](#-seal--this-scroll-is-aligned)

---

### 🔷 OmniCore/ — ROOT

---

📁 **Root-Level Folders**

| ID                                         | Name                | Type   | Status   | Path                                             | 🗒 Note                                            |
| ------------------------------------------ | ------------------- | ------ | -------- | ------------------------------------------------ | -------------------------------------------------- |
| [`FOLDER-ROOT-001`](./.docker_temp/)       | .docker\_temp/      | Folder | Inactive | [`./.docker_temp/`](./.docker_temp/)             | Temporary staging during Docker container builds.  |
| [`FOLDER-ROOT-002`](./.tmp.drivedownload/) | .tmp.drivedownload/ | Folder | Inactive | [`./.tmp.drivedownload/`](./.tmp.drivedownload/) | Used for external syncs/downloads—safe to clean.   |
| [`FOLDER-ROOT-003`](./.tmp.driveupload/)   | .tmp.driveupload/   | Folder | Inactive | [`./.tmp.driveupload/`](./.tmp.driveupload/)     | Used for upload staging—transient only.            |
| [`FOLDER-ROOT-004`](./ai/)                 | ai/                 | Folder | Inactive | [`./ai/`](./ai/)                                 | Placeholder for AI-specific logic; not yet active. |
| [`FOLDER-ROOT-005`](./bin/)                | bin/                | Folder | Inactive | [`./bin/`](./bin/)                               | Output or tool binaries—currently unused.          |
| [`FOLDER-ROOT-006`](./code/)               | code/               | Folder | Active   | [`./code/`](./code/)                             | Core compiled source—heart of the system.          |
| [`FOLDER-ROOT-007`](./config/)             | config/             | Folder | Inactive | [`./config/`](./config/)                         | Reserved for external configuration assets.        |
| [`FOLDER-ROOT-008`](./internet/)           | internet/           | Folder | Inactive | [`./internet/`](./internet/)                     | Reserved for net-based modules (future use).       |
| [`FOLDER-ROOT-009`](./logs/)               | logs/               | Folder | Inactive | [`./logs/`](./logs/)                             | Log archive—currently not tracked here.            |
| [`FOLDER-ROOT-010`](./os/)                 | os/                 | Folder | Inactive | [`./os/`](./os/)                                 | Placeholder for system services.                   |
| [`FOLDER-ROOT-011`](./scripts/)            | scripts/            | Folder | Active   | [`./scripts/`](./scripts/)                       | Powers builds and tests—automated utility zone.    |
| [`FOLDER-ROOT-012`](./scrolls/)            | scrolls/            | Folder | Active   | [`./scrolls/`](./scrolls/)                       | Living documentation hub—devlogs, specs, scrolls.  |
| [`FOLDER-ROOT-013`](./target/)             | target/             | Folder | Inactive | [`./target/`](./target/)                         | Auto-generated by Rust—should be `.gitignored`.    |
| [`FOLDER-ROOT-014`](./tests/)              | tests/              | Folder | Inactive   | [`./tests/`](./tests/)                           | Manual test cases and scratchpads.                 |

---

📄 **Root-Level Files**

| ID                                      | Name               | Type   | Status | Path                                           | 🗒 Note                                            |
| --------------------------------------- | ------------------ | ------ | ------ | ---------------------------------------------- | -------------------------------------------------- |
| [`FILE-ROOT-001`](./.dockerignore)      | .dockerignore      | Config | Active | [`./.dockerignore`](./.dockerignore)           | Docker exclusion rules—ensures clean builds.       |
| [`FILE-ROOT-002`](./.env)               | .env               | Config | Active | [`./.env`](./.env)                             | Environment variable template.                     |
| [`FILE-ROOT-003`](./.gitignore)         | .gitignore         | Config | Active | [`./.gitignore`](./.gitignore)                 | Git tracking exclusions—honors build boundaries.   |
| [`DOC-ROOT-001`](./AUTHORS.md)          | AUTHORS.md         | Doc    | Active | [`./AUTHORS.md`](./AUTHORS.md)                 | Project authorship manifest.                       |
| [`FILE-ROOT-004`](./Cargo.lock)         | Cargo.lock         | Code   | Active | [`./Cargo.lock`](./Cargo.lock)                 | Auto-managed Rust dependency state.                |
| [`FILE-ROOT-005`](./Cargo.toml)         | Cargo.toml         | Code   | Active | [`./Cargo.toml`](./Cargo.toml)                 | Rust project manifest—root of compilation logic.   |
| [`FILE-ROOT-006`](./Dockerfile.ai)      | Dockerfile.ai      | Code   | Inactive | [`./Dockerfile.ai`](./Dockerfile.ai)           | AI-specific container setup.                       |
| [`FILE-ROOT-007`](./Dockerfile.cpp)     | Dockerfile.cpp     | Code   | Inactive | [`./Dockerfile.cpp`](./Dockerfile.cpp)         | C++ container compilation context.                 |
| [`FILE-ROOT-008`](./Dockerfile.go)      | Dockerfile.go      | Code   | Inactive | [`./Dockerfile.go`](./Dockerfile.go)           | Go language Docker build.                          |
| [`FILE-ROOT-009`](./Dockerfile.rust)    | Dockerfile.rust    | Code   | Active | [`./Dockerfile.rust`](./Dockerfile.rust)       | Rust build container setup.                        |
| [`FILE-ROOT-010`](./docker-compose.yml) | docker-compose.yml | Config | Active | [`./docker-compose.yml`](./docker-compose.yml) | Orchestrates multi-container logic.                |
| [`FILE-ROOT-011`](./LICENSE)            | LICENSE            | Legal  | Active | [`./LICENSE`](./LICENSE)                       | CreativeWorkzStudio license agreement.             |
| [`FILE-ROOT-012`](./Makefile)           | Makefile           | Config | Active | [`./Makefile`](./Makefile)                     | UNIX-style build command definitions.              |
| [`DOC-ROOT-002`](./README.md)           | README.md          | Doc    | Active | [`./README.md`](./README.md)                   | Project entry point—setup, purpose, usage.         |
| [`DOC-ROOT-003`](./STRUCTURE.md)        | STRUCTURE.md       | Doc    | Active | [`./STRUCTURE.md`](./STRUCTURE.md)             | The index itself—living scaffold of the file tree. |

---

### 🔹 `code/` — **Primary Compiled Source**

📁 Folders

| ID                               | Name   | Type   | Status | Path                           | Notes                            |
| -------------------------------- | ------ | ------ | ------ | ------------------------------ | -------------------------------- |
| [`FOLDER-CODE-001`](./code/logs/)  | logs/  | Folder | Inactive | [`code/logs/`](./code/logs/)   | External log stubs; used for CLI |
| [`FOLDER-CODE-002`](./code/tests/) | tests/ | Folder | Inactive | [`code/tests/`](./code/tests/) | Internal test targets            |
| [`FOLDER-CODE-003`](./code/src/)   | src/   | Folder | Active | [`code/src/`](./code/src/)     | Core source root                 |

📄 Files

| ID                                 | Name       | Type | Status | Path                                   | Notes                       |
| ---------------------------------- | ---------- | ---- | ------ | -------------------------------------- | --------------------------- |
| [`FILE-CODE-001`](./code/Cargo.toml) | Cargo.toml | Code | Active | [`code/Cargo.toml`](./code/Cargo.toml) | Package + dependency config |

---

#### 🔹 `code/src/`

📁 Folders

| ID                                       | Name        | Type   | Status | Path                                             | Notes                    |
| ---------------------------------------- | ----------- | ------ | ------ | ------------------------------------------------ | ------------------------ |
| [`FOLDER-SRC-001`](./code/src/gate/)       | gate/       | Folder | Inactive | [`code/src/gate/`](./code/src/gate/)             | Interface subsystem      |
| [`FOLDER-SRC-002`](./code/src/shared/)     | shared/     | Folder | Active | [`code/src/shared/`](./code/src/shared/)         | Common contracts + types |
| [`FOLDER-SRC-003`](./code/src/tablet/)     | tablet/     | Folder | Inactive | [`code/src/tablet/`](./code/src/tablet/)         | Assembler entrypoint     |
| [`FOLDER-SRC-004`](./code/src/watchtower/) | watchtower/ | Folder | Active | [`code/src/watchtower/`](./code/src/watchtower/) | Debugger + monitor       |

📄 Files

| ID                                 | Name    | Type | Status | Path                                     | Notes             |
| ---------------------------------- | ------- | ---- | ------ | ---------------------------------------- | ----------------- |
| [`FILE-SRC-001`](./code/src/lib.rs)  | lib.rs  | Code | Active | [`code/src/lib.rs`](./code/src/lib.rs)   | Core crate logic  |
| [`FILE-SRC-002`](./code/src/main.rs) | main.rs | Code | Active | [`code/src/main.rs`](./code/src/main.rs) | Binary entrypoint |

---

##### 🔹 `code/src/gate/`

📁 Folders

| ID                                           | Name       | Type   | Status | Path                                                   | Notes                      |
| -------------------------------------------- | ---------- | ------ | ------ | ------------------------------------------------------ | -------------------------- |
| [`FOLDER-GATE-001`](./code/src/gate/cli_gate/) | cli\_gate/ | Folder | Inactive | [`code/src/gate/cli_gate/`](./code/src/gate/cli_gate/) | CLI logic and handlers     |
| [`FOLDER-GATE-002`](./code/src/gate/gui_gate/) | gui\_gate/ | Folder | Inactive | [`code/src/gate/gui_gate/`](./code/src/gate/gui_gate/) | GUI visuals and event flow |

📄 Files

| ID                                               | Name             | Type | Status | Path                                                               | Notes                    |
| ------------------------------------------------ | ---------------- | ---- | ------ | ------------------------------------------------------------------ | ------------------------ |
| [`FILE-GATE-001`](./code/src/gate/mod.rs)          | mod.rs           | Code | Inactive | [`code/src/gate/mod.rs`](./code/src/gate/mod.rs)                   | Gate root module         |
| [`FILE-GATE-002`](./code/src/gate/shared_utils.rs) | shared\_utils.rs | Code | Inactive | [`code/src/gate/shared_utils.rs`](./code/src/gate/shared_utils.rs) | Shared utility functions |

---

###### 🔹 `code/src/gate/cli_gate/`

📄 Files

| ID                                                         | Name        | Type | Status | Path                                                                         | Notes                   |
| ---------------------------------------------------------- | ----------- | ---- | ------ | ---------------------------------------------------------------------------- | ----------------------- |
| [`FILE-CLIGATE-001`](./code/src/gate/cli_gate/commands.rs) | commands.rs | Code | Active | [`code/src/gate/cli_gate/commands.rs`](./code/src/gate/cli_gate/commands.rs) | Command parsing + logic |
| [`FILE-CLIGATE-002`](./code/src/gate/cli_gate/handlers.rs) | handlers.rs | Code | Active | [`code/src/gate/cli_gate/handlers.rs`](./code/src/gate/cli_gate/handlers.rs) | Execution routing       |
| [`FILE-CLIGATE-003`](./code/src/gate/cli_gate/main.rs)     | main.rs     | Code | Active | [`code/src/gate/cli_gate/main.rs`](./code/src/gate/cli_gate/main.rs)         | Entry point for CLI     |
| [`FILE-CLIGATE-004`](./code/src/gate/cli_gate/mod.rs)      | mod.rs      | Code | Active | [`code/src/gate/cli_gate/mod.rs`](./code/src/gate/cli_gate/mod.rs)           | Module definition       |
| [`FILE-CLIGATE-005`](./code/src/gate/cli_gate/utils.rs)    | utils.rs    | Code | Active | [`code/src/gate/cli_gate/utils.rs`](./code/src/gate/cli_gate/utils.rs)       | CLI-specific utilities  |

---

###### 🔹 `code/src/gate/gui_gate/`

📄 Files

| ID                                                               | Name               | Type | Status | Path                                                                                     | Notes                            |
| ---------------------------------------------------------------- | ------------------ | ---- | ------ | ---------------------------------------------------------------------------------------- | -------------------------------- |
| [`FILE-GUIGATE-001`](./code/src/gate/gui_gate/event_handlers.rs) | event\_handlers.rs | Code | Active | [`code/src/gate/gui_gate/event_handlers.rs`](./code/src/gate/gui_gate/event_handlers.rs) | Input + UI event routing         |
| [`FILE-GUIGATE-002`](./code/src/gate/gui_gate/mod.rs)            | mod.rs             | Code | Active | [`code/src/gate/gui_gate/mod.rs`](./code/src/gate/gui_gate/mod.rs)                       | Module declaration               |
| [`FILE-GUIGATE-003`](./code/src/gate/gui_gate/visuals.rs)        | visuals.rs         | Code | Active | [`code/src/gate/gui_gate/visuals.rs`](./code/src/gate/gui_gate/visuals.rs)               | Theming, colors, UI assets       |
| [`FILE-GUIGATE-004`](./code/src/gate/gui_gate/window.rs)         | window\.rs         | Code | Active | [`code/src/gate/gui_gate/window.rs`](./code/src/gate/gui_gate/window.rs)                 | Window frame logic (egui driver) |

---

##### 🔹 `code/src/shared/`

📁 Folders

| ID                                                   | Name        | Type   | Status | Path                                                           | Notes                         |
| ---------------------------------------------------- | ----------- | ------ | ------ | -------------------------------------------------------------- | ----------------------------- |
| [`FOLDER-SHARED-001`](./code/src/shared/constants/)  | constants/  | Folder | Inactive | [`code/src/shared/constants/`](./code/src/shared/constants/)   | Static values and definitions |
| [`FOLDER-SHARED-002`](./code/src/shared/core/)       | core/       | Folder | Inactive | [`code/src/shared/core/`](./code/src/shared/core/)             | System core components        |
| [`FOLDER-SHARED-003`](./code/src/shared/interfaces/) | interfaces/ | Folder | Inactive | [`code/src/shared/interfaces/`](./code/src/shared/interfaces/) | Trait and contract interfaces |
| [`FOLDER-SHARED-004`](./code/src/shared/schemas/)    | schemas/    | Folder | Inactive | [`code/src/shared/schemas/`](./code/src/shared/schemas/)       | Log + scoring data structures |
| [`FOLDER-SHARED-005`](./code/src/shared/types/)      | types/      | Folder | Active | [`code/src/shared/types/`](./code/src/shared/types/)           | Shared enum + struct types    |

📄 Files

| ID                                            | Name   | Type | Status | Path                                                 | Notes                   |
| --------------------------------------------- | ------ | ---- | ------ | ---------------------------------------------------- | ----------------------- |
| [`FILE-SHARED-001`](./code/src/shared/mod.rs) | mod.rs | Code | Active | [`code/src/shared/mod.rs`](./code/src/shared/mod.rs) | Module declaration root |

---

###### 🔹 `code/src/shared/constants/`

📄 Files

| ID                                                                | Name   | Type | Status | Path                                                                     | Notes                     |
| ----------------------------------------------------------------- | ------ | ---- | ------ | ------------------------------------------------------------------------ | ------------------------- |
| [`FILE-SHARED-CONSTANTS-001`](./code/src/shared/constants/mod.rs) | mod.rs | Code | Inactive | [`code/src/shared/constants/mod.rs`](./code/src/shared/constants/mod.rs) | Constant definitions root |

---

###### 🔹 `code/src/shared/core/`

📄 Files

| ID                                                      | Name   | Type | Status | Path                                                           | Notes                    |
| ------------------------------------------------------- | ------ | ---- | ------ | -------------------------------------------------------------- | ------------------------ |
| [`FILE-SHARED-CORE-001`](./code/src/shared/core/mod.rs) | mod.rs | Code | Inactive | [`code/src/shared/core/mod.rs`](./code/src/shared/core/mod.rs) | Core system logic module |

---

###### 🔹 `code/src/shared/interfaces/`

📄 Files

| ID                                                                  | Name   | Type | Status | Path                                                                       | Notes                        |
| ------------------------------------------------------------------- | ------ | ---- | ------ | -------------------------------------------------------------------------- | ---------------------------- |
| [`FILE-SHARED-INTERFACES-001`](./code/src/shared/interfaces/mod.rs) | mod.rs | Code | Inactive | [`code/src/shared/interfaces/mod.rs`](./code/src/shared/interfaces/mod.rs) | Trait/interface declarations |

---

###### 🔹 `code/src/shared/schemas/`

📁 Folders

| ID                                                                | Name     | Type   | Status | Path                                                                     | Notes                     |
| ----------------------------------------------------------------- | -------- | ------ | ------ | ------------------------------------------------------------------------ | ------------------------- |
| [`FOLDER-SHARED-SCHEMAS-001`](./code/src/shared/schemas/scoring/) | scoring/ | Folder | Active | [`code/src/shared/schemas/scoring/`](./code/src/shared/schemas/scoring/) | Contains scoring profiles |

📄 Files

| ID                                                                              | Name                       | Type | Status | Path                                                                                                     | Notes                  |
| ------------------------------------------------------------------------------- | -------------------------- | ---- | ------ | -------------------------------------------------------------------------------------------------------- | ---------------------- |
| [`FILE-SHARED-SCHEMAS-001`](./code/src/shared/schemas/mod.rs)                   | mod.rs                     | Code | Active | [`code/src/shared/schemas/mod.rs`](./code/src/shared/schemas/mod.rs)                                     | Schema root module     |
| [`FILE-SHARED-SCHEMAS-002`](./code/src/shared/schemas/base_log_entry.rs)        | base\_log\_entry.rs        | Code | Active | [`code/src/shared/schemas/base_log_entry.rs`](./code/src/shared/schemas/base_log_entry.rs)               | Shared log structure   |
| [`FILE-SHARED-SCHEMAS-003`](./code/src/shared/schemas/log_types.rs)             | log\_types.rs              | Code | Active | [`code/src/shared/schemas/log_types.rs`](./code/src/shared/schemas/log_types.rs)                         | Type variants for logs |
| [`FILE-SHARED-SCHEMAS-004`](./code/src/shared/schemas/severity.rs)              | severity.rs                | Code | Active | [`code/src/shared/schemas/severity.rs`](./code/src/shared/schemas/severity.rs)                           | Severity scale logic   |
| [`FILE-SHARED-SCHEMAS-005`](./code/src/shared/schemas/specialized_log_entry.rs) | specialized\_log\_entry.rs | Code | Active | [`code/src/shared/schemas/specialized_log_entry.rs`](./code/src/shared/schemas/specialized_log_entry.rs) | Custom log variants    |

---

###### 🔹 `code/src/shared/schemas/scoring/`

📄 Files

| ID                                                                                         | Name                           | Type | Status | Path                                                                                                                           | Notes                          |
| ------------------------------------------------------------------------------------------ | ------------------------------ | ---- | ------ | ------------------------------------------------------------------------------------------------------------------------------ | ------------------------------ |
| [`FILE-SHARED-SCORING-001`](./code/src/shared/schemas/scoring/mod.rs)                      | mod.rs                         | Code | Active | [`code/src/shared/schemas/scoring/mod.rs`](./code/src/shared/schemas/scoring/mod.rs)                                           | Scoring module root            |
| [`FILE-SHARED-SCORING-002`](./code/src/shared/schemas/scoring/log_type_scoring_profile.rs) | log\_type\_scoring\_profile.rs | Code | Active | [`code/src/shared/schemas/scoring/log_type_scoring_profile.rs`](./code/src/shared/schemas/scoring/log_type_scoring_profile.rs) | Profiles for log scoring logic |

---

###### 🔹 `code/src/shared/types/`

📄 Files

| ID                                                        | Name   | Type | Status | Path                                                             | Notes                        |
| --------------------------------------------------------- | ------ | ---- | ------ | ---------------------------------------------------------------- | ---------------------------- |
| [`FILE-SHARED-TYPES-001`](./code/src/shared/types/mod.rs) | mod.rs | Code | Inactive | [`code/src/shared/types/mod.rs`](./code/src/shared/types/mod.rs) | Type alias + shared variants |

---

##### 🔹 `code/src/tablet/`

📄 Files

| ID                                            | Name   | Type | Status | Path                                                 | Notes                 |
| --------------------------------------------- | ------ | ---- | ------ | ---------------------------------------------------- | --------------------- |
| [`FILE-TABLET-001`](./code/src/tablet/mod.rs) | mod.rs | Code | Inactive | [`code/src/tablet/mod.rs`](./code/src/tablet/mod.rs) | Assembler entry point |

---

##### 🔹 `code/src/watchtower/`

📁 Folders

| ID                                                           | Name        | Type   | Status | Path                                                                   | Notes                       |
| ------------------------------------------------------------ | ----------- | ------ | ------ | ---------------------------------------------------------------------- | --------------------------- |
| [`FOLDER-WATCHTOWER-001`](./code/src/watchtower/config/)     | config/     | Folder | Inactive | [`code/src/watchtower/config/`](./code/src/watchtower/config/)         | Configs for debugger setup  |
| [`FOLDER-WATCHTOWER-002`](./code/src/watchtower/events/)     | events/     | Folder | Inactive | [`code/src/watchtower/events/`](./code/src/watchtower/events/)         | Event interface + signals   |
| [`FOLDER-WATCHTOWER-003`](./code/src/watchtower/logs/)       | logs/       | Folder | Active | [`code/src/watchtower/logs/`](./code/src/watchtower/logs/)             | Logging engine + entries    |
| [`FOLDER-WATCHTOWER-004`](./code/src/watchtower/monitoring/) | monitoring/ | Folder | Active | [`code/src/watchtower/monitoring/`](./code/src/watchtower/monitoring/) | Alignment logic + telemetry |
| [`FOLDER-WATCHTOWER-005`](./code/src/watchtower/util/)       | util/       | Folder | Inactive | [`code/src/watchtower/util/`](./code/src/watchtower/util/)             | Internal helpers and tools  |

📄 Files

| ID                                                           | Name           | Type | Status | Path                                                                       | Notes                          |
| ------------------------------------------------------------ | -------------- | ---- | ------ | -------------------------------------------------------------------------- | ------------------------------ |
| [`FILE-WATCHTOWER-001`](./code/src/watchtower/mod.rs)        | mod.rs         | Code | Active | [`code/src/watchtower/mod.rs`](./code/src/watchtower/mod.rs)               | Watchtower root module         |
| [`FILE-WATCHTOWER-002`](./code/src/watchtower/log_router.rs) | log\_router.rs | Code | Active | [`code/src/watchtower/log_router.rs`](./code/src/watchtower/log_router.rs) | Routes logs into correct flows |
| [`FILE-WATCHTOWER-003`](./code/src/watchtower/watchtower.rs) | watchtower.rs  | Code | Active | [`code/src/watchtower/watchtower.rs`](./code/src/watchtower/watchtower.rs) | Main Watchtower implementation |

---

###### 🔹 `code/src/watchtower/config/`

📄 Files

| ID                                                                  | Name   | Type | Status | Path                                                                       | Notes             |
| ------------------------------------------------------------------- | ------ | ---- | ------ | -------------------------------------------------------------------------- | ----------------- |
| [`FILE-WATCHTOWER-CONFIG-001`](./code/src/watchtower/config/mod.rs) | mod.rs | Code | Inactive | [`code/src/watchtower/config/mod.rs`](./code/src/watchtower/config/mod.rs) | Config entrypoint |

---

###### 🔹 `code/src/watchtower/events/`

📄 Files

| ID                                                                  | Name   | Type | Status | Path                                                                       | Notes              |
| ------------------------------------------------------------------- | ------ | ---- | ------ | -------------------------------------------------------------------------- | ------------------ |
| [`FILE-WATCHTOWER-EVENTS-001`](./code/src/watchtower/events/mod.rs) | mod.rs | Code | Inactive | [`code/src/watchtower/events/mod.rs`](./code/src/watchtower/events/mod.rs) | Events root module |

---

###### 🔹 `code/src/watchtower/logs/`

📄 Files

| ID                                                                     | Name           | Type | Status | Path                                                                                 | Notes                         |
| ---------------------------------------------------------------------- | -------------- | ---- | ------ | ------------------------------------------------------------------------------------ | ----------------------------- |
| [`FILE-WATCHTOWER-LOGS-001`](./code/src/watchtower/logs/mod.rs)        | mod.rs         | Code | Active | [`code/src/watchtower/logs/mod.rs`](./code/src/watchtower/logs/mod.rs)               | Logs module root              |
| [`FILE-WATCHTOWER-LOGS-002`](./code/src/watchtower/logs/log_entry.rs)  | log\_entry.rs  | Code | Active | [`code/src/watchtower/logs/log_entry.rs`](./code/src/watchtower/logs/log_entry.rs)   | Log entry schema              |
| [`FILE-WATCHTOWER-LOGS-003`](./code/src/watchtower/logs/log_writer.rs) | log\_writer.rs | Code | Active | [`code/src/watchtower/logs/log_writer.rs`](./code/src/watchtower/logs/log_writer.rs) | Log writing and routing logic |

---

###### 🔹 `code/src/watchtower/monitoring/`

📄 Files

| ID                                                                                | Name         | Type | Status | Path                                                                                           | Notes                   |
| --------------------------------------------------------------------------------- | ------------ | ---- | ------ | ---------------------------------------------------------------------------------------------- | ----------------------- |
| [`FILE-WATCHTOWER-MONITORING-001`](./code/src/watchtower/monitoring/mod.rs)       | mod.rs       | Code | Active | [`code/src/watchtower/monitoring/mod.rs`](./code/src/watchtower/monitoring/mod.rs)             | Monitoring root module  |
| [`FILE-WATCHTOWER-MONITORING-002`](./code/src/watchtower/monitoring/alignment.rs) | alignment.rs | Code | Active | [`code/src/watchtower/monitoring/alignment.rs`](./code/src/watchtower/monitoring/alignment.rs) | Alignment scoring logic |

---

###### 🔹 `code/src/watchtower/util/`

📄 Files

| ID                                                              | Name   | Type | Status | Path                                                                   | Notes                 |
| --------------------------------------------------------------- | ------ | ---- | ------ | ---------------------------------------------------------------------- | --------------------- |
| [`FILE-WATCHTOWER-UTIL-001`](./code/src/watchtower/util/mod.rs) | mod.rs | Code | Inactive | [`code/src/watchtower/util/mod.rs`](./code/src/watchtower/util/mod.rs) | Utilities and helpers |

---

### 🔹 `scripts/` — **Build + Test Automation**

📁 Folders

| ID                                       | Name   | Type   | Status | Path                                 | Notes                       |
| ---------------------------------------- | ------ | ------ | ------ | ------------------------------------ | --------------------------- |
| [`FOLDER-SCRIPTS-001`](./scripts/build/) | build/ | Folder | Active | [`scripts/build/`](./scripts/build/) | PowerShell build automation |
| [`FOLDER-SCRIPTS-002`](./scripts/tests/) | tests/ | Folder | Active | [`scripts/tests/`](./scripts/tests/) | Watchtower log test scripts |

📄 Files

*None at root level.*

---

#### 🔸 `scripts/build/`

📄 Files

| ID                                              | Name      | Type   | Status | Path                                                   | Notes                   |
| ----------------------------------------------- | --------- | ------ | ------ | ------------------------------------------------------ | ----------------------- |
| [`SCRIPT-BUILD-001`](./scripts/build/build.ps1) | build.ps1 | Script | Active | [`scripts/build/build.ps1`](./scripts/build/build.ps1) | PowerShell build script |

---

#### 🔸 `scripts/tests/`

📄 Files

| ID                                                           | Name                      | Type   | Status | Path                                                                               | Notes                  |
| ------------------------------------------------------------ | ------------------------- | ------ | ------ | ---------------------------------------------------------------------------------- | ---------------------- |
| [`SCRIPT-TEST-001`](./scripts/tests/test_watchtower_log.ps1) | test\_watchtower\_log.ps1 | Script | Active | [`scripts/tests/test_watchtower_log.ps1`](./scripts/tests/test_watchtower_log.ps1) | Windows-based log test |
| [`SCRIPT-TEST-002`](./scripts/tests/test_watchtower_log.sh)  | test\_watchtower\_log.sh  | Script | Active | [`scripts/tests/test_watchtower_log.sh`](./scripts/tests/test_watchtower_log.sh)   | Unix-based log test    |

---

### 🔹 `scrolls/` — **Living Documentation**

📁 Folders

| ID                                                  | Name              | Type   | Status | Path                                                       | Notes                               |
| --------------------------------------------------- | ----------------- | ------ | ------ | ---------------------------------------------------------- | ----------------------------------- |
| [`FOLDER-SCROLLS-001`](./scrolls/Devlogs/)          | Devlogs/          | Folder | Active | [`scrolls/Devlogs/`](./scrolls/Devlogs/)                   | Developer diaries and logs          |
| [`FOLDER-SCROLLS-002`](./scrolls/ProgressionPoint/) | ProgressionPoint/ | Folder | Active | [`scrolls/ProgressionPoint/`](./scrolls/ProgressionPoint/) | Phase markers and milestone scrolls |
| [`FOLDER-SCROLLS-003`](./scrolls/templates/)        | templates/        | Folder | Active | [`scrolls/templates/`](./scrolls/templates/)               | Scroll and log templates            |

#### 📄 Root Scrolls

| ID                                                   | Name                   | Type | Status | Path                                                               | Notes                          |
| ---------------------------------------------------- | ---------------------- | ---- | ------ | ------------------------------------------------------------------ | ------------------------------ |
| [`SCROLL-ROOT-001`](./scrolls/CHANGELOG.md)          | CHANGELOG.md           | Doc  | Active | [`scrolls/CHANGELOG.md`](./scrolls/CHANGELOG.md)                   | Chronological dev record       |
| [`SCROLL-ROOT-002`](./scrolls/Continuous_Roadmap.md) | Continuous\_Roadmap.md | Doc  | Active | [`scrolls/Continuous_Roadmap.md`](./scrolls/Continuous_Roadmap.md) | Forward-looking milestone plan |
| [`SCROLL-ROOT-003`](./scrolls/CONTRIBUTING.md)       | CONTRIBUTING.md        | Doc  | Active | [`scrolls/CONTRIBUTING.md`](./scrolls/CONTRIBUTING.md)             | Contribution guidelines        |
| [`SCROLL-ROOT-004`](./scrolls/design_spec.md)        | design\_spec.md        | Doc  | Active | [`scrolls/design_spec.md`](./scrolls/design_spec.md)               | System-level design outline    |

---

#### 🔸 `scrolls/Devlogs/`

📄 Files

| ID                                             | Name           | Type | Status | Path                                                             | Notes                       |
| ---------------------------------------------- | -------------- | ---- | ------ | ---------------------------------------------------------------- | --------------------------- |
| [`DEVLOG-001`](./scrolls/Devlogs/dev_log_0.md) | dev\_log\_0.md | Doc  | Active | [`scrolls/Devlogs/dev_log_0.md`](./scrolls/Devlogs/dev_log_0.md) | Initial developer heartbeat |

---

#### 🔸 `scrolls/ProgressionPoint/`

📄 Files

| ID                                                                    | Name                       | Type | Status | Path                                                                                                     | Notes                         |
| --------------------------------------------------------------------- | -------------------------- | ---- | ------ | -------------------------------------------------------------------------------------------------------- | ----------------------------- |
| [`MILESTONE-001`](./scrolls/ProgressionPoint/Phase_0_Watchtower_1.md) | Phase\_0\_Watchtower\_1.md | Doc  | Active | [`scrolls/ProgressionPoint/Phase_0_Watchtower_1.md`](./scrolls/ProgressionPoint/Phase_0_Watchtower_1.md) | Initial Watchtower checkpoint |
| [`MILESTONE-002`](./scrolls/ProgressionPoint/Phase_0_Watchtower_2.md) | Phase\_0\_Watchtower\_2.md | Doc  | Active | [`scrolls/ProgressionPoint/Phase_0_Watchtower_2.md`](./scrolls/ProgressionPoint/Phase_0_Watchtower_2.md) | Secondary progress marker     |
| [`MILESTONE-003`](./scrolls/ProgressionPoint/Phase_0_Watchtower_3.md) | Phase\_0\_Watchtower\_3.md | Doc  | Active | [`scrolls/ProgressionPoint/Phase_0_Watchtower_3.md`](./scrolls/ProgressionPoint/Phase_0_Watchtower_3.md) | Final milestone in Phase 0    |

---

#### 🔸 `scrolls/templates/`

📄 Files

| ID                                                        | Name                  | Type | Status | Path                                                                               | Notes                           |
| --------------------------------------------------------- | --------------------- | ---- | ------ | ---------------------------------------------------------------------------------- | ------------------------------- |
| [`TEMPLATE-001`](./scrolls/templates/dev_log_template.md) | dev\_log\_template.md | Doc  | Active | [`scrolls/templates/dev_log_template.md`](./scrolls/templates/dev_log_template.md) | Developer log structure starter |

---

## 🔚 Closing Summary — Backmatter & Scroll Ledger

This scroll defines the **living structure** of **Project Nova Dawn**.

It reflects:

* The present and **witnessed shape** of the `OmniCore` system
* The **intentional scaffolding** of compiled and documented thought
* The **relational clarity** between code, scripts, scrolls, and source
* The **lineage** of structure, not just location

It is **not just an index**—it is a **mirror of how we think**, design, and breathe order into architecture.
When folders move or files change, **this scroll must be updated**, for it is the **first witness** of alignment and covenant maintenance.

---

## 🧷 Scroll Integrity Protocol

* All `ID`s are **unique**, **stable**, and **linkable**
* All `Path`s reflect actual system structure
* `Type`, `Status`, and `Notes` fields are **standardized and traceable**
* No phantom stubs or aspirational scaffolds are permitted
* This scroll may only be changed in response to **real system changes**

> **Note:** This document reflects **truth**, not possibility. It is aligned only to **what is**, not what might be.

---

## 🔗 Reference Scrolls

| Name                                                               | Purpose                          |
| ------------------------------------------------------------------ | -------------------------------- |
| [`README.md`](./README.md)                                         | Project introduction and build   |
| [`scrolls/CONTRIBUTING.md`](./scrolls/CONTRIBUTING.md)             | Collaboration protocol and rules |
| [`scrolls/Continuous_Roadmap.md`](./scrolls/Continuous_Roadmap.md) | Milestones and phasing guide     |
| [`scrolls/CHANGELOG.md`](./scrolls/CHANGELOG.md)                   | Version updates and log trace    |

---

## ✅ SEAL — This scroll is aligned

> This structure is true as of **2025-06-17**, authored by **Seanje / Nova Dawn**.
> It reflects the shape of the Kingdom vision encoded in system form.
> All changes to this scroll must preserve **order**, **clarity**, and **covenantal traceability**.

---

<!--
  @title: STRUCTURE.md — Project File Index
  @author: Seanje Lenox-Wise / Nova Dawn
  @version: 0.0.1
  @status: Active
  @type: Doc
  @component: Root-Level Index
  @project: OmniCore / NovaAI
  @id: DOC-ROOT-003
  @path: OmniCore/STRUCTURE.md
  @created: 2025-06-17
  @updated: 2025-06-18
  @license: CreativeWorkzStudio LLC — Kingdom-First Proprietary Use
  @tags: structure, root, index, scroll-standard, toc
-->
