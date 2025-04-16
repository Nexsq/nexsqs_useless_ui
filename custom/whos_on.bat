@echo off
setlocal enabledelayedexpansion
echo fetching connected users . . .
echo.
set /a count=0
for /f "tokens=1 delims= " %%i in ('arp -a ^| findstr /v "Interface" ^| findstr /v "255.255.255.255" ^| findstr /v "224.0.0"') do (
	call :checkIP %%i
)
echo.
echo total connected users: !count!
pause
exit
:checkIP
	ping -n 1 %1 >nul
	if %errorlevel%==0 (
		set /a count+=1
		for /f "tokens=2 delims=:" %%b in ('nslookup %1 2^>nul ^| findstr /i "Name"') do (
			echo [online] %1 %%b
			goto :eof
		)
		echo [online] %1 *
	)
	goto :eof