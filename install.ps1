# install.ps1 - Install rustlink locally on Windows
# Usage: .\install.ps1

$ErrorActionPreference = "Stop"

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "  RustLink Local Installation" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan

# Check if Rust is installed
$rustCheck = Get-Command cargo -ErrorAction SilentlyContinue
if (-not $rustCheck) {
    Write-Host "❌ Rust is not installed." -ForegroundColor Red
    Write-Host ""
    Write-Host "Install Rust with:" -ForegroundColor Yellow
    Write-Host "  winget install Rustlang.Rust.MSVC" -ForegroundColor White
    exit 1
}

Write-Host "✓ Rust found: $(cargo --version)" -ForegroundColor Green

# Get script directory
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $ScriptDir

# Build release
Write-Host ""
Write-Host "Building rustlink..." -ForegroundColor Yellow
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Build failed" -ForegroundColor Red
    exit 1
}

# Find binary location
$BinaryPath = Join-Path $ScriptDir "target\release\rustlink.exe"

# Check if binary exists (also check without .exe for WSL compat)
if (-not (Test-Path $BinaryPath)) {
    $BinaryPathNoExt = Join-Path $ScriptDir "target\release\rustlink"
    if (Test-Path $BinaryPathNoExt) {
        $BinaryPath = $BinaryPathNoExt
    } else {
        Write-Host "❌ Build failed - binary not found" -ForegroundColor Red
        exit 1
    }
}

# Add to user PATH if needed
$InstallDir = "$env:USERPROFILE\.local\bin"
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

$LinkPath = Join-Path $InstallDir "rustlink.exe"
if (-not (Test-Path $LinkPath)) {
    Copy-Item $BinaryPath $LinkPath -Force
    Write-Host "✓ Copied to $LinkPath" -ForegroundColor Green

    # Check if already in PATH
    $pathEntries = $env:Path -split ";"
    if ($InstallDir -notin $pathEntries) {
        Write-Host ""
        Write-Host "Add to your PATH:" -ForegroundColor Yellow
        Write-Host "  \$env:Path += ';$InstallDir'" -ForegroundColor White
        Write-Host "Or via System Properties > Environment Variables" -ForegroundColor White
    }
} else {
    Write-Host "✓ Binary already exists at $LinkPath" -ForegroundColor Green
}

Write-Host ""
Write-Host "✅ Installation complete!" -ForegroundColor Green
Write-Host ""
Write-Host "Usage:" -ForegroundColor Cyan
Write-Host "  rustlink init <username>   # Create identity"
Write-Host "  rustlink status            # Check status"
Write-Host "  rustlink run               # Start P2P node"
