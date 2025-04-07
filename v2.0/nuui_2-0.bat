<# : Batch portion
@echo off & setlocal enabledelayedexpansion

set "scriptPath=%~dp0"
set "scriptName=%~n0"
set "filePath=%scriptPath%NUUI_settings.txt"

if not exist "%filePath%" (
	echo Creating settings file at "%filePath%"
	echo color = White>> "%filePath%"
	echo darkTheme = false>> "%filePath%"
	echo menuStyle = v1>> "%filePath%"
	echo pingDelay = 500>> "%filePath%"
	echo portScanDelay = 500>> "%filePath%"
	echo microMacroKey = F15>> "%filePath%"
	echo microMacroDelay = 30000>> "%filePath%"
	echo showHiddenFiles = false>> "%filePath%"
	echo showLogo = true>> "%filePath%"
	attrib +h "%filePath%"
)

set "menu[0]=sys_fetch"
set "menu[1]=cleanup"
set "menu[2]=ping_tool"
set "menu[3]=port_scan"
set "menu[4]=micro_macro"
set "menu[5]=macro"
set "menu[6]=quick_start"
set "menu[7]=quick_download"
set "menu[8]=game_of_life"

set "settings[0]=color"
set "settings[1]=dark_theme"
set "settings[2]=menu_style"
set "settings[3]=ping_delay"
set "settings[4]=port_scan_delay"
set "settings[5]=micro_macro_key"
set "settings[6]=micro_macro_delay"
set "settings[7]=show_hidden_files"
set "settings[8]=show_logo"
set "settings[9]=auto_start"

set "default=0"

powershell -noprofile -executionpolicy remotesigned "iex ((gc '%~f0') -join \"`n\")"

: end batch / begin PowerShell hybrid chimera #>

$menu = gci env: | ?{ $_.Name -match "^menu\[\d+\]$" } | %{ $_.Value }
$settingsMenu = gci env: | ?{ $_.Name -match "^settings\[\d+\]$" } | %{ $_.Value }
[int]$selection = $env:default
[int]$global:settingsSelection = $env:default

$darkTheme = Get-Content -Path $env:filePath | Where-Object { $_ -match "darkTheme = (.*)" } | ForEach-Object { $matches[1] }
if (-not $darkTheme) {
	$darkTheme = "false"  # default value
	$fg = "White"
} elseif ($darkTheme -eq "true") {
	$fg = "Gray"
} else {
	$fg = "White"
}
$bg = $Host.UI.RawUI.BackgroundColor

$name = "Nexsq's Useless UI v2.0"
$logo = @(
"                     +-.         "
"                    -ssooo.      "
"   .                -soooooo     "
"   s\               -ooossooo-   "
"  'oos-             -osssssooo.  "
"  'soo+\.           -oooos+-+o.  "
"  'ssso\\.          -so+oo+\:-   "
"  'ssoos+\'         .s+++ooo+:.  "
"  'ss\sso+\'        .s++++oooo.  "
"  'sss\ssos+-       .ss++++ooo:  "
"  'ssssss++s-       .s\s++++oo:  "
"  'ssss+ssso-       .soo+++ooo:  "
"  :sss+++s\o.       -oo+++oooo:  "
"  :ss++++++o.       -sso+\+++o.  "
"  :s+++++sso.       '+ooss+ooo.  "
"  :ssss++sso.        '\+ssoo\o.  "
"   -:s+ssooo.         '\+sosoo.  "
"  's+-+sssso.           \+ssoo.  "
"  'sss\sssss.            'ssoo.  "
"  'ssoos+sss-              \s+.  "
"   -sssoooss-               \s   "
"    ssssooss-                '   "
"     .ssssss-                    "
"        '-+-                     "
)

$showLogo = Get-Content -Path $env:filePath | Where-Object { $_ -match "showLogo = (.*)" } | ForEach-Object { $matches[1] }
if (-not $showLogo) {
	$showLogo = "true"  # default value
}
if ($showLogo -eq "false") {
	$global:renderedLogo = @()
} else {
	$global:renderedLogo = $logo
}

$global:renderedLogoLineLength = " " * $global:renderedLogo[0].Length

function getKey {
	while (-not ((37..40 + 13 + 32 + 48..(47 + $menu.Length) + 27 + 9) -contains $x)) {
		$x = $Host.UI.RawUI.ReadKey('NoEcho,IncludeKeyDown').VirtualKeyCode
	}
	$x
}

function systemFetch {
	$darkTheme = Get-Content -Path $env:filePath | Where-Object { $_ -match "darkTheme = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $darkTheme) {
		$darkTheme = "false"  # default value
		$fg = "White"
	} elseif ($darkTheme -eq "true") {
		$fg = "Gray"
	} else {
		$fg = "White"
	}

	$currentColor = Get-Content -Path $env:filePath | Where-Object { $_ -match "color = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $currentColor) {
		$currentColor = "White"  # default color
	}

	Add-Type -AssemblyName System.Windows.Forms

	$os = Get-CimInstance Win32_OperatingSystem | Select-Object -ExpandProperty Caption
	$machinehost = $env:COMPUTERNAME
	$kernel = Get-CimInstance Win32_OperatingSystem | Select-Object -ExpandProperty Version

	$uptime = (Get-CimInstance Win32_OperatingSystem).LastBootUpTime
	$uptimeSpan = (Get-Date) - $uptime
	$uptimeDays = [math]::Floor($uptimeSpan.TotalDays)
	if ($uptimeDays -eq 1) { $daysFormat = "day" } else { $daysFormat = "days" }
	$uptimeHours = [math]::Floor($uptimeSpan.TotalHours) % 24
	if ($uptimeHours -eq 1) { $hoursFormat = "hour" } else { $hoursFormat = "hours" }
	$uptimeMinutes = [math]::Floor($uptimeSpan.TotalMinutes) % 60
	if ($uptimeMinutes -eq 1) { $minutesFormat = "min" } else { $minutesFormat = "mins"}

	$shell = $PSVersionTable.PSVersion.ToString()
	$screenBounds = [System.Windows.Forms.Screen]::PrimaryScreen.Bounds
	$resolution = "$($screenBounds.Width)x$($screenBounds.Height)"
	$terminal = if ($Host.Name -eq "ConsoleHost") { "Windows Terminal" } else { $Host.Name }

	$cpu = Get-CimInstance Win32_Processor | Select-Object -ExpandProperty Name
	$gpu = Get-CimInstance Win32_VideoController | Select-Object -ExpandProperty Name

	$memory = Get-CimInstance Win32_ComputerSystem
	$osMemory = Get-CimInstance Win32_OperatingSystem
	$totalMemory = [math]::round($memory.TotalPhysicalMemory / 1MB)
	$availableMemory = [math]::round($osMemory.FreePhysicalMemory / 1KB)
	$usedMemory = [math]::round($totalMemory - $availableMemory)
	$memoryPercentage = [math]::round(($usedMemory / $totalMemory) * 100)

	$drives = Get-PSDrive -PSProvider FileSystem
	$totalSpace = [math]::round(($drives | ForEach-Object { ($_.Used + $_.Free) / 1GB } | Measure-Object -Sum).Sum)
	$freeSpace = [math]::round(($drives | ForEach-Object { $_.Free / 1GB } | Measure-Object -Sum).Sum)
	$usedSpace = [math]::round(($drives | ForEach-Object { $_.Used / 1GB } | Measure-Object -Sum).Sum)
	$spacePercentage = [math]::round(($usedSpace / $totalSpace) * 100)

	$info = @(
		"OS: $os"
		"Host: $machinehost"
		"Kernel: $kernel"
		"Uptime: $uptimeDays $daysFormat $uptimeHours $hoursFormat $uptimeMinutes $minutesFormat"
		"Shell: $shell"
		"Resolution: $resolution"
		"Terminal: $terminal"
		"CPU: $cpu"
		"GPU: $gpu"
		"Ram: $usedMemory MB / $totalMemory MB (${memoryPercentage}%)"
		"Storage: $usedSpace GB / $totalSpace GB (${spacePercentage}%)"
	)
	$fetchColors = @(
		"Black"
		"DarkRed"
		"DarkYellow"
		"DarkGreen"
		"DarkCyan"
		"DarkBlue"
		"Magenta"
		"Gray"
	)

	cls
	write-host "$($global:renderedLogo[0])    " -NoNewline -f $currentColor -b $bg
	write-host "$env:USERNAME"-NoNewline -f $currentColor -b $bg
	write-host "@"-NoNewline -f $fg -b $bg
	write-host "$machinehost" -f $currentColor -b $bg
	write-host "$($global:renderedLogo[1])    " -NoNewline -f $currentColor -b $bg
	write-host ("-" * ($env:USERNAME.Length + $machinehost.Length + 1)) -NoNewline -f $fg -b $bg
	write-host "" -f $fg -b $bg
	for ($i = 0; $i -lt $info.Length + 1; $i++) {
		if ($i -lt $info.Length) {
			write-host "$($global:renderedLogo[$i + 2])    " -NoNewline -f $currentColor -b $bg
			$parts = $info[$i] -split ":", 2
			write-host "$($parts[0])" -NoNewline -f $currentColor -b $bg
			write-host ":$($parts[1])" -f $fg -b $bg
		} else {
			write-host "$($global:renderedLogo[$i + 2])    " -f $currentColor -b $bg
		}
	}
	write-host "$($global:renderedLogo[$i + 2])    " -NoNewline -f $currentColor -b $bg
	for ($i = 0; $i -lt $fetchColors.Length; $i++) {
		write-host "   " -NoNewline -f $fg -b $fetchColors[$i]
	}
	write-host "" -f $fg -b $bg

	for ($i = $info.Length + 2; $i -lt $global:renderedLogo.Length - 1; $i++) {
		write-host "$($global:renderedLogo[$i + 2])    " -f $currentColor -b $bg
	}

	while ($true) {
		[int]$key = getKey
		if ($key -eq 27) {
			return
		}
	}
}

