#!/bin/bash

# ==========================================
# 📜 Watchtower Log Test Script (Bash)
# ==========================================
# Runs the Watchtower binary inside Docker and prints outputs
# Author: Seanje Lenox-Wise / Nova Dawn
# Phase: Phase 0 — Validation Loop

set -e

echo "📡 [Watchtower Test] Running Watchtower monitoring log..."

# 🐳 Execute compiled binary directly (not via cargo)
docker-compose exec rust-service ./omnicode

# 📜 Human-readable scroll output
LOG_FILE="./logs/watchtower.log"
JSON_FILE="./logs/watchtower.json"

echo ""
if [[ -f "$LOG_FILE" ]]; then
  echo "📜 [Log Output] watchtower.log:"
  cat "$LOG_FILE"
else
  echo "⚠️  No scroll log found at $LOG_FILE"
fi

echo ""
if [[ -f "$JSON_FILE" ]]; then
  echo "📦 [JSON Output] watchtower.json:"
  jq '.' "$JSON_FILE" 2>/dev/null || cat "$JSON_FILE"
else
  echo "⚠️  No JSON file found at $JSON_FILE"
fi

echo ""
echo "✅ [Test Complete] Watchtower logging sequence finished."
