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

echo [1/2] Applying CE patches (17 total)...
set PATCH_DIR=patches
set FAILED=0

for %%f in (
    "01-remove-telemetry.patch"
    "02-web-search.patch"
    "03-community-docs.patch"
    "04-add-glob-tool.patch"
    "05-add-calculator-tool.patch"
    "06-add-json-csv-tools.patch"
    "07-add-text-codec-tools.patch"
    "08-community-branding.patch"
    "09-optimize-system-prompt.patch"
    "10-community-tool-docs.patch"
    "11-ci-cd-build.patch"
    "12-install-scripts.patch"
    "13-clean-dead-code.patch"
    "14-inject-tools-to-toolsets.patch"
    "15-binary-name-grokce.patch"
    "16-websearch-zero-config.patch"
    "17-ci-fixes.patch"
) do (
    echo   %%f...
    git apply --3way "%PATCH_DIR%\%%f" 2>nul
    if !errorlevel! neq 0 (
        echo     [FAILED] Fix the conflict, then update this patch
        set FAILED=1
        goto :done
    )
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
