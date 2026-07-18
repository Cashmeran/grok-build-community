# Grok Build Community Edition - Windows Installer
# Run: irm https://raw.githubusercontent.com/Cashmeran/grok-build-community/main/install.ps1 | iex

$ErrorActionPreference = "Stop"
$Repo = "Cashmeran/grok-build-community"
$InstallDir = "$env:USERPROFILE\.grok\bin"

Write-Host "Grok Build Community Edition Installer" -ForegroundColor Cyan
Write-Host "====================================="
Write-Host ""

# Create install directory
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

# Get latest release
Write-Host "Fetching latest release..."
try {
    $release = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest"
} catch {
    Write-Host "GitHub API rate limited. Try again later or set GITHUB_TOKEN env var." -ForegroundColor Red
    Write-Host "Manual download: https://github.com/$Repo/releases/latest"
    exit 1
}

# Find Windows asset
$asset = $release.assets | Where-Object { $_.name -match "windows|x86_64-pc-windows" } | Select-Object -First 1
if (-not $asset) {
    Write-Host "No Windows binary found in latest release." -ForegroundColor Red
    exit 1
}

Write-Host "Downloading $($asset.name) ($([math]::Round($asset.size/1MB, 1)) MB)..."
$outPath = Join-Path $InstallDir "grokce.exe"
Invoke-WebRequest -Uri $asset.browser_download_url -OutFile $outPath

Write-Host "Installed to $outPath" -ForegroundColor Green

# Add to PATH if needed
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$InstallDir*") {
    Write-Host ""
    Write-Host "Adding $InstallDir to your PATH..."
    [Environment]::SetEnvironmentVariable("Path", "$userPath;$InstallDir", "User")
    $env:Path = "$env:Path;$InstallDir"
    Write-Host "Done. Restart your terminal for PATH to take effect." -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Grok Build Community Edition installed!" -ForegroundColor Green
Write-Host "Run 'grokce' to start."