function cleanup {
	$darkTheme = Get-Content -Path $env:filePath | Where-Object { $_ -match "darkTheme = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $darkTheme) {
		$darkTheme = "false"  # default value
		$fg = "White"
	} elseif ($darkTheme -eq "true") {
		$fg = "Gray"
	} else {
		$fg = "White"
	}

	$currentColor = Get-Content -Path $env:filePath | Where-Object { $_ -match "color = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $currentColor) {
		$currentColor = "White"  # default color
	}

	write-host "Cleaning " -NoNewline -f $currentColor -b $bg

	Remove-Item -Path c:\windows\temp\* -Force -Recurse -ErrorAction SilentlyContinue
	Remove-Item -Path C:\WINDOWS\Prefetch\* -Force -ErrorAction SilentlyContinue
	Remove-Item -Path $env:TEMP\* -Force -Recurse -ErrorAction SilentlyContinue
	New-Item -Path c:\windows\temp -ItemType Directory -Force | Out-Null
	New-Item -Path $env:TEMP -ItemType Directory -Force | Out-Null

	write-host "completed" -f $currentColor -b $bg
	write-host "Press <escape> key to exit" -f $currentColor -b $bg
	while ($true) {
		[int]$key = getKey
		if ($key -eq 27) {
			return
		}
	}
}

function pingTool {
	$darkTheme = Get-Content -Path $env:filePath | Where-Object { $_ -match "darkTheme = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $darkTheme) {
		$darkTheme = "false"  # default value
		$fg = "White"
	} elseif ($darkTheme -eq "true") {
		$fg = "Gray"
	} else {
		$fg = "White"
	}

	$currentColor = Get-Content -Path $env:filePath | Where-Object { $_ -match "color = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $currentColor) {
		$currentColor = "White"  # default color
	}

	$pingDelay = Get-Content -Path $env:filePath | Where-Object { $_ -match "pingDelay = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $pingDelay) {
		$pingDelay = 500  # default delay
	}

	write-host "Enter IP address: " -NoNewline -f $currentColor -b $bg
	$target = Read-Host
	if (-not $target.Trim()) {
		return
	}
	write-host "Pinging $target" -f $currentColor -b $bg
	$ping = New-Object System.Net.NetworkInformation.Ping

	write-host "Press <escape> key to stop" -f $currentColor -b $bg
	$host.UI.RawUI.FlushInputBuffer()

	while ($true) {
		$result = $ping.Send($target)
		write-host "Reply from $($result.Address): bytes=$($result.Buffer.Length) time=$($result.RoundtripTime)ms TTL=$($result.Options.Ttl)" -f $fg -b $bg
		Start-Sleep -m $pingDelay

		if ([Console]::KeyAvailable) {
			$key = [Console]::ReadKey($true)
			if ($key.Key -eq [ConsoleKey]::Escape) {
				return
			}
		}
	}
}

function portScan {
	$darkTheme = Get-Content -Path $env:filePath | Where-Object { $_ -match "darkTheme = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $darkTheme) {
		$darkTheme = "false"  # default value
		$fg = "White"
	} elseif ($darkTheme -eq "true") {
		$fg = "Gray"
	} else {
		$fg = "White"
	}

	$currentColor = Get-Content -Path $env:filePath | Where-Object { $_ -match "color = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $currentColor) {
		$currentColor = "White"  # default color
	}

	$portScanDelay = Get-Content -Path $env:filePath | Where-Object { $_ -match "portScanDelay = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $portScanDelay) {
		$portScanDelay = 500  # default delay
	}

	write-host "Enter IP address: " -NoNewline -f $currentColor -b $bg
	$target = Read-Host
	if (-not $target.Trim()) {
		return
	}
	write-host "Enter port range ( split - ): " -NoNewline -f $currentColor -b $bg
	$portRange = Read-Host
	if (-not $portRange.Trim()) {
		return
	}
	$startPort, $endPort = $portRange -split "-"
	$startPort = [int]$startPort
	$endPort = [int]$endPort

	write-host "Scanning $target for open ports $startPort-$endPort" -f $currentColor -b $bg
	write-host "Press <escape> key to stop" -f $currentColor -b $bg

	$ports = @()
	for ($port = $startPort; $port -le $endPort; $port++) {
		$socket = New-Object System.Net.Sockets.TcpClient
		if ($socket.ConnectAsync($target, $port).Wait($portScanDelay)) {
			$ports += $port
			write-host "Port $port is open" -f $currentColor -b $bg
		} else {
			write-host "Port $port is closed" -f $fg -b $bg
		}
		if ([Console]::KeyAvailable) {
			$key = [Console]::ReadKey($true)
			if ($key.Key -eq [ConsoleKey]::Escape) {
				write-host "Scan cancelled" -f $currentColor -b $bg
				write-host "Press <escape> key to exit" -f $currentColor -b $bg
				write-host "Open ports: $($ports -join ", ")" -f $currentColor -b $bg
				while ($true) {
					[int]$key = getKey
					if ($key -eq 27) {
						return
					}
				}
			}
		}
	}
	write-host "Open ports: $($ports -join ", ")" -f $currentColor -b $bg
	write-host "Press <escape> key to exit" -f $currentColor -b $bg
	while ($true) {
		[int]$key = getKey
		if ($key -eq 27) {
			return
		}
	}
}

function microMacro {
	$darkTheme = Get-Content -Path $env:filePath | Where-Object { $_ -match "darkTheme = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $darkTheme) {
		$darkTheme = "false"  # default value
		$fg = "White"
	} elseif ($darkTheme -eq "true") {
		$fg = "Gray"
	} else {
		$fg = "White"
	}

	$currentColor = Get-Content -Path $env:filePath | Where-Object { $_ -match "color = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $currentColor) {
		$currentColor = "White"  # default color
	}

	$microMacroKey = Get-Content -Path $env:filePath | Where-Object { $_ -match "microMacroKey = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $microMacroKey) {
		$microMacroKey = "F15"  # default key
	}

	$microMacroDelay = Get-Content -Path $env:filePath | Where-Object { $_ -match "microMacroDelay = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $microMacroDelay) {
		$microMacroDelay = 30000  # default delay
	}

	$microMacroDelay = [int]$microMacroDelay

	if ($microMacroDelay -le 1000) {
		$delayUnit = "ms"
		$displayDelay = $microMacroDelay
	} elseif ($microMacroDelay -gt 60000) {
		$delayUnit = "m"
		$displayDelay = $microMacroDelay / 60000
	} else {
		$delayUnit = "s"
		$displayDelay = $microMacroDelay / 1000
	}

	if ($microMacroKey -eq "RandomNum") {
		write-host "Simulating random number every $displayDelay$delayUnit" -f $fg -b $bg
		write-host "Press <escape> key to stop" -f $currentColor -b $bg
		Add-Type -AssemblyName System.Windows.Forms

		$stop = $false
		$keyPressThread = {
			param ($microMacroDelay)
			while (!$stop) {
				$randomNumber = Get-Random -Minimum 0 -Maximum 9
				[System.Windows.Forms.SendKeys]::SendWait($randomNumber)
				Start-Sleep -m $microMacroDelay
			}
		}.GetNewClosure()

		$thread = [PowerShell]::Create().AddScript($keyPressThread).AddArgument($microMacroDelay)
		$handle = $thread.BeginInvoke()

		while ($true) {
			[int]$key = getKey
			if ($key -eq 27) {
				$stop = $true
				$thread.Stop() | Out-Null
				$thread.Dispose() | Out-Null
				$handle.AsyncWaitHandle.WaitOne() | Out-Null
				return
			}
		}
	} else {
		write-host "Simulating"$microMacroKey" key every $displayDelay$delayUnit"  -f $fg -b $bg
		write-host "Press <escape> key to stop" -f $currentColor -b $bg
	
		Add-Type -AssemblyName System.Windows.Forms
		
		$stop = $false
		$keyPressThread = {
			param ($microMacroKey, $microMacroDelay)
			while (!$stop) {
				if ($microMacroKey -eq "Space") {
					[System.Windows.Forms.SendKeys]::SendWait(" ")
				} else {
					[System.Windows.Forms.SendKeys]::SendWait("{$microMacroKey}")
				}
				Start-Sleep -m $microMacroDelay
			}
		}.GetNewClosure()
		
		$thread = [PowerShell]::Create().AddScript($keyPressThread).AddArgument($microMacroKey).AddArgument($microMacroDelay)
		$handle = $thread.BeginInvoke()
		
		while ($true) {
			[int]$key = getKey
			if ($key -eq 27) {
				$stop = $true
				$thread.Stop() | Out-Null
				$thread.Dispose() | Out-Null
				$handle.AsyncWaitHandle.WaitOne() | Out-Null
				return
			}
		}
	}
}

