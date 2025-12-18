# Mission Integration Script
# Integrates completed mission code into the main engine

param(
    [Parameter(Mandatory=$true)]
    [string]$MissionNumber
)

$missionPath = "missions\mission$MissionNumber"
$srcPath = "src"
$testsPath = "tests"

Write-Host "Integrating Mission $MissionNumber into main engine..." -ForegroundColor Green

# Check if mission exists
if (-not (Test-Path $missionPath)) {
    Write-Error "Mission $MissionNumber not found at $missionPath"
    exit 1
}

# Copy source files (exclude lib.rs)
Write-Host "Copying source files..." -ForegroundColor Yellow
Get-ChildItem "$missionPath\src\*.rs" -Exclude "lib.rs" | ForEach-Object {
    Copy-Item $_.FullName "$srcPath\$($_.Name)" -Force
    Write-Host "  ✓ Copied $($_.Name)" -ForegroundColor Gray
}

# Copy test files
if (Test-Path "$missionPath\tests") {
    Write-Host "Copying test files..." -ForegroundColor Yellow
    Get-ChildItem "$missionPath\tests\*.rs" | ForEach-Object {
        Copy-Item $_.FullName "$testsPath\$($_.Name)" -Force
        Write-Host "  ✓ Copied $($_.Name)" -ForegroundColor Gray
    }
    
    # Update imports from mission2_movegen to prawn
    Write-Host "Updating test imports..." -ForegroundColor Yellow
    Get-ChildItem "$testsPath\*.rs" | ForEach-Object {
        $content = Get-Content $_.FullName -Raw
        $updated = $content -replace "mission\d+_\w+::", "prawn::"
        Set-Content $_.FullName -Value $updated -NoNewline
    }
    Write-Host "  ✓ Updated imports to use 'prawn'" -ForegroundColor Gray
}

Write-Host ""
Write-Host "✓ Mission $MissionNumber files copied successfully!" -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Cyan
Write-Host "  1. Update src\lib.rs to add the new modules"
Write-Host "  2. Run: cargo test -- --skip perft_startpos_depth_6 --skip perft_kiwipete_depth_5"
Write-Host "  3. Run: cargo test (full test suite with slow tests)"
Write-Host "  4. Commit the changes"
