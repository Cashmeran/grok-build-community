@echo off
chcp 65001 >nul
setlocal enabledelayedexpansion

echo.
echo ============================================
echo   Grok Build Community Edition - One-Click Build
echo ============================================
echo.

if not exist "patches\" (
    echo [ERROR] Run from project root directory
    exit /b 1
)

where cargo >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] Rust not found. Install: https://rustup.rs
    exit /b 1
)

echo [1/2] Applying community edition patch...
set PATCH_DIR=patches
set FAILED=0

git apply --3way "%PATCH_DIR%\00-community-full.patch" 2>nul
if !errorlevel! neq 0 (
    echo   [FAILED] Fix the conflict, then update the patch
    set FAILED=1
    goto :done
)

:done
if %FAILED% equ 1 (
    echo.
    echo Fix: manually edit code -^> git diff ^> patches/failed-patch.patch
    exit /b 1
)

echo.
echo [2/2] Building release...
cargo build -p xai-grok-pager-bin --release
if %errorlevel% neq 0 ( echo [ERROR] Build failed! & exit /b 1 )

echo.
echo ============================================
echo   Done! Run: grokce
echo ============================================