function macro {
	$darkTheme = Get-Content -Path $env:filePath | Where-Object { $_ -match "darkTheme = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $darkTheme) {
		$darkTheme = "false"  # default value
		$fg = "White"
	} elseif ($darkTheme -eq "true") {
		$fg = "Gray"
	} else {
		$fg = "White"
	}

	$currentColor = Get-Content -Path $env:filePath | Where-Object { $_ -match "color = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $currentColor) {
		$currentColor = "White"  # default color
	}

	$showHiddenFiles = Get-Content -Path $env:filePath | Where-Object { $_ -match "showHiddenFiles = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $showHiddenFiles) {
		$showHiddenFiles = "false"  # default value
	}

	$macroConfigPath = Join-Path -Path $env:scriptPath -ChildPath "NUUI_MacroConfig.txt"
	if (!(Test-Path -Path $macroConfigPath)) {
		write-host "Creating Macro config file" -f $fg -b $bg
		New-Item -Path $macroConfigPath -ItemType File | Out-Null
		if ($showHiddenFiles -eq "false") {
			attrib +h $macroConfigPath
		}
		Invoke-Item $macroConfigPath
	} else {
		$macroConfig = Get-Content -Path $macroConfigPath
		Add-Type -AssemblyName System.Windows.Forms
		$stop = $false
		$loop = $false
		$loopCount = 1
		foreach ($line in $macroConfig) {
			if ($line -match "loop (\d+)") {
				$loop = $true
				$loopCount = $matches[1]
				break
			} elseif ($line -match "loop") {
				$loop = $true
				$loopCount = -1
				break
			}
		}
		$n = 0
		$i = 0

		$macroThread = {
			param ($macroConfig, $loop, $loopCount)
			$n = 0
			$i = 0
			while (!$stop) {
				foreach ($line in $macroConfig) {
					if ($line -match "loop (\d+)" -or $line -match "loop") {
						continue
					} elseif ($line -match "sleep (\d+)") {
						Start-Sleep -m $matches[1]
					} elseif ($line -eq "Enter") {
						[System.Windows.Forms.SendKeys]::SendWait("{Enter}")
					} elseif ($line -eq "Space") {
						[System.Windows.Forms.SendKeys]::SendWait(" ")
					} elseif ($line -eq "RanNum") {
						$randomNumber = Get-Random -Minimum 0 -Maximum 10
						[System.Windows.Forms.SendKeys]::SendWait($randomNumber)
					} elseif ($line -eq "n?") {
						[System.Windows.Forms.SendKeys]::SendWait($n)
					} elseif ($line -eq "n++") {
						$n++
					} elseif ($line -eq "n--") {
						$n--
					} else {
						[System.Windows.Forms.SendKeys]::SendWait("{$line}")
					}
				}
				$i++
				if ($loopCount -ne -1 -and $i -ge $loopCount) {
					break
				}
			}
		}.GetNewClosure()

		$thread = [PowerShell]::Create().AddScript($macroThread).AddArgument($macroConfig).AddArgument($loop).AddArgument($loopCount)
		$handle = $thread.BeginInvoke()

		write-host "Simulating keys" -f $fg -b $bg
		write-host "Press <escape> key to stop" -f $currentColor -b $bg
		while ($true) {
			[int]$key = getKey
			if ($key -eq 27) {
				$stop = $true
				$thread.Stop() | Out-Null
				$thread.Dispose() | Out-Null
				$handle.AsyncWaitHandle.WaitOne() | Out-Null
				return
			}
		}
	}
}

function quickStart {
	$showHiddenFiles = Get-Content -Path $env:filePath | Where-Object { $_ -match "showHiddenFiles = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $showHiddenFiles) {
		$showHiddenFiles = "false"  # default value
	}

	$quickStartFolderPath = Join-Path -Path $env:scriptPath -ChildPath "NUUI_QuickStartFolder"
	if (!(Test-Path -Path $quickStartFolderPath)) {
		write-host "Creating QuickStart folder" -f $fg -b $bg
		New-Item -Path $quickStartFolderPath -ItemType Directory | Out-Null
		if ($showHiddenFiles -eq "false") {
			attrib +h $quickStartFolderPath
		}
		Invoke-Item $quickStartFolderPath
	} else {
		write-host "Executing files" -f $fg -b $bg
		$files = Get-ChildItem -Path $quickStartFolderPath -File
		foreach ($file in $files) {
			write-host "Executing file: $($file.Name)" -f $fg -b $bg
			Start-Job -ScriptBlock { & $args[0] } -ArgumentList $file.FullName | Out-Null
		}
	}
}

function quickDownload {
	$darkTheme = Get-Content -Path $env:filePath | Where-Object { $_ -match "darkTheme = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $darkTheme) {
		$darkTheme = "false"  # default value
		$fg = "White"
	} elseif ($darkTheme -eq "true") {
		$fg = "Gray"
	} else {
		$fg = "White"
	}

	$currentColor = Get-Content -Path $env:filePath | Where-Object { $_ -match "color = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $currentColor) {
		$currentColor = "White"  # default color
	}

	$showHiddenFiles = Get-Content -Path $env:filePath | Where-Object { $_ -match "showHiddenFiles = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $showHiddenFiles) {
		$showHiddenFiles = "false"  # default value
	}

	$quickDownloadConfigPath = Join-Path -Path $env:scriptPath -ChildPath "NUUI_QuickDownloadConfig.txt"
	if (!(Test-Path -Path $quickDownloadConfigPath)) {
		write-host "Creating QuickDownload config file" -f $fg -b $bg
		New-Item -Path $quickDownloadConfigPath -ItemType File | Out-Null
		if ($showHiddenFiles -eq "false") {
			attrib +h $quickDownloadConfigPath
		}
		Invoke-Item $quickDownloadConfigPath
	} else {
		write-host "Downloading items" -f $fg -b $bg
		write-host "Press <escape> key to stop" -f $currentColor -b $bg
		$downloadAddresses = Get-Content -Path $quickDownloadConfigPath
		foreach ($address in $downloadAddresses) {
			$filename = $address.Split("/")[-1]
			$uri = [System.Uri]$address
			$request = [System.Net.HttpWebRequest]::Create($uri)
			$response = $request.GetResponse()
			$totalBytes = $response.ContentLength
			$responseStream = $response.GetResponseStream()
			$destinationPath = Join-Path -Path $env:TEMP -ChildPath $filename
			$buffer = New-Object byte[] 10240
			$bytesRead = 0
			$percentComplete = 0
			$stream = [System.IO.File]::OpenWrite($destinationPath)
			do {
				if ([Console]::KeyAvailable) {
					$key = [Console]::ReadKey($true)
					if ($key.Key -eq [ConsoleKey]::Escape) {
						write-host "Download cancelled" -f $fg -b $bg
						$stream.Close()
						$responseStream.Close()
						Remove-Item -Path $destinationPath -Force
						return
					}
				}
				$bytesRead = $responseStream.Read($buffer, 0, $buffer.Length)
				$stream.Write($buffer, 0, $bytesRead)
				$percentComplete = ($stream.Position / $totalBytes) * 100
				$percentComplete = [math]::Round($percentComplete, 1)
				write-host "Downloading $filename - $percentComplete%" -NoNewline -f $fg -b $bg
				[Console]::CursorLeft = 0
			} while ($bytesRead -gt 0)
			$stream.Close()
			$responseStream.Close()
			write-host "Download complete - $filename" -f $fg -b $bg
			Move-Item -Path $destinationPath -Destination $filename -Force
		}
	}
}

