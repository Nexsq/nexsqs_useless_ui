@echo off
powershell -noprofile -executionpolicy bypass -command ^
"Add-Type -AssemblyName System.Windows.Forms; ^
Add-Type -TypeDefinition 'using System; using System.Runtime.InteropServices; public class KeyDetector { [DllImport(\"user32.dll\")] public static extern short GetAsyncKeyState(int vKey); }'; ^
$key = $null; ^
Write-Host '[!] press your capture key...' -ForegroundColor DarkYellow; ^
while (!$key) { ^
    for ($i=1; $i -le 254; $i++) { ^
        if ([KeyDetector]::GetAsyncKeyState($i) -band 0x8000) { ^
            $key = $i; ^
            Write-Host (\"`n[+] capturing at $([char]$i) (KeyCode: $i)`n\") -ForegroundColor DarkGreen; ^
            break; ^
        } ^
    } ^
    Start-Sleep -Milliseconds 50; ^
} ^
while ($true) { ^
    if ([KeyDetector]::GetAsyncKeyState($key) -band 0x8000) { ^
        $pos = [System.Windows.Forms.Cursor]::Position; ^
        [System.Windows.Forms.Clipboard]::SetText(\"$($pos.X) $($pos.Y)\"); ^
        Write-Host (\"r[+] copied: X: $($pos.X), Y: $($pos.Y)\") -ForegroundColor DarkRed; ^
        while ([KeyDetector]::GetAsyncKeyState($key) -band 0x8000) { Start-Sleep -Milliseconds 10 } ^
    } ^
    Start-Sleep -Milliseconds 10; ^
}"