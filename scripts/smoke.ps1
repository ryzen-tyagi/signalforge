$ErrorActionPreference = "Stop"

Write-Host "Checking Rust workspace metadata..."
cargo metadata --no-deps | Out-Null

Write-Host "Checking web package metadata..."
npm --workspace @signalforge/web pkg get name | Out-Null

Write-Host "Smoke checks completed."

