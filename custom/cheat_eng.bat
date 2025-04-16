@echo off
:findSettings
for /f "delims=" %%f in ('dir /s /b /a "settings.toml" 2^>nul') do (
	set "settings_file=%%f"
)
if not defined settings_file (
	cd .. 
	goto findSettings
)
:loop
echo settings.toml file found in: %settings_file%
echo.
setlocal enabledelayedexpansion
set index=1
for /f "tokens=1,* delims==" %%a in ('findstr "=" "%settings_file%"') do (
	set "line=%%a=%%b"
	echo !index!. !line!
	set /a index+=1
)
echo.
set choice=1
set /p choice="index or q to quit: "
if /i "%choice%"=="q" exit
set index=1
for /f "tokens=1,* delims==" %%a in ('findstr "=" "%settings_file%"') do (
	if !index! equ %choice% (
		set "setting=%%a"
		set "current_value=%%b"
	)
	set /a index+=1
)
if not defined setting (
	echo invalid choice
	pause
	exit
)
echo selected: %setting%=%current_value%
echo.
set /p new_value="%setting%= "
if defined new_value (
	echo updating %setting% to %new_value% . . .
	(
		for /f "tokens=1,* delims==" %%a in ('findstr "=" "%settings_file%"') do (
			if "%%a"=="%setting%" (
				echo %setting%= %new_value%
			) else (
				echo %%a=%%b
			)
		)
	) > "%settings_file%.new"
	move /y "%settings_file%.new" "%settings_file%"
	echo done
) else (
	echo no change in %setting%
)
pause
cls
goto loop