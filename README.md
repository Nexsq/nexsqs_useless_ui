<h1>nexsqs_useless_ui</h1>
<h5>A completely useless terminal UI originally in chimera powershell, now in rust.</h5>
<details>
  <summary><h4>Version 1.0</h4></summary>
  <h6>&nbsp;(written in chimera powershell)</h6>
  <h6>&nbsp;(please do not use this version, there is no point in that)</h6>
  <h4>&nbsp;• ping</h4>
    <span>&nbsp;&nbsp;&nbsp;pings a selected IP every settings.ping_delay<br>
    &nbsp;&nbsp;&nbsp;needs a refresh to ping (not intentional)</span>
  <h4>&nbsp;• port_scan</h4>
    <span>&nbsp;&nbsp;&nbsp;scans a selected IP for open ports in a given range, where the timeout is settings.port_scan_delay</span>
  <h4>&nbsp;• cleanup</h4>
    <span>&nbsp;&nbsp;&nbsp;clears those directories:<br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>C:\Windows\Temp\*</code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>C:\WINDOWS\Prefetch\*</code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>$env:TEMP\*</code></span>
  <h4>&nbsp;• macro</h4>
    <span>&nbsp;&nbsp;&nbsp;on the first launch, it creates a "NUUI_MacroConfig.txt" file in the working directory<br>
    &nbsp;&nbsp;&nbsp;if the file "NUUI_MacroConfig.txt" is found, reads and executes valid commands:<br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>sleep &lt;milliseconds&gt;</code><i> (sleeps for a given duration)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>Enter</code><i> (simulates a click of enter)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>Space</code><i> (simulates a click of space)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>RandomNum</code><i> (simulates a click of a random number in range 0-9)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>n?</code><i> (simulates a click of the current value of the variable n)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>n++</code><i> (increments the n variable by one)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>n--</code><i> (decrements the n variable by one)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>loop &lt;times&gt;</code><i> (put at the end of the macro to determine how many times it will replay)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>&lt;any other symbol or some keys&gt;</code><i> (simulates a click of that symbol or that key)</i><br>
    &nbsp;&nbsp;&nbsp;example NUUI_MacroConfig.txt:<br></span>
      <h6>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>sleep 1000    </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>N             </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>U             </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>U             </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>I             </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>sleep 100     </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>Space         </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>sleep 100     </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>RandomNum     </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>sleep 100     </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>Enter         </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>sleep 1000    </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>n?            </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>n++           </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>n?            </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>n--           </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>n?            </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>sleep 100     </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>1             </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>2             </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>3             </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>4             </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>sleep 100     </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>F11           </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>sleep 100     </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>!             </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>@             </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>#             </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>$             </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>loop 10       </code><br></h6>
  <h4>&nbsp;• micro_macro</h4>
    <span>&nbsp;&nbsp;&nbsp;simulates a click of settings.micro_macro_key every settings.micro_macro_delay</span>
  <h4>&nbsp;• quick_start</h4>
    <span>&nbsp;&nbsp;&nbsp;on the first launch, it creates a "NUUI_QuickStartFolder" folder in the working directory<br>
    &nbsp;&nbsp;&nbsp;if the folder "NUUI_QuickStartFolder" is found, opens all files in that folder</span>
  <h4>&nbsp;• quick_download</h4>
    <span>&nbsp;&nbsp;&nbsp;on the first launch, it creates a "NUUI_QuickDownloadConfig.txt" file in the working directory<br>
    &nbsp;&nbsp;&nbsp;if the file "NUUI_QuickDownloadConfig.txt" is found, downloads files from links in that file<i> (one line for one file)</i></span>
