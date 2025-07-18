<h1>nexsqs_useless_ui</h1>
<h5>A completely useless terminal UI originally in chimera powershell, now in rust.</h5>
<details>
  <summary><h4>Version 1.0</h4></summary>
  <h6>&nbsp;(written in chimera powershell)</h6>
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
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>Enter</code><i> (simulates an enter click)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>Space</code><i> (simulates a space click)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>RandomNum</code><i> (simulates a click of a random number in range 0-9)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>n?</code><i> (simulates a click of the current value of the variable n)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>n++</code><i> (increments the n variable by one)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>n--</code><i> (decrements the n variable by one)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>loop &lt;times&gt;</code><i> (put at the end of the macro to determine how many times it will replay)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>&lt;any other symbol or some keys&gt;</code><i> (simulates a click of that symbol or that key)</i><br>
    <details><summary><span>example NUUI_MacroConfig.txt:</span></summary><br>
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
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>loop 10       </code><br></h6></details>
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
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>Enter</code><i> (simulates an enter click)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>Space</code><i> (simulates a space click)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>RanNum</code><i> (simulates a click of a random number in range 0-9)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>n?</code><i> (simulates a click of the current value of the variable n)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>n++</code><i> (increments the n variable by one)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>n--</code><i> (decrements the n variable by one)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>loop &lt;times&gt;</code><i> (put at the end of the macro to determine how many times it will replay)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>&lt;any other symbol or some keys&gt;</code><i> (simulates a click of that symbol or that key)</i></span>
    <details><summary><span>example NUUI_MacroConfig.txt:</span></summary><br>
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
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>loop 10       </code><br></h6></details>
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
    <span>&nbsp;&nbsp;&nbsp;allows to create macros in txt format saved in the NUUI_config\Macros\* directory<br>
    &nbsp;&nbsp;&nbsp;the macro will loop depending on settings.macro_loop<br>
    &nbsp;&nbsp;&nbsp;can also set a hotkey<i> (settings.macro_hotkey)</i><br>
    &nbsp;&nbsp;&nbsp;valid macro commands:<br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code># &lt;comment&gt;</code><i> (comments will be printed in a different color in the console)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>delay &lt;milliseconds&gt;</code><i> (sleeps for a given duration)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>click &lt;key&gt;</code><i> (clicks a given key)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>press &lt;key&gt;</code><i> (keeps a given key pressed)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>release &lt;key&gt;</code><i> (releases a given key)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>mouse_click &lt;mouse key&gt;</code><i> (clicks a given mouse key)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>mouse_press &lt;mouse key&gt;</code><i> (keeps a given mouse key pressed)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>mouse_release &lt;mouse key&gt;</code><i> (releases a given mouse key)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>mouse_move &lt;x y&gt;</code><i> (moves cursor to the given coordinates)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>mouse_scroll &lt;amount&gt;</code><i> (scrolls the mouse wheel)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>string &lt;text&gt;</code><i> (prints a given text)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>jump &lt;line&gt;</code><i> (jumps to a given line)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>if &lt;condition {, }&gt;</code><i> (executes code inside brackets only if condition met)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>(, ) &lt;replays (blank for infinite)&gt;</code><i> (loops the code inside brackets)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>let &lt;name = value&gt;</code><i> (create a variable or update it [operators like +, - are valid])</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>let &lt;name = static.time_hour&gt;</code><i> (create a variable with current hour)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>let &lt;name = static.time_minute&gt;</code><i> (create a variable with current minute)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>let &lt;name = static.time_second&gt;</code><i> (create a variable with current second)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>let &lt;name = static.mouse_x&gt;</code><i> (create a variable with current mouse x coordinate)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>let &lt;name = static.mouse_y&gt;</code><i> (create a variable with current mouse y coordinate)</i><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- <code>let &lt;name = static.current_line&gt;</code><i> (create a variable with current line)</i></span>
  <details><summary><span>example macro.txt:</span></summary><br>
      <h6>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code># add comments                           </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>delay 2500                               </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>                                         </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code># create variables                       </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>let var = 3                              </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>let scnd = static.time_second            </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>                                         </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code># use variables                          </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>string current second:                   </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>string $scnd                             </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>                                         </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>                                         </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code># update variables                       </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>let var = $var + 2                       </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>                                         </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code># create if block                        </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>if $var > 3 {                            </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code># condition met                          </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>}                                        </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>                                         </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code># paste text:                            </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>press ctrl                               </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>click v                                  </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>release ctrl                             </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>                                         </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>delay 5000                               </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>                                         </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code># do things with mouse                   </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>mouse_press left                         </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code># absolute coordinates                   </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>mouse_move 1000 500                      </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>mouse_release left                       </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>mouse_click right                        </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code># relative coordinates                   </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>mouse_move 100 100 rel                   </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>mouse_scroll $var                        </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>mouse_scroll -5                          </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>                                         </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>delay 2500                               </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>                                         </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code># loops are possible                     </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>(                                        </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>  delay 1000                             </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>  # nested loops are also possible       </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>  (                                      </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>    delay 1000                           </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>  # 5 for five replays                   </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>  ) 5                                    </code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code># leave blank after ")" for infinite loop</code><br>
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<code>)                                        </code><br></h6></details>
  <h4>&nbsp;• tetris</h4>

  <span>&nbsp;&nbsp;&nbsp;find out yourself ;)</span>
  <h4>&nbsp;• game_of_life</h4>
    <span>&nbsp;&nbsp;&nbsp;find out yourself ;)</span>
</details>

![Your NUUI your STYLE](https://github.com/user-attachments/assets/948c7316-b23a-4cb6-808e-9d37f48dae81)
