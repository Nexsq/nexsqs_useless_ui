@echo off
set /p "counter_title=counter name: "
cls
powershell -noprofile -executionpolicy bypass -command ^
"Add-Type -TypeDefinition 'using System; using System.Runtime.InteropServices; public class KeyDetector { [DllImport(\"user32.dll\")] public static extern short GetAsyncKeyState(int vKey); [DllImport(\"user32.dll\")] public static extern IntPtr GetForegroundWindow(); [DllImport(\"user32.dll\")] public static extern int GetWindowThreadProcessId(IntPtr hWnd, out int lpdwProcessId); }'; ^
function Get-KeyName($code) { ^
    $keyMap = @{ ^
        0x08='[BACKSPACE]'; 0x09='[TAB]'; 0x0D='[ENTER]'; 0x1B='[ESC]'; ^
        0x20='[SPACE]'; 0x25='[LEFT]'; 0x26='[UP]'; 0x27='[RIGHT]'; 0x28='[DOWN]'; ^
        0x41='A'; 0x42='B'; 0x43='C'; 0x44='D'; 0x45='E'; 0x46='F'; 0x47='G'; ^
        0x48='H'; 0x49='I'; 0x4A='J'; 0x4B='K'; 0x4C='L'; 0x4D='M'; 0x4E='N'; ^
        0x4F='O'; 0x50='P'; 0x51='Q'; 0x52='R'; 0x53='S'; 0x54='T'; 0x55='U'; ^
        0x56='V'; 0x57='W'; 0x58='X'; 0x59='Y'; 0x5A='Z'; 0x30='0'; 0x31='1'; ^
        0x32='2'; 0x33='3'; 0x34='4'; 0x35='5'; 0x36='6'; 0x37='7'; 0x38='8'; ^
        0x39='9' ^
    }; ^
    if ($keyMap.ContainsKey($code)) { return $keyMap[$code] } else { return ('0x{0:X2}' -f $code) } ^
}; ^
Write-Host \"[!] press any key to count it for: %counter_title%\" -ForegroundColor DarkYellow; ^
$keyToCount = $null; $count = 0; ^
while (!$keyToCount) { ^
    for ($i=1; $i -le 254; $i++) { ^
        if ([KeyDetector]::GetAsyncKeyState($i) -band 0x8000) { ^
            $keyToCount = $i; ^
            $keyName = Get-KeyName $i; ^
            Write-Host (\"[+] now counting $keyName`n\") -ForegroundColor DarkGreen; ^
            break; ^
        } ^
    } ^
    Start-Sleep -Milliseconds 50; ^
}; ^
Write-Host (\"`r{0} count: {1} \" -f '%counter_title%', $count) -NoNewline -ForegroundColor DarkRed; ^
while ($true) { ^
    if ([KeyDetector]::GetAsyncKeyState($keyToCount) -band 0x8000) { ^
        $count++; ^
        Write-Host (\"`r{0} count: {1} \" -f '%counter_title%', $count) -NoNewline -ForegroundColor DarkRed; ^
        Start-Sleep -Milliseconds 500; ^
    } ^
    Start-Sleep -Milliseconds 50; ^
}"