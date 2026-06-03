@echo off
setlocal enabledelayedexpansion

set "PROJECT_DIR=%~dp0"
cd /d "%PROJECT_DIR%"

echo [BUILD] Compiling mender...
call rustup run nightly cargo build -Z json-target-spec --release --target-dir "%PROJECT_DIR%target" --manifest-path "%PROJECT_DIR%mender\Cargo.toml" --bin mender

if %ERRORLEVEL% neq 0 (
    echo [ERROR] Build failed with exit code %ERRORLEVEL%
    exit /b %ERRORLEVEL%
)

copy /Y "%PROJECT_DIR%target\release\mender.exe" "%PROJECT_DIR%mender-tool.exe" >nul

if %ERRORLEVEL% neq 0 (
    echo [ERROR] Failed to copy mender-tool.exe to the project root.
    exit /b %ERRORLEVEL%
)

echo [BUILD] Output created at "%PROJECT_DIR%mender-tool.exe"