function gameOfLife {
	$darkTheme = Get-Content -Path $env:filePath | Where-Object { $_ -match "darkTheme = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $darkTheme) {
		$darkTheme = "false"  # default value
		$fg = "White"
	} elseif ($darkTheme -eq "true") {
		$fg = "Gray"
	} else {
		$fg = "White"
	}

	$currentColor = Get-Content -Path $env:filePath | Where-Object { $_ -match "color = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $currentColor) {
		$currentColor = "White"  # default color
	}

	function gameOfLifeSettings {
		while ($true) {
			cls
			for ($i = 0; $i -lt $global:gridSize; $i++) {
				$currentGridLine = ""
				if ($i -lt $global:renderedLogo.Length) { write-host "$($global:renderedLogo[$i])    " -NoNewline -f $currentColor -b $bg } else { write-host $global:renderedLogoLineLength -NoNewline; write-host "    " -NoNewline }
				for ($n = 0; $n -lt $global:gridSize; $n++) {
					if ($n -eq $global:currentSelection[0] -and $i -eq $global:currentSelection[1]) {
						if ($global:grid[$n, $i]) {
							$currentGridLine = $currentGridLine + "`b[" + ([char]0x25A0) + "]"
						} else {
							$currentGridLine = $currentGridLine + "`b[" + ([char]0x00B7) + "]"
						}
					} elseif ($global:grid[$n, $i]) {
						$currentGridLine = $currentGridLine + ([char]0x25A0) + " "
					} else {
						$currentGridLine = $currentGridLine + ([char]0x00B7) + " "
					}
				}
				if ($currentGridLine.Contains("[")) {
					$parts = $currentGridLine -split '(\[|\])'
					write-host $parts[0] -NoNewline -f $fg -b $bg
					write-host $parts[1] -NoNewline -f $currentColor -b $bg
					write-host $parts[2] -NoNewline -f $fg -b $bg
					write-host $parts[3] -NoNewline -f $currentColor -b $bg
					write-host $parts[4] -f $fg -b $bg
				} else {
					write-host $currentGridLine -f $fg -b $bg
				}
			}
			if ($($global:renderedLogo[$global:gridSize])) { write-host "$($global:renderedLogo[$global:gridSize])" -f $currentColor -b $bg } else { write-host $global:renderedLogoLineLength }
			if ($($global:renderedLogo[$global:gridSize + 1])) { write-host "$($global:renderedLogo[$global:gridSize + 1])" -f $currentColor -b $bg -NoNewline } else { write-host $global:renderedLogoLineLength -NoNewline }
			if ($gameOfLifeSettingsSelection -eq 0) {
				write-host "  > grid_size" -NoNewline -f $bg -b $currentColor
				write-host ": $global:gridSize x $global:gridSize <  " -f $bg -b $currentColor
			} else {
				write-host "    grid_size" -NoNewline -f $currentColor -b $bg
				write-host ": $global:gridSize x $global:gridSize" -f $fg -b $bg
			}
			if ($($global:renderedLogo[$global:gridSize + 2])) { write-host "$($global:renderedLogo[$global:gridSize + 2])" -f $currentColor -b $bg -NoNewline } else { write-host $global:renderedLogoLineLength -NoNewline }
			if ($gameOfLifeSettingsSelection -eq 1) {
				write-host "  > resolution" -NoNewline -f $bg -b $currentColor
				if ($global:lowerResolution) {
					write-host ": 1 : 4 <  " -f $bg -b $currentColor
				} else {
					write-host ": 1 : 1 <  " -f $bg -b $currentColor
				}
			} else {
				write-host "    resolution" -NoNewline -f $currentColor -b $bg
				if ($global:lowerResolution) {
					write-host ": 1 : 4" -f $fg -b $bg
				} else {
					write-host ": 1 : 1" -f $fg -b $bg
				}
			}
			if ($($global:renderedLogo[$global:gridSize + 3])) { write-host "$($global:renderedLogo[$global:gridSize + 3])" -f $currentColor -b $bg -NoNewline } else { write-host $global:renderedLogoLineLength -NoNewline }
			if ($gameOfLifeSettingsSelection -eq 2) {
				write-host "  > use_NUUI_colors" -NoNewline -f $bg -b $currentColor
				if ($global:useColor) {
					write-host ": 1 <  " -f $bg -b $currentColor
				} else {
					write-host ": 0 <  " -f $bg -b $currentColor
				}
			} else {
				write-host "    use_NUUI_colors" -NoNewline -f $currentColor -b $bg
				if ($global:useColor) {
					write-host ": 1" -f $fg -b $bg
				} else {
					write-host ": 0" -f $fg -b $bg
				}
			}
			for ($i = $global:gridSize + 4; $i -lt $global:renderedLogo.Length; $i++) {
				write-host "$($global:renderedLogo[$i])" -f $currentColor -b $bg
			}
			write-host ""

			[int]$key = getKey
			switch ($key) {

				# left
				37 {
					if ($gameOfLifeSettingsSelection -eq 0) {
						if ($global:gridSize -gt 16) { $global:gridSize -= 4; $global:grid = New-Object 'object[,]' $global:gridSize, $global:gridSize; $global:currentSelection = @(0,0) } else { $global:gridSize = 64 }
					} elseif ($gameOfLifeSettingsSelection -eq 1) {
						if ($global:lowerResolution) { $global:lowerResolution = $false } else { $global:lowerResolution = $true }
					} elseif ($gameOfLifeSettingsSelection -eq 2) {
						if ($global:useColor) { $global:useColor = $false } else { $global:useColor = $true }
					}
				}

				# right
				39 {
					if ($gameOfLifeSettingsSelection -eq 0) {
						if ($global:gridSize -lt 64) { $global:gridSize += 4; $global:grid = New-Object 'object[,]' $global:gridSize, $global:gridSize; $global:currentSelection = @(0,0) } else { $global:gridSize = 16 }
					} elseif ($gameOfLifeSettingsSelection -eq 1) {
						if ($global:lowerResolution) { $global:lowerResolution = $false } else { $global:lowerResolution = $true }
					} elseif ($gameOfLifeSettingsSelection -eq 2) {
						if ($global:useColor) { $global:useColor = $false } else { $global:useColor = $true }
					}
				}

				# up
				38 { if ($gameOfLifeSettingsSelection) { $gameOfLifeSettingsSelection-- } else { $gameOfLifeSettingsSelection = 3 - 1 }; break }
				# down
				40 { if ($gameOfLifeSettingsSelection -lt 3 - 1) { $gameOfLifeSettingsSelection++ } else { $gameOfLifeSettingsSelection = 0 }; break }

				# escape
				27 { return }

				# tab
				9 { return }
			}
		}
	}

	function gameOfLifeSimulation {
		$gameOfLifeDelay = 500  # default delay
		$speedDisplay = 0
		while ($true) {
			if ([Console]::KeyAvailable) {
				[int]$key = getKey
				switch ($key) {

					# left
					37 { if ($gameOfLifeDelay -gt 100) { $gameOfLifeDelay -= 100; $speedDisplay = 2 } else { $gameOfLifeDelay = 50; $speedDisplay = 2 } }
					# right
					39 { if ($gameOfLifeDelay -lt 100) { $gameOfLifeDelay = 100; $speedDisplay = 2 } elseif ($gameOfLifeDelay -lt 1000) { $gameOfLifeDelay += 100; $speedDisplay = 2 } }

					# escape
					27 { return }

					# tab
					9 { gameOfLifeSettings }
				}
			}

			cls
			if ($global:lowerResolution) {
				for ($i = 0; $i -lt $gridSize; $i += 2) {
					$currentGridLine = ""
					if ($i / 2 -lt $global:renderedLogo.Length) { write-host "$($global:renderedLogo[$i / 2])    " -NoNewline -f $currentColor -b $bg } else { write-host $global:renderedLogoLineLength -NoNewline; write-host "    " -NoNewline }
					for ($n = 0; $n -lt $gridSize; $n += 2) {
						$cellResolution = 0
						if ($grid[$n, $i]) { $cellResolution += 1 }
						if ($grid[($n + 1), $i]) { $cellResolution += 2 }
						if ($grid[$n, ($i + 1)]) { $cellResolution += 4 }
						if ($grid[($n + 1), ($i + 1)]) { $cellResolution += 8 }

						if ($cellResolution -eq 1) { 
							$currentGridLine = $currentGridLine + ([char]0x2580) + " "
						} elseif ($cellResolution -eq 2) {
							$currentGridLine = $currentGridLine + " " + ([char]0x2580)
						} elseif ($cellResolution -eq 3) {
							$currentGridLine = $currentGridLine + ([char]0x2580 + [char]0x2580)
						} elseif ($cellResolution -eq 4) {
							$currentGridLine = $currentGridLine + ([char]0x2584) + " "
						} elseif ($cellResolution -eq 5) {
							$currentGridLine = $currentGridLine + ([char]0x2588) + " "
						} elseif ($cellResolution -eq 6) {
							$currentGridLine = $currentGridLine + ([char]0x2584 + [char]0x2580)
						} elseif ($cellResolution -eq 7) {
							$currentGridLine = $currentGridLine + ([char]0x2588 + [char]0x2580)
						} elseif ($cellResolution -eq 8) {
							$currentGridLine = $currentGridLine + " " + ([char]0x2584)
						} elseif ($cellResolution -eq 9) {
							$currentGridLine = $currentGridLine + ([char]0x2580 + [char]0x2584)
						} elseif ($cellResolution -eq 10) {
							$currentGridLine = $currentGridLine + " " + ([char]0x2588)
						} elseif ($cellResolution -eq 11) {
							$currentGridLine = $currentGridLine + ([char]0x2580 + [char]0x2588)
						} elseif ($cellResolution -eq 12) {
							$currentGridLine = $currentGridLine + ([char]0x2584 + [char]0x2584)
						} elseif ($cellResolution -eq 13) {
							$currentGridLine = $currentGridLine + ([char]0x2588 + [char]0x2584)
						} elseif ($cellResolution -eq 14) {
							$currentGridLine = $currentGridLine + ([char]0x2584 + [char]0x2588)
						} elseif ($cellResolution -eq 15) {
							$currentGridLine = $currentGridLine + ([char]0x2588 + [char]0x2588)
						} else { 
							$currentGridLine = $currentGridLine + "  "
						}
					}
					if ($useColor) {
						write-host $currentGridLine -f $currentColor -b $bg
					} else {
						write-host $currentGridLine -f $fg -b $bg
					}
				}
			} else {
				for ($i = 0; $i -lt $gridSize; $i++) {
					$currentGridLine = ""
					if ($i -lt $global:renderedLogo.Length) { write-host "$($global:renderedLogo[$i])    " -NoNewline -f $currentColor -b $bg } else { write-host $global:renderedLogoLineLength -NoNewline; write-host "    " -NoNewline }
					for ($n = 0; $n -lt $gridSize; $n++) {
						if ($grid[$n, $i]) {
							$currentGridLine = $currentGridLine + ([char]0x2588 + [char]0x2588)
						} else {
							$currentGridLine = $currentGridLine + "  "
						}
					}
					if ($useColor) {
						write-host $currentGridLine -f $currentColor -b $bg
					} else {
						write-host $currentGridLine -f $fg -b $bg
					}
				}
			}
			if ($global:lowerResolution) { $gridSize = $gridSize / 2 }
			if ($($global:renderedLogo[$gridSize])) { write-host "$($global:renderedLogo[$gridSize])" -f $currentColor -b $bg } else { write-host $global:renderedLogoLineLength -NoNewline }
			if ($($global:renderedLogo[$gridSize + 1])) { write-host "$($global:renderedLogo[$gridSize + 1])" -NoNewline -f $currentColor -b $bg } else { write-host $global:renderedLogoLineLength -NoNewline }
			if ($speedDisplay -gt 0) {
				write-host "    delay: $gameOfLifeDelay" -f $currentColor -b $bg
				$speedDisplay--
			} else {
				write-host ""
			}
			for ($i = $gridSize + 2; $i -lt $global:renderedLogo.Length; $i++) {
				write-host "$($global:renderedLogo[$i])" -f $currentColor -b $bg
			}
			write-host ""
			if ($global:lowerResolution) { $gridSize = $gridSize * 2 }

			$futureGrid = $grid.Clone()
			for ($i = 0; $i -lt $gridSize; $i++) {
				for ($n = 0; $n -lt $gridSize; $n++) {
					$neighbours = 0
					for ($j = -1; $j -le 1; $j++) {
						for ($k = -1; $k -le 1; $k++) {
							if ($j -eq 0 -and $k -eq 0) { continue }
							if ($n + $j -ge 0 -and $n + $j -lt $gridSize -and $i + $k -ge 0 -and $i + $k -lt $gridSize) {
								if ($grid[($n + $j), ($i + $k)]) { $neighbours++ }
							}
						}
					}
					if ($grid[$n, $i] -and $neighbours -lt 2) { $futureGrid[$n, $i] = $false }
					if ($grid[$n, $i] -and $neighbours -gt 3) { $futureGrid[$n, $i] = $false }
					if (!($grid[$n, $i]) -and $neighbours -eq 3) { $futureGrid[$n, $i] = $true }
				}
			}
			$grid = $futureGrid
			Start-Sleep -m $gameOfLifeDelay
		}
	}

	$global:gridSize = 16
	$global:lowerResolution = $false
	$global:useColor = $false
	$global:currentSelection = @(0,0)
	$gameOfLifeSettingsSelection = 0
	$global:grid = New-Object 'object[,]' $global:gridSize, $global:gridSize
	while ($true) {
		cls
		for ($i = 0; $i -lt $global:gridSize; $i++) {
			$currentGridLine = ""
			if ($i -lt $global:renderedLogo.Length) { write-host "$($global:renderedLogo[$i])    " -NoNewline -f $currentColor -b $bg } else { write-host $global:renderedLogoLineLength -NoNewline; write-host "    " -NoNewline }
			for ($n = 0; $n -lt $global:gridSize; $n++) {
				if ($n -eq $global:currentSelection[0] -and $i -eq $global:currentSelection[1]) {
					if ($global:grid[$n, $i]) {
						$currentGridLine = $currentGridLine + "`b[" + ([char]0x25A0) + "]"
					} else {
						$currentGridLine = $currentGridLine + "`b[" + ([char]0x00B7) + "]"
					}
				} elseif ($global:grid[$n, $i]) {
					$currentGridLine = $currentGridLine + ([char]0x25A0) + " "
				} else {
					$currentGridLine = $currentGridLine + ([char]0x00B7) + " "
				}
			}
			if ($currentGridLine.Contains("[")) {
				$parts = $currentGridLine -split '(\[|\])'
				write-host $parts[0] -NoNewline -f $fg -b $bg
				write-host $parts[1] -NoNewline -f $currentColor -b $bg
				write-host $parts[2] -NoNewline -f $fg -b $bg
				write-host $parts[3] -NoNewline -f $currentColor -b $bg
				write-host $parts[4] -f $fg -b $bg
			} else {
				write-host $currentGridLine -f $fg -b $bg
			}
		}
		if ($($global:renderedLogo[$global:gridSize])) { write-host "$($global:renderedLogo[$global:gridSize])" -f $currentColor -b $bg } else { write-host $global:renderedLogoLineLength }
		if ($($global:renderedLogo[$global:gridSize + 1])) { write-host "$($global:renderedLogo[$global:gridSize + 1])" -f $currentColor -b $bg -NoNewline } else { write-host $global:renderedLogoLineLength -NoNewline }
		write-host "    grid_size" -NoNewline -f $currentColor -b $bg
		write-host ": $global:gridSize x $global:gridSize" -f $fg -b $bg
		if ($($global:renderedLogo[$global:gridSize + 2])) { write-host "$($global:renderedLogo[$global:gridSize + 2])" -f $currentColor -b $bg -NoNewline } else { write-host $global:renderedLogoLineLength -NoNewline }
		write-host "    resolution" -NoNewline -f $currentColor -b $bg
		if ($global:lowerResolution) {
			write-host ": 1 : 4" -f $fg -b $bg
		} else {
			write-host ": 1 : 1" -f $fg -b $bg
		}
		if ($($global:renderedLogo[$global:gridSize + 3])) { write-host "$($global:renderedLogo[$global:gridSize + 3])" -f $currentColor -b $bg -NoNewline } else { write-host $global:renderedLogoLineLength -NoNewline }
		write-host "    use_NUUI_colors" -NoNewline -f $currentColor -b $bg
		if ($global:useColor) {
			write-host ": 1" -f $fg -b $bg
		} else {
			write-host ": 0" -f $fg -b $bg
		}
		for ($i = $global:gridSize + 4; $i -lt $global:renderedLogo.Length; $i++) {
			write-host "$($global:renderedLogo[$i])" -f $currentColor -b $bg
		}
		write-host ""

		[int]$key = getKey
		switch ($key) {

			# left
			37 { if ($global:currentSelection[0]) { $global:currentSelection[0]-- } else { $global:currentSelection[0] = $global:gridSize - 1 }; break }
			# up
			38 { if ($global:currentSelection[1]) { $global:currentSelection[1]-- } else { $global:currentSelection[1] = $global:gridSize - 1 }; break }
			# right
			39 { if ($global:currentSelection[0] -lt $global:gridSize - 1) { $global:currentSelection[0]++ } else { $global:currentSelection[0] = 0 }; break }
			# down
			40 { if ($global:currentSelection[1] -lt $global:gridSize - 1) { $global:currentSelection[1]++ } else { $global:currentSelection[1] = 0 }; break }

			# escape
			27 { return }

			# tab
			9 { gameOfLifeSettings }

			# space
			32 { if ($global:grid[$global:currentSelection[0], $global:currentSelection[1]]) { $global:grid[$global:currentSelection[0], $global:currentSelection[1]] = $false } else { $global:grid[$global:currentSelection[0], $global:currentSelection[1]] = $true } }

			# enter
			13 { gameOfLifeSimulation }
		}
	}
}

