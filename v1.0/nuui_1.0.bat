<# : Batch portion
@echo off & setlocal enabledelayedexpansion
a
set "scriptPath=%~dp0"
set "filePath=%scriptPath%NUUI_settings.txt"

if not exist "%filePath%" (
	echo Creating settings file at "%filePath%"
	echo color = White>> "%filePath%"
	echo pingDelay = 500>> "%filePath%"
	echo portScanDelay = 500>> "%filePath%"
	echo microMacroKey = F15>> "%filePath%"
	echo microMacroDelay = 30000>> "%filePath%"
	attrib +h "%filePath%"
)

set "menu[0]=settings"
set "menu[1]=ping"
set "menu[2]=port_scan"
set "menu[3]=cleanup"
set "menu[4]=macro"
set "menu[5]=micro_macro"
set "menu[6]=quick_start"
set "menu[7]=quick_download"

set "default=0"

powershell -noprofile -executionpolicy remotesigned "iex ((gc '%~f0') -join \"`n\")"

: end batch / begin PowerShell hybrid chimera #>

$menu = gci env: | ?{ $_.Name -match "^menu\[\d+\]$" } | %{ $_.Value }
[int]$selection = $env:default
$fg = $Host.UI.RawUI.ForegroundColor
$bg = $Host.UI.RawUI.BackgroundColor

function getKey {
	while (-not ((37..40 + 13 + 48..55 + 27) -contains $x)) {
		$x = $Host.UI.RawUI.ReadKey('NoEcho,IncludeKeyDown').VirtualKeyCode
	}
	$x
}

function PingTool {
	cls
	$currentColor = Get-Content -Path $env:filePath | Where-Object { $_ -match "color = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $currentColor) {
		$currentColor = "White"  # default color
	}

	$pingDelay = Get-Content -Path $env:filePath | Where-Object { $_ -match "pingDelay = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $pingDelay) {
		$pingDelay = 500  # default delay
	}

	write-host "
	      ::::    :::         :::    :::         :::    :::          ::::::::::: 
	     :+:+:   :+:         :+:    :+:         :+:    :+:              :+:      
	    :+:+:+  +:+         +:+    +:+         +:+    +:+              +:+       
	   +#+ +:+ +#+         +#+    +:+         +#+    +:+              +#+        
	  +#+  +#+#+#         +#+    +#+         +#+    +#+              +#+         
	 #+#   #+#+#         #+#    #+#         #+#    #+#              #+#          
	###    ####          ########           ########           ###########       v1.0

	                                Ping

	" -f $currentColor -b $bg

	$target = Read-Host "Enter IP address"
	Write-Host "Pinging $target" -f $currentColor
	$ping = New-Object System.Net.NetworkInformation.Ping

	Write-Host "Press <escape> key to stop" -f $currentColor
	$host.UI.RawUI.FlushInputBuffer()

	while ($true) {
		$result = $ping.Send($target)
		Write-Host "Reply from $($result.Address): bytes=$($result.Buffer.Length) time=$($result.RoundtripTime)ms TTL=$($result.Options.Ttl)"
		Start-Sleep -m $pingDelay

		if ($host.UI.RawUI.KeyAvailable) {
			$key = $host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
			if ($key.VirtualKeyCode -eq 27) {
				break
			}
		}
	}
}

