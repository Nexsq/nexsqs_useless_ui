@echo off
setlocal enabledelayedexpansion

color f

:getPath
set /p "installPath=[?] Enter installation path: "
if "%installPath%"=="" goto getPath

:getVersion
set /p "version=[?] Enter version [1/2/3]: "
if "%version%"=="1" goto valid
if "%version%"=="2" goto valid
if "%version%"=="3" goto getCustom
echo [!] Invalid version
goto getVersion

:getCustom
set /p "custom=[?] Include plugins for v3.0 [y/n]: "
if /i "%custom%"=="y" goto valid
if /i "%custom%"=="n" goto valid
goto getCustom

:valid
echo.
if not exist "%installPath%" (
    echo [+] Creating directory: %installPath%
    mkdir "%installPath%" 2>nul
    if errorlevel 1 (
        echo [!] Error: Cannot create directory
        pause
        exit /b 1
    )
)

if "%version%"=="1" set "url=https://raw.githubusercontent.com/Nexsq/nexsqs_useless_ui/main/v1.0/nuui_1-0.bat"
if "%version%"=="2" set "url=https://raw.githubusercontent.com/Nexsq/nexsqs_useless_ui/main/v2.0/nuui_2-0.bat"
if "%version%"=="3" set "url=https://raw.githubusercontent.com/Nexsq/nexsqs_useless_ui/main/v3.0/nuui_3-0.exe"

if "%version%"=="1" set "outputFile=nuui_1-0.bat"
if "%version%"=="2" set "outputFile=nuui_2-0.bat"
if "%version%"=="3" set "outputFile=nuui_3-0.exe"

echo [+] Downloading %outputFile% to %installPath%...
echo.

set "psCommand=$url = '%url%';"
set "psCommand=!psCommand! $outputPath = [System.IO.Path]::Combine('%installPath%', '%outputFile%');"
set "psCommand=!psCommand! try {"
set "psCommand=!psCommand!     Invoke-WebRequest -Uri $url -OutFile $outputPath -UseBasicParsing;"
set "psCommand=!psCommand!     if (Test-Path $outputPath) {"
set "psCommand=!psCommand!         Write-Host '[+] Main download successful' -ForegroundColor DarkGreen;"
set "psCommand=!psCommand!         Write-Host '[+] File saved to: ' $outputPath -ForegroundColor DarkGreen;"
set "psCommand=!psCommand!     } else {"
set "psCommand=!psCommand!         Write-Host '[!] Main download failed: File not found after download' -ForegroundColor Red;"
set "psCommand=!psCommand!     }"
set "psCommand=!psCommand! } catch {"
set "psCommand=!psCommand!     Write-Host '[!] Main download failed: ' $_.Exception.Message;"
set "psCommand=!psCommand! }"

