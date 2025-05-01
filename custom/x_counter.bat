@echo off
set /p "counter_title=name: "
cls
powershell -noprofile -executionpolicy bypass -command ^
"Add-Type -AssemblyName System.Windows.Forms; ^
Add-Type -AssemblyName System.Drawing; ^
Add-Type -TypeDefinition 'using System; using System.Runtime.InteropServices; public class KeyDetector { [DllImport(\"user32.dll\")] public static extern short GetAsyncKeyState(int vKey); [DllImport(\"user32.dll\")] public static extern IntPtr GetForegroundWindow(); [DllImport(\"user32.dll\")] public static extern int GetWindowText(IntPtr hWnd, System.Text.StringBuilder text, int count); }'; ^
$count = 0; ^
$keyToCount = $null; ^
$form = $null; ^
$label = $null; ^
$consoleTitle = 'Counting: ' + '%counter_title%'; ^
$keyWasPressed = $false; ^
$keyWasPressedDecrement = $false; ^
$consoleWindow = [System.Diagnostics.Process]::GetCurrentProcess().MainWindowHandle; ^
function Get-ActiveWindowTitle { ^
    $hwnd = [KeyDetector]::GetForegroundWindow(); ^
    if ($hwnd -ne [IntPtr]::Zero) { ^
        $title = New-Object System.Text.StringBuilder(256); ^
        [void][KeyDetector]::GetWindowText($hwnd, $title, $title.Capacity); ^
        return $title.ToString(); ^
    } ^
    return $null; ^
} ^
Write-Host \"[!] press any key to use it for %counter_title%\" -ForegroundColor DarkYellow; ^
while (!$keyToCount) { ^
    for ($i=1; $i -le 254; $i++) { ^
        if ([KeyDetector]::GetAsyncKeyState($i) -band 0x8000) { ^
            $keyToCount = $i; ^
            $keyName = if ($i -ge 65 -and $i -le 90) { [char]$i } else { ('0x{0:X2}' -f $i) }; ^
            Write-Host (\"`n[+] now counting at $keyName\") -ForegroundColor DarkGreen; ^
            Write-Host \"[+] [space] or [ent] for AOT window\" -ForegroundColor DarkGreen; ^
            Write-Host \"[+] [backspace] or [del] to decrease`n\" -ForegroundColor DarkGreen; ^
            break; ^
        } ^
    } ^
    Start-Sleep -Milliseconds 50; ^
} ^
while ([KeyDetector]::GetAsyncKeyState($keyToCount) -band 0x8000) { ^
    Start-Sleep -Milliseconds 10; ^
} ^
$Host.UI.RawUI.WindowTitle = $consoleTitle; ^
Write-Host (\"`r{0} count: {1} \" -f '%counter_title%', $count) -NoNewline -ForegroundColor DarkRed; ^
try { ^
    while ($true) { ^
        $keyState = [KeyDetector]::GetAsyncKeyState($keyToCount) -band 0x8000; ^
        ^
        if ($keyState -and -not $keyWasPressed) { ^
            $count++; ^
            $keyWasPressed = $true; ^
            Write-Host (\"`r{0} count: {1} \" -f '%counter_title%', $count) -NoNewline -ForegroundColor DarkRed; ^
            if ($form -and $form.Visible) { ^
                $label.Text = $count.ToString(); ^
                [System.Windows.Forms.Application]::DoEvents(); ^
            } ^
        } ^
        elseif (-not $keyState) { ^
            $keyWasPressed = $false; ^
        } ^
        ^
        $activeWindow = Get-ActiveWindowTitle; ^
        if ($activeWindow -eq $consoleTitle) { ^
            if ([KeyDetector]::GetAsyncKeyState(0x20) -band 0x8000 -or [KeyDetector]::GetAsyncKeyState(0x0D) -band 0x8000) { ^
                if ($form -eq $null) { ^
                    $form = New-Object System.Windows.Forms.Form; ^
                    $form.Text = '%counter_title%'; ^
                    $form.TopMost = $true; ^
                    $form.FormBorderStyle = 'FixedSingle'; ^
                    $form.MinimizeBox = $false; ^
                    $form.MaximizeBox = $false; ^
                    $form.ControlBox = $true; ^
                    $form.ShowInTaskbar = $true; ^
                    $form.Size = New-Object System.Drawing.Size(160, 80); ^
                    $form.StartPosition = 'Manual'; ^
                    $screenWidth = [System.Windows.Forms.Screen]::PrimaryScreen.WorkingArea.Width; ^
                    $form.Location = New-Object System.Drawing.Point(($screenWidth - 180), 20); ^
                    $form.BackColor = [System.Drawing.Color]::Black; ^
                    $form.ForeColor = [System.Drawing.Color]::White; ^
                    ^
                    $label = New-Object System.Windows.Forms.Label; ^
                    $label.Font = New-Object System.Drawing.Font('Consolas', 24, [System.Drawing.FontStyle]::Bold); ^
                    $label.TextAlign = [System.Windows.Forms.HorizontalAlignment]::Center; ^
                    $label.Dock = [System.Windows.Forms.DockStyle]::Fill; ^
                    $label.Text = $count.ToString(); ^
                    $form.Controls.Add($label); ^
                    ^
                    $form.Add_FormClosing({ ^
                        $form.Hide(); ^
                        $_.Cancel = $true; ^
                    }); ^
                    ^
                    $form.Show(); ^
                } ^
                elseif ($form.Visible) { ^
                    $form.Hide(); ^
                } ^
                else { ^
                    $form.Show(); ^
                } ^
                Start-Sleep -Milliseconds 100; ^
            } ^
            ^
            $backspacePressed = [KeyDetector]::GetAsyncKeyState(0x08) -band 0x8000; ^
            $deletePressed = [KeyDetector]::GetAsyncKeyState(0x2E) -band 0x8000; ^
            if (($backspacePressed -or $deletePressed) -and (-not $keyWasPressedDecrement)) { ^
                if ($count -gt 0) { ^
                    $count--; ^
                    Write-Host (\"`r{0} count: {1} \" -f '%counter_title%', $count) -NoNewline -ForegroundColor DarkRed; ^
                    if ($form -and $form.Visible) { ^
                        $label.Text = $count.ToString(); ^
                        [System.Windows.Forms.Application]::DoEvents(); ^
                    } ^
                } ^
                $keyWasPressedDecrement = $true; ^
            } ^
            elseif (-not ($backspacePressed -or $deletePressed)) { ^
                $keyWasPressedDecrement = $false; ^
            } ^
        } ^
        ^
        if ($form -and $form.Visible) { ^
            $label.Text = $count.ToString(); ^
            [System.Windows.Forms.Application]::DoEvents(); ^
        } ^
        Start-Sleep -Milliseconds 10; ^
    } ^
} finally { ^
    if ($form) { $form.Close(); $form.Dispose(); } ^
}"