$macroConfigPath = Join-Path -Path $env:scriptPath -ChildPath "NUUI_MacroConfig.txt"
$quickStartFolderPath = Join-Path -Path $env:scriptPath -ChildPath "NUUI_QuickStartFolder"
$quickDownloadConfigPath = Join-Path -Path $env:scriptPath -ChildPath "NUUI_QuickDownloadConfig.txt"

function settings {
	$menuStyles = @("v1", "v2")
	$menuStyleIndex = 0
	$menuStyle = Get-Content -Path $env:filePath | Where-Object { $_ -match "menuStyle = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $menuStyle) {
		$menuStyle = "v1"  # default value
	} else {
		$menuStyleIndex = [Array]::IndexOf($menuStyles, $menuStyle)
	}

	$showLogo = Get-Content -Path $env:filePath | Where-Object { $_ -match "showLogo = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $showLogo) {
		$showLogo = "true"  # default value
	}

	$darkTheme = Get-Content -Path $env:filePath | Where-Object { $_ -match "darkTheme = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $darkTheme) {
		$darkTheme = "false"  # default value
		$fg = "White"
	} elseif ($darkTheme -eq "true") {
		$fg = "Gray"
	} else {
		$fg = "White"
	}

	$colors = @("White", "Red", "Yellow", "Green", "Cyan", "Blue", "Magenta")
	$currentColorIndex = 0
	$currentColor = Get-Content -Path $env:filePath | Where-Object { $_ -match "color = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $currentColor) {
		$currentColor = "White"  # default color
	} elseif ($currentColor -eq "DarkGray") {
		$currentColorIndex = 0
	} elseif ($darkTheme -eq "true") {
		$currentColorIndex = [Array]::IndexOf($colors, $currentColor -replace "Dark", "")
	} else {
		$currentColorIndex = [Array]::IndexOf($colors, $currentColor)
	}

	$pingDelays = @(10, 50, 100, 200, 500, 1000)
	$pingDelayIndex = 4
	$pingDelay = Get-Content -Path $env:filePath | Where-Object { $_ -match "pingDelay = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $pingDelay) {
		$pingDelay = 500  # default delay
	} else {
		$pingDelayIndex = [Array]::IndexOf($pingDelays, [int]$pingDelay)
	}

	$portScanDelays = @(10, 50, 100, 200, 500, 1000)
	$portScanDelayIndex = 4
	$portScanDelay = Get-Content -Path $env:filePath | Where-Object { $_ -match "portScanDelay = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $portScanDelay) {
		$portScanDelay = 500  # default delay
	} else {
		$portScanDelayIndex = [Array]::IndexOf($portScanDelays, [int]$portScanDelay)
	}

	$microMacroKeys = @("F15", "RandomNum", "Enter", "Space", "E")
	$microMacroKeyIndex = 0
	$microMacroKey = Get-Content -Path $env:filePath | Where-Object { $_ -match "microMacroKey = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $microMacroKey) {
		$microMacroKey = "F15"  # default key
	} else {
		$microMacroKeyIndex = [Array]::IndexOf($microMacroKeys, $microMacroKey)
	}

	$microMacroDelays = @(200, 500, 1000, 5000, 10000, 30000, 60000, 120000, 300000, 600000)
	$microMacroDelayIndex = 5
	$microMacroDelay = Get-Content -Path $env:filePath | Where-Object { $_ -match "microMacroDelay = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $microMacroDelay) {
		$microMacroDelay = 500  # default delay
	} else {
		$microMacroDelayIndex = [Array]::IndexOf($microMacroDelays, [int]$microMacroDelay)
	}

	$showHiddenFiles = Get-Content -Path $env:filePath | Where-Object { $_ -match "showHiddenFiles = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $showHiddenFiles) {
		$showHiddenFiles = "false"  # default value
	}

	while (1) {
		settingsMenu
		[int]$key = getKey
		switch ($key) {

			# left
			37 { 
				if ($global:settingsSelection -eq 0) {
					$currentColorIndex = ($currentColorIndex - 1 + $colors.Length) % $colors.Length
					$currentColor = $colors[$currentColorIndex]
					if ($currentColor -eq "White" -and $darkTheme -eq "true") {
						$currentColor = "DarkGray"
					} elseif ($darkTheme -eq "true") {
						$currentColor = "Dark" + $currentColor
					}
				} elseif ($global:settingsSelection -eq 1) {
					if ($currentColor -eq "White" -and $darkTheme -eq "false") {
						$darkTheme = "true"
						$fg = "Gray"
						$currentColor = "DarkGray"
					} elseif ($darkTheme -eq "false") {
						$darkTheme = "true"
						$fg = "Gray"
						$currentColor = "Dark" + $currentColor
					} elseif ($currentColor -eq "DarkGray" -and $darkTheme -eq "true") {
						$darkTheme = "false"
						$fg = "White"
						$currentColor = "White"
					} else {
						$darkTheme = "false"
						$fg = "White"
						$currentColor = $currentColor -replace "Dark"
					}
				} elseif ($global:settingsSelection -eq 2) {
					$menuStyleIndex = ($menuStyleIndex - 1 + $menuStyles.Length) % $menuStyles.Length
					$menuStyle = $menuStyles[$menuStyleIndex]
				} elseif ($global:settingsSelection -eq 3) {
					$pingDelayIndex = ($pingDelayIndex - 1 + $pingDelays.Length) % $pingDelays.Length
					$pingDelay = $pingDelays[$pingDelayIndex]
				} elseif ($global:settingsSelection -eq 4) {
					$portScanDelayIndex = ($portScanDelayIndex - 1 + $portScanDelays.Length) % $portScanDelays.Length
					$portScanDelay = $portScanDelays[$portScanDelayIndex]
				} elseif ($global:settingsSelection -eq 5) {
					$microMacroKeyIndex = ($microMacroKeyIndex - 1 + $microMacroKeys.Length) % $microMacroKeys.Length
					$microMacroKey = $microMacroKeys[$microMacroKeyIndex]
				} elseif ($global:settingsSelection -eq 6) {
					$microMacroDelayIndex = ($microMacroDelayIndex - 1 + $microMacroDelays.Length) % $microMacroDelays.Length
					$microMacroDelay = $microMacroDelays[$microMacroDelayIndex]
				} elseif ($global:settingsSelection -eq 7) {
					if ($showHiddenFiles -eq "false") {
						$showHiddenFiles = "true"
						attrib -h $macroConfigPath | Out-Null
						attrib -h $quickStartFolderPath | Out-Null
						attrib -h $quickDownloadConfigPath | Out-Null
					} else {
						$showHiddenFiles = "false"
						attrib +h $macroConfigPath | Out-Null
						attrib +h $quickStartFolderPath | Out-Null
						attrib +h $quickDownloadConfigPath | Out-Null
					}
				} elseif ($global:settingsSelection -eq 8) {
					if ($showLogo -eq "false") {
						$showLogo = "true"
						$global:renderedLogo = $logo
						$global:renderedLogoLineLength = " " * $global:renderedLogo[0].Length
					} else {
						$showLogo = "false"
						$global:renderedLogo = @()
						$global:renderedLogoLineLength = ""
					}
				} elseif ($global:settingsSelection -eq 9) {
					if (!(Test-Path -Path "$env:APPDATA\Microsoft\Windows\Start Menu\Programs\Startup\$env:scriptName.lnk")) {
						$shell = New-Object -ComObject WScript.Shell
						$shortcut = $shell.CreateShortcut("$env:APPDATA\Microsoft\Windows\Start Menu\Programs\Startup\$env:scriptName.lnk")
						$shortcut.TargetPath = "$env:scriptPath\$env:scriptName.bat"
						$shortcut.Save()
						$autoStart = "true"
					} else {
						Remove-Item "$env:APPDATA\Microsoft\Windows\Start Menu\Programs\Startup\$env:scriptName.lnk" -ErrorAction SilentlyContinue
						$autoStart = "false"
					}
				}

				$content = Get-Content -Path $env:filePath
				$content = $content | Where-Object { $_ -notmatch "color = (.*)" }
				$content = $content | Where-Object { $_ -notmatch "darkTheme = (.*)" }
				$content = $content | Where-Object { $_ -notmatch "menuStyle = (.*)" }
				$content = $content | Where-Object { $_ -notmatch "pingDelay = (.*)" }
				$content = $content | Where-Object { $_ -notmatch "portScanDelay = (.*)" }
				$content = $content | Where-Object { $_ -notmatch "microMacroKey = (.*)" }
				$content = $content | Where-Object { $_ -notmatch "microMacroDelay = (.*)" }
				$content = $content | Where-Object { $_ -notmatch "showHiddenFiles = (.*)" }
				$content = $content | Where-Object { $_ -notmatch "showLogo = (.*)" }
				$content += "color = $currentColor`n"
				$content += "darkTheme = $darkTheme`n"
				$content += "menuStyle = $menuStyle`n"
				$content += "pingDelay = $pingDelay`n"
				$content += "portScanDelay = $portScanDelay`n"
				$content += "microMacroKey = $microMacroKey`n"
				$content += "microMacroDelay = $microMacroDelay`n"
				$content += "showHiddenFiles = $showHiddenFiles`n"
				$content += "showLogo = $showLogo`n"
				$content | Set-Content -Path $env:filePath
				break
			}

			# right
			39 { 
				if ($global:settingsSelection -eq 0) {
					$currentColorIndex = ($currentColorIndex + 1 + $colors.Length) % $colors.Length
					$currentColor = $colors[$currentColorIndex]
					if ($currentColor -eq "White" -and $darkTheme -eq "true") {
						$currentColor = "DarkGray"
					} elseif ($darkTheme -eq "true") {
						$currentColor = "Dark" + $currentColor
					}
				} elseif ($global:settingsSelection -eq 1) {
					if ($currentColor -eq "White" -and $darkTheme -eq "false") {
						$darkTheme = "true"
						$fg = "Gray"
						$currentColor = "DarkGray"
					} elseif ($darkTheme -eq "false") {
						$darkTheme = "true"
						$fg = "Gray"
						$currentColor = "Dark" + $currentColor
					} elseif ($currentColor -eq "DarkGray" -and $darkTheme -eq "true") {
						$darkTheme = "false"
						$fg = "White"
						$currentColor = "White"
					} else {
						$darkTheme = "false"
						$fg = "White"
						$currentColor = $currentColor -replace "Dark"
					}
				} elseif ($global:settingsSelection -eq 2) {
					$menuStyleIndex = ($menuStyleIndex + 1) % $menuStyles.Length
					$menuStyle = $menuStyles[$menuStyleIndex]
				} elseif ($global:settingsSelection -eq 3) {
					$pingDelayIndex = ($pingDelayIndex + 1) % $pingDelays.Length
					$pingDelay = $pingDelays[$pingDelayIndex]
				} elseif ($global:settingsSelection -eq 4) {
					$portScanDelayIndex = ($portScanDelayIndex + 1) % $portScanDelays.Length
					$portScanDelay = $portScanDelays[$portScanDelayIndex]
				} elseif ($global:settingsSelection -eq 5) {
					$microMacroKeyIndex = ($microMacroKeyIndex + 1) % $microMacroKeys.Length
					$microMacroKey = $microMacroKeys[$microMacroKeyIndex]
				} elseif ($global:settingsSelection -eq 6) {
					$microMacroDelayIndex = ($microMacroDelayIndex + 1) % $microMacroDelays.Length
					$microMacroDelay = $microMacroDelays[$microMacroDelayIndex]
				} elseif ($global:settingsSelection -eq 7) {
					if ($showHiddenFiles -eq "false") {
						$showHiddenFiles = "true"
						attrib -h $macroConfigPath | Out-Null
						attrib -h $quickStartFolderPath | Out-Null
						attrib -h $quickDownloadConfigPath | Out-Null
					} else {
						$showHiddenFiles = "false"
						attrib +h $macroConfigPath | Out-Null
						attrib +h $quickStartFolderPath | Out-Null
						attrib +h $quickDownloadConfigPath | Out-Null
					}
				} elseif ($global:settingsSelection -eq 8) {
					if ($showLogo -eq "false") {
						$showLogo = "true"
						$global:renderedLogo = $logo
						$global:renderedLogoLineLength = " " * $global:renderedLogo[0].Length
					} else {
						$showLogo = "false"
						$global:renderedLogo = @()
						$global:renderedLogoLineLength = ""
					}
				} elseif ($global:settingsSelection -eq 9) {
					if (!(Test-Path -Path "$env:APPDATA\Microsoft\Windows\Start Menu\Programs\Startup\$env:scriptName.lnk")) {
						$shell = New-Object -ComObject WScript.Shell
						$shortcut = $shell.CreateShortcut("$env:APPDATA\Microsoft\Windows\Start Menu\Programs\Startup\$env:scriptName.lnk")
						$shortcut.TargetPath = "$env:scriptPath\$env:scriptName.bat"
						$shortcut.Save()
						$autoStart = "true"
					} else {
						Remove-Item "$env:APPDATA\Microsoft\Windows\Start Menu\Programs\Startup\$env:scriptName.lnk" -ErrorAction SilentlyContinue
						$autoStart = "false"
					}
				}

				$content = Get-Content -Path $env:filePath
				$content = $content | Where-Object { $_ -notmatch "color = (.*)" }
				$content = $content | Where-Object { $_ -notmatch "darkTheme = (.*)" }
				$content = $content | Where-Object { $_ -notmatch "menuStyle = (.*)" }
				$content = $content | Where-Object { $_ -notmatch "pingDelay = (.*)" }
				$content = $content | Where-Object { $_ -notmatch "portScanDelay = (.*)" }
				$content = $content | Where-Object { $_ -notmatch "microMacroKey = (.*)" }
				$content = $content | Where-Object { $_ -notmatch "microMacroDelay = (.*)" }
				$content = $content | Where-Object { $_ -notmatch "showHiddenFiles = (.*)" }
				$content = $content | Where-Object { $_ -notmatch "showLogo = (.*)" }
				$content += "color = $currentColor`n"
				$content += "darkTheme = $darkTheme`n"
				$content += "menuStyle = $menuStyle`n"
				$content += "pingDelay = $pingDelay`n"
				$content += "portScanDelay = $portScanDelay`n"
				$content += "microMacroKey = $microMacroKey`n"
				$content += "microMacroDelay = $microMacroDelay`n"
				$content += "showHiddenFiles = $showHiddenFiles`n"
				$content += "showLogo = $showLogo`n"
				$content | Set-Content -Path $env:filePath
				break
			}

			# up
			38 { if ($global:settingsSelection) { $global:settingsSelection-- } else { $global:settingsSelection = $settingsMenu.Length - 1 }; break }
			# down
			40 { if ($global:settingsSelection -lt ($settingsMenu.Length - 1)) { $global:settingsSelection++ } else { $global:settingsSelection = 0 }; break }

			# escape
			27 { exit }

			# tab
			9 { return }

			# numbers
			default {
				if ($key -gt 13) {$global:settingsSelection = $key - 48}
			}
		}
	}
}

