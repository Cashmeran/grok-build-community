@echo off
chcp 65001 >nul
setlocal enabledelayedexpansion

echo.
echo ============================================
echo   Grok Build Community Edition - Build
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

echo [1/2] Applying community patches...
set PATCH_DIR=patches
set FAILED=0

REM Apply patches in order. Add new patches here as they are created.
git apply "%PATCH_DIR%\00-community-foundation.patch" 2>nul
if !errorlevel! neq 0 (
    echo   [FAILED] 00-community-foundation.patch - fix conflicts and retry
    set FAILED=1
    goto :done
)
git apply "%PATCH_DIR%\01-community-ci.patch" 2>nul
if !errorlevel! neq 0 (
    echo   [FAILED] 01-community-ci.patch - fix conflicts and retry
    set FAILED=1
    goto :done
)
git apply "%PATCH_DIR%\02-community-prompt.patch" 2>nul
if !errorlevel! neq 0 (
    echo   [FAILED] 02-community-prompt.patch - fix conflicts and retry
    set FAILED=1
    goto :done
)
git apply "%PATCH_DIR%\03-community-tools.patch" 2>nul
if !errorlevel! neq 0 (
    echo   [FAILED] 03-community-tools.patch - fix conflicts and retry
    set FAILED=1
    goto :done
)
git apply "%PATCH_DIR%\04-community-i18n.patch" 2>nul
if !errorlevel! neq 0 (
    echo   [FAILED] 04-community-i18n.patch - fix conflicts and retry
    set FAILED=1
    goto :done
)

:done
if %FAILED% equ 1 (
    echo.
    echo Fix conflicts: manually edit code ^> git add ^> regenerate patch
    exit /b 1
)

echo.
echo [2/2] Building release...
cargo build -p xai-grok-pager-bin --release
if %errorlevel% neq 0 ( echo [ERROR] Build failed! & exit /b 1 )

echo.
echo ============================================
echo   Done! Binary: target\release\xai-grok-pager.exe
echo   Run: grokce
echo ============================================