function PortScan {
    cls
    $currentColor = Get-Content -Path $env:filePath | Where-Object { $_ -match "color = (.*)" } | ForEach-Object { $matches[1] }
    if (-not $currentColor) {
        $currentColor = "White"  # default color
    }

    $portScanDelay = Get-Content -Path $env:filePath | Where-Object { $_ -match "portScanDelay = (.*)" } | ForEach-Object { $matches[1] }
    if (-not $portScanDelay) {
        $portScanDelay = 500  # default delay
    }

    write-host "
          ::::    :::         :::    :::         :::    :::          ::::::::::: 
         :+:+:   :+:         :+:    :+:         :+:    :+:              :+:      
        :+:+:+  +:+         +:+    +:+         +:+    +:+              +:+       
       +#+ +:+ +#+         +#+    +#+         +#+    +#+              +#+        
      +#+  +#+#+#         +#+    +#+         +#+    +#+              +#+         
     #+#   #+#+#         #+#    #+#         #+#    #+#              #+#          
    ###    ####          ########           ########           ###########       v1.0

                                Port Scan

    " -f $currentColor -b $bg

    $target = Read-Host "Enter IP address"
    $portRange = Read-Host "Enter a port range ( - in between)"
    $startPort, $endPort = $portRange -split "-"
    $startPort = [int]$startPort
    $endPort = [int]$endPort

    Write-Host "Scanning $target for open ports $startPort-$endPort" -f $currentColor
	Write-Host "Press <escape> key to stop" -f $currentColor

    $ports = @()
    for ($port = $startPort; $port -le $endPort; $port++) {
        $socket = New-Object System.Net.Sockets.TcpClient
        if ($socket.ConnectAsync($target, $port).Wait($portScanDelay)) {
            $ports += $port
            Write-Host "Port $port is open" -f $currentColor
        } else {
            Write-Host "Port $port is closed"
        }
        if ([Console]::KeyAvailable) {
            $key = [Console]::ReadKey($true)
            if ($key.Key -eq [ConsoleKey]::Escape) {
                Write-Host "Scan cancelled"
                return
            }
        }
    }
    Write-Host "Open ports: $($ports -join ", ")" -f $currentColor
    Read-Host "Press <enter> key to continue"
}

function CleanupByNexsq {
	Remove-Item -Path c:\windows\temp\* -Force -Recurse
	Remove-Item -Path C:\WINDOWS\Prefetch\* -Force
	Remove-Item -Path $env:TEMP\* -Force -Recurse
	New-Item -Path c:\windows\temp -ItemType Directory
	New-Item -Path $env:TEMP -ItemType Directory
}

function macro {
    cls
    $currentColor = Get-Content -Path $env:filePath | Where-Object { $_ -match "color = (.*)" } | ForEach-Object { $matches[1] }
    if (-not $currentColor) {
        $currentColor = "White"  # default color
    }

    write-host "
          ::::    :::         :::    :::         :::    :::          ::::::::::: 
         :+:+:   :+:         :+:    :+:         :+:    :+:              :+:      
        :+:+:+  +:+         +:+    +:+         +:+    +:+              +:+       
       +#+ +:+ +#+         +#+    +#+         +#+    +#+              +#+        
      +#+  +#+#+#         +#+    +#+         +#+    +#+              +#+         
     #+#   #+#+#         #+#    #+#         #+#    #+#              #+#          
    ###    ####          ########           ########           ###########       v1.0

                                Macro

    " -f $currentColor -b $bg

    $macroConfigPath = Join-Path -Path $env:scriptPath -ChildPath "NUUI_MacroConfig.txt"
    if (!(Test-Path -Path $macroConfigPath)) {
        New-Item -Path $macroConfigPath -ItemType File
        Write-Host "Creating Macro config file"
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
                    } elseif ($line -eq "RandomNum") {
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

        Write-Host "Simulating keys"
		Write-Host "Press <escape> key to stop" -f $currentColor
        while ($true) {
            $key = $Host.UI.RawUI.ReadKey('NoEcho,IncludeKeyDown').VirtualKeyCode
            if ($key -eq 27) {
                $stop = $true
                $thread.Stop()
                $thread.Dispose()
                $handle.AsyncWaitHandle.WaitOne()
                return
            }
        }
    }
}

