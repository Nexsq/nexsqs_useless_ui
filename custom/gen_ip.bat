@echo off
setlocal enabledelayedexpansion
:loop
cls
for /L %%i in (1,1,4) do (
    set /a octet%%i=!random! %% 256
)
echo random IP: !octet1!.!octet2!.!octet3!.!octet4!
echo.
set /p "user=[Ent] for random IP | [q] to quit: "
if /i "!user!"=="q" exit
goto loop