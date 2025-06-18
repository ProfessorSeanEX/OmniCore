# --- 🌿 Load environment variables from .env file ---
$envFile = ".env"
if (Test-Path $envFile) {
    Get-Content $envFile | ForEach-Object {
        if ($_ -match '^\s*#') { return }          # Skip comments
        if ($_ -match '^\s*$') { return }          # Skip blank lines

        $parts = $_ -split '=', 2
        if ($parts.Count -eq 2) {
            $key = $parts[0].Trim()
            $val = $parts[1].Trim().Trim('"')      # Strip quotes if present
            Set-Item -Path "env:$key" -Value $val
        }
    }
    Write-Host "📦 Loaded environment variables from .env" -ForegroundColor Gray
} else {
    Write-Host "⚠️ No .env file found. Continuing without injected env vars." -ForegroundColor Yellow
}

# --- 🧹 Set a custom temporary directory to avoid file locking issues ---
$customTemp = "$PSScriptRoot\.docker_temp"
if (-Not (Test-Path $customTemp)) {
    New-Item -ItemType Directory -Path $customTemp | Out-Null
}
$env:TEMP = $customTemp
$env:TMP = $customTemp
Write-Host "🧽 Using custom temp directory: $customTemp" -ForegroundColor Gray

# --- ⚙ Enable Docker BuildKit ---
$env:DOCKER_BUILDKIT = "1"
Write-Host "🛠️ BuildKit enabled." -ForegroundColor Gray

# --- 🧼 Optional: clean out old compose temp files before build ---
Get-ChildItem -Path $customTemp -Filter "compose*" -ErrorAction SilentlyContinue | Remove-Item -Force -ErrorAction SilentlyContinue
Get-ChildItem -Path $customTemp -Filter ".tmp-compose*" -ErrorAction SilentlyContinue | Remove-Item -Force -ErrorAction SilentlyContinue

# --- 🏗 Begin Docker Compose Build ---
Write-Host "`n🚀 Starting Docker Compose build for rust-service..." -ForegroundColor Cyan
docker compose build rust-service

# --- ✅ Completion Status ---
if ($LASTEXITCODE -eq 0) {
    Write-Host "`n✅ Build completed successfully." -ForegroundColor Green
} else {
    Write-Host "`n❌ Build failed. Check logs for errors." -ForegroundColor Red
    exit $LASTEXITCODE
}