function MicroMacro {
	cls
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

	write-host "
	      ::::    :::         :::    :::         :::    :::          ::::::::::: 
	     :+:+:   :+:         :+:    :+:         :+:    :+:              :+:      
	    :+:+:+  +:+         +:+    +:+         +:+    +:+              +:+       
	   +#+ +:+ +#+         +#+    +#+         +#+    +#+              +#+        
	  +#+  +#+#+#         +#+    +#+         +#+    +#+              +#+         
	 #+#   #+#+#         #+#    #+#         #+#    #+#              #+#          
	###    ####          ########           ########           ###########       v1.0

	                             Micro Macro

	" -f $currentColor -b $bg

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
		Write-Host "Simulating random number every "$displayDelay$delayUnit
		Write-Host "Press <escape> key to stop" -f $currentColor
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
			$key = $Host.UI.RawUI.ReadKey('NoEcho,IncludeKeyDown').VirtualKeyCode
			if ($key -eq 27) {
				$stop = $true
				$thread.Stop()
				$thread.Dispose()
				$handle.AsyncWaitHandle.WaitOne()
				return
			}
		}
	} else {
		Write-Host "Simulating"$microMacroKey" key every "$displayDelay$delayUnit
		Write-Host "Press <escape> key to stop" -f $currentColor
	
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
			$key = $Host.UI.RawUI.ReadKey('NoEcho,IncludeKeyDown').VirtualKeyCode
			if ($key -eq 27) {
				$stop = $true
				$thread.Stop()
				$thread.Dispose()
				$handle.AsyncWaitHandle.WaitOne()
				return
			}
		}
	}
}

function QuickStart {
	$quickStartFolderPath = Join-Path -Path $env:scriptPath -ChildPath "NUUI_QuickStartFolder"
	if (!(Test-Path -Path $quickStartFolderPath)) {
		New-Item -Path $quickStartFolderPath -ItemType Directory
		Write-Host "Creating QuickStart folder"
	} else {
		Write-Host "QuickStart folder exists, executing files"
	}

	$files = Get-ChildItem -Path $quickStartFolderPath -File
	foreach ($file in $files) {
		Write-Host "Executing file: $($file.Name)"
		Start-Job -ScriptBlock { & $args[0] } -ArgumentList $file.FullName | Out-Null
	}
}

function QuickDownload {
	cls
	$currentColor = Get-Content -Path $env:filePath | Where-Object { $_ -match "color = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $currentColor) {
		$currentColor = "White"  # default color
	}

	write-host "
	      ::::    :::         :::    :::         :::    :::          ::::::::::: 
	     :+:+:   :+:         :+:    :+:         :+:    :+:              :+:      
	    :+:+:+  +:+         +:+    +:+         +:+    +:+              +:+       
	   +#+ +:+ +#+         +#+    +#+         +#+    +#+              +#+        
	  +#+  +#+#+#         +#+    +#+         +#+    +#+              +#+         
	 #+#   #+#+#         #+#    #+#         #+#    #+#              #+#          
	###    ####          ########           ########           ###########       v1.0

	                            Quick Download

	" -f $currentColor -b $bg

	$quickDownloadConfigPath = Join-Path -Path $env:scriptPath -ChildPath "NUUI_QuickDownloadConfig.txt"
	if (!(Test-Path -Path $quickDownloadConfigPath)) {
		New-Item -Path $quickDownloadConfigPath -ItemType File
		Write-Host "Creating QuickDownload config file"
	} else {
		Write-Host "QuickDownload config file exists, downloading items"
		Write-Host "Press <escape> key to stop" -f $currentColor
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
						Write-Host "Download cancelled"
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
				Write-Host "Downloading $filename - $percentComplete%" -NoNewline
				[Console]::CursorLeft = 0
			} while ($bytesRead -gt 0)
			$stream.Close()
			$responseStream.Close()
			Write-Host "Download complete - $filename"
			Move-Item -Path $destinationPath -Destination $filename -Force
		}
	}
}

