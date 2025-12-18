#!/usr/bin/env pwsh
<#
.SYNOPSIS
    AI-powered commit and push to GitHub
.DESCRIPTION
    This script stages all changes, generates an AI commit message, and pushes to GitHub
.PARAMETER Message
    Optional manual commit message (if not provided, AI will generate one)
.PARAMETER Push
    Whether to push after committing (default: true)
.PARAMETER Auto
    Skip confirmation and auto-commit with AI message
.EXAMPLE
    .\quick-commit.ps1
    (AI generates commit message, asks for confirmation)
.EXAMPLE
    .\quick-commit.ps1 -Auto
    (AI generates and commits without asking)
.EXAMPLE
    .\quick-commit.ps1 "Manual commit message"
    (Use manual message instead of AI)
#>

param(
    [Parameter(Mandatory=$false, Position=0)]
    [string]$Message = "",
    
    [Parameter(Mandatory=$false)]
    [bool]$Push = $true,
    
    [Parameter(Mandatory=$false)]
    [switch]$Auto
)

# Change to repository root
$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoRoot = Split-Path -Parent $scriptPath
Set-Location $repoRoot

# Stage all changes
Write-Host "📝 Staging changes..." -ForegroundColor Cyan
git add -A

# Check if there are changes to commit
$status = git status --porcelain
if ([string]::IsNullOrWhiteSpace($status)) {
    Write-Host "⚠️  No changes to commit!" -ForegroundColor Yellow
    exit 0
}

# Generate or use provided commit message
if ([string]::IsNullOrWhiteSpace($Message)) {
    Write-Host "🤖 Generating AI commit message..." -ForegroundColor Cyan
    
    # Get the diff
    $diff = git diff --cached
    
    # Check if GitHub Copilot CLI is available
    $ghCopilot = Get-Command "gh" -ErrorAction SilentlyContinue
    
    if ($ghCopilot -and (gh copilot --version 2>$null)) {
        # Use GitHub Copilot CLI
        $Message = gh copilot suggest -t git "Generate a concise commit message for these changes: $diff" | Select-Object -Last 1
    } else {
        # Fallback: Generate based on changed files
        $changedFiles = git diff --cached --name-only
        $fileList = $changedFiles -join ", "
        
        if ($changedFiles.Count -eq 1) {
            $Message = "Update $($changedFiles[0])"
        } elseif ($changedFiles.Count -le 3) {
            $Message = "Update $fileList"
        } else {
            $extensions = $changedFiles | ForEach-Object { [System.IO.Path]::GetExtension($_) } | Select-Object -Unique
            if ($extensions -contains ".rs") {
                $Message = "Update Rust code ($($changedFiles.Count) files)"
            } elseif ($extensions -contains ".md") {
                $Message = "Update documentation ($($changedFiles.Count) files)"
            } else {
                $Message = "Update $($changedFiles.Count) files"
            }
        }
    }
    
    Write-Host ""
    Write-Host "📋 Generated commit message:" -ForegroundColor Green
    Write-Host "   $Message" -ForegroundColor White
    Write-Host ""
    
    if (-not $Auto) {
        $response = Read-Host "Use this message? (Y/n/edit)"
        if ($response -eq "n") {
            Write-Host "❌ Commit cancelled" -ForegroundColor Yellow
            git reset HEAD .
            exit 0
        } elseif ($response -eq "edit") {
            $Message = Read-Host "Enter commit message"
        }
    }
}

Write-Host "💾 Committing..." -ForegroundColor Cyan
git commit -m $Message

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Commit failed!" -ForegroundColor Red
    exit 1
}

if ($Push) {
    Write-Host "🚀 Pushing to GitHub..." -ForegroundColor Cyan
    $branch = git branch --show-current
    git push origin $branch
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Successfully pushed to GitHub!" -ForegroundColor Green
    } else {
        Write-Host "❌ Push failed!" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "✅ Committed successfully (not pushed)" -ForegroundColor Green
}