</details>
<details>
  <summary><h4>Version 2.0</h4></summary>
  <h6>&nbsp;(written in chimera powershell)</h6>
  <h4>&nbsp;• sys_fetch</h4>
    <span>&nbsp;&nbsp;&nbsp;shows various information about the system</span>
  <h4>&nbsp;• cleanup</h4>
    <span>&nbsp;&nbsp;&nbsp;clears those directories:<br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>C:\Windows\Temp\*</code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>C:\WINDOWS\Prefetch\*</code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>$env:TEMP\*</code></span>
  <h4>&nbsp;• ping_tool</h4>
    <span>&nbsp;&nbsp;&nbsp;pings a selected IP every settings.ping_delay</span>
  <h4>&nbsp;• port_scan</h4>
    <span>&nbsp;&nbsp;&nbsp;scans a selected IP for open ports in a given range, where the timeout is settings.port_scan_delay</span>
  <h4>&nbsp;• micro_macro</h4>
    <span>&nbsp;&nbsp;&nbsp;simulates a click of settings.micro_macro_key every settings.micro_macro_delay</span>
  <h4>&nbsp;• macro</h4>
    <span>&nbsp;&nbsp;&nbsp;on the first launch, it creates a "NUUI_MacroConfig.txt" file in the working directory<br>
    &nbsp;&nbsp;&nbsp;if the file "NUUI_MacroConfig.txt" is found, reads and executes valid commands:<br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>sleep &lt;milliseconds&gt;</code><i> (sleeps for a given duration)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>Enter</code><i> (simulates a click of enter)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>Space</code><i> (simulates a click of space)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>RanNum</code><i> (simulates a click of a random number in range 0-9)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>n?</code><i> (simulates a click of the current value of the variable n)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>n++</code><i> (increments the n variable by one)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>n--</code><i> (decrements the n variable by one)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>loop &lt;times&gt;</code><i> (put at the end of the macro to determine how many times it will replay)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>&lt;any other symbol or some keys&gt;</code><i> (simulates a click of that symbol or that key)</i></span>
    &nbsp;&nbsp;&nbsp;example NUUI_MacroConfig.txt:<br></span>
      <h6>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>sleep 1000    </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>N             </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>U             </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>U             </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>I             </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>sleep 100     </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>Space         </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>sleep 100     </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>RanNum        </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>sleep 100     </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>Enter         </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>sleep 1000    </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>n?            </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>n++           </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>n?            </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>n--           </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>n?            </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>sleep 100     </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>1             </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>2             </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>3             </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>4             </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>sleep 100     </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>F11           </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>sleep 100     </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>!             </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>@             </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>#             </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>$             </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>loop 10       </code><br></h6>
  <h4>&nbsp;• quick_start</h4>
    <span>&nbsp;&nbsp;&nbsp;on the first launch, it creates a "NUUI_QuickStartFolder" folder in the working directory<br>
    &nbsp;&nbsp;&nbsp;if the folder "NUUI_QuickStartFolder" is found, opens all files in that folder</span>
  <h4>&nbsp;• quick_download</h4>
    <span>&nbsp;&nbsp;&nbsp;on the first launch, it creates a "NUUI_QuickDownloadConfig.txt" file in the working directory<br>
    &nbsp;&nbsp;&nbsp;if the file "NUUI_QuickDownloadConfig.txt" is found, downloads files from links in that file<i> (one line for one file)</i></span>
  <h4>&nbsp;• game_of_life</h4>
    <span>&nbsp;&nbsp;&nbsp;find out yourself ;)</span>
</details>
<details>
  <summary><h4>Version 3.0</h4></summary>
  <h6>&nbsp;(written in rust)</h6>
  <h4>&nbsp;• ping_tool</h4>
    <span>&nbsp;&nbsp;&nbsp;pings a selected IP every settings.ping_delay</span>
  <h4>&nbsp;• port_scan</h4>
    <span>&nbsp;&nbsp;&nbsp;scans a selected IP for open ports in a given range, where the timeout is settings.port_scan_timeout</span>
  <h4>&nbsp;• micro_macro</h4>
    <span>&nbsp;&nbsp;&nbsp;simulates a click of settings.micro_macro_key every settings.micro_macro_delay</br>
    &nbsp;&nbsp;&nbsp;can also set a hotkey<i> (settings.micro_macro_hotkey)</i> </span>
  <h4>&nbsp;• macro</h4>
    <span>&nbsp;&nbsp;&nbsp;on the first launch, it creates a "NUUI_MacroConfig.txt" file in the working directory<br>
    &nbsp;&nbsp;&nbsp;if the file "NUUI_MacroConfig.txt" is found, reads and executes valid commands:<br>
    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>sleep &lt;milliseconds&gt;</code><i> (sleeps for a given duration)</i><br>
    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>Enter</code><i> (simulates a click of enter)</i><br>
    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>Space</code><i> (simulates a click of space)</i><br>
    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>RanNum</code><i> (simulates a click of a random number in range 0-9)</i><br>
    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>n?</code><i> (simulates a click of the current value of the variable n)</i><br>
    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>n++</code><i> (increments the n variable by one)</i><br>
    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>n--</code><i> (decrements the n variable by one)</i><br>
    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>loop &lt;times&gt;</code><i> (put at the end of the macro to determine how many times it will replay)</i><br>
    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>&lt;any other symbol&gt;</code><i> (simulates a click of that symbol)</i></span>
</details>
