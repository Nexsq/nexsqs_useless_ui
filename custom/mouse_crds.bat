@echo off
powershell -noprofile -executionpolicy bypass -command ^
"Add-Type -AssemblyName System.Windows.Forms; ^
Add-Type -TypeDefinition 'using System; using System.Runtime.InteropServices; public class KeyDetector { [DllImport(\"user32.dll\")] public static extern short GetAsyncKeyState(int vKey); }'; ^
$roundTo = 0; ^
while ($roundTo -lt 1) { ^
    try { ^
        $input = Read-Host 'round to'; ^
        $roundTo = [Math]::Abs([int]$input); ^
        if ($roundTo -eq 0) { $roundTo = 1 } ^
    } catch { Write-Host '[!] invalid number' -ForegroundColor Red } ^
} ^
$key = $null; ^
Start-Sleep -Milliseconds 100; ^
Write-Host '[!] Press your capture key...' -ForegroundColor DarkYellow; ^
while (!$key) { ^
    for ($i=1; $i -le 254; $i++) { ^
        if ([KeyDetector]::GetAsyncKeyState($i) -band 0x8000) { ^
            $key = $i; ^
            Write-Host (\"`n[+] Capturing at $([char]$i) (KeyCode: $i)`n\") -ForegroundColor DarkGreen; ^
            break; ^
        } ^
    } ^
    Start-Sleep -Milliseconds 50; ^
} ^
while ($true) { ^
    if ([KeyDetector]::GetAsyncKeyState($key) -band 0x8000) { ^
        $pos = [System.Windows.Forms.Cursor]::Position; ^
        $roundedX = [Math]::Round($pos.X / $roundTo) * $roundTo; ^
        $roundedY = [Math]::Round($pos.Y / $roundTo) * $roundTo; ^
        [System.Windows.Forms.Clipboard]::SetText(\"$roundedX $roundedY\"); ^
        Write-Host (\"[+] Copied: X: $roundedX, Y: $roundedY\") -ForegroundColor DarkRed; ^
        while ([KeyDetector]::GetAsyncKeyState($key) -band 0x8000) { Start-Sleep -Milliseconds 10 } ^
    } ^
    Start-Sleep -Milliseconds 10; ^
}"