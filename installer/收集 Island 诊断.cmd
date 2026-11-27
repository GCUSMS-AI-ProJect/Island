@echo off
rem Portable package: the diagnostics script is embedded in message-island.exe.
start "" "%~dp0message-island.exe" --collect-diagnostics