function settingsMenu {
	cls
	if ($menuStyle -eq "v1") {
		write-host "$($global:renderedLogo[0])    " -NoNewline -f $currentColor -b $bg
		write-host "$name" -f $currentColor -b $bg
		write-host "$($global:renderedLogo[1])    " -NoNewline -f $currentColor -b $bg
		write-host ("-" * $name.Length) -NoNewline -f $fg -b $bg
		write-host "" -f $fg -b $bg
		for ($i = 0; $item = $menu[$i]; $i++) {
			$global:renderedLogoLine = $global:renderedLogo[$i + 2]
			if ($i -lt $menu.Length) {
				write-host "$global:renderedLogoLine    " -NoNewline -f $currentColor -b $bg
				write-host "$i" -NoNewline -f $currentColor -b $bg
				write-host ": $item" -f $fg -b $bg
			}
		}

		write-host "$($global:renderedLogo[$i + 2])    " -NoNewline -f $currentColor -b $bg
		write-host ("-" * $name.Length) -NoNewline -f $fg -b $bg
		write-host "" -f $fg -b $bg
		for ($i = $menu.Length + 3; $i -lt $settingsMenu.Length + ($menu.Length + 3) + [Math]::Max(0, ($global:renderedLogo.Length - (($menu.Length + 3) + $settingsMenu.Length))); $i++) {
			$global:renderedLogoLine = $global:renderedLogo[$i]
			if ($i - ($menu.Length + 3) -lt $settingsMenu.Length) {
				if ($i - ($menu.Length + 3) -eq $global:settingsSelection) {
					$item = $($settingsMenu[$i - ($menu.Length + 3)])
					if ($i - ($menu.Length + 3) -eq 1) {
						if ($darkTheme -eq "true") {
							write-host "$global:renderedLogoLine   " -NoNewline -f $currentColor -b $bg
							write-host "  > $item < 1  " -f $bg -b $currentColor
						} else {
							write-host "$global:renderedLogoLine   " -NoNewline -f $currentColor -b $bg
							write-host "  > $item < 0  " -f $bg -b $currentColor
						}
					} elseif ($i - ($menu.Length + 3) -eq 2) {
						write-host "$global:renderedLogoLine   " -NoNewline -f $currentColor -b $bg
						write-host "  > $item < $menuStyle  " -f $bg -b $currentColor
					} elseif ($i - ($menu.Length + 3) -eq 3) {
						write-host "$global:renderedLogoLine   " -NoNewline -f $currentColor -b $bg
						write-host "  > $item < $($pingDelay)ms  " -f $bg -b $currentColor
					} elseif ($i - ($menu.Length + 3) -eq 4) {
						write-host "$global:renderedLogoLine   " -NoNewline -f $currentColor -b $bg
						write-host "  > $item < $($portScanDelay)ms  " -f $bg -b $currentColor
					} elseif ($i - ($menu.Length + 3) -eq 5) {
						write-host "$global:renderedLogoLine   " -NoNewline -f $currentColor -b $bg
						write-host "  > $item < $($microMacroKey)  " -f $bg -b $currentColor
					} elseif ($i - ($menu.Length + 3) -eq 6) {
						$microMacroDelay = [int]$microMacroDelay
						if ($microMacroDelay -le 1000) {
							$delayUnit = "ms"
							$displayDelay = $microMacroDelay
						} elseif ($microMacroDelay -gt 60000) {
							$delayUnit = "m"
							$displayDelay = $microMacroDelay / 60000
						} else {
							$delayUnit = "s"
							$displayDelay = $microMacroDelay / 1000
						}
						write-host "$global:renderedLogoLine   " -NoNewline -f $currentColor -b $bg
						write-host "  > $item < $($displayDelay)$($delayUnit)  " -f $bg -b $currentColor
					} elseif ($i - ($menu.Length + 3) -eq 7) {
						if ($showHiddenFiles -eq "true") {
							write-host "$global:renderedLogoLine   " -NoNewline -f $currentColor -b $bg
							write-host "  > $item < 1  " -f $bg -b $currentColor
						} else {
							write-host "$global:renderedLogoLine   " -NoNewline -f $currentColor -b $bg
							write-host "  > $item < 0  " -f $bg -b $currentColor
						}
					} elseif ($i - ($menu.Length + 3) -eq 8) {
						if ($showLogo -eq "true") {
							write-host "$global:renderedLogoLine   " -NoNewline -f $currentColor -b $bg
							write-host "  > $item < 1  " -f $bg -b $currentColor
						} else {
							write-host "$global:renderedLogoLine   " -NoNewline -f $currentColor -b $bg
							write-host "  > $item < 0  " -f $bg -b $currentColor
						}
					} elseif ($i - ($menu.Length + 3) -eq 9) {
						if (!(Test-Path -Path "$env:APPDATA\Microsoft\Windows\Start Menu\Programs\Startup\$env:scriptName.lnk")) {
							write-host "$global:renderedLogoLine   " -NoNewline -f $currentColor -b $bg
							write-host "  > $item < 0  " -f $bg -b $currentColor
						} else {
							write-host "$global:renderedLogoLine   " -NoNewline -f $currentColor -b $bg
							write-host "  > $item < 1  " -f $bg -b $currentColor
						}
					} else {
						write-host "$global:renderedLogoLine   " -NoNewline -f $currentColor -b $bg
						write-host "  > $item <  " -f $bg -b $currentColor
					}
				} else {
					write-host "$global:renderedLogoLine    " -NoNewline -f $currentColor -b $bg
					write-host "$($i - ($menu.Length + 3))" -NoNewline -f $currentColor -b $bg
					write-host ": $($settingsMenu[$i - ($menu.Length + 3)])" -f $fg -b $bg
				}
			} else {
				write-host "$global:renderedLogoLine   " -f $currentColor -b $bg
			}
		}
		write-host ""
	} elseif ($menuStyle -eq "v2") {
		write-host "    $name" -f $currentColor -b $bg
		write-host ("    " + "-" * $name.Length) -NoNewline -f $fg -b $bg
		if ($global:renderedLogo) { write-host "" }
		write-host ($global:renderedLogo -join "`n") -f $currentColor -b $bg
		if ($global:renderedLogo) { write-host "" }
		$maxItemLength = ($menu | Measure-Object -Property Length -Maximum).Maximum
		for ($i = 0; $item = $menu[$i]; $i++) {
			if ($i -lt $menu.Length) {
				if ($i -eq $settingsSelection) {
					write-host "    $i" -NoNewline -f $currentColor -b $bg
					write-host (": $item" + " " * ($maxItemLength - $item.Length + 4)) -NoNewline -f $fg -b $bg
					if ($i -eq 1) {
						if ($darkTheme -eq "true") {
							write-host "  > $($settingsMenu[$i]) < 1  " -f $bg -b $currentColor
						} else {
							write-host "  > $($settingsMenu[$i]) < 0  " -f $bg -b $currentColor
						}
					} elseif ($i -eq 2) {
						write-host "  > $($settingsMenu[$i]) < $menuStyle  " -f $bg -b $currentColor
					} elseif ($i -eq 3) {
						write-host "  > $($settingsMenu[$i]) < $($pingDelay)ms  " -f $bg -b $currentColor
					} elseif ($i -eq 4) {
						write-host "  > $($settingsMenu[$i]) < $($portScanDelay)ms  " -f $bg -b $currentColor
					} elseif ($i -eq 5) {
						write-host "  > $($settingsMenu[$i]) < $($microMacroKey)  " -f $bg -b $currentColor
					} elseif ($i -eq 6) {
						$microMacroDelay = [int]$microMacroDelay
						if ($microMacroDelay -le 1000) {
							$delayUnit = "ms"
							$displayDelay = $microMacroDelay
						} elseif ($microMacroDelay -gt 60000) {
							$delayUnit = "m"
							$displayDelay = $microMacroDelay / 60000
						} else {
							$delayUnit = "s"
							$displayDelay = $microMacroDelay / 1000
						}
						write-host "  > $($settingsMenu[$i]) < $($displayDelay)$($delayUnit)  " -f $bg -b $currentColor
					} elseif ($i -eq 7) {
						if ($showHiddenFiles -eq "true") {
							write-host "  > $($settingsMenu[$i]) < 1  " -f $bg -b $currentColor
						} else {
							write-host "  > $($settingsMenu[$i]) < 0  " -f $bg -b $currentColor
						}
					} elseif ($i -eq 8) {
						if ($showLogo -eq "true") {
							write-host "  > $($settingsMenu[$i]) < 1  " -f $bg -b $currentColor
						} else {
							write-host "  > $($settingsMenu[$i]) < 0  " -f $bg -b $currentColor
						}
					} elseif ($i -eq 9) {
						if (!(Test-Path -Path "$env:APPDATA\Microsoft\Windows\Start Menu\Programs\Startup\$env:scriptName.lnk")) {
							write-host "  > $($settingsMenu[$i]) < 0  " -f $bg -b $currentColor
						} else {
							write-host "  > $($settingsMenu[$i]) < 1  " -f $bg -b $currentColor
						}
					} else {
						write-host "  > $($settingsMenu[$i]) <  " -f $bg -b $currentColor
					}
				} else {
					write-host "    $i" -NoNewline -f $currentColor -b $bg
					write-host (": $item" + " " * ($maxItemLength - $item.Length + 5) + "|") -NoNewline -f $fg -b $bg
					write-host "  $($settingsMenu[$i])" -f $fg -b $bg
				}
			}
		}
		write-host ""
	} else {
		write-host "Menu style version does not exist." -f $currentColor -b $bg
		write-host "Press <escape> key to exit" -f $currentColor -b $bg
		while ($true) {
			[int]$key = getKey
			if ($key -eq 27) {
				exit
			}
		}
	}
}

