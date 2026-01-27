@echo off
setlocal enabledelayedexpansion

:: Unified build and distribute script for taskflow (Windows Batch)
:: Usage: 
::   build.bat          - Build for current host only
::   build.bat --all    - Build for all supported platforms
::   build.bat <target> - Build for a specific target (limited support in batch)

set BUILD_TARGET=%1

:: Detect host info
set HOST_OS=windows
set HOST_ARCH=amd64
if "%PROCESSOR_ARCHITECTURE%"=="ARM64" set HOST_ARCH=arm64

:: Move to project root
cd ..

if "%BUILD_TARGET%"=="--all" (
    echo --- Building all platforms ---
    rmdir /s /q dist 2>nul
    call :build_and_package macos_arm64 aarch64-apple-darwin false
    call :build_and_package macos_amd64 x86_64-apple-darwin false
    call :build_and_package linux_amd64 x86_64-unknown-linux-gnu false
    call :build_and_package linux_arm64 aarch64-unknown-linux-musl false
    call :build_and_package windows_amd64 x86_64-pc-windows-gnu true
    echo ✅ Build and distribution complete!
    goto :eof
)

if "%BUILD_TARGET%"=="" (
    echo --- Building for current host (%HOST_OS%_%HOST_ARCH%) ---
    rmdir /s /q dist 2>nul
    cargo build --release
    if errorlevel 1 goto :error
    
    set BIN_EXT=
    set IS_WIN=false
    if "%HOST_OS%"=="windows" (
        set BIN_EXT=.exe
        set IS_WIN=true
    )
    
    call :package %HOST_OS%_%HOST_ARCH% target\release\taskflow!BIN_EXT! !IS_WIN!
    echo ✅ Build and distribution complete!
    goto :eof
)

:: Handle individual targets
if "%BUILD_TARGET%"=="macos_arm64" (
    call :build_and_package macos_arm64 aarch64-apple-darwin false
) else if "%BUILD_TARGET%"=="macos_amd64" (
    call :build_and_package macos_amd64 x86_64-apple-darwin false
) else if "%BUILD_TARGET%"=="linux_amd64" (
    call :build_and_package linux_amd64 x86_64-unknown-linux-gnu false
) else if "%BUILD_TARGET%"=="linux_arm64" (
    call :build_and_package linux_arm64 aarch64-unknown-linux-musl false
) else if "%BUILD_TARGET%"=="windows_amd64" (
    call :build_and_package windows_amd64 x86_64-pc-windows-gnu true
) else (
    echo ❌ Unknown build target: %BUILD_TARGET%
    echo Usage: build.bat [--all | macos_arm64 | macos_amd64 | linux_amd64 | linux_arm64 | windows_amd64]
    exit /b 1
)

echo ✅ Build and distribution complete!
goto :eof

:build_and_package
set PLATFORM_NAME=%1
set TARGET_TRIPLE=%2
set IS_WIN=%3

echo 🚀 Building for %PLATFORM_NAME% (%TARGET_TRIPLE%)...
cargo build --release --target %TARGET_TRIPLE%
if errorlevel 1 goto :error

set BIN_NAME=taskflow
if "%IS_WIN%"=="true" set BIN_NAME=taskflow.exe

call :package %PLATFORM_NAME% target\%TARGET_TRIPLE%\release\%BIN_NAME% %IS_WIN%
goto :eof

:package
set PLATFORM_DIR=%1
set BIN_SRC=%2
set IS_WINDOWS=%3

set DIST_PATH=dist\%PLATFORM_DIR%\taskflow
echo 📦 Packaging for %PLATFORM_DIR% -^> %DIST_PATH%

mkdir %DIST_PATH%\bin 2>nul

if "%IS_WINDOWS%"=="true" (
    copy %BIN_SRC% %DIST_PATH%\bin\taskflow.exe >nul
) else (
    copy %BIN_SRC% %DIST_PATH%\bin\taskflow >nul
)

copy assets\SKILL.md %DIST_PATH%\SKILL.md >nul
goto :eof

:error
echo ❌ Build failed!
exit /b 1
