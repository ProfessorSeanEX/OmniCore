################################################################
# 🌀 SCROLL DECLARATION — Go Runtime Container (Internet Module)
# Purpose: Compiles and runs the Go-based internet module.
# Structure: Metadata → Opening → Body → Closing
################################################################

##############################################################
# 📜 Go Container — Root Dockerfile (Phase 0)
# This scroll governs containerization for the Go module.
#
# ┌────────────────────────────────────────────┐
# │ BLOCK: OPENING (title, purpose, authorship)│
# └────────────────────────────────────────────┘
##############################################################

# 🧾 METADATA — Scroll Identity & Authorship
# These declarations identify the scroll’s origin, version, and scope.
# Do not alter without Watchtower review.

# Title:        Dockerfile — Internet Module (Go)
# Author:       Seanje Lenox-Wise / Nova Dawn
# Version:      0.0.1
# Status:       Inactive
# Type:         Config
# Component:    Docker Build — Internet Module (Go)
# Project:      OmniCore / OmniCode
# ID:           FILE-ROOT-008
# Path:         OmniCore/Dockerfile.go
# Created:      2025-06-17
# Last Updated: 2025-06-18
# License:      CreativeWorkzStudio LLC — Kingdom-First Proprietary Use

# 📝 DESCRIPTION — Scroll Purpose
# This Dockerfile builds and runs the Go module used by the Internet subsystem.
# It uses the official Go image to compile and execute the service inside the container.

# ------------------------------------------------------------
# 🧱 BASE IMAGE — Go Build Environment
# ------------------------------------------------------------
# Uses latest stable Go base image from DockerHub
FROM golang:latest

# ------------------------------------------------------------
# 📁 WORKDIR — Container Internal Workspace
# ------------------------------------------------------------
# All operations within the container will occur in /app
WORKDIR /app

##############################################################
# ┌────────────────────────────────────────────┐
# │ BLOCK: BODY — Source Setup and Build Logic │
# └────────────────────────────────────────────┘
##############################################################

# ------------------------------------------------------------
# 📦 Copy Source — Import Go source files into container
# ------------------------------------------------------------
# Transfers full source tree from host into the container
COPY . .

# ------------------------------------------------------------
# 🧱 Compile — Build Go binary (optional for small projects)
# ------------------------------------------------------------
# Builds the Go project using default go build behavior
RUN go build

##############################################################
# ┌────────────────────────────────────────────┐
# │ BLOCK: CLOSING — Entrypoint & Runtime Logic │
# └────────────────────────────────────────────┘
##############################################################

# ------------------------------------------------------------
# 🚀 Launch Go App — Set container command
# ------------------------------------------------------------
# Runs the Go app via go run targeting main.go
CMD ["go", "run", "main.go"]

# ------------------------------------------------------------
# 🔚 LOGIC TERMINUS — End of Executable Flow
# All runtime targets end above this line. Below is annotation only.
# ------------------------------------------------------------

## 🔚 Scroll Closure Summary:
##   This Dockerfile prepares and launches the Go-based internet module.
##   It uses a clean base, compiles the code, and runs it directly.

## 🚨 Covenant Warning:
##   Do not alter the CMD or base image without Watchtower confirmation.
##   Simplicity is part of the design—do not over-engineer.

## 📅 Scroll Metadata:
##   _version_:        0.0.1  
##   _last updated_:   2025-06-17  
##   _author_:         Seanje Lenox-Wise / Nova Dawn  
##   _status_:         Active  
##   _component_:      Docker Build — Internet Module (Go)  
##   _project_:        OmniCore / OmniCode  

## 🧾 Scroll Seal:
##   This scroll is covenantal—preserve its simplicity and structure.
##   Even light containers require heavy integrity.