function settings {
	$settingsMenu = @("color", "dark_theme", "ping_delay", "port_scan_delay", "micro_macro_key", "micro_macro_delay", "auto_start", "reset_settings")
	[int]$settingsSelection = 0

	$colors = @("White", "Red", "Yellow", "Green", "Cyan", "Blue", "Magenta")
	$currentColorIndex = 0
	$currentColor = Get-Content -Path $env:filePath | Where-Object { $_ -match "color = (.*)" } | ForEach-Object { $matches[1] }
	if ($currentColor -eq "White" -and $darkTheme -eq "true") {
		$currentColor = "DarkGray"
	} elseif ($darkTheme -eq "true") {
		$currentColor = "Dark" + $currentColor
	}
	if (-not $currentColor) {
		$currentColor = "White"  # default color
	} else {
		$currentColorIndex = [Array]::IndexOf($colors, $currentColor)
	}

	$darkTheme = Get-Content -Path $env:filePath | Where-Object { $_ -match "darkTheme = (.*)" } | ForEach-Object { $matches[1] }
    if (-not $darkTheme) {
        $darkTheme = "false"  # default value
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

	while (1) {
		settingsMenu
		[int]$key = getKey
		switch ($key) {

			# left
			37 { 
				if ($settingsSelection -eq 0) {
				    $currentColorIndex = ($currentColorIndex - 1 + $colors.Length) % $colors.Length
				    $currentColor = $colors[$currentColorIndex]
					if ($currentColor -eq "White" -and $darkTheme -eq "true") {
						$currentColor = "DarkGray"
					} elseif ($darkTheme -eq "true") {
						$currentColor = "Dark" + $currentColor
					}
				} elseif ($settingsSelection -eq 1) {
					if ($currentColor -eq "White" -and $darkTheme -eq "false") {
						$darkTheme = "true"
						$currentColor = "DarkGray"
					} elseif ($darkTheme -eq "false") {
						$darkTheme = "true"
						$currentColor = "Dark" + $currentColor
					} elseif ($currentColor -eq "DarkGray" -and $darkTheme -eq "true") {
						$darkTheme = "false"
						$currentColor = "White"
					} else {
						$darkTheme = "false"
						$currentColor = $currentColor -replace "Dark"
					}
				} elseif ($settingsSelection -eq 2) {
				    $pingDelayIndex = ($pingDelayIndex - 1 + $pingDelays.Length) % $pingDelays.Length
				    $pingDelay = $pingDelays[$pingDelayIndex]
				} elseif ($settingsSelection -eq 3) {
				    $portScanDelayIndex = ($portScanDelayIndex - 1 + $portScanDelays.Length) % $portScanDelays.Length
				    $portScanDelay = $portScanDelays[$portScanDelayIndex]
				} elseif ($settingsSelection -eq 4) {
				    $microMacroKeyIndex = ($microMacroKeyIndex - 1 + $microMacroKeys.Length) % $microMacroKeys.Length
				    $microMacroKey = $microMacroKeys[$microMacroKeyIndex]
				} elseif ($settingsSelection -eq 5) {
				    $microMacroDelayIndex = ($microMacroDelayIndex - 1 + $microMacroDelays.Length) % $microMacroDelays.Length
				    $microMacroDelay = $microMacroDelays[$microMacroDelayIndex]
				} elseif ($settingsSelection -eq 6) {
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
				} elseif ($settingsSelection -eq 7) {
					Remove-Item -Path $env:filePath -Force
					exit
				}

				$content = Get-Content -Path $env:filePath
				$content = $content | Where-Object { $_ -notmatch "color = (.*)" }
				$content = $content | Where-Object { $_ -notmatch "darkTheme = (.*)" }
				$content = $content | Where-Object { $_ -notmatch "pingDelay = (.*)" }
				$content = $content | Where-Object { $_ -notmatch "portScanDelay = (.*)" }
				$content = $content | Where-Object { $_ -notmatch "microMacroKey = (.*)" }
				$content = $content | Where-Object { $_ -notmatch "microMacroDelay = (.*)" }
				$content += "color = $currentColor`n"
				$content += "darkTheme = $darkTheme`n"
				$content += "pingDelay = $pingDelay`n"
				$content += "portScanDelay = $portScanDelay`n"
				$content += "microMacroKey = $microMacroKey`n"
				$content += "microMacroDelay = $microMacroDelay`n"
				$content | Set-Content -Path $env:filePath
				break
			}

			# right
			39 { 
				if ($settingsSelection -eq 0) {
				    $currentColorIndex = ($currentColorIndex + 1) % $colors.Length
				    $currentColor = $colors[$currentColorIndex]
					if ($currentColor -eq "White" -and $darkTheme -eq "true") {
						$currentColor = "DarkGray"
					} elseif ($darkTheme -eq "true") {
						$currentColor = "Dark" + $currentColor
					}
				} elseif ($settingsSelection -eq 1) {
					if ($currentColor -eq "White" -and $darkTheme -eq "false") {
						$darkTheme = "true"
						$currentColor = "DarkGray"
					} elseif ($darkTheme -eq "false") {
						$darkTheme = "true"
						$currentColor = "Dark" + $currentColor
					} elseif ($currentColor -eq "DarkGray" -and $darkTheme -eq "true") {
						$darkTheme = "false"
						$currentColor = "White"
					} else {
						$darkTheme = "false"
						$currentColor = $currentColor -replace "Dark"
					}
				} elseif ($settingsSelection -eq 2) {
				    $pingDelayIndex = ($pingDelayIndex + 1) % $pingDelays.Length
				    $pingDelay = $pingDelays[$pingDelayIndex]
				} elseif ($settingsSelection -eq 3) {
				    $portScanDelayIndex = ($portScanDelayIndex + 1) % $portScanDelays.Length
				    $portScanDelay = $portScanDelays[$portScanDelayIndex]
				} elseif ($settingsSelection -eq 4) {
				    $microMacroKeyIndex = ($microMacroKeyIndex + 1) % $microMacroKeys.Length
				    $microMacroKey = $microMacroKeys[$microMacroKeyIndex]
				} elseif ($settingsSelection -eq 5) {
				    $microMacroDelayIndex = ($microMacroDelayIndex + 1) % $microMacroDelays.Length
				    $microMacroDelay = $microMacroDelays[$microMacroDelayIndex]
				} elseif ($settingsSelection -eq 6) {
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
				} elseif ($settingsSelection -eq 7) {
					Remove-Item -Path $env:filePath -Force
					exit
				}

				$content = Get-Content -Path $env:filePath
				$content = $content | Where-Object { $_ -notmatch "color = (.*)" }
				$content = $content | Where-Object { $_ -notmatch "darkTheme = (.*)" }
				$content = $content | Where-Object { $_ -notmatch "pingDelay = (.*)" }
				$content = $content | Where-Object { $_ -notmatch "portScanDelay = (.*)" }
				$content = $content | Where-Object { $_ -notmatch "microMacroKey = (.*)" }
				$content = $content | Where-Object { $_ -notmatch "microMacroDelay = (.*)" }
				$content += "color = $currentColor`n"
				$content += "darkTheme = $darkTheme`n"
				$content += "pingDelay = $pingDelay`n"
				$content += "portScanDelay = $portScanDelay`n"
				$content += "microMacroKey = $microMacroKey`n"
				$content += "microMacroDelay = $microMacroDelay`n"
				$content | Set-Content -Path $env:filePath
				break
			}

			# up
			38 { if ($settingsSelection) { $settingsSelection-- } else { $settingsSelection = $settingsMenu.Length - 1 }; break }
			# down
			40 { if ($settingsSelection -lt ($settingsMenu.Length - 1)) { $settingsSelection++ } else { $settingsSelection = 0 }; break }

			# escape
			27 { return }
		}
	}
}

function settingsMenu {
	cls

	$date = Get-Date
    $currentTime = $date.ToString("HH:mm")

	write-host "
	      ::::    :::         :::    :::         :::    :::          ::::::::::: 
	     :+:+:   :+:         :+:    :+:         :+:    :+:              :+:      
	    :+:+:+  +:+         +:+    +:+         +:+    +:+              +:+       
	   +#+ +:+ +#+         +#+    +:+         +#+    +:+              +#+        
	  +#+  +#+#+#         +#+    +#+         +#+    +#+              +#+         
	 #+#   #+#+#         #+#    #+#         #+#    #+#              #+#          
	###    ####          ########           ########           ###########       v1.0

	                               Settings

	                                $currentTime

	" -f $currentColor -b $bg

	for ($i=0; $item = $settingsMenu[$i]; $i++) {
		if ($i -eq $settingsSelection) {
			if ($i -eq 1) {
				if ($darkTheme -eq "true") {
						write-host "  > $item < 1  " -f $bg -b $currentColor
					} else {
						write-host "  > $item < 0  " -f $bg -b $currentColor
					}
			} elseif ($i -eq 2) {
				write-host "  > $item < $($pingDelay)ms  " -f $bg -b $currentColor
			} elseif ($i -eq 3) {
				write-host "  > $item < $($portScanDelay)ms  " -f $bg -b $currentColor
			} elseif ($i -eq 4) {
				write-host "  > $item < $($microMacroKey)  " -f $bg -b $currentColor
			} elseif ($i -eq 5) {
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
				write-host "  > $item < $($displayDelay)$($delayUnit)  " -f $bg -b $currentColor
			} elseif ($i -eq 6) {
				if (!(Test-Path -Path "$env:APPDATA\Microsoft\Windows\Start Menu\Programs\Startup\$env:scriptName.lnk")) {
						write-host "  > $item < 0  " -f $bg -b $currentColor
					} else {
						write-host "  > $item < 1  " -f $bg -b $currentColor
					}
			} else {
				write-host "  > $item <  " -f $bg -b $currentColor
			}
		} else {
			write-host " $i`: $item" -f $fg -b $bg
		}
	}
}

function menu {
	cls
	$currentColor = Get-Content -Path $env:filePath | Where-Object { $_ -match "color = (.*)" } | ForEach-Object { $matches[1] }
	if (-not $currentColor) {
		$currentColor = "White"  # default color
	}

	$date = Get-Date
    $currentTime = $date.ToString("HH:mm")

	cls
	write-host "
	      ::::    :::         :::    :::         :::    :::          ::::::::::: 
	     :+:+:   :+:         :+:    :+:         :+:    :+:              :+:      
	    :+:+:+  +:+         +:+    +:+         +:+    +:+              +:+       
	   +#+ +:+ +#+         +#+    +:+         +#+    +:+              +#+        
	  +#+  +#+#+#         +#+    +#+         +#+    +#+              +#+         
	 #+#   #+#+#         #+#    #+#         #+#    #+#              #+#          
	###    ####          ########           ########           ###########       v1.0

	  Nexsq's                      Useless                          UI

	                                $currentTime

	" -f $currentColor -b $bg

	for ($i=0; $item = $menu[$i]; $i++) {
		if ($i -eq $selection) {
			write-host "  > $item <  " -f $bg -b $currentColor
		} else {
			write-host " $i`: $item" -f $fg -b $bg
		}
	}
	1
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

		# number or enter
		default { 
			if ($key -gt 13) {$selection = $key - 48};
			switch ($selection) {
				0 { settings }
				1 { PingTool }
				2 { PortScan }
				3 { CleanupByNexsq }
				4 { macro }
				5 { MicroMacro }
				6 { QuickStart }
				7 { QuickDownload }
			}
		}
	}
}
