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
* **ID:** DOC-STRUCTURE-001
* **Created:** 2025-06-17
* **Last Updated:** 2025-06-17
* **License:** CreativeWorkzStudio LLC — Kingdom-First Proprietary Use

---

## 📚 Table of Contents

* [🔷 OmniCore/ — ROOT](#🔷-omnicore--root)
* [🔹 code/ — Primary Compiled Source](#🔹-code--primary-compiled-source)

  * [🧩 gate/cli\_gate/ — CLI Logic](#🧩-gatecli_gate--command-line-interface-logic)
  * [🖼️ gate/gui\_gate/ — GUI Logic](#🖼️-gategui_gate--graphical-interface-logic)
  * [🔗 shared/ — Common Types](#🔗-shared--common-types-interfaces-and-contracts)
  * [🧱 tablet/ — Assembler System](#🧱-tablet--assembler-system)
  * [🔭 watchtower/ — Debugger](#🔭-watchtower--debugger--alignment-monitor)
* [📁 scripts/ — Build + Test Automation](#📁-scripts--build--test-automation)
* [📁 scrolls/ — Living Documentation](#📁-scrolls--living-documentation)
* [🔚 Closing Summary](#-closing-summary)
* [🔗 References](#-references)
* [✅ SEAL — This scroll is aligned.](#️-seal--this-scroll-is-aligned)

---

### 🔷 `OmniCore/` — **ROOT**

**📁 FOLDERS:**

* `.docker_temp/`
* `.tmp.drivedownload/`
* `.tmp.driveupload/`
* `ai/`
* `bin/`
* `code/`
* `config/`
* `internet/`
* `logs/`
* `os/`
* `scripts/`
* `scrolls/`
* `target/`
* `tests/`

**📄 FILES:**

* `.dockerignore`
* `.env`
* `.gitignore`
* `AUTHORS.md`
* `Cargo.lock`
* `Cargo.toml`
* `Dockerfile.ai`
* `Dockerfile.cpp`
* `Dockerfile.go`
* `Dockerfile.rust`
* `docker-compose.yml`
* `LICENSE`
* `Makefile`
* `README.md`
* `STRUCTURE.md`

---

### 🔹 `code/` — **Primary Compiled Source**

**📁 FOLDERS:**

* `code/logs/`
* `code/tests/`
* `code/src/`
* `code/src/gate/`
* `code/src/gate/cli_gate/`
* `code/src/gate/gui_gate/`
* `code/src/shared/`
* `code/src/shared/constants/`
* `code/src/shared/core/`
* `code/src/shared/interfaces/`
* `code/src/shared/schemas/`
* `code/src/shared/schemas/scoring/`
* `code/src/shared/types/`
* `code/src/tablet/`
* `code/src/watchtower/`
* `code/src/watchtower/config/`
* `code/src/watchtower/events/`
* `code/src/watchtower/logs/`
* `code/src/watchtower/monitoring/`
* `code/src/watchtower/util/`

**📄 FILES:**

* `code/Cargo.toml`
* `code/src/lib.rs`
* `code/src/main.rs`
* `code/src/gate/mod.rs`
* `code/src/gate/shared_utils.rs`

---

#### 🧩 `gate/cli_gate/` — Command Line Interface Logic

* `code/src/gate/cli_gate/commands.rs`
* `code/src/gate/cli_gate/handlers.rs`
* `code/src/gate/cli_gate/main.rs`
* `code/src/gate/cli_gate/mod.rs`
* `code/src/gate/cli_gate/utils.rs`

---

#### 🖼️ `gate/gui_gate/` — Graphical Interface Logic

* `code/src/gate/gui_gate/event_handlers.rs`
* `code/src/gate/gui_gate/mod.rs`
* `code/src/gate/gui_gate/visuals.rs`
* `code/src/gate/gui_gate/window.rs`

---

#### 🔗 `shared/` — Common Types, Interfaces, and Contracts

* `code/src/shared/mod.rs`
* `code/src/shared/constants/mod.rs`
* `code/src/shared/core/mod.rs`
* `code/src/shared/interfaces/mod.rs`
* `code/src/shared/types/mod.rs`

📜 **Schemas**

* `code/src/shared/schemas/mod.rs`
* `code/src/shared/schemas/base_log_entry.rs`
* `code/src/shared/schemas/log_types.rs`
* `code/src/shared/schemas/severity.rs`
* `code/src/shared/schemas/specialized_log_entry.rs`

📊 **Scoring Submodule**

* `code/src/shared/schemas/scoring/mod.rs`
* `code/src/shared/schemas/scoring/log_type_scoring_profile.rs`

---

#### 🧱 `tablet/` — Assembler System

* `code/src/tablet/mod.rs`

---

#### 🔭 `watchtower/` — Debugger + Alignment Monitor

* `code/src/watchtower/mod.rs`
* `code/src/watchtower/log_router.rs`
* `code/src/watchtower/watchtower.rs`

📁 **Submodules**

* `code/src/watchtower/config/mod.rs`
* `code/src/watchtower/events/mod.rs`
* `code/src/watchtower/util/mod.rs`

📜 **Logs**

* `code/src/watchtower/logs/mod.rs`
* `code/src/watchtower/logs/log_entry.rs`
* `code/src/watchtower/logs/log_writer.rs`

📊 **Monitoring**

* `code/src/watchtower/monitoring/mod.rs`
* `code/src/watchtower/monitoring/alignment.rs`

---

### 📁 `scripts/` — **Build + Test Automation**

**🧰 Build Tools:**

* `scripts/build/build.ps1`

**🧪 Test Scripts:**

* `scripts/tests/test_watchtower_log.ps1`
* `scripts/tests/test_watchtower_log.sh`

---

### 📁 `scrolls/` — **Living Documentation**

**🪵 Devlogs/** — Developer diaries and heartbeat updates

* `scrolls/Devlogs/dev_log_0.md`

**🌀 ProgressionPoint/** — Phase markers and milestone scrolls

* `scrolls/ProgressionPoint/Phase_0_Watchtower_1.md`
* `scrolls/ProgressionPoint/Phase_0_Watchtower_2.md`
* `scrolls/ProgressionPoint/Phase_0_Watchtower_3.md`

**📜 templates/** — Writing scaffolds and documentation templates

* `scrolls/templates/dev_log_template.md`

**📄 Root Scrolls** — System-wide documentation

* `scrolls/CHANGELOG.md`
* `scrolls/Continuous_Roadmap.md`
* `scrolls/CONTRIBUTING.md`
* `scrolls/design_spec.md`

---

## 🔚 Closing Summary

This scroll defines the living structure of Project Nova Dawn.

It reflects:

* The present shape of the OmniCore system
* The organized scaffolding of compiled and documented thought
* The relational clarity between code, scripts, scrolls, and source

It is **not just an index**—it is a mirror of how we think, design, and breathe order into our architecture. When folders move or files change, **this scroll must be updated**, for it is the first witness of our alignment.

---

## 🔗 References

* `README.md` — Project intro and build instructions  
* `scrolls/CONTRIBUTING.md` — Collaboration structure  
* `scrolls/Continuous_Roadmap.md` — Milestones and phasing  
* `scrolls/CHANGELOG.md` — Versioned updates and log  

---

## ✅ SEAL — This scroll is aligned

> This structure is true as of **2025-06-17**, authored by **Seanje / Nova Dawn**.  
> It reflects the shape of the Kingdom vision encoded in system form.  
> All changes to this scroll must preserve order, clarity, and covenantal traceability.  
