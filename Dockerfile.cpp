################################################################
# 🌀 SCROLL DECLARATION — C++ Runtime Container (OS Component)
# Purpose: Compiles and runs the C++-based OS subsystem.
# Structure: Metadata → Opening → Body → Closing
################################################################

##############################################################
# 📜 C++ Container — Root Dockerfile (Phase 0)
# This scroll governs containerization for the OS component.
#
# ┌────────────────────────────────────────────┐
# │ BLOCK: OPENING (title, purpose, authorship)│
# └────────────────────────────────────────────┘
##############################################################

# 🧾 METADATA — Scroll Identity & Authorship
# These declarations identify the scroll’s origin, version, and scope.
# Do not alter without Watchtower review.

# Title:        Dockerfile — OS Module (C++)
# Author:       Seanje Lenox-Wise / Nova Dawn
# Version:      0.0.1
# Status:       Inactive
# Type:         Config
# Component:    Docker Build — OS Module (C++)
# Project:      OmniCore / OmniCode
# ID:           FILE-ROOT-007
# Path:         OmniCore/Dockerfile.cpp
# Created:      2025-06-17
# Last Updated: 2025-06-18
# License:      CreativeWorkzStudio LLC — Kingdom-First Proprietary Use

# 📝 DESCRIPTION — Scroll Purpose
# This Dockerfile compiles and runs the C++ OS component.
# It uses the GCC image for compilation and executes the result directly.

# ------------------------------------------------------------
# 🧱 BASE IMAGE — C++ Build Environment
# ------------------------------------------------------------
# Uses the latest GCC container to compile C++ source
FROM gcc:latest

# ------------------------------------------------------------
# 📁 WORKDIR — Container Internal Workspace
# ------------------------------------------------------------
# All operations will occur within the /app directory
WORKDIR /app


##############################################################
# ┌────────────────────────────────────────────┐
# │ BLOCK: BODY — Source Setup and Build Logic │
# └────────────────────────────────────────────┘
##############################################################

# ------------------------------------------------------------
# 📦 Copy Source — Import C++ source into container
# ------------------------------------------------------------
# Transfers all files from host to /app in container
COPY . .

# ------------------------------------------------------------
# 🧱 Compile — Build the OS component binary
# ------------------------------------------------------------
# Uses g++ to compile the os_component source file
RUN g++ -o os_component os_component.cpp


##############################################################
# ┌────────────────────────────────────────────┐
# │ BLOCK: CLOSING — Entrypoint & Runtime Logic │
# └────────────────────────────────────────────┘
##############################################################

# ------------------------------------------------------------
# 🚀 Launch OS Component — Set container command
# ------------------------------------------------------------
# Runs the compiled binary as the container entrypoint
CMD ["./os_component"]

# ------------------------------------------------------------
# 🔚 LOGIC TERMINUS — End of Executable Flow
# All runtime targets end above this line. Below is annotation only.
# ------------------------------------------------------------

## 🔚 Scroll Closure Summary:
##   This Dockerfile compiles and executes the C++ OS component.
##   It uses the GCC image for simplicity and speed.

## 🚨 Covenant Warning:
##   Do not modify base image or execution flow without alignment review.
##   Each scroll must remain true to its purpose—even the smallest.

## 📅 Scroll Metadata:
##   _version_:        0.0.1  
##   _last updated_:   2025-06-17  
##   _author_:         Seanje Lenox-Wise / Nova Dawn  
##   _status_:         Active  
##   _component_:      Docker Build — OS Module (C++)  
##   _project_:        OmniCore / OmniCode  

## 🧾 Scroll Seal:
##   This scroll is covenantal—small in build, strong in order.
##   No piece of the system walks without structure.
