# PowerShell installer for cascolor (Windows)
$ErrorActionPreference = "Stop"

$REPO = "casapps/cascolor"
$BIN_NAME = "cascolor.exe"
$INSTALL_DIR = "$env:USERPROFILE\bin"

Write-Host "Installing cascolor for Windows..." -ForegroundColor Green

# Detect architecture
$ARCH = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "x86" }
$BINARY_NAME = "cascolor-windows-$ARCH.exe"

# Get latest release
$LATEST = (Invoke-RestMethod "https://api.github.com/repos/$REPO/releases/latest").tag_name
$DOWNLOAD_URL = "https://github.com/$REPO/releases/download/$LATEST/$BINARY_NAME"

Write-Host "Downloading version $LATEST..."

# Create install directory if it doesn't exist
if (!(Test-Path $INSTALL_DIR)) {
    New-Item -ItemType Directory -Path $INSTALL_DIR | Out-Null
}

# Download and install
$TMP_FILE = "$env:TEMP\cascolor.exe"
Invoke-WebRequest -Uri $DOWNLOAD_URL -OutFile $TMP_FILE

Move-Item -Path $TMP_FILE -Destination "$INSTALL_DIR\$BIN_NAME" -Force

Write-Host "✓ Installed to $INSTALL_DIR\$BIN_NAME" -ForegroundColor Green
Write-Host ""
Write-Host "Add to PATH: `$env:PATH += `";$INSTALL_DIR`"" -ForegroundColor Yellow
