@echo off
SETLOCAL EnableDelayedExpansion

echo [%date% %time%] Starting service...

REM Check for Rust installation
where rustc >nul 2>nul
IF !ERRORLEVEL! NEQ 0 (
    echo Error: Rust is not installed
    exit /b 1
)

REM Check if .env exists
IF NOT EXIST .env (
    echo Error: .env file not found
    echo Creating default .env file...
    copy .env.example .env >nul 2>nul
    IF !ERRORLEVEL! NEQ 0 (
        echo Failed to create .env file
        exit /b 1
    )
)

REM Create logs directory
IF NOT EXIST logs mkdir logs

REM Backup existing database
IF EXIST database.sqlite (
    echo Creating database backup...
    copy database.sqlite "logs\backup_%date:~-4,4%%date:~-10,2%%date:~-7,2%.sqlite" >nul
)

REM Build the project
echo Building project...
cargo build --release >> logs\build_%date:~-4,4%%date:~-10,2%%date:~-7,2%.log 2>&1
IF !ERRORLEVEL! NEQ 0 (
    echo Build failed. Check logs for details.
    exit /b 1
)

REM Run the application
echo Running application...
cargo run --release >> logs\run_%date:~-4,4%%date:~-10,2%%date:~-7,2%.log 2>&1

IF !ERRORLEVEL! NEQ 0 (
    echo Application failed. Check logs for details.
    exit /b 1
)

ENDLOCAL