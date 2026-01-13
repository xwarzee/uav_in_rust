@echo off
REM Script to run FitNesse with UAV Swarm API tests (Windows)

setlocal

set FITNESSE_VERSION=20251025
set FITNESSE_JAR=fitnesse-%FITNESSE_VERSION%-standalone.jar
set FITNESSE_PORT=8000

if not defined FITNESSE_PORT set FITNESSE_PORT=8000

echo =========================================
echo UAV Swarm API - FitNesse Test Runner
echo =========================================
echo.

REM Check if FitNesse JAR exists
if not exist "%FITNESSE_JAR%" (
    echo FitNesse JAR not found. Please download it from:
    echo https://github.com/unclebob/fitnesse/releases/download/v%FITNESSE_VERSION%/fitnesse-%FITNESSE_VERSION%-standalone.jar
    echo.
    echo Place it in the fitnesse directory.
    pause
    exit /b 1
)

REM Check if fixtures are built
if not exist "fixtures\target\fitnesse-fixtures-1.0.0-jar-with-dependencies.jar" (
    echo Fixtures not built. Building...
    cd fixtures
    call mvn clean package
    cd ..
    echo Build complete.
    echo.
)

echo Starting FitNesse on port %FITNESSE_PORT%...
echo Access the wiki at: http://localhost:%FITNESSE_PORT%/UavSwarmApi
echo.
echo Press Ctrl+C to stop FitNesse
echo.

java -jar "%FITNESSE_JAR%" -p %FITNESSE_PORT%

endlocal