function menu {
	$menuStyle = Get-Content -Path $env:filePath | Where-Object { $_ -match "menuStyle = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $menuStyle) {
		$menuStyle = "v1"  # default value
	}

	$showLogo = Get-Content -Path $env:filePath | Where-Object { $_ -match "showLogo = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $showLogo) {
		$showLogo = "true"  # default value
	}

	$darkTheme = Get-Content -Path $env:filePath | Where-Object { $_ -match "darkTheme = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $darkTheme) {
		$darkTheme = "false"  # default value
		$fg = "White"
	} elseif ($darkTheme -eq "true") {
		$fg = "Gray"
	} else {
		$fg = "White"
	}

	$currentColor = Get-Content -Path $env:filePath | Where-Object { $_ -match "color = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $currentColor) {
		$currentColor = "White"  # default color
	}

	cls
	if ($menuStyle -eq "v1") {
		write-host "$($global:renderedLogo[0])    " -NoNewline -f $currentColor -b $bg
		write-host "$name" -f $currentColor -b $bg
		write-host "$($global:renderedLogo[1])    " -NoNewline -f $currentColor -b $bg
		write-host ("-" * $name.Length) -NoNewline -f $fg -b $bg
		write-host "" -f $fg -b $bg
		for ($i = 0; $item = $menu[$i]; $i++) {
			$global:renderedLogoLine = $global:renderedLogo[$i + 2]
			if ($i -lt $menu.Length) {
				if ($i -eq $selection) {
					write-host "$global:renderedLogoLine   " -NoNewline -f $currentColor -b $bg
					write-host "  > $item <  " -f $bg -b $currentColor
				} else {
					write-host "$global:renderedLogoLine    " -NoNewline -f $currentColor -b $bg
					write-host "$i" -NoNewline -f $currentColor -b $bg
					write-host ": $item" -f $fg -b $bg
				}
			}
		}
		1

		write-host "$($global:renderedLogo[$i + 2])    " -NoNewline -f $currentColor -b $bg
		write-host ("-" * $name.Length) -NoNewline -f $fg -b $bg
		write-host "" -f $fg -b $bg
		for ($i = $menu.Length + 3; $i -lt $settingsMenu.Length + ($menu.Length + 3) + [Math]::Max(0, ($global:renderedLogo.Length - (($menu.Length + 3) + $settingsMenu.Length))); $i++) {
			$global:renderedLogoLine = $global:renderedLogo[$i]
			if ($i - ($menu.Length + 3) -lt $settingsMenu.Length) {
				write-host "$global:renderedLogoLine    " -NoNewline -f $currentColor -b $bg
				write-host "$($i - ($menu.Length + 3))" -NoNewline -f $currentColor -b $bg
				write-host ": $($settingsMenu[$i - ($menu.Length + 3)])" -f $fg -b $bg
			} else {
				write-host "$global:renderedLogoLine    " -f $currentColor -b $bg
			}
		}
		write-host ""
	} elseif ($menuStyle -eq "v2") {
		write-host "    $name" -f $currentColor -b $bg
		write-host ("    " + "-" * $name.Length) -NoNewline -f $fg -b $bg
		if ($global:renderedLogo) { write-host "" }
		write-host ($global:renderedLogo -join "`n") -f $currentColor -b $bg
		if ($global:renderedLogo) { write-host "" }
		$maxItemLength = ($menu | Measure-Object -Property Length -Maximum).Maximum
		for ($i = 0; $item = $menu[$i]; $i++) {
			if ($i -lt $menu.Length) {
				if ($i -eq $selection) {
					write-host "   " -NoNewline
					write-host "  > $item <  " -NoNewline -f $bg -b $currentColor
					write-host (" " * ($maxItemLength - $item.Length + 1) + "|") -NoNewline -f $fg -b $bg
					write-host " " $settingsMenu[$i] -f $fg -b $bg
				} else {
					write-host "    $i" -NoNewline -f $currentColor -b $bg
					write-host (": $item" + " " * ($maxItemLength - $item.Length + 5) + "|") -NoNewline -f $fg -b $bg
					write-host " " $settingsMenu[$i] -f $fg -b $bg
				}
			}
		}
		1
		write-host ""
	} else {
		write-host "Menu style version does not exist." -f $currentColor -b $bg
		write-host "Press <escape> key to exit" -f $currentColor -b $bg
		while ($true) {
			[int]$key = getKey
			if ($key -eq 27) {
				exit
			}
		}
	}
}

