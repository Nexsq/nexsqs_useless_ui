@echo off
setlocal enabledelayedexpansion
set "BASE_DIR=%~dp0yt_dwn_dir"
set "YTDLP=%BASE_DIR%\yt-dlp.exe"
set "DOWNLOAD_DIR=%BASE_DIR%\yt_dwnlds"
if not exist "%BASE_DIR%" (
    mkdir "%BASE_DIR%"
    attrib +h "%BASE_DIR%"
) else (
    attrib +h "%BASE_DIR%" >nul 2>&1
)
if not exist "%DOWNLOAD_DIR%" mkdir "%DOWNLOAD_DIR%"
if not exist "%YTDLP%" (
    echo yt-dlp.exe not found, downloading . . .
    powershell -Command "Invoke-WebRequest -Uri 'https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe' -OutFile '%YTDLP%'"
    if exist "%YTDLP%" (
        echo download complete
    ) else (
        echo failed to download yt-dlp, check your internet connection
        pause
        exit
    )
)
set MODE=mp4
set PLMODE=--no-playlist
:loop
cls
echo input a YouTube URL
echo or:
echo   [mp3] to switch to MP3 mode
echo   [mp4] to switch to MP4 mode (default)
echo   [playlist] to enable full playlist mode
echo   [noplaylist] to disable playlists (default)
echo   [q] to quit
echo.
echo format: !MODE! / playlist mode: !PLMODE!
echo.
set /p url=input: 
if /i "%url%"=="q" exit
if /i "%url%"=="mp3" (
    set MODE=mp3
    echo switched to MP3 mode
    timeout /t 1 >nul
    goto loop
)
if /i "%url%"=="mp4" (
    set MODE=mp4
    echo switched to MP4 mode
    timeout /t 1 >nul
    goto loop
)
if /i "%url%"=="playlist" (
    set PLMODE=--yes-playlist
    echo playlist mode enabled
    timeout /t 1 >nul
    goto loop
)
if /i "%url%"=="noplaylist" (
    set PLMODE=--no-playlist
    echo playlist mode disabled
    timeout /t 1 >nul
    goto loop
)
echo.
echo downloading to: %DOWNLOAD_DIR%
echo.
if /i "!MODE!"=="mp3" (
    "%YTDLP%" "%url%" !PLMODE! -x --audio-format mp3 -o "%DOWNLOAD_DIR%\%%(title)s.%%(ext)s"
) else (
    "%YTDLP%" "%url%" !PLMODE! -f "bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]" -o "%DOWNLOAD_DIR%\%%(title)s.%%(ext)s"
)
if errorlevel 1 (
    echo.
    echo something went wrong, try again
    timeout /t 3 >nul
    goto loop
)

echo.
echo done, file saved in: %DOWNLOAD_DIR%
echo.
pause
goto loop