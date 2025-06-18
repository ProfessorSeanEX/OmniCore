# ==========================================
# 📜 Watchtower Log Test Script (PowerShell)
# ==========================================
# Runs the Watchtower logger inside Docker and prints outputs
# Author: Seanje Lenox-Wise / Nova Dawn
# Phase: Phase 0 — Validation Loop

Write-Host "📡 [Watchtower Test] Running Watchtower monitoring log..."

# 🐳 Run the binary inside the rust-service container
docker-compose exec rust-service ./omnicode

# 📝 Log Output — Scroll Format
$logPath = ".\logs\watchtower.log"
$jsonPath = ".\logs\watchtower.json"

if (Test-Path $logPath) {
    Write-Host "`n📜 [Log Output] watchtower.log:`n"
    Get-Content $logPath
} else {
    Write-Warning "⚠️ Log file not found at $logPath"
}

# 📦 Log Output — JSON Format
if (Test-Path $jsonPath) {
    Write-Host "`n📦 [JSON Output] watchtower.json:`n"
    Get-Content $jsonPath | ConvertFrom-Json | ConvertTo-Json -Depth 5
} else {
    Write-Warning "⚠️ JSON file not found at $jsonPath"
}

Write-Host "`n✅ [Test Complete] Watchtower test finished."