while (menu) {
	[int]$key = getKey
	switch ($key) {

		# left or up
		37 { if ($selection) { $selection-- } else { $selection = $menu.Length - 1 }; break }
		38 { if ($selection) { $selection-- } else { $selection = $menu.Length - 1 }; break }
		# right or down
		39 { if ($selection -lt ($menu.Length - 1)) { $selection++ } else { $selection = 0 }; break }
		40 { if ($selection -lt ($menu.Length - 1)) { $selection++ } else { $selection = 0 }; break }

		# escape
		27 { exit }

		# tab
		9 { settings }

		# space
		32 {
			$showHiddenFiles = Get-Content -Path $env:filePath | Where-Object { $_ -match "showHiddenFiles = (.*)" } | ForEach-Object { $matches[1] }
			if (-not $showHiddenFiles) {
				$showHiddenFiles = "false"  # default value
			}

			switch ($selection) {
				5 {
					if (!(Test-Path -Path $macroConfigPath)) {
						write-host "Creating Macro config file" -f $fg -b $bg
						New-Item -Path $macroConfigPath -ItemType File | Out-Null
						if ($showHiddenFiles -eq "false") {
							attrib +h $macroConfigPath
						}
						Invoke-Item $macroConfigPath
					} else {
						Invoke-Item $macroConfigPath
					}
				}
				6 {
					if (!(Test-Path -Path $quickStartFolderPath)) {
						write-host "Creating QuickStart folder" -f $fg -b $bg
						New-Item -Path $quickStartFolderPath -ItemType Directory | Out-Null
						if ($showHiddenFiles -eq "false") {
							attrib +h $quickStartFolderPath
						}
						Invoke-Item $quickStartFolderPath
					} else {
						Invoke-Item $quickStartFolderPath
					}
				}
				7 {
					if (!(Test-Path -Path $quickDownloadConfigPath)) {
						write-host "Creating QuickDownload config file" -f $fg -b $bg
						New-Item -Path $quickDownloadConfigPath -ItemType File | Out-Null
						if ($showHiddenFiles -eq "false") {
							attrib +h $quickDownloadConfigPath
						}
						Invoke-Item $quickDownloadConfigPath
					} else {
						Invoke-Item $quickDownloadConfigPath
					}
				}
			}
			break
		}

		# number or enter
		default {
			if ($key -gt 13) {$selection = $key - 48}
			switch ($selection) {
				0 { systemFetch }
				1 { cleanup }
				2 { pingTool }
				3 { portScan }
				4 { microMacro }
				5 { macro }
				6 { quickStart }
				7 { quickDownload }
				8 { gameOfLife }
			}
		}
	}
}