if "%version%"=="3" (
    set "psCommand=!psCommand! Write-Host '';"
    if /i "%custom%"=="y" (
        set "psCommand=!psCommand! $configDir = [System.IO.Path]::Combine('%installPath%', 'NUUI_config');"
        set "psCommand=!psCommand! if (-not (Test-Path $configDir)) {"
        set "psCommand=!psCommand!     New-Item -ItemType Directory -Path $configDir | Out-Null;"
        set "psCommand=!psCommand!     Write-Host '[+] Created config directory: ' $configDir;"
        set "psCommand=!psCommand! }"
        set "psCommand=!psCommand! attrib +h $configDir;"
        set "psCommand=!psCommand! Write-Host '[+] Config directory hidden successfully';"
        
        set "psCommand=!psCommand! $settingsFile = [System.IO.Path]::Combine($configDir, 'settings.toml');"

        set "psCommand=!psCommand! Set-Content -Path $settingsFile -Value 'color = \"grey\"';"
        set "psCommand=!psCommand! Add-Content -Path $settingsFile -Value 'dark_theme = false';"
        set "psCommand=!psCommand! Add-Content -Path $settingsFile -Value 'ping_delay = 500';"
        set "psCommand=!psCommand! Add-Content -Path $settingsFile -Value 'port_scan_timeout = 500';"
        set "psCommand=!psCommand! Add-Content -Path $settingsFile -Value 'micro_macro_hotkey = \"None\"';"
        set "psCommand=!psCommand! Add-Content -Path $settingsFile -Value 'micro_macro_key = \"F15\"';"
        set "psCommand=!psCommand! Add-Content -Path $settingsFile -Value 'micro_macro_delay = 30000';"
        set "psCommand=!psCommand! Add-Content -Path $settingsFile -Value 'macro_hotkey = \"None\"';"
        set "psCommand=!psCommand! Add-Content -Path $settingsFile -Value 'macro_restart_when_pausing = false';"
        set "psCommand=!psCommand! Add-Content -Path $settingsFile -Value 'macro_loop = true';"
        set "psCommand=!psCommand! Add-Content -Path $settingsFile -Value 'tetris_use_colors = false';"
        set "psCommand=!psCommand! Add-Content -Path $settingsFile -Value 'tetris_show_ghost = true';"
        set "psCommand=!psCommand! Add-Content -Path $settingsFile -Value 'tetris_speed_multiplier = 1.0';"
        set "psCommand=!psCommand! Add-Content -Path $settingsFile -Value 'game_of_life_simulate_delay = 200';"
        set "psCommand=!psCommand! Add-Content -Path $settingsFile -Value 'game_of_life_save_input = false';"
        set "psCommand=!psCommand! Add-Content -Path $settingsFile -Value 'game_of_life_show_generation = true';"
        set "psCommand=!psCommand! Add-Content -Path $settingsFile -Value 'hide_help = false';"
        set "psCommand=!psCommand! Add-Content -Path $settingsFile -Value 'show_config_files = false';"
        set "psCommand=!psCommand! Add-Content -Path $settingsFile -Value 'show_clock = true';"
        set "psCommand=!psCommand! Add-Content -Path $settingsFile -Value 'show_size = false';"
        set "psCommand=!psCommand! Add-Content -Path $settingsFile -Value 'options = [\"ping_tool\", \"port_scan\", \"micro_macro\", \"macro\", \"tetris\", \"game_of_life\", \"custom\\cheat_eng\", \"custom\\gen_ip\", \"custom\\mouse_crds\", \"custom\\whos_on\", \"custom\\win_cleanup\", \"custom\\x_counter\", \"custom\\yt_dwn\", \"custom\\yt_dwn_dir\\yt_dwnlds\"]';"

        set "psCommand=!psCommand! $customDir = [System.IO.Path]::Combine('%installPath%', 'custom');"
        set "psCommand=!psCommand! if (-not (Test-Path $customDir)) { New-Item -ItemType Directory -Path $customDir | Out-Null };"
        set "psCommand=!psCommand! Write-Host '';"
        set "psCommand=!psCommand! Write-Host '[+] Downloading custom plugins' -ForegroundColor DarkYellow;"
        set "psCommand=!psCommand! Write-Host '';"
        set "psCommand=!psCommand! $apiUrl = 'https://api.github.com/repos/Nexsq/nexsqs_useless_ui/contents/custom';"
        set "psCommand=!psCommand! try {"
        set "psCommand=!psCommand!     $items = Invoke-RestMethod -Uri $apiUrl -UseBasicParsing;"
        set "psCommand=!psCommand!     foreach ($item in $items) {"
        set "psCommand=!psCommand!         if ($item.type -eq 'file') {"
        set "psCommand=!psCommand!             $fileUrl = $item.download_url;"
        set "psCommand=!psCommand!             $filePath = [System.IO.Path]::Combine($customDir, $item.name);"
        set "psCommand=!psCommand!             Invoke-WebRequest -Uri $fileUrl -OutFile $filePath -UseBasicParsing;"
        set "psCommand=!psCommand!             Write-Host '[+] Downloaded: ' $item.name;"
        set "psCommand=!psCommand!         }"
        set "psCommand=!psCommand!     }"
        set "psCommand=!psCommand!     Write-Host '';"
        set "psCommand=!psCommand!     Write-Host '[+] Custom plugins download completed' -ForegroundColor DarkGreen;"
        set "psCommand=!psCommand! } catch {"
        set "psCommand=!psCommand!     Write-Host '';"
        set "psCommand=!psCommand!     Write-Host '[+] Custom plugins download failed: ' $_.Exception.Message;"
        set "psCommand=!psCommand! }"
    )
)

powershell -noprofile -executionpolicy bypass -command "!psCommand!"

echo.
pause
endlocal