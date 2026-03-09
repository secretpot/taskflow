@echo off
setlocal enabledelayedexpansion

:: Taskflow Installation Script for Windows
:: This script installs taskflow.exe to a folder in users' PATH

echo 🚀 Installing taskflow for Windows...

set BINARY_SRC=%~dp0..\bin\taskflow.exe

if not exist "!BINARY_SRC!" (
    echo ❌ Binary not found at !BINARY_SRC!
    echo Please run 'scripts\build.sh' first.
    exit /b 1
)

:: Option 1: Install to WindowsApps folder (usually in PATH)
set TARGET_DIR=%LOCALAPPDATA%\Microsoft\WindowsApps
set TARGET_PATH=!TARGET_DIR!\taskflow.exe

echo 📦 Copying taskflow.exe to !TARGET_DIR!...
copy /Y "!BINARY_SRC!" "!TARGET_PATH!" >nul

if %ERRORLEVEL% EQU 0 (
    echo ✅ Taskflow installed successfully to !TARGET_PATH!
    echo You can now run 'taskflow' from anywhere!
) else (
    echo ❌ Installation failed. Please ensure you have permissions for !TARGET_DIR!
    exit /b 1
)

endlocal
