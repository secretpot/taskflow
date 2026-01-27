@echo off
setlocal

:: Unified build and distribute script for taskflow (Windows Batch)
:: Usage: 
::   build.bat          - Build for current host only
::   build.bat --all    - Build for all (currently limited to host logic, can be expanded)

set BUILD_ALL=false
if "%1"=="--all" set BUILD_ALL=true

:: Simple architecture detection (common for Windows)
set HOST_OS=windows
set HOST_ARCH=amd64
if "%PROCESSOR_ARCHITECTURE%"=="ARM64" set HOST_ARCH=arm64

echo --- Building for current host (%HOST_OS%_%HOST_ARCH%) ---

cd ..
cargo build --release

if errorlevel 1 goto :error

:: Create dist structure
set DIST_DIR=dist\%HOST_OS%_%HOST_ARCH%\taskflow
if exist dist rmdir /s /q dist
mkdir %DIST_DIR%\bin

:: Copy assets
copy target\release\taskflow.exe %DIST_DIR%\bin\taskflow.exe
copy assets\SKILL.md %DIST_DIR%\SKILL.md

echo ✅ Build and distribution complete!
goto :eof

:error
echo ❌ Build failed!
exit /b 1
