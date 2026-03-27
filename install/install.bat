@echo off
REM AutoHarness One-Click Installer for Windows
REM Usage: install.bat [install|uninstall]

setlocal enabledelayedexpansion

set NAME=autoharness
set VERSION=0.1.0
set INSTALL_DIR=%USERPROFILE%\.local\bin

if "%1"=="" goto install
if "%1"=="install" goto install
if "%1"=="uninstall" goto uninstall
if "%1"=="--help" goto help
goto help

:install
echo [INFO] Installing %NAME%...

REM Create install directory
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"

REM Copy binary
copy /Y "%~dp0%NAME%-windows-x86_64.exe" "%INSTALL_DIR%\%NAME%.exe" >nul

if errorlevel 1 (
    echo [ERROR] Binary not found: %NAME%-windows-x86_64.exe
    exit /b 1
)

echo [INFO] Installed: %INSTALL_DIR%\%NAME%.exe
echo [INFO] Add %INSTALL_DIR% to your PATH
echo.
echo To add to PATH, run:
echo   setx PATH "%INSTALL_DIR%;%PATH%"
goto done

:uninstall
if exist "%INSTALL_DIR%\%NAME%.exe" (
    del /f /q "%INSTALL_DIR%\%NAME%.exe"
    echo [INFO] Uninstalled: %INSTALL_DIR%\%NAME%.exe
) else (
    echo [WARN] %NAME% is not installed.
)
goto done

:help
echo AutoHarness One-Click Installer v%VERSION%
echo.
echo Usage:
echo   install.bat           Install
echo   install.bat uninstall  Uninstall
echo   install.bat --help     Show help

:done
endlocal