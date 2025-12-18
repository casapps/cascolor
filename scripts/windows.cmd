@echo off
REM Windows batch installer for cascolor

echo Installing cascolor for Windows...
echo.
echo This script requires PowerShell.
echo Running PowerShell installer...
echo.

powershell -ExecutionPolicy Bypass -File "%~dp0windows.ps1"

if %ERRORLEVEL% EQU 0 (
    echo.
    echo Installation complete!
) else (
    echo.
    echo Installation failed!
    exit /b 1
)
