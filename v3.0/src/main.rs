use chrono::Local;
use crossterm::cursor;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::{
    style::{Color, SetBackgroundColor, SetForegroundColor},
    terminal,
};
use enigo::{
    Button, Coordinate,
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings as EnigoSettings,
};
use inputbot::KeybdKey::*;
use inputbot::{KeybdKey, MouseButton};
use rand::Rng;
use serde_derive::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::path::{Path, PathBuf};
use std::process;
use std::process::Command;
use std::sync::{Arc, LazyLock, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use sysinfo::System;
use wmi::{COMLibrary, WMIConnection};

fn version() -> String {
    let version = "v3.17";
    version.to_string()
}

fn logo() -> (String, String, String) {
    let logo_0 = String::from(
        r#"
  █▀█ █▀▀ █ █ █▀▀ █▀█ █ █▀▀
  █ █ █▀▀ ▄▀▄ ▀▀█ ▀▀█   ▀▀█
  ▀ ▀ ▀▀▀ ▀ ▀ ▀▀▀   ▀   ▀▀▀
    "#,
    );
    let logo_1 = String::from(
        r#"
 █ █ █▀▀ █▀▀ █   █▀▀ █▀▀ █▀▀
 █ █ ▀▀█ █▀▀ █   █▀▀ ▀▀█ ▀▀█
 ▀▀▀ ▀▀▀ ▀▀▀ ▀▀▀ ▀▀▀ ▀▀▀ ▀▀▀
    "#,
    );
    let logo_2 = format!(
        r#"
 █ █ ▀█▀  
 █ █  █   
 ▀▀▀ ▀▀▀  {}
    "#,
        version()
    );
    (logo_0, logo_1, logo_2)
}

fn main_options() -> Vec<String> {
    let main_options = vec!["menu", "settings", "fetch"];
    main_options.into_iter().map(String::from).collect()
}

#[derive(Serialize, Deserialize, Debug)]
struct Settings {
    color: String,
    dark_theme: bool,
    ping_delay: u64,
    port_scan_timeout: u64,
    micro_macro_hotkey: String,
    micro_macro_key: String,
    micro_macro_delay: u64,
    macro_hotkey: String,
    macro_restart_when_pausing: bool,
    macro_loop: bool,
    tetris_use_colors: bool,
    tetris_show_ghost: bool,
    tetris_speed_multiplier: f64,
    game_of_life_simulate_delay: u64,
    game_of_life_save_input: bool,
    game_of_life_show_generation: bool,
    hide_help: bool,
    show_config_files: bool,
    show_clock: bool,
    show_size: bool,
    options: Vec<String>,
}
impl Settings {
    fn new() -> Self {
        Settings {
            color: "grey".to_string(),
            dark_theme: false,
            ping_delay: 500,
            port_scan_timeout: 500,
            micro_macro_hotkey: "None".to_string(),
            micro_macro_key: "F15".to_string(),
            micro_macro_delay: 30000,
            macro_hotkey: "None".to_string(),
            macro_restart_when_pausing: false,
            macro_loop: true,
            tetris_use_colors: false,
            tetris_show_ghost: true,
            tetris_speed_multiplier: 1.0,
            game_of_life_simulate_delay: 200,
            game_of_life_save_input: false,
            game_of_life_show_generation: true,
            hide_help: false,
            show_config_files: false,
            show_clock: true,
            show_size: false,
            options: vec![
                "ping_tool".to_string(),
                "port_scan".to_string(),
                "micro_macro".to_string(),
                "macro".to_string(),
                "tetris".to_string(),
                "game_of_life".to_string(),
            ],
        }
    }
    fn load() -> Self {
        let dir = Path::new("NUUI_config");
        let file_path = dir.join("settings.toml");
        if !dir.exists() {
            fs::create_dir_all(dir).expect("Failed to create config directory");
            {
                use std::process::Command;
                Command::new("attrib")
                    .args(&["+H", dir.to_str().unwrap()])
                    .status()
                    .expect("Failed to set directory as hidden on Windows");
            }
        }
        if !file_path.exists() {
            let default_settings = Settings::new();
            default_settings.save();
            println!("Created default settings.toml");
            return default_settings;
        }
        let mut file = File::open(&file_path).expect("Failed to open settings.toml");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read settings.toml");
        match toml::from_str(&contents) {
            Ok(settings) => settings,
            Err(_) => {
                println!("Invalid settings.toml format. Using default settings.");
                let default_settings = Settings::new();
                default_settings.save();
                default_settings
            }
        }
    }
    fn save(&self) {
        let dir = Path::new("NUUI_config");
        let file_path = dir.join("settings.toml");
        let toml_string = toml::to_string(self).expect("Failed to serialize settings");
        let mut file = File::create(&file_path).expect("Failed to open settings.toml for writing");
        file.write_all(toml_string.as_bytes())
            .expect("Failed to write updated settings");
    }
    fn set_color(&mut self, new_color: &str) {
        self.color = new_color.to_string();
        self.save();
    }
    fn set_dark_theme(&mut self, new_value: bool) {
        self.dark_theme = new_value;
        self.save();
    }
    fn set_ping_delay(&mut self, new_delay: u64) {
        self.ping_delay = new_delay.clamp(0, u64::MAX);
        self.save();
    }
    fn set_port_scan_timeout(&mut self, new_delay: u64) {
        self.port_scan_timeout = new_delay.clamp(0, u64::MAX);
        self.save();
    }
    fn set_micro_macro_hotkey(&mut self, new_hotkey: &str) {
        self.micro_macro_hotkey = new_hotkey.to_string();
        self.save();
    }
    fn set_micro_macro_key(&mut self, new_key: &str) {
        self.micro_macro_key = new_key.to_string();
        self.save();
    }
    fn set_micro_macro_delay(&mut self, new_delay: u64) {
        self.micro_macro_delay = new_delay.clamp(0, u64::MAX);
        self.save();
    }
    fn set_macro_hotkey(&mut self, new_hotkey: &str) {
        self.macro_hotkey = new_hotkey.to_string();
        self.save();
    }
    fn set_macro_restart_when_pausing(&mut self, new_value: bool) {
        self.macro_restart_when_pausing = new_value;
        self.save();
    }
    fn set_macro_loop(&mut self, new_value: bool) {
        self.macro_loop = new_value;
        self.save();
    }
    fn set_tetris_use_colors(&mut self, new_value: bool) {
        self.tetris_use_colors = new_value;
        self.save();
    }
    fn set_tetris_show_ghost(&mut self, new_value: bool) {
        self.tetris_show_ghost = new_value;
        self.save();
    }
    fn set_tetris_speed_multiplier(&mut self, new_multiplier: f64) {
        self.tetris_speed_multiplier = new_multiplier.clamp(0.0, f64::MAX);
        self.save();
    }
    fn set_game_of_life_simulate_delay(&mut self, new_delay: u64) {
        self.game_of_life_simulate_delay = new_delay.clamp(0, u64::MAX);
        self.save();
    }
    fn set_game_of_life_save_input(&mut self, new_value: bool) {
        self.game_of_life_save_input = new_value;
        self.save();
    }
    fn set_game_of_life_show_generation(&mut self, new_value: bool) {
        self.game_of_life_show_generation = new_value;
        self.save();
    }
    fn set_hide_help(&mut self, new_value: bool) {
        self.hide_help = new_value;
        self.save();
    }
    fn set_show_config_files(&mut self, new_value: bool) {
        self.show_config_files = new_value;
        self.save();
    }
    fn set_show_clock(&mut self, new_value: bool) {
        self.show_clock = new_value;
        self.save();
    }
    fn set_show_size(&mut self, new_value: bool) {
        self.show_size = new_value;
        self.save();
    }
    fn add_option(&mut self, path: &str) {
        self.options.push(path.to_string());
        self.save();
    }
    fn remove_option(&mut self, index: usize) {
        let protected_options = [
            "ping_tool",
            "port_scan",
            "micro_macro",
            "macro",
            "tetris",
            "game_of_life",
        ];
        if let Some(option) = self.options.get(index) {
            if !protected_options.contains(&option.as_str()) {
                self.options.remove(index);
                self.save();
            }
        }
    }
}

fn get_key() -> Option<(KeyCode, KeyModifiers)> {
    if event::poll(Duration::ZERO).unwrap() {
        if let Event::Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event::read().unwrap()
        {
            if code == KeyCode::Char('h') || code == KeyCode::Char('H') {
                let mut help_open = HELP_OPEN.lock().unwrap();
                *help_open = !*help_open;
            }
            return Some((code, modifiers));
        }
    }
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum InputKey {
    Keyboard(KeybdKey),
    Mouse(MouseButton),
}

fn background_get_key(prev_state: &mut HashMap<InputKey, bool>) -> Option<InputKey> {
    let keys = vec![
        F1Key, F2Key, F3Key, F4Key, F5Key, F6Key, F7Key, F8Key, F9Key, F10Key, F11Key, F12Key,
    ];
    let mouse_buttons = vec![MouseButton::X1Button, MouseButton::X2Button];
    for &key in &keys {
        let is_pressed = key.is_pressed();
        let was_pressed = prev_state
            .get(&InputKey::Keyboard(key))
            .copied()
            .unwrap_or(false);
        if is_pressed && !was_pressed {
            prev_state.insert(InputKey::Keyboard(key), true);
            return Some(InputKey::Keyboard(key));
        }
        prev_state.insert(InputKey::Keyboard(key), is_pressed);
    }
    for &button in &mouse_buttons {
        let is_pressed = button.is_pressed();
        let was_pressed = prev_state
            .get(&InputKey::Mouse(button))
            .copied()
            .unwrap_or(false);
        if is_pressed && !was_pressed {
            prev_state.insert(InputKey::Mouse(button), true);
            return Some(InputKey::Mouse(button));
        }
        prev_state.insert(InputKey::Mouse(button), is_pressed);
    }
    None
}

fn string_to_key(key: &str) -> Option<InputKey> {
    match key {
        "F1" => Some(InputKey::Keyboard(F1Key)),
        "F2" => Some(InputKey::Keyboard(F2Key)),
        "F3" => Some(InputKey::Keyboard(F3Key)),
        "F4" => Some(InputKey::Keyboard(F4Key)),
        "F5" => Some(InputKey::Keyboard(F5Key)),
        "F6" => Some(InputKey::Keyboard(F6Key)),
        "F7" => Some(InputKey::Keyboard(F7Key)),
        "F8" => Some(InputKey::Keyboard(F8Key)),
        "F9" => Some(InputKey::Keyboard(F9Key)),
        "F10" => Some(InputKey::Keyboard(F10Key)),
        "F11" => Some(InputKey::Keyboard(F11Key)),
        "F12" => Some(InputKey::Keyboard(F12Key)),
        "X1Mouse" => Some(InputKey::Mouse(MouseButton::X1Button)),
        "X2Mouse" => Some(InputKey::Mouse(MouseButton::X2Button)),
        _ => None,
    }
}

fn get_time() -> String {
    let current_time = Local::now();
    current_time.format("%H:%M").to_string()
}

fn get_color(color_type: &str) -> Color {
    let settings = Settings::load();
    match color_type {
        "theme" => {
            if settings.dark_theme {
                return Color::Grey;
            } else {
                return Color::White;
            }
        }
        "main" => {
            if settings.dark_theme {
                match settings.color.to_lowercase().as_str() {
                    "grey" => Color::DarkGrey,
                    "red" => Color::DarkRed,
                    "yellow" => Color::DarkYellow,
                    "green" => Color::DarkGreen,
                    "cyan" => Color::DarkCyan,
                    "blue" => Color::DarkBlue,
                    "magenta" => Color::DarkMagenta,
                    _ => Color::DarkGrey,
                }
            } else {
                match settings.color.to_lowercase().as_str() {
                    "grey" => Color::Grey,
                    "red" => Color::Red,
                    "yellow" => Color::Yellow,
                    "green" => Color::Green,
                    "cyan" => Color::Cyan,
                    "blue" => Color::Blue,
                    "magenta" => Color::Magenta,
                    _ => Color::Grey,
                }
            }
        }
        _ => Color::Reset,
    }
}

fn clear() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Failed to clear the terminal");
    } else {
        Command::new("clear")
            .status()
            .expect("Failed to clear the terminal");
    }
}

fn render_top(current_option: &str, new_option: Option<&str>, new_option_selected: bool) -> String {
    let settings = Settings::load();
    let (width, height) = terminal::size().unwrap();
    let current_time = get_time();
    let (logo_0, logo_1, logo_2) = logo();
    let logo_0_lines: Vec<&str> = logo_0.lines().collect();
    let logo_1_lines: Vec<&str> = logo_1.lines().collect();
    let logo_2_lines: Vec<&str> = logo_2.lines().collect();
    let mut main_options = main_options();
    let mut output = String::new();
    let mut new_options_count = 0;
    let mut new_option_index = 2;
    if new_option_selected {
        new_option_index = 1;
    }
    if !main_options.contains(&current_option.to_string()) {
        main_options.insert(1, current_option.to_string());
        new_options_count += 1;
    }
    if let Some(option) = new_option {
        main_options.insert(new_option_index, option.to_string());
        new_options_count += 1;
    }
    let main_options_length: u16 = main_options
        .iter()
        .map(|option| option.len() as u16 + 5)
        .sum();
    for i in 0..logo_0_lines.len() {
        output.push_str(&format!(
            "{}{}{}{}{}{}\n",
            SetForegroundColor(get_color("main")),
            logo_0_lines[i],
            SetForegroundColor(get_color("theme")),
            logo_1_lines[i],
            SetForegroundColor(get_color("main")),
            logo_2_lines[i]
        ));
    }
    output.push_str(&format!(
        "{}{}{}{}",
        SetForegroundColor(get_color("theme")),
        cursor::MoveUp(logo_0_lines.len() as u16 - 1),
        cursor::MoveToColumn(width - 7),
        if settings.show_clock {
            current_time
        } else {
            "".to_string()
        }
    ));
    output.push_str(&format!(
        "{}{}{}{}{}",
        SetForegroundColor(get_color("theme")),
        cursor::MoveDown(1),
        cursor::MoveToColumn(
            width - width.to_string().len() as u16 - height.to_string().len() as u16 - 3
        ),
        if settings.show_size {
            format!("{}x{}", width, height)
        } else {
            "".to_string()
        },
        cursor::MoveUp(1)
    ));
    output.push_str(&format!(
        "{}{}",
        cursor::MoveDown(logo_0_lines.len() as u16 - 2),
        cursor::MoveToColumn(0)
    ));
    for (index, option) in main_options.iter().enumerate() {
        if index == 1 + new_options_count {
            if width > main_options_length + 1 {
                output.push_str(&format!(
                    "{}",
                    cursor::MoveRight(width - main_options_length - 1)
                ))
            }
        }
        let dashes = "─".repeat(option.len());
        output.push_str(" ╭─");
        output.push_str(&dashes);
        output.push_str("─╮");
    }
    output.push_str(" ");
    for (index, option) in main_options.iter().enumerate() {
        if index == 1 + new_options_count {
            if width > main_options_length + 1 {
                output.push_str(&format!(
                    "{}",
                    cursor::MoveRight(width - main_options_length - 1)
                ))
            }
        }
        output.push_str(" │ ");
        output.push_str(option);
        output.push_str(" │");
    }
    output.push_str(&format!(
        "{}{}",
        cursor::MoveToColumn(0),
        cursor::MoveDown(1)
    ));
    output.push_str("╭");
    for (index, option) in main_options.iter().enumerate() {
        if index == 1 + new_options_count {
            if width > main_options_length + 1 {
                let dashes = "─".repeat(width as usize - main_options_length as usize - 1);
                output.push_str(&dashes)
            }
        }
        if current_option == option {
            let spaces = " ".repeat(option.len());
            output.push_str("╯ ");
            output.push_str(&spaces);
            output.push_str(" ╰");
        } else {
            let dashes = "─".repeat(option.len());
            output.push_str("┴─");
            output.push_str(&dashes);
            output.push_str("─┴");
        }
        if index != main_options.len() - 1 {
            output.push_str("─");
        }
    }
    output.push_str("╮");
    output.to_string()
}

static HELP_OPEN: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
fn render_bottom(mid_length: u16, help_string: String, help_more_string: String) -> String {
    let settings = Settings::load();
    let (width, height) = terminal::size().unwrap();
    let (logo_0, _, _) = logo();
    let logo_0_lines: Vec<&str> = logo_0.lines().collect();
    let dashes = "─".repeat((width - 2) as usize);
    let help_open = HELP_OPEN.lock().unwrap();
    let mut help_string = help_string;
    if *help_open {
        help_string += " less: $[h]$ |"
    } else {
        help_string += " more: $[h]$ |"
    };
    let mut help_more_height = 0;
    let help_more_string_lines: Vec<&str> = help_more_string.lines().collect();
    if !settings.hide_help && *help_open {
        help_more_height = help_more_string.lines().count() as u16
    }
    let mut output = String::new();
    if !settings.hide_help {
        help_more_height += 1
    };
    if height > logo_0_lines.len() as u16 + mid_length + 3
        && height > 8 + help_more_height + mid_length
    {
        for _ in 0..height - 8 - help_more_height - mid_length {
            output.push_str(&format!("│{}│\n", cursor::MoveToColumn(width)));
        }
    }
    output.push_str("╰");
    output.push_str(&dashes);
    output.push_str("╯");
    if !settings.hide_help {
        if *help_open {
            for i in 0..help_more_string.lines().count() {
                output.push_str(&format!("{}", cursor::MoveDown(1)));
                output.push_str(&format!(
                    "{}",
                    cursor::MoveToColumn(
                        (width / 2).saturating_sub(
                            (help_more_string_lines[i]
                                .chars()
                                .filter(|&c| c != '$')
                                .count() as u16)
                                / 2
                        )
                    )
                ));
                let help_more_string_lines_parts: Vec<&str> =
                    help_more_string_lines[i].split('$').collect();
                for (i, part) in help_more_string_lines_parts.iter().enumerate() {
                    if i % 2 == 1 {
                        output.push_str(&format!("{}", SetForegroundColor(Color::Black)));
                        output.push_str(&format!("{}", SetBackgroundColor(get_color("main"))));
                        output.push_str(part);
                        output.push_str(&format!("{}", SetForegroundColor(get_color("theme"))));
                        output.push_str(&format!("{}", SetBackgroundColor(Color::Reset)));
                    } else {
                        output.push_str(part);
                    }
                }
            }
        }
        output.push_str(&format!("{}", cursor::MoveToRow(height)));
        if width / 2 > help_string.chars().filter(|&c| c != '$').count() as u16 / 2 {
            output.push_str(&format!(
                "{}",
                cursor::MoveToColumn(
                    width / 2 - help_string.chars().filter(|&c| c != '$').count() as u16 / 2
                )
            ))
        }
        let help_string_parts: Vec<&str> = help_string.split('$').collect();
        for (i, part) in help_string_parts.iter().enumerate() {
            if i % 2 == 1 {
                output.push_str(&format!("{}", SetForegroundColor(Color::Black)));
                output.push_str(&format!("{}", SetBackgroundColor(get_color("main"))));
                output.push_str(part);
                output.push_str(&format!("{}", SetForegroundColor(get_color("theme"))));
                output.push_str(&format!("{}", SetBackgroundColor(Color::Reset)));
            } else {
                output.push_str(part);
            }
        }
    }
    output.push_str(&format!("{}", cursor::MoveToRow(height)));
    output.push_str(&format!("{}", cursor::MoveToColumn(2)));
    if height > logo_0_lines.len() as u16 + mid_length + 3 {
        output.push_str(&format!("{}", cursor::MoveUp(height - 9 - mid_length)));
    } else {
        output.push_str(&format!("{}", cursor::MoveUp(1)));
    };
    output.to_string()
}

fn ping_tool() {
    let settings = Settings::load();
    let help_string =
        String::from("| quit: $[esc]$ | change tab: $[a]/[d]$ | change ip: $[ent]$ |");
    let help_more_string =
        String::from(r#"| return: $[q]$ | change tab: $[backtab]/[tab]$ | change ip: $[space]$ |"#);
    fn render_ping_tool(help_string: &String, help_more_string: &String, ip: &String) {
        let mut stdout = io::stdout();
        let mut output = String::new();
        let (width, _) = terminal::size().unwrap();
        output.push_str(&render_top("ping_tool", None, false));
        output.push_str(&format!(
            "│ Pinging: {}{}│",
            ip,
            cursor::MoveToColumn(width),
        ));
        output.push_str(&render_bottom(
            1,
            help_string.clone(),
            help_more_string.clone(),
        ));
        output.push_str(&format!("{}", cursor::MoveUp(1)));
        execute!(stdout, crossterm::terminal::BeginSynchronizedUpdate).unwrap();
        clear();
        print!("{}", output);
        execute!(stdout, crossterm::terminal::EndSynchronizedUpdate).unwrap();
        stdout.flush().unwrap();
    }
    fn add_ping(pings: &mut Vec<String>, ping: String, help_more_string_lines: u16) {
        let settings = Settings::load();
        let (_, height) = terminal::size().unwrap();
        let mut help_length = 0;
        if !settings.hide_help {
            help_length += 1;
            let help_open = HELP_OPEN.lock().unwrap();
            if *help_open {
                help_length += help_more_string_lines
            }
        }
        let max_pings = height.saturating_sub(12 + help_length).max(1) as usize;
        while pings.len() > max_pings {
            if !pings.is_empty() {
                pings.remove(0);
            }
        }
        if !ping.is_empty() {
            pings.push(ping);
        }
    }
    fn print_pings(pings: &mut Vec<String>) -> usize {
        let (width, _) = terminal::size().unwrap();
        let mut stdout = io::stdout();
        let start_y = 9;
        for i in 0..pings.len() {
            execute!(stdout, cursor::MoveTo(0, start_y + i as u16)).unwrap();
            print!("\r│{}│", " ".repeat(width as usize - 2));
        }
        for (i, ping) in pings.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(2, start_y + i as u16)).unwrap();
            print!("{}", ping);
        }
        stdout.flush().unwrap();
        pings.len()
    }
    let mut pings = Vec::new();
    let mut ping_seq = 1;
    let mut ip = String::new();
    let help_line_count = help_more_string.lines().count() as u16;
    let mut last_render_time = get_time();
    let (mut last_width, mut last_height) = terminal::size().unwrap();
    let mut needs_rendering = true;
    let mut stdout = io::stdout();
    render_ping_tool(&help_string, &help_more_string, &ip);
    execute!(stdout, cursor::MoveUp(1)).unwrap();
    print!("Pinging: ");
    stdout.flush().unwrap();
    let mut new_ip = String::new();
    io::stdin().read_line(&mut new_ip).unwrap();
    let new_ip = new_ip.trim();
    ip = new_ip.to_string();
    if ip.is_empty() {
        return;
    };
    let ip = ip.trim();
    let mut last_ping = Instant::now();
    fn ping(ip: &str) -> Option<(f64, u32)> {
        let output = {
            Command::new("ping")
                .arg("-n")
                .arg("1")
                .arg(ip)
                .output()
                .expect("Failed to execute ping")
        };
        if output.status.success() {
            let stdout = std::str::from_utf8(&output.stdout).unwrap();
            let time = stdout
                .lines()
                .find(|line| line.contains("Average"))
                .and_then(|line| line.split("=").last())
                .and_then(|avg| avg.trim().strip_suffix("ms"))
                .and_then(|ms| ms.parse::<f64>().ok());
            let ttl = stdout
                .lines()
                .find(|line| line.contains("TTL"))
                .and_then(|line| line.split("TTL=").nth(1))
                .and_then(|ttl| ttl.split_whitespace().next())
                .and_then(|ttl| ttl.parse::<u32>().ok())
                .unwrap_or(64);
            if let Some(ms) = time {
                return Some((ms, ttl));
            }
        }
        None
    }
    print_pings(&mut pings);
    loop {
        if let Some((code, _)) = get_key() {
            needs_rendering = true;
            match code {
                KeyCode::Tab | KeyCode::Char('d') | KeyCode::Char('D') => settings_menu(),
                KeyCode::BackTab | KeyCode::Char('a') | KeyCode::Char('A') => return,
                KeyCode::Char('q') | KeyCode::Char('Q') => return,
                KeyCode::Esc => process::exit(0),
                KeyCode::Enter => {
                    ping_tool();
                    return;
                }
                KeyCode::Char(' ') => {
                    ping_tool();
                    return;
                }
                _ => {}
            }
        }
        if last_ping.elapsed() >= Duration::from_millis(settings.ping_delay) {
            match ping(ip) {
                Some((ms, ttl)) => {
                    let ping_status = format!("Ping: {:.0} ms (seq={} ttl={})", ms, ping_seq, ttl);
                    add_ping(&mut pings, ping_status, help_line_count);
                }
                None => {
                    let ping_status = format!("Ping to {} failed", ip);
                    add_ping(&mut pings, ping_status, help_line_count);
                }
            }
            ping_seq += 1;
            let num_pings = print_pings(&mut pings);
            execute!(stdout, cursor::MoveUp(num_pings as u16)).unwrap();
            last_ping = Instant::now();
        }
        let current_time = get_time();
        let (width, height) = terminal::size().unwrap();
        if width != last_width
            || height != last_height
            || current_time != last_render_time
            || needs_rendering
        {
            render_ping_tool(&help_string, &help_more_string, &ip.to_string());
            add_ping(&mut pings, "".to_string(), help_line_count);
            print_pings(&mut pings);
            last_render_time = current_time;
            last_width = width;
            last_height = height;
            needs_rendering = false;
        }
    }
}

fn port_scan() {
    let settings = Settings::load();
    let help_string =
        String::from("| quit: $[esc]$ | change tab: $[a]/[d]$ | change ip: $[ent]$ |");
    let help_more_string =
        String::from(r#"| return: $[q]$ | change tab: $[backtab]/[tab]$ | change ip: $[space]$ |"#);
    fn render_port_scan(help_string: &String, help_more_string: &String, ip: &String, port: i32) {
        let mut stdout = io::stdout();
        let mut output = String::new();
        let (width, _) = terminal::size().unwrap();
        output.push_str(&render_top("port_scan", None, false));
        output.push_str(&format!("│ Ip: {}{}│", ip, cursor::MoveToColumn(width),));
        if port == -1 {
            output.push_str(&format!("│ Port: {}│", cursor::MoveToColumn(width),));
        } else {
            output.push_str(&format!("│ Port: {}{}│", port, cursor::MoveToColumn(width),));
        }
        output.push_str(&render_bottom(
            2,
            help_string.clone(),
            help_more_string.clone(),
        ));
        output.push_str(&format!("{}", cursor::MoveUp(2)));
        execute!(stdout, crossterm::terminal::BeginSynchronizedUpdate).unwrap();
        clear();
        print!("{}", output);
        execute!(stdout, crossterm::terminal::EndSynchronizedUpdate).unwrap();
        stdout.flush().unwrap();
    }
    fn add_port_scan(port_scan: String, help_more_string_lines: u16) {
        let settings = Settings::load();
        let (_, height) = terminal::size().unwrap();
        let mut help_length = 0;
        if !settings.hide_help {
            help_length += 1;
            let help_open = HELP_OPEN.lock().unwrap();
            if *help_open {
                help_length += help_more_string_lines
            }
        }
        let max_port_scans = height.saturating_sub(14 + help_length).max(1) as usize;
        let mut port_scans = PORT_SCANS.lock().unwrap();
        while port_scans.len() > max_port_scans {
            if !port_scans.is_empty() {
                port_scans.remove(0);
            }
        }
        if !port_scan.is_empty() {
            port_scans.push(port_scan);
        }
    }
    fn clear_port_scans() {
        let mut port_scans = PORT_SCANS.lock().unwrap();
        port_scans.clear()
    }
    fn print_port_scans(port_scans: &Vec<String>) -> usize {
        let (width, _) = terminal::size().unwrap();
        let mut stdout = io::stdout();
        let start_y = 10;
        for i in 0..port_scans.len() {
            execute!(stdout, cursor::MoveTo(0, start_y + i as u16)).unwrap();
            print!("\r│{}│", " ".repeat(width as usize - 2));
        }
        for (i, port_scan) in port_scans.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(2, start_y + i as u16)).unwrap();
            print!("{}", port_scan);
        }
        stdout.flush().unwrap();
        port_scans.len()
    }
    fn add_open_port(port_scan: String, open_ports: &mut Vec<String>) {
        if !open_ports.contains(&port_scan) {
            open_ports.push(port_scan)
        }
    }
    fn clear_open_ports() {
        let mut open_ports = OPEN_PORTS.lock().unwrap();
        open_ports.clear()
    }
    fn print_open_ports(help_more_string_lines: u16, open_ports: &Vec<String>) -> usize {
        let settings = Settings::load();
        let (_, height) = terminal::size().unwrap();
        let mut help_length = 0;
        if !settings.hide_help {
            help_length += 1;
            let help_open = HELP_OPEN.lock().unwrap();
            if *help_open {
                help_length += help_more_string_lines
            }
        }
        let mut stdout = io::stdout();
        execute!(stdout, cursor::MoveTo(2, height - 2 - help_length)).unwrap();
        print!("Open Ports: ");
        let mut first = true;
        for open_port in open_ports.iter() {
            if first {
                first = false
            } else {
                print!(", ")
            }
            print!("{}", open_port);
        }
        stdout.flush().unwrap();
        open_ports.len()
    }
    fn get_port_num() -> u16 {
        let num = PORT_NUM.lock().unwrap();
        *num
    }
    fn set_port_num(new_port_num: u16) {
        let mut num = PORT_NUM.lock().unwrap();
        *num = new_port_num
    }
    fn clear_port_num() {
        let mut num = PORT_NUM.lock().unwrap();
        *num = 0
    }
    static PORT_SCANS: LazyLock<Arc<Mutex<Vec<String>>>> =
        LazyLock::new(|| Arc::new(Mutex::new(Vec::new())));
    static OPEN_PORTS: LazyLock<Arc<Mutex<Vec<String>>>> =
        LazyLock::new(|| Arc::new(Mutex::new(Vec::new())));
    static PORT_NUM: LazyLock<Mutex<u16>> = LazyLock::new(|| Mutex::new(0));
    let mut ip = String::new();
    let help_line_count = help_more_string.lines().count() as u16;
    let mut last_render_time = get_time();
    let (mut last_width, mut last_height) = terminal::size().unwrap();
    let mut needs_rendering = true;
    let mut stdout = io::stdout();
    render_port_scan(&help_string, &help_more_string, &ip, -1);
    execute!(stdout, cursor::MoveUp(1)).unwrap();
    print!("Ip: ");
    stdout.flush().unwrap();
    let mut new_ip = String::new();
    io::stdin().read_line(&mut new_ip).unwrap();
    let new_ip = new_ip.trim();
    ip = new_ip.to_string();
    if ip.is_empty() {
        return;
    }
    let ip = ip.trim();
    execute!(stdout, cursor::MoveToColumn(2)).unwrap();
    print!("Port: ");
    stdout.flush().unwrap();
    let mut port = String::new();
    io::stdin().read_line(&mut port).unwrap();
    let port = port.trim();
    if port.is_empty() {
        return;
    }
    if let Ok(port) = port.parse::<u16>() {
        set_port_num(port);
    } else {
        execute!(stdout, cursor::MoveToColumn(2)).unwrap();
        eprintln!("Error: Invalid port '{}'", port);
    }
    let starting_port = port;
    let mut last_port_scan = Instant::now();
    enum PortStatus {
        Open,
        Closed,
        Error(String),
    }
    fn check_port(ip: &str, port: u16) -> PortStatus {
        let settings = Settings::load();
        let address = format!("{}:{}", ip, port);
        match address.to_socket_addrs() {
            Ok(mut addrs) => {
                if let Some(addr) = addrs.next() {
                    match TcpStream::connect_timeout(
                        &addr,
                        Duration::from_millis(settings.port_scan_timeout),
                    ) {
                        Ok(_) => PortStatus::Open,
                        Err(_) => PortStatus::Closed,
                    }
                } else {
                    PortStatus::Error(format!("Error: No valid address found for '{}'", ip))
                }
            }
            Err(_) => PortStatus::Error(format!("Error: Unable to resolve IP address '{}'", ip)),
        }
    }
    print_port_scans(&PORT_SCANS.lock().unwrap());
    let mut handle: Option<thread::JoinHandle<()>> = None;
    loop {
        if let Some((code, _)) = get_key() {
            needs_rendering = true;
            match code {
                KeyCode::Tab | KeyCode::Char('d') | KeyCode::Char('D') => {
                    if let Some(h) = handle.take() {
                        h.join().unwrap();
                    };
                    clear_port_scans();
                    clear_open_ports();
                    clear_port_num();
                    settings_menu()
                }
                KeyCode::BackTab | KeyCode::Char('a') | KeyCode::Char('A') => {
                    if let Some(h) = handle.take() {
                        h.join().unwrap();
                    };
                    clear_port_scans();
                    clear_open_ports();
                    clear_port_num();
                    return;
                }
                KeyCode::Char('q') | KeyCode::Char('Q') => return,
                KeyCode::Esc => process::exit(0),
                KeyCode::Enter => {
                    if let Some(h) = handle.take() {
                        h.join().unwrap();
                    };
                    clear_port_scans();
                    clear_open_ports();
                    clear_port_num();
                    port_scan();
                    return;
                }
                KeyCode::Char(' ') => {
                    if let Some(h) = handle.take() {
                        h.join().unwrap();
                    };
                    clear_port_scans();
                    clear_open_ports();
                    clear_port_num();
                    port_scan();
                    return;
                }
                _ => {}
            }
        }
        if last_port_scan.elapsed() >= Duration::from_millis(settings.port_scan_timeout) {
            if let Some(h) = handle.take() {
                h.join().unwrap()
            }
            let ip_clone = ip.to_string();
            handle = Some(thread::spawn(move || {
                match check_port(&ip_clone, get_port_num()) {
                    PortStatus::Open => {
                        let port_status = format!(
                            "Port {} {}open{}",
                            get_port_num(),
                            SetForegroundColor(get_color("main")),
                            SetForegroundColor(get_color("theme"))
                        );
                        add_port_scan(port_status, help_line_count);
                        add_open_port(get_port_num().to_string(), &mut OPEN_PORTS.lock().unwrap());
                        if get_port_num() < u16::MAX {
                            set_port_num(get_port_num() + 1)
                        }
                    }
                    PortStatus::Closed => {
                        let port_status = format!("Port {} closed", get_port_num());
                        add_port_scan(port_status, help_line_count);
                        if get_port_num() < u16::MAX {
                            set_port_num(get_port_num() + 1)
                        }
                    }
                    PortStatus::Error(err) => {
                        add_port_scan(err, help_line_count);
                    }
                }
            }));
            let num_port_scans = print_port_scans(&PORT_SCANS.lock().unwrap());
            execute!(stdout, cursor::MoveUp(num_port_scans as u16)).unwrap();
            last_port_scan = Instant::now();
            print_open_ports(help_line_count, &OPEN_PORTS.lock().unwrap());
        }
        let current_time = get_time();
        let (width, height) = terminal::size().unwrap();
        if width != last_width
            || height != last_height
            || current_time != last_render_time
            || needs_rendering
        {
            render_port_scan(
                &help_string,
                &help_more_string,
                &ip.to_string(),
                starting_port.parse::<i32>().unwrap_or(0),
            );
            add_port_scan("".to_string(), help_line_count);
            print_port_scans(&PORT_SCANS.lock().unwrap());
            print_open_ports(help_line_count, &OPEN_PORTS.lock().unwrap());
            last_render_time = current_time;
            last_width = width;
            last_height = height;
            needs_rendering = false;
        }
    }
}

fn micro_macro() {
    fn render_micro_macro(micro_macro_active: bool) {
        let settings = Settings::load();
        let mut stdout = io::stdout();
        let help_string =
            String::from("| quit: $[esc]$ | change tab: $[a]/[d]$ | change status: $[ent]$ |");
        let help_more_string = format!(
            r#"| return: $[q]$ | change tab: $[backtab]/[tab]$ | change status: $[Space]/[{}]$ |"#,
            settings.micro_macro_hotkey
        );
        let (width, _) = terminal::size().unwrap();
        let mut output = String::new();
        let is_active = if micro_macro_active {
            "active"
        } else {
            "inactive"
        };
        let delay = settings.micro_macro_delay as usize;
        let (display_delay, delay_unit) = if delay <= 1000 {
            (delay, "ms")
        } else if delay > 60000 {
            (delay / 60000, "m")
        } else {
            (delay / 1000, "s")
        };
        output.push_str(&render_top(
            "micro_macro",
            Some("micro_macro_settings"),
            false,
        ));
        output.push_str(&format!(
            "│ Status: {}{}{}{}{}{}│\n",
            SetBackgroundColor(get_color("main")),
            SetForegroundColor(Color::Black),
            is_active,
            SetForegroundColor(get_color("theme")),
            SetBackgroundColor(Color::Reset),
            cursor::MoveToColumn(width)
        ));
        output.push_str(&format!("│{}│\n", cursor::MoveToColumn(width)));
        output.push_str(&format!(
            "│ Hotkey: {}{}[{}]{}{}{}│\n",
            SetBackgroundColor(get_color("main")),
            SetForegroundColor(Color::Black),
            settings.micro_macro_hotkey,
            SetForegroundColor(get_color("theme")),
            SetBackgroundColor(Color::Reset),
            cursor::MoveToColumn(width)
        ));
        output.push_str(&format!("│{}│\n", cursor::MoveToColumn(width)));
        output.push_str(&format!(
            "│ Pressing {} every {}{}│\n",
            settings.micro_macro_key,
            format!("{}{}", display_delay, delay_unit),
            cursor::MoveToColumn(width)
        ));
        output.push_str(&render_bottom(5, help_string, help_more_string));
        execute!(stdout, crossterm::terminal::BeginSynchronizedUpdate).unwrap();
        clear();
        print!("{}", output);
        execute!(stdout, crossterm::terminal::EndSynchronizedUpdate).unwrap();
        stdout.flush().unwrap();
    }
    fn micro_macro_settings() {
        let mut settings = Settings::load();
        fn render_micro_macro_settings(menu_selected: usize, menu_options: &[&str]) {
            let settings = Settings::load();
            let mut stdout = io::stdout();
            let help_string = String::from("| quit: $[esc]$ | change tab: $[a]/[d]$ | scroll: $[w]/[s]$ | change setting: $[←]/[→]$ |");
            let help_more_string = String::from(
                r#"| change setting: $[ent]$ | select: $[0-9]$ |
| return: $[q]$ | change tab: $[backtab]/[tab]$ | scroll: $[↑]/[↓]$ |"#,
            );
            let (width, _) = terminal::size().unwrap();
            let mut output = String::new();
            output.push_str(&render_top(
                "micro_macro_settings",
                Some("micro_macro"),
                true,
            ));
            for i in 0..menu_options.len() {
                if i == menu_selected {
                    output.push_str(&format!(
                        "│{}{} {} › {} {}{}{}{}│\n",
                        SetBackgroundColor(get_color("main")),
                        SetForegroundColor(Color::Black),
                        i,
                        menu_options[i],
                        if menu_options[i] == "key" {
                            settings.micro_macro_key.to_string() + " "
                        } else if menu_options[i] == "delay" {
                            let delay = settings.micro_macro_delay as usize;
                            let (display_delay, delay_unit) = if delay <= 1000 {
                                (delay, "ms")
                            } else if delay > 60000 {
                                (delay / 60000, "m")
                            } else {
                                (delay / 1000, "s")
                            };
                            format!("{}{} ", display_delay, delay_unit)
                        } else if menu_options[i] == "hotkey" {
                            settings.micro_macro_hotkey.to_string() + " "
                        } else {
                            " ".to_string()
                        },
                        SetForegroundColor(get_color("theme")),
                        SetBackgroundColor(Color::Reset),
                        cursor::MoveToColumn(width)
                    ));
                } else {
                    output.push_str(&format!(
                        "│{} {} {}| {}{}{}│\n",
                        SetForegroundColor(get_color("main")),
                        i,
                        SetForegroundColor(Color::DarkGrey),
                        SetForegroundColor(get_color("theme")),
                        menu_options[i],
                        cursor::MoveToColumn(width)
                    ));
                }
            }
            output.push_str(&render_bottom(
                menu_options.len() as u16,
                help_string,
                help_more_string,
            ));
            execute!(stdout, crossterm::terminal::BeginSynchronizedUpdate).unwrap();
            clear();
            print!("{}", output);
            execute!(stdout, crossterm::terminal::EndSynchronizedUpdate).unwrap();
            stdout.flush().unwrap();
        }
        let micro_macro_settings_menu_options = ["key", "delay", "custom_delay", "hotkey"];
        let mut micro_macro_settings_menu_selected = 0;
        let micro_macro_keys = ["F15", "RandomNum", "Enter", "Space", "E", "F", "LMB", "RMB"];
        let mut micro_macro_key_index = micro_macro_keys
            .iter()
            .position(|&c| c == settings.micro_macro_key)
            .unwrap_or(0);
        let micro_macro_delays = [
            5, 10, 25, 50, 75, 100, 200, 500, 1000, 5000, 10000, 30000, 60000, 120000, 300000,
            600000,
        ];
        let mut micro_macro_delay_index = micro_macro_delays
            .iter()
            .position(|&c| c == settings.micro_macro_delay)
            .unwrap_or(0);
        let micro_macro_hotkeys = [
            "None", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "X1Mouse", "X2Mouse",
        ];
        let mut micro_macro_hotkey_index = micro_macro_hotkeys
            .iter()
            .position(|&c| c == settings.micro_macro_hotkey)
            .unwrap_or(0);
        let mut last_render_time = get_time();
        let (mut last_width, mut last_height) = terminal::size().unwrap();
        let mut needs_rendering = true;
        loop {
            if let Some((code, _)) = get_key() {
                needs_rendering = true;
                match code {
                    KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                        if micro_macro_settings_menu_selected > 0 {
                            micro_macro_settings_menu_selected -= 1
                        } else {
                            micro_macro_settings_menu_selected =
                                micro_macro_settings_menu_options.len() - 1
                        }
                    }
                    KeyCode::Left => match micro_macro_settings_menu_selected {
                        0 => {
                            if micro_macro_key_index > 0 {
                                settings.set_micro_macro_key(
                                    micro_macro_keys[micro_macro_key_index - 1],
                                )
                            } else {
                                settings.set_micro_macro_key(
                                    micro_macro_keys[micro_macro_keys.len() - 1],
                                )
                            };
                            if micro_macro_key_index > 0 {
                                micro_macro_key_index -= 1
                            } else {
                                micro_macro_key_index = micro_macro_keys.len() - 1
                            }
                        }
                        1 => {
                            if micro_macro_delay_index > 0 {
                                settings.set_micro_macro_delay(
                                    micro_macro_delays[micro_macro_delay_index - 1],
                                )
                            } else {
                                settings.set_micro_macro_delay(
                                    micro_macro_delays[micro_macro_delays.len() - 1],
                                )
                            };
                            if micro_macro_delay_index > 0 {
                                micro_macro_delay_index -= 1
                            } else {
                                micro_macro_delay_index = micro_macro_delays.len() - 1
                            }
                        }
                        2 => {
                            let mut custom_micro_macro_delay = String::new();
                            print!("Enter delay in ms: ");
                            io::stdout().flush().unwrap();
                            io::stdin()
                                .read_line(&mut custom_micro_macro_delay)
                                .unwrap();
                            let custom_micro_macro_delay = custom_micro_macro_delay.trim();
                            if !custom_micro_macro_delay.is_empty() {
                                if let Ok(delay) = custom_micro_macro_delay.parse::<u64>() {
                                    settings.set_micro_macro_delay(delay)
                                } else {
                                    println!("Invalid input")
                                }
                            }
                        }
                        3 => {
                            if micro_macro_hotkey_index > 0 {
                                settings.set_micro_macro_hotkey(
                                    micro_macro_hotkeys[micro_macro_hotkey_index - 1],
                                )
                            } else {
                                settings.set_micro_macro_hotkey(
                                    micro_macro_hotkeys[micro_macro_hotkeys.len() - 1],
                                )
                            };
                            if micro_macro_hotkey_index > 0 {
                                micro_macro_hotkey_index -= 1
                            } else {
                                micro_macro_hotkey_index = micro_macro_hotkeys.len() - 1
                            }
                        }
                        _ => {}
                    },
                    KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
                        if micro_macro_settings_menu_selected
                            < micro_macro_settings_menu_options.len() - 1
                        {
                            micro_macro_settings_menu_selected += 1
                        } else {
                            micro_macro_settings_menu_selected = 0
                        }
                    }
                    KeyCode::Right | KeyCode::Enter => match micro_macro_settings_menu_selected {
                        0 => {
                            settings.set_micro_macro_key(
                                micro_macro_keys
                                    [(micro_macro_key_index + 1) % micro_macro_keys.len()],
                            );
                            micro_macro_key_index =
                                (micro_macro_key_index + 1) % micro_macro_keys.len()
                        }
                        1 => {
                            settings.set_micro_macro_delay(
                                micro_macro_delays
                                    [(micro_macro_delay_index + 1) % micro_macro_delays.len()],
                            );
                            micro_macro_delay_index =
                                (micro_macro_delay_index + 1) % micro_macro_delays.len()
                        }
                        2 => {
                            let mut custom_micro_macro_delay = String::new();
                            print!("Enter delay in ms: ");
                            io::stdout().flush().unwrap();
                            io::stdin()
                                .read_line(&mut custom_micro_macro_delay)
                                .unwrap();
                            let custom_micro_macro_delay = custom_micro_macro_delay.trim();
                            if !custom_micro_macro_delay.is_empty() {
                                if let Ok(delay) = custom_micro_macro_delay.parse::<u64>() {
                                    settings.set_micro_macro_delay(delay)
                                } else {
                                    println!("Invalid input")
                                }
                            }
                        }
                        3 => {
                            settings.set_micro_macro_hotkey(
                                micro_macro_hotkeys
                                    [(micro_macro_hotkey_index + 1) % micro_macro_hotkeys.len()],
                            );
                            micro_macro_hotkey_index =
                                (micro_macro_hotkey_index + 1) % micro_macro_hotkeys.len()
                        }
                        _ => {}
                    },
                    KeyCode::Tab | KeyCode::Char('d') | KeyCode::Char('D') => settings_menu(),
                    KeyCode::BackTab | KeyCode::Char('a') | KeyCode::Char('A') => return,
                    KeyCode::Char('q') | KeyCode::Char('Q') => return,
                    KeyCode::Esc => process::exit(0),
                    KeyCode::Char(c) if c.is_digit(10) => {
                        let num = c.to_digit(10).unwrap() as usize;
                        if num < micro_macro_settings_menu_options.len() {
                            micro_macro_settings_menu_selected = num;
                        };
                    }
                    _ => {}
                }
            }
            let current_time = get_time();
            let (width, height) = terminal::size().unwrap();
            if width != last_width
                || height != last_height
                || current_time != last_render_time
                || needs_rendering
            {
                render_micro_macro_settings(
                    micro_macro_settings_menu_selected,
                    &micro_macro_settings_menu_options,
                );
                last_render_time = current_time;
                last_width = width;
                last_height = height;
                needs_rendering = false;
            }
        }
    }
    let mut last_render_time = get_time();
    let (mut last_width, mut last_height) = terminal::size().unwrap();
    let mut needs_rendering = true;
    let mut micro_macro_active = false;
    let mut last_micro_macro_active = micro_macro_active;
    let mut last_click = Instant::now();
    let mut prev_state = HashMap::new();
    loop {
        let settings = Settings::load();
        if let Some((code, _)) = get_key() {
            needs_rendering = true;
            match code {
                KeyCode::Tab | KeyCode::Char('d') | KeyCode::Char('D') => micro_macro_settings(),
                KeyCode::BackTab | KeyCode::Char('a') | KeyCode::Char('A') => return,
                KeyCode::Char('q') | KeyCode::Char('Q') => return,
                KeyCode::Esc => process::exit(0),
                KeyCode::Enter => micro_macro_active = !micro_macro_active,
                KeyCode::Char(' ') => micro_macro_active = !micro_macro_active,
                _ => {}
            }
        }
        if let Some(code) = background_get_key(&mut prev_state) {
            if let Some(hotkey_enum) = string_to_key(&settings.micro_macro_hotkey) {
                if code == hotkey_enum {
                    micro_macro_active = !micro_macro_active;
                }
            }
        }
        if micro_macro_active != last_micro_macro_active {
            last_micro_macro_active = micro_macro_active;
            render_micro_macro(micro_macro_active);
        }
        if micro_macro_active {
            if last_click.elapsed() >= Duration::from_millis(settings.micro_macro_delay) {
                let mut enigo = Enigo::new(&EnigoSettings::default()).unwrap();
                match settings.micro_macro_key.as_str() {
                    "F15" => {
                        enigo.key(Key::F15, Click).ok();
                    }
                    "RandomNum" => {
                        enigo
                            .key(
                                Key::Unicode(
                                    char::from_digit(rand::thread_rng().gen_range(0..=9), 10)
                                        .unwrap(),
                                ),
                                Click,
                            )
                            .ok();
                    }
                    "Enter" => {
                        enigo.key(Key::Return, Click).ok();
                    }
                    "Space" => {
                        enigo.key(Key::Space, Click).ok();
                    }
                    "E" => {
                        enigo.key(Key::E, Click).ok();
                    }
                    "F" => {
                        enigo.key(Key::F, Click).ok();
                    }
                    "LMB" => {
                        enigo.button(Button::Left, Click).ok();
                    }
                    "RMB" => {
                        enigo.button(Button::Right, Click).ok();
                    }
                    _ => {}
                }
                last_click = Instant::now();
            }
        }
        let current_time = get_time();
        let (width, height) = terminal::size().unwrap();
        if width != last_width
            || height != last_height
            || current_time != last_render_time
            || needs_rendering
        {
            render_micro_macro(micro_macro_active);
            last_render_time = current_time;
            last_width = width;
            last_height = height;
            needs_rendering = false;
        }
    }
}

fn macro_tool() {
    fn render_macro_tool_menu(menu_selected: usize, menu_options: &[&str], current_dir: &Path) {
        let mut stdout = io::stdout();
        let help_string = String::from(
            "| quit: $[esc]$ | change tab: $[a]/[d]$ | scroll: $[w]/[s]$ | select: $[ent]$ |",
        );
        let help_more_string = String::from(
            r#"| select: $[0-9]$ | edit: $[space]$ | delete: $[del]/[backspace]$ | back: $[←]/[→]$ |
    | return: $[q]$ | change tab: $[backtab]/[tab]$ | scroll: $[↑]/[↓]$ |"#,
        );
        let (width, _) = terminal::size().unwrap();
        let mut output = String::new();
        let current_folder_name = if current_dir == Path::new("NUUI_config/Macros") {
            "macro".to_string()
        } else {
            current_dir
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
        };
        output.push_str(&render_top(
            &current_folder_name,
            Some("macro_settings"),
            false,
        ));
        for i in 0..menu_options.len() {
            let mut prefix = "|";
            if i >= 2 {
                let item_path = current_dir.join(&menu_options[i]);
                if item_path.is_dir() {
                    prefix = "•"
                }
            }
            if i == 0 || i == 1 {
                if i == menu_selected {
                    output.push_str(&format!(
                        "│{}{}   › {}  {}{}{}│\n",
                        SetBackgroundColor(get_color("main")),
                        SetForegroundColor(Color::Black),
                        menu_options[i],
                        SetForegroundColor(get_color("theme")),
                        SetBackgroundColor(Color::Reset),
                        cursor::MoveToColumn(width)
                    ));
                } else {
                    output.push_str(&format!(
                        "│{}   {}|{} {}{}│\n",
                        SetForegroundColor(get_color("main")),
                        SetForegroundColor(Color::DarkGrey),
                        SetForegroundColor(get_color("theme")),
                        menu_options[i],
                        cursor::MoveToColumn(width)
                    ));
                }
            } else {
                let mut spaces = " ";
                if i > 11 {
                    spaces = ""
                }
                if i == menu_selected {
                    output.push_str(&format!(
                        "│{}{}{}{} › {}  {}{}{}│\n",
                        SetBackgroundColor(get_color("main")),
                        SetForegroundColor(Color::Black),
                        spaces,
                        i - 2,
                        menu_options[i],
                        SetForegroundColor(get_color("theme")),
                        SetBackgroundColor(Color::Reset),
                        cursor::MoveToColumn(width)
                    ));
                } else {
                    output.push_str(&format!(
                        "│{}{}{}{} {} {}{}{}│\n",
                        SetForegroundColor(get_color("main")),
                        spaces,
                        i - 2,
                        SetForegroundColor(Color::DarkGrey),
                        prefix,
                        SetForegroundColor(get_color("theme")),
                        menu_options[i],
                        cursor::MoveToColumn(width)
                    ));
                }
            }
        }
        output.push_str(&render_bottom(
            menu_options.len() as u16,
            help_string,
            help_more_string,
        ));
        execute!(stdout, crossterm::terminal::BeginSynchronizedUpdate).unwrap();
        clear();
        print!("{}", output);
        execute!(stdout, crossterm::terminal::EndSynchronizedUpdate).unwrap();
        stdout.flush().unwrap();
    }
    fn macro_tool_settings(macro_path: &String) {
        let mut settings = Settings::load();
        fn render_macro_tool_settings(
            menu_selected: usize,
            menu_options: &[&str],
            macro_path: &String,
        ) {
            let settings = Settings::load();
            let mut stdout = io::stdout();
            let help_string = String::from("| quit: $[esc]$ | change tab: $[a]/[d]$ | scroll: $[w]/[s]$ | change setting: $[←]/[→]$ |");
            let help_more_string = String::from(
                r#"| change setting: $[ent]$ | select: $[0-9]$ |
| return: $[q]$ | change tab: $[backtab]/[tab]$ | scroll: $[↑]/[↓]$ |"#,
            );
            let (width, _) = terminal::size().unwrap();
            let mut output = String::new();
            output.push_str(&render_top("macro_settings", Some(&macro_path), true));
            for i in 0..menu_options.len() {
                if i == menu_selected {
                    output.push_str(&format!(
                        "│{}{} {} › {} {}{}{}{}│\n",
                        SetBackgroundColor(get_color("main")),
                        SetForegroundColor(Color::Black),
                        i,
                        menu_options[i],
                        if menu_options[i] == "loop" {
                            if settings.macro_loop {
                                "1 ".to_string()
                            } else {
                                "0 ".to_string()
                            }
                        } else if menu_options[i] == "restart_when_pausing" {
                            if settings.macro_restart_when_pausing {
                                "1 ".to_string()
                            } else {
                                "0 ".to_string()
                            }
                        } else if menu_options[i] == "hotkey" {
                            settings.macro_hotkey.to_string() + " "
                        } else {
                            " ".to_string()
                        },
                        SetForegroundColor(get_color("theme")),
                        SetBackgroundColor(Color::Reset),
                        cursor::MoveToColumn(width)
                    ));
                } else {
                    output.push_str(&format!(
                        "│{} {} {}| {}{}{}│\n",
                        SetForegroundColor(get_color("main")),
                        i,
                        SetForegroundColor(Color::DarkGrey),
                        SetForegroundColor(get_color("theme")),
                        menu_options[i],
                        cursor::MoveToColumn(width)
                    ));
                }
            }
            output.push_str(&render_bottom(
                menu_options.len() as u16,
                help_string,
                help_more_string,
            ));
            execute!(stdout, crossterm::terminal::BeginSynchronizedUpdate).unwrap();
            clear();
            print!("{}", output);
            execute!(stdout, crossterm::terminal::EndSynchronizedUpdate).unwrap();
            stdout.flush().unwrap();
        }
        let macro_settings_menu_options = ["loop", "restart_when_pausing", "hotkey"];
        let mut macro_settings_menu_selected = 0;
        let macro_hotkeys = [
            "None", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "X1Mouse", "X2Mouse",
        ];
        let mut macro_hotkey_index = macro_hotkeys
            .iter()
            .position(|&c| c == settings.macro_hotkey)
            .unwrap_or(0);
        let mut last_render_time = get_time();
        let (mut last_width, mut last_height) = terminal::size().unwrap();
        let mut needs_rendering = true;
        loop {
            if let Some((code, _)) = get_key() {
                needs_rendering = true;
                match code {
                    KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                        if macro_settings_menu_selected > 0 {
                            macro_settings_menu_selected -= 1
                        } else {
                            macro_settings_menu_selected = macro_settings_menu_options.len() - 1
                        }
                    }
                    KeyCode::Left => match macro_settings_menu_selected {
                        0 => settings.set_macro_loop(!settings.macro_loop),
                        1 => settings
                            .set_macro_restart_when_pausing(!settings.macro_restart_when_pausing),
                        2 => {
                            if macro_hotkey_index > 0 {
                                settings.set_macro_hotkey(macro_hotkeys[macro_hotkey_index - 1])
                            } else {
                                settings.set_macro_hotkey(macro_hotkeys[macro_hotkeys.len() - 1])
                            };
                            if macro_hotkey_index > 0 {
                                macro_hotkey_index -= 1
                            } else {
                                macro_hotkey_index = macro_hotkeys.len() - 1
                            }
                        }
                        _ => {}
                    },
                    KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
                        if macro_settings_menu_selected < macro_settings_menu_options.len() - 1 {
                            macro_settings_menu_selected += 1
                        } else {
                            macro_settings_menu_selected = 0
                        }
                    }
                    KeyCode::Right | KeyCode::Enter => match macro_settings_menu_selected {
                        0 => settings.set_macro_loop(!settings.macro_loop),
                        1 => settings
                            .set_macro_restart_when_pausing(!settings.macro_restart_when_pausing),
                        2 => {
                            settings.set_macro_hotkey(
                                macro_hotkeys[(macro_hotkey_index + 1) % macro_hotkeys.len()],
                            );
                            macro_hotkey_index = (macro_hotkey_index + 1) % macro_hotkeys.len()
                        }
                        _ => {}
                    },
                    KeyCode::Tab | KeyCode::Char('d') | KeyCode::Char('D') => settings_menu(),
                    KeyCode::BackTab | KeyCode::Char('a') | KeyCode::Char('A') => return,
                    KeyCode::Char('q') | KeyCode::Char('Q') => return,
                    KeyCode::Esc => process::exit(0),
                    KeyCode::Char(c) if c.is_digit(10) => {
                        let num = c.to_digit(10).unwrap() as usize;
                        if num < macro_settings_menu_options.len() {
                            macro_settings_menu_selected = num;
                        };
                    }
                    _ => {}
                }
            }
            let current_time = get_time();
            let (width, height) = terminal::size().unwrap();
            if width != last_width
                || height != last_height
                || current_time != last_render_time
                || needs_rendering
            {
                render_macro_tool_settings(
                    macro_settings_menu_selected,
                    &macro_settings_menu_options,
                    macro_path,
                );
                last_render_time = current_time;
                last_width = width;
                last_height = height;
                needs_rendering = false;
            }
        }
    }
    fn macro_tool_macro(macro_path: &String, dir: &PathBuf) {
        fn render_macro_tool_macro(macro_path: &String, macro_active: bool) {
            let settings = Settings::load();
            let mut stdout = io::stdout();
            let help_string = String::from("| quit: $[esc]$ | change tab: $[a]/[d]$ | change status: $[ent]$ | back: $[←]/[→]$ |");
            let help_more_string = format!(
                r#"| return: $[q]$ | change tab: $[backtab]/[tab]$ | change status: $[Space]/[{}]$ |"#,
                settings.macro_hotkey
            );
            let (width, _) = terminal::size().unwrap();
            let mut output = String::new();
            let is_active = if macro_active { "active" } else { "inactive" };
            output.push_str(&render_top(
                format!("{}", macro_path).as_str(),
                Some("macro_settings"),
                false,
            ));
            output.push_str(&format!(
                "│ Status: {}{}{}{}{}{}│\n",
                SetBackgroundColor(get_color("main")),
                SetForegroundColor(Color::Black),
                is_active,
                SetForegroundColor(get_color("theme")),
                SetBackgroundColor(Color::Reset),
                cursor::MoveToColumn(width)
            ));
            output.push_str(&format!("│{}│\n", cursor::MoveToColumn(width)));
            output.push_str(&format!(
                "│ Hotkey: {}{}[{}]{}{}{}│\n",
                SetBackgroundColor(get_color("main")),
                SetForegroundColor(Color::Black),
                settings.macro_hotkey,
                SetForegroundColor(get_color("theme")),
                SetBackgroundColor(Color::Reset),
                cursor::MoveToColumn(width)
            ));
            output.push_str(&render_bottom(3, help_string, help_more_string));
            execute!(stdout, crossterm::terminal::BeginSynchronizedUpdate).unwrap();
            clear();
            print!("{}", output);
            execute!(stdout, crossterm::terminal::EndSynchronizedUpdate).unwrap();
            stdout.flush().unwrap();
        }
        fn add_macro_action(
            macro_actions: &mut Vec<String>,
            macro_action: String,
            help_more_string_lines: u16,
        ) {
            let settings = Settings::load();
            let (_, height) = terminal::size().unwrap();
            let mut help_length = 0;
            if !settings.hide_help {
                help_length += 1;
                let help_open = HELP_OPEN.lock().unwrap();
                if *help_open {
                    help_length += help_more_string_lines
                }
            }
            let max_macro_actions = height.saturating_sub(14 + help_length).max(1) as usize;
            while macro_actions.len() > max_macro_actions {
                if !macro_actions.is_empty() {
                    macro_actions.remove(0);
                }
            }
            macro_actions.push(macro_action);
        }
        fn print_macro_actions(macro_actions: &mut Vec<String>) -> usize {
            let (width, _) = terminal::size().unwrap();
            let mut stdout = io::stdout();
            let start_y = 11;
            for i in 0..macro_actions.len() {
                execute!(stdout, cursor::MoveTo(0, start_y + i as u16)).unwrap();
                print!("\r│{}│", " ".repeat(width as usize - 2));
            }
            for (i, macro_action) in macro_actions.iter().enumerate() {
                execute!(stdout, cursor::MoveTo(2, start_y + i as u16)).unwrap();
                if macro_action.starts_with('#') {
                    print!(
                        "{}",
                        &format!(
                            "{}{}{}",
                            SetForegroundColor(get_color("main")),
                            macro_action,
                            SetForegroundColor(get_color("theme")),
                        )
                    );
                } else if macro_action.starts_with("[warning]") {
                    print!(
                        "{}",
                        &format!(
                            "{}{}{}",
                            SetForegroundColor(Color::DarkGrey),
                            macro_action,
                            SetForegroundColor(get_color("theme")),
                        )
                    );
                } else {
                    print!("{}", macro_action);
                }
            }
            stdout.flush().unwrap();
            macro_actions.len()
        }
        fn get_key_from_str(key_str: &str) -> Option<Key> {
            let key_map: HashMap<&str, Key> = [
                ("meta", Key::Meta),
                ("start", Key::Meta),
                ("win", Key::Meta),
                ("shift", Key::Shift),
                ("ctrl", Key::Control),
                ("control", Key::Control),
                ("alt", Key::Alt),
                ("space", Key::Space),
                ("ent", Key::Return),
                ("enter", Key::Return),
                ("return", Key::Return),
                ("escape", Key::Escape),
                ("del", Key::Delete),
                ("backspace", Key::Backspace),
                ("tab", Key::Tab),
                ("capslock", Key::CapsLock),
                ("up", Key::UpArrow),
                ("uparrow", Key::UpArrow),
                ("down", Key::DownArrow),
                ("downarrow", Key::DownArrow),
                ("left", Key::LeftArrow),
                ("leftarrow", Key::LeftArrow),
                ("right", Key::RightArrow),
                ("rightarrow", Key::RightArrow),
                ("f1", Key::F1),
                ("f2", Key::F2),
                ("f3", Key::F3),
                ("f4", Key::F4),
                ("f5", Key::F5),
                ("f6", Key::F6),
                ("f7", Key::F7),
                ("f8", Key::F8),
                ("f9", Key::F9),
                ("f10", Key::F10),
                ("f11", Key::F11),
                ("f12", Key::F12),
                ("f13", Key::F13),
                ("f14", Key::F14),
                ("f15", Key::F15),
                ("f16", Key::F16),
                ("f17", Key::F17),
                ("f18", Key::F18),
                ("f19", Key::F19),
                ("f20", Key::F20),
                ("f21", Key::F21),
                ("f22", Key::F22),
                ("f23", Key::F23),
                ("f24", Key::F24),
            ]
            .iter()
            .cloned()
            .collect();
            if key_str.len() == 1 {
                Some(Key::Unicode(key_str.chars().next().unwrap()))
            } else {
                key_map.get(&key_str.to_lowercase()[..]).cloned()
            }
        }
        let mut macro_active = false;
        let mut last_macro_active = macro_active;
        let mut last_render_time = get_time();
        let (mut last_width, mut last_height) = terminal::size().unwrap();
        let mut needs_rendering = true;
        let mut current_line = 0;
        let mut passed_delay = Instant::now();
        let mut current_delay = 0;
        struct LoopState {
            start_line: usize,
            end_line: usize,
            replays_left: u64,
        }
        let mut if_stack: Vec<bool> = Vec::new();
        let mut skip_depth: u32 = 0;
        let mut loop_stack: Vec<LoopState> = Vec::new();
        let mut found_loops: Vec<(u64, u64)> = Vec::new();
        let mut completed_loops: Vec<(u64, u64)> = Vec::new();
        let mut active_loop_starts = HashSet::new();
        let mut macro_actions: Vec<String> = Vec::new();
        let help_more_string_lines = 1;
        let mut prev_state = HashMap::new();
        let mut jumping = false;
        let mut variables: HashMap<String, String> = HashMap::new();
        let mut enigo = Enigo::new(&EnigoSettings::default()).unwrap();
        fn resolve_variable<'a>(
            raw: &'a str,
            variables: &'a HashMap<String, String>,
        ) -> Result<&'a str, String> {
            if let Some(var_name) = raw.strip_prefix('$') {
                match variables.get(var_name) {
                    Some(val) => Ok(val.as_str()),
                    None => Err(var_name.to_string()),
                }
            } else {
                Ok(raw)
            }
        }
        fn evaluate_condition(
            tokens: &[&str],
            variables: &HashMap<String, String>,
        ) -> Result<bool, String> {
            if tokens.len() != 3 {
                return Err("Condition must have exactly 3 parts".into());
            }
            let left = resolve_variable(tokens[0], variables)?;
            let op = tokens[1];
            let right = resolve_variable(tokens[2], variables)?;
            if let (Ok(left_num), Ok(right_num)) = (left.parse::<i64>(), right.parse::<i64>()) {
                match op {
                    "=" | "==" => Ok(left_num == right_num),
                    "!=" => Ok(left_num != right_num),
                    "<" => Ok(left_num < right_num),
                    ">" => Ok(left_num > right_num),
                    "<=" => Ok(left_num <= right_num),
                    ">=" => Ok(left_num >= right_num),
                    _ => Err(format!("Unknown operator: {}", op)),
                }
            } else {
                match op {
                    "=" | "==" => Ok(left == right),
                    "!=" => Ok(left != right),
                    "<" => Ok(left < right),
                    ">" => Ok(left > right),
                    "<=" => Ok(left <= right),
                    ">=" => Ok(left >= right),
                    _ => Err(format!("Unknown operator: {}", op)),
                }
            }
        }
        loop {
            let settings = Settings::load();
            if let Some((code, _)) = get_key() {
                needs_rendering = true;
                match code {
                    KeyCode::Left | KeyCode::Right => return,
                    KeyCode::Tab | KeyCode::Char('d') | KeyCode::Char('D') => {
                        macro_tool_settings(macro_path)
                    }
                    KeyCode::BackTab | KeyCode::Char('a') | KeyCode::Char('A') => {
                        main();
                        return;
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') => return,
                    KeyCode::Esc => process::exit(0),
                    KeyCode::Enter => {
                        macro_active = !macro_active;
                    }
                    KeyCode::Char(' ') => {
                        macro_active = !macro_active;
                    }
                    _ => {}
                }
            }
            if let Some(code) = background_get_key(&mut prev_state) {
                if let Some(hotkey_enum) = string_to_key(&settings.macro_hotkey) {
                    if code == hotkey_enum {
                        macro_active = !macro_active;
                    }
                }
            }
            if macro_active != last_macro_active {
                last_macro_active = macro_active;
                render_macro_tool_macro(macro_path, macro_active);
                print_macro_actions(&mut macro_actions);
                current_delay = 0;
                if settings.macro_restart_when_pausing {
                    current_line = 0;
                    variables.clear();
                    found_loops.clear();
                    completed_loops.clear();
                    loop_stack.clear();
                    active_loop_starts.clear();
                }
            }
            if macro_active {
                if passed_delay.elapsed() >= Duration::from_millis(current_delay) {
                    current_delay = 0;
                    let file = File::open(dir.join(format!("{}.txt", macro_path)))
                        .expect("Failed to open macro file");
                    let reader = BufReader::new(file);
                    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
                    if current_line < lines.len() {
                        let line = &lines[current_line];
                        let trimmed_line = line.trim();
                        if skip_depth > 0 {
                            let command_parts: Vec<&str> =
                                trimmed_line.split_whitespace().collect();
                            if !command_parts.is_empty() {
                                if command_parts[0] == "if" && command_parts.last() == Some(&"{") {
                                    skip_depth += 1;
                                } else if command_parts[0] == "}" {
                                    skip_depth -= 1;
                                }
                            }
                            current_line += 1;
                            continue;
                        }
                        if trimmed_line.is_empty() {
                            current_line += 1;
                            continue;
                        }
                        let command_parts: Vec<&str> = trimmed_line.split_whitespace().collect();
                        match command_parts.get(0).map(|&s| s.to_lowercase()) {
                            Some(ref cmd) if cmd == "#" => {
                                if command_parts.len() > 1 {
                                    if let Some(mut text) = trimmed_line.strip_prefix("#") {
                                        text = text.trim();
                                        add_macro_action(
                                            &mut macro_actions,
                                            format!("# {}", text),
                                            help_more_string_lines,
                                        );
                                    }
                                }
                            }
                            Some(ref cmd) if cmd == "let" || cmd == "var" => {
                                if command_parts.len() >= 4 && command_parts[2] == "=" {
                                    let key = command_parts[1].to_string();
                                    let expr = command_parts[3..].join(" ");
                                    let mut tokens = Vec::new();
                                    for token in expr.split_whitespace() {
                                        if "+-*/".contains(token) {
                                            tokens.push(token.to_string());
                                        } else {
                                            match resolve_variable(token, &variables) {
                                                Ok(val) => tokens.push(val.to_string()),
                                                Err(var_name) => {
                                                    add_macro_action(
                                                        &mut macro_actions,
                                                        format!(
                                                            "[warning] Variable not defined: {}",
                                                            var_name
                                                        ),
                                                        help_more_string_lines,
                                                    );
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                    let mut output: Option<i64> = None;
                                    let mut op: Option<String> = None;
                                    let mut math_failed = false;
                                    for token in tokens {
                                        if ["+", "-", "*", "/"].contains(&token.as_str()) {
                                            op = Some(token);
                                            continue;
                                        }
                                        match token.parse::<i64>() {
                                            Ok(num) => {
                                                if let Some(prev) = output {
                                                    match op.as_deref() {
                                                        Some("+") => output = Some(prev + num),
                                                        Some("-") => output = Some(prev - num),
                                                        Some("*") => output = Some(prev * num),
                                                        Some("/") => output = Some(prev / num),
                                                        _ => {}
                                                    }
                                                } else {
                                                    output = Some(num);
                                                }
                                            }
                                            Err(_) => {
                                                math_failed = true;
                                                break;
                                            }
                                        }
                                    }
                                    if !math_failed && output.is_some() {
                                        let val = output.unwrap().to_string();
                                        variables.insert(key.clone(), val.clone());
                                        add_macro_action(
                                            &mut macro_actions,
                                            format!("Set variable: {} = {}", key, val),
                                            help_more_string_lines,
                                        );
                                    } else {
                                        let fallback = match resolve_variable(&expr, &variables) {
                                            Ok(val) => val.to_string(),
                                            Err(var_name) => {
                                                add_macro_action(
                                                    &mut macro_actions,
                                                    format!(
                                                        "[warning] Variable not defined: {}",
                                                        var_name
                                                    ),
                                                    help_more_string_lines,
                                                );
                                                continue;
                                            }
                                        };
                                        variables.insert(key.clone(), fallback.clone());
                                        add_macro_action(
                                            &mut macro_actions,
                                            format!("Set variable: {} = {}", key, fallback),
                                            help_more_string_lines,
                                        );
                                    }
                                } else {
                                    add_macro_action(
                                        &mut macro_actions,
                                        "[warning] Invalid variable usage".to_string(),
                                        help_more_string_lines,
                                    );
                                }
                            }
                            Some(ref cmd) if cmd == "(" => {
                                if !active_loop_starts.contains(&current_line) {
                                    active_loop_starts.insert(current_line);
                                    let start_line = current_line;
                                    loop_stack.push(LoopState {
                                        start_line,
                                        end_line: 0,
                                        replays_left: 0,
                                    });
                                    add_macro_action(
                                        &mut macro_actions,
                                        format!("Starting loop at line {}", start_line),
                                        help_more_string_lines,
                                    );
                                }
                            }
                            Some(ref cmd) if cmd == ")" => {
                                if let Some(last_index) = loop_stack.len().checked_sub(1) {
                                    let top = &mut loop_stack[last_index];
                                    if top.end_line == 0 {
                                        top.end_line = current_line;
                                        if let Some(replays_str) = command_parts.get(1) {
                                            match resolve_variable(replays_str, &variables) {
                                                Ok(resolved_val) => {
                                                    if let Ok(parsed_replays) =
                                                        resolved_val.parse::<u64>()
                                                    {
                                                        if parsed_replays > 1 {
                                                            top.replays_left = parsed_replays - 1;
                                                            current_line = top.start_line;
                                                            passed_delay = Instant::now();
                                                            add_macro_action(
                                                                &mut macro_actions,
                                                                format!(
                                                                    "Looping back to line {} ({} replays left)",
                                                                    top.start_line, top.replays_left
                                                                ),
                                                                help_more_string_lines,
                                                            );
                                                            continue;
                                                        }
                                                    } else {
                                                        add_macro_action(
                                                            &mut macro_actions,
                                                            format!("[warning] Invalid replay count: {}", resolved_val),
                                                            help_more_string_lines,
                                                        );
                                                    }
                                                }
                                                Err(var_name) => {
                                                    add_macro_action(
                                                        &mut macro_actions,
                                                        format!(
                                                            "[warning] Variable not defined: {}",
                                                            var_name
                                                        ),
                                                        help_more_string_lines,
                                                    );
                                                }
                                            }
                                        } else {
                                            top.replays_left = u64::MAX;
                                            current_line = top.start_line;
                                            passed_delay = Instant::now();
                                            add_macro_action(
                                                &mut macro_actions,
                                                format!(
                                                    "Looping back to line {} (infinite)",
                                                    top.start_line
                                                ),
                                                help_more_string_lines,
                                            );
                                            continue;
                                        }
                                        let finished = loop_stack.pop().unwrap();
                                        active_loop_starts.remove(&finished.start_line);
                                        add_macro_action(
                                            &mut macro_actions,
                                            format!(
                                                "Completed loop from line {}",
                                                finished.start_line
                                            ),
                                            help_more_string_lines,
                                        );
                                    } else {
                                        if top.replays_left == u64::MAX {
                                            current_line = top.start_line;
                                            passed_delay = Instant::now();
                                            add_macro_action(
                                                &mut macro_actions,
                                                format!(
                                                    "Looping back to line {} (infinite)",
                                                    top.start_line
                                                ),
                                                help_more_string_lines,
                                            );
                                            continue;
                                        } else if top.replays_left > 0 {
                                            top.replays_left -= 1;
                                            if top.replays_left > 0 {
                                                current_line = top.start_line;
                                                passed_delay = Instant::now();
                                                add_macro_action(
                                                    &mut macro_actions,
                                                    format!(
                                                        "Looping back to line {} ({} replays left)",
                                                        top.start_line, top.replays_left
                                                    ),
                                                    help_more_string_lines,
                                                );
                                                continue;
                                            } else {
                                                let finished = loop_stack.pop().unwrap();
                                                active_loop_starts.remove(&finished.start_line);
                                                add_macro_action(
                                                    &mut macro_actions,
                                                    format!(
                                                        "Completed loop from line {}",
                                                        finished.start_line
                                                    ),
                                                    help_more_string_lines,
                                                );
                                            }
                                        }
                                    }
                                } else {
                                    add_macro_action(
                                        &mut macro_actions,
                                        "[warning] Unmatched ')' with no '('".to_string(),
                                        help_more_string_lines,
                                    );
                                }
                            }
                            Some(ref cmd) if cmd == "if" => {
                                if skip_depth > 0 {
                                    current_line += 1;
                                    continue;
                                }
                                if command_parts.len() < 5 || command_parts.last() != Some(&"{") {
                                    add_macro_action(
                                        &mut macro_actions,
                                        "[warning] Invalid if: syntax is 'if [var] [op] [value] {{'".to_string(),
                                        help_more_string_lines,
                                    );
                                    current_line += 1;
                                    continue;
                                }
                                let condition_tokens = &command_parts[1..command_parts.len() - 1];
                                match evaluate_condition(condition_tokens, &variables) {
                                    Ok(condition_met) => {
                                        if_stack.push(condition_met);
                                        if condition_met {
                                            add_macro_action(
                                                &mut macro_actions,
                                                format!("Condition met at line: {}", current_line),
                                                help_more_string_lines,
                                            );
                                        } else {
                                            add_macro_action(
                                                &mut macro_actions,
                                                format!(
                                                    "Condition not met at line: {}",
                                                    current_line
                                                ),
                                                help_more_string_lines,
                                            );
                                            skip_depth = 1;
                                        }
                                    }
                                    Err(e) => {
                                        add_macro_action(
                                            &mut macro_actions,
                                            format!("[warning] Condition error: {}", e),
                                            help_more_string_lines,
                                        );
                                    }
                                }
                                current_line += 1;
                                continue;
                            }
                            Some(ref cmd) if cmd == "}" => {
                                if skip_depth > 0 {
                                    skip_depth -= 1;
                                    current_line += 1;
                                    continue;
                                }
                                if if_stack.pop().is_none() {
                                    add_macro_action(
                                        &mut macro_actions,
                                        "[warning] Unmatched '}'".to_string(),
                                        help_more_string_lines,
                                    );
                                }
                                current_line += 1;
                                continue;
                            }
                            Some(ref cmd)
                                if cmd == "jump"
                                    || cmd == "jumpto"
                                    || cmd == "jump_to"
                                    || cmd == "goto"
                                    || cmd == "go_to" =>
                            {
                                if let Some(line_str) = command_parts.get(1) {
                                    match resolve_variable(line_str, &variables) {
                                        Ok(resolved) => {
                                            if let Ok(line) = resolved.parse::<usize>() {
                                                add_macro_action(
                                                    &mut macro_actions,
                                                    format!("Jumped to line: {}", line),
                                                    help_more_string_lines,
                                                );
                                                jumping = true;
                                                current_line = line
                                            } else {
                                                add_macro_action(
                                                    &mut macro_actions,
                                                    format!(
                                                        "[warning] Invalid line value: {}",
                                                        resolved
                                                    ),
                                                    help_more_string_lines,
                                                );
                                            }
                                        }
                                        Err(var_name) => {
                                            add_macro_action(
                                                &mut macro_actions,
                                                format!(
                                                    "[warning] Variable not defined: {}",
                                                    var_name
                                                ),
                                                help_more_string_lines,
                                            );
                                        }
                                    }
                                }
                            }
                            Some(ref cmd) if cmd == "delay" || cmd == "sleep" || cmd == "wait" => {
                                if let Some(delay_str) = command_parts.get(1) {
                                    match resolve_variable(delay_str, &variables) {
                                        Ok(resolved) => {
                                            if let Ok(delay_ms) = resolved.parse::<u64>() {
                                                add_macro_action(
                                                    &mut macro_actions,
                                                    format!("Delay for {} ms", delay_ms),
                                                    help_more_string_lines,
                                                );
                                                current_delay = delay_ms;
                                            } else {
                                                add_macro_action(
                                                    &mut macro_actions,
                                                    format!(
                                                        "[warning] Invalid delay value: {}",
                                                        resolved
                                                    ),
                                                    help_more_string_lines,
                                                );
                                            }
                                        }
                                        Err(var_name) => {
                                            add_macro_action(
                                                &mut macro_actions,
                                                format!(
                                                    "[warning] Variable not defined: {}",
                                                    var_name
                                                ),
                                                help_more_string_lines,
                                            );
                                        }
                                    }
                                }
                            }
                            Some(ref cmd)
                                if cmd == "mouse_click"
                                    || cmd == "mouseclick"
                                    || cmd == "click_mouse"
                                    || cmd == "clickmouse"
                                    || cmd == "mouse" =>
                            {
                                if let Some(raw_button_str) = command_parts.get(1) {
                                    match resolve_variable(raw_button_str, &variables) {
                                        Ok(button_str) => {
                                            match button_str.to_lowercase().as_str() {
                                                "left" | "LMB" => {
                                                    enigo.button(Button::Left, Click).ok();
                                                    add_macro_action(
                                                        &mut macro_actions,
                                                        format!("Clicked: left mouse button"),
                                                        help_more_string_lines,
                                                    );
                                                }
                                                "right" | "RMB" => {
                                                    enigo.button(Button::Right, Click).ok();
                                                    add_macro_action(
                                                        &mut macro_actions,
                                                        format!("Clicked: right mouse button"),
                                                        help_more_string_lines,
                                                    );
                                                }
                                                "middle" | "MMB" => {
                                                    enigo.button(Button::Middle, Click).ok();
                                                    add_macro_action(
                                                        &mut macro_actions,
                                                        format!("Clicked: middle mouse button"),
                                                        help_more_string_lines,
                                                    );
                                                }
                                                _ => add_macro_action(
                                                    &mut macro_actions,
                                                    format!(
                                                        "[warning] Unknown mouse button: {}",
                                                        button_str
                                                    ),
                                                    help_more_string_lines,
                                                ),
                                            }
                                        }
                                        Err(var_name) => {
                                            add_macro_action(
                                                &mut macro_actions,
                                                format!(
                                                    "[warning] Variable not defined: {}",
                                                    var_name
                                                ),
                                                help_more_string_lines,
                                            );
                                        }
                                    }
                                }
                            }
                            Some(ref cmd)
                                if cmd == "mouse_press"
                                    || cmd == "mousepress"
                                    || cmd == "mouse_hold"
                                    || cmd == "mousehold"
                                    || cmd == "press_mouse"
                                    || cmd == "pressmouse"
                                    || cmd == "hold_mouse"
                                    || cmd == "holdmouse" =>
                            {
                                if let Some(raw_button_str) = command_parts.get(1) {
                                    match resolve_variable(raw_button_str, &variables) {
                                        Ok(button_str) => {
                                            match button_str.to_lowercase().as_str() {
                                                "left" | "LMB" => {
                                                    enigo.button(Button::Left, Press).ok();
                                                    add_macro_action(
                                                        &mut macro_actions,
                                                        format!("Pressed: left mouse button"),
                                                        help_more_string_lines,
                                                    );
                                                }
                                                "right" | "RMB" => {
                                                    enigo.button(Button::Right, Press).ok();
                                                    add_macro_action(
                                                        &mut macro_actions,
                                                        format!("Pressed: right mouse button"),
                                                        help_more_string_lines,
                                                    );
                                                }
                                                "middle" | "MMB" => {
                                                    enigo.button(Button::Middle, Press).ok();
                                                    add_macro_action(
                                                        &mut macro_actions,
                                                        format!("Pressed: middle mouse button"),
                                                        help_more_string_lines,
                                                    );
                                                }
                                                _ => add_macro_action(
                                                    &mut macro_actions,
                                                    format!(
                                                        "[warning] Unknown mouse button: {}",
                                                        button_str
                                                    ),
                                                    help_more_string_lines,
                                                ),
                                            }
                                        }
                                        Err(var_name) => {
                                            add_macro_action(
                                                &mut macro_actions,
                                                format!(
                                                    "[warning] Variable not defined: {}",
                                                    var_name
                                                ),
                                                help_more_string_lines,
                                            );
                                        }
                                    }
                                }
                            }
                            Some(ref cmd)
                                if cmd == "mouse_release"
                                    || cmd == "mouserelease"
                                    || cmd == "release_mouse"
                                    || cmd == "releasemouse" =>
                            {
                                if let Some(raw_button_str) = command_parts.get(1) {
                                    match resolve_variable(raw_button_str, &variables) {
                                        Ok(button_str) => {
                                            match button_str.to_lowercase().as_str() {
                                                "left" | "LMB" => {
                                                    enigo.button(Button::Left, Release).ok();
                                                    add_macro_action(
                                                        &mut macro_actions,
                                                        format!("Released: left mouse button"),
                                                        help_more_string_lines,
                                                    );
                                                }
                                                "right" | "RMB" => {
                                                    enigo.button(Button::Right, Release).ok();
                                                    add_macro_action(
                                                        &mut macro_actions,
                                                        format!("Released: right mouse button"),
                                                        help_more_string_lines,
                                                    );
                                                }
                                                "middle" | "MMB" => {
                                                    enigo.button(Button::Middle, Release).ok();
                                                    add_macro_action(
                                                        &mut macro_actions,
                                                        format!("Released: middle mouse button"),
                                                        help_more_string_lines,
                                                    );
                                                }
                                                _ => add_macro_action(
                                                    &mut macro_actions,
                                                    format!(
                                                        "[warning] Unknown mouse button: {}",
                                                        button_str
                                                    ),
                                                    help_more_string_lines,
                                                ),
                                            }
                                        }
                                        Err(var_name) => {
                                            add_macro_action(
                                                &mut macro_actions,
                                                format!(
                                                    "[warning] Variable not defined: {}",
                                                    var_name
                                                ),
                                                help_more_string_lines,
                                            );
                                        }
                                    }
                                }
                            }
                            Some(ref cmd)
                                if cmd == "mouse_scroll"
                                    || cmd == "mousescroll"
                                    || cmd == "scroll_mouse"
                                    || cmd == "scrollmouse"
                                    || cmd == "scroll" =>
                            {
                                if let Some(length_str) = command_parts.get(1) {
                                    match resolve_variable(length_str, &variables) {
                                        Ok(resolved) => {
                                            if let Ok(length) = resolved.parse::<i32>() {
                                                enigo.scroll(length, enigo::Axis::Vertical).ok();
                                                add_macro_action(
                                                    &mut macro_actions,
                                                    format!("Scrolled by: {}", length),
                                                    help_more_string_lines,
                                                );
                                            } else {
                                                add_macro_action(
                                                    &mut macro_actions,
                                                    format!(
                                                        "[warning] Invalid scroll value: {}",
                                                        resolved
                                                    ),
                                                    help_more_string_lines,
                                                );
                                            }
                                        }
                                        Err(var_name) => {
                                            add_macro_action(
                                                &mut macro_actions,
                                                format!(
                                                    "[warning] Variable not defined: {}",
                                                    var_name
                                                ),
                                                help_more_string_lines,
                                            );
                                        }
                                    }
                                }
                            }
                            Some(ref cmd)
                                if cmd == "mouse_move"
                                    || cmd == "mousemove"
                                    || cmd == "move_mouse"
                                    || cmd == "movemouse"
                                    || cmd == "move_to"
                                    || cmd == "moveto"
                                    || cmd == "move" =>
                            {
                                if let (Some(x_raw), Some(y_raw)) =
                                    (command_parts.get(1), command_parts.get(2))
                                {
                                    let x_val = resolve_variable(x_raw, &variables);
                                    let y_val = resolve_variable(y_raw, &variables);
                                    match (x_val, y_val) {
                                        (Ok(x_str), Ok(y_str)) => {
                                            if let (Ok(x), Ok(y)) =
                                                (x_str.parse::<i32>(), y_str.parse::<i32>())
                                            {
                                                let mut relative = false;
                                                if let Some(mode_str) = command_parts.get(3) {
                                                    let mode_str = mode_str.to_lowercase();
                                                    relative = mode_str == "rel"
                                                        || mode_str == "relative"
                                                        || mode_str == "r";
                                                }
                                                if relative {
                                                    enigo.move_mouse(x, y, Coordinate::Rel).ok();
                                                    add_macro_action(
                                                        &mut macro_actions,
                                                        format!("Moved mouse by ({}, {})", x, y),
                                                        help_more_string_lines,
                                                    );
                                                } else {
                                                    enigo.move_mouse(x, y, Coordinate::Abs).ok();
                                                    add_macro_action(
                                                        &mut macro_actions,
                                                        format!("Moved mouse to ({}, {})", x, y),
                                                        help_more_string_lines,
                                                    );
                                                }
                                            } else {
                                                add_macro_action(
                                                    &mut macro_actions,
                                                    format!(
                                                        "[warning] Invalid coordinates: {}, {}",
                                                        x_str, y_str
                                                    ),
                                                    help_more_string_lines,
                                                );
                                            }
                                        }
                                        (Err(missing), _) | (_, Err(missing)) => {
                                            add_macro_action(
                                                &mut macro_actions,
                                                format!(
                                                    "[warning] Variable not defined: {}",
                                                    missing
                                                ),
                                                help_more_string_lines,
                                            );
                                        }
                                    }
                                }
                            }
                            Some(ref cmd) if cmd == "click" => {
                                if let Some(key_str_raw) = command_parts.get(1) {
                                    match resolve_variable(key_str_raw, &variables) {
                                        Ok(key_str) => {
                                            if let Some(key) = get_key_from_str(key_str) {
                                                enigo.key(key, Click).ok();
                                                add_macro_action(
                                                    &mut macro_actions,
                                                    format!("Clicked key: {}", key_str),
                                                    help_more_string_lines,
                                                );
                                            } else {
                                                add_macro_action(
                                                    &mut macro_actions,
                                                    format!("[warning] Unknown key: {}", key_str),
                                                    help_more_string_lines,
                                                );
                                            }
                                        }
                                        Err(var_name) => {
                                            add_macro_action(
                                                &mut macro_actions,
                                                format!(
                                                    "[warning] Variable not defined: {}",
                                                    var_name
                                                ),
                                                help_more_string_lines,
                                            );
                                        }
                                    }
                                }
                            }
                            Some(ref cmd) if cmd == "press" || cmd == "hold" => {
                                if let Some(key_str_raw) = command_parts.get(1) {
                                    match resolve_variable(key_str_raw, &variables) {
                                        Ok(key_str) => {
                                            if let Some(key) = get_key_from_str(key_str) {
                                                enigo.key(key, Press).ok();
                                                add_macro_action(
                                                    &mut macro_actions,
                                                    format!("Pressed key: {}", key_str),
                                                    help_more_string_lines,
                                                );
                                            } else {
                                                add_macro_action(
                                                    &mut macro_actions,
                                                    format!("[warning] Unknown key: {}", key_str),
                                                    help_more_string_lines,
                                                );
                                            }
                                        }
                                        Err(var_name) => {
                                            add_macro_action(
                                                &mut macro_actions,
                                                format!(
                                                    "[warning] Variable not defined: {}",
                                                    var_name
                                                ),
                                                help_more_string_lines,
                                            );
                                        }
                                    }
                                }
                            }
                            Some(ref cmd) if cmd == "release" => {
                                if let Some(key_str_raw) = command_parts.get(1) {
                                    match resolve_variable(key_str_raw, &variables) {
                                        Ok(key_str) => {
                                            if let Some(key) = get_key_from_str(key_str) {
                                                enigo.key(key, Release).ok();
                                                add_macro_action(
                                                    &mut macro_actions,
                                                    format!("Released key: {}", key_str),
                                                    help_more_string_lines,
                                                );
                                            } else {
                                                add_macro_action(
                                                    &mut macro_actions,
                                                    format!("[warning] Unknown key: {}", key_str),
                                                    help_more_string_lines,
                                                );
                                            }
                                        }
                                        Err(var_name) => {
                                            add_macro_action(
                                                &mut macro_actions,
                                                format!(
                                                    "[warning] Variable not defined: {}",
                                                    var_name
                                                ),
                                                help_more_string_lines,
                                            );
                                        }
                                    }
                                }
                            }
                            Some(ref cmd) if cmd == "string" || cmd == "text" => {
                                if command_parts.len() > 1 {
                                    if let Some(mut text) = trimmed_line.strip_prefix("string") {
                                        text = text.trim();
                                        if text.starts_with('$') {
                                            let key_str = &text[1..];
                                            match variables.get(key_str) {
                                                Some(value) => {
                                                    enigo.text(value).ok();
                                                    add_macro_action(
                                                        &mut macro_actions,
                                                        format!("Typed: {}", value),
                                                        help_more_string_lines,
                                                    );
                                                }
                                                None => {
                                                    add_macro_action(
                                                        &mut macro_actions,
                                                        format!(
                                                            "[warning] Variable not defined: {}",
                                                            key_str
                                                        ),
                                                        help_more_string_lines,
                                                    );
                                                }
                                            }
                                        } else {
                                            enigo.text(&text).ok();
                                            add_macro_action(
                                                &mut macro_actions,
                                                format!("Typed: {}", text),
                                                help_more_string_lines,
                                            );
                                        }
                                    }
                                }
                            }
                            _ => {
                                add_macro_action(
                                    &mut macro_actions,
                                    format!("[warning] Unknown command: {}", trimmed_line),
                                    help_more_string_lines,
                                );
                            }
                        }
                    }
                    print_macro_actions(&mut macro_actions);
                    passed_delay = Instant::now();
                    if current_line < lines.len() {
                        if !jumping {
                            current_line += 1
                        }
                        jumping = false
                    } else {
                        current_line = 0;
                        variables.clear();
                        found_loops.clear();
                        completed_loops.clear();
                        loop_stack.clear();
                        active_loop_starts.clear();
                        if !settings.macro_loop {
                            macro_active = false;
                        }
                    }
                }
            }
            let current_time = get_time();
            let (width, height) = terminal::size().unwrap();
            if width != last_width
                || height != last_height
                || current_time != last_render_time
                || needs_rendering
            {
                render_macro_tool_macro(macro_path, macro_active);
                print_macro_actions(&mut macro_actions);
                last_render_time = current_time;
                last_width = width;
                last_height = height;
                needs_rendering = false;
            }
        }
    }
    fn refresh_macro_menu(
        macro_menu_options: &mut Vec<String>,
        macro_menu_selected: &mut usize,
        current_dir: &PathBuf,
    ) {
        macro_menu_options.clear();
        macro_menu_options.push("new_macro".to_string());
        macro_menu_options.push("new_folder".to_string());
        if let Ok(entries) = fs::read_dir(&current_dir) {
            for entry in entries.flatten() {
                if let Some(stem) = entry.path().file_stem() {
                    let stem_str = stem.to_string_lossy().into_owned();
                    macro_menu_options.push(stem_str);
                }
            }
        }
        if *macro_menu_selected >= macro_menu_options.len() {
            *macro_menu_selected = macro_menu_options.len().saturating_sub(1);
        }
    }
    let mut stdout = io::stdout();
    let mut macro_menu_options: Vec<String> =
        vec!["new_macro".to_string(), "new_folder".to_string()];
    let mut macro_menu_selected = 0;
    let mut last_render_time = get_time();
    let (mut last_width, mut last_height) = terminal::size().unwrap();
    let mut needs_rendering = true;
    let dir = Path::new("NUUI_config");
    let macros_dir = dir.join("Macros");
    let mut current_dir = macros_dir.clone();
    let mut path_stack = vec![macros_dir.clone()];
    if !macros_dir.exists() {
        fs::create_dir_all(&macros_dir)
            .expect("Failed to create macros directory inside config directory");
    }
    match fs::read_dir(&macros_dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let filename = entry.file_name();
                    if let Some(stem) = Path::new(&filename).file_stem() {
                        let stem_str = stem.to_string_lossy().into_owned();
                        macro_menu_options.push(stem_str);
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to read directory: {}", e);
        }
    }
    loop {
        if let Some((code, _)) = get_key() {
            needs_rendering = true;
            match code {
                KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                    if macro_menu_selected > 0 {
                        macro_menu_selected -= 1
                    } else {
                        macro_menu_selected = macro_menu_options.len() - 1
                    }
                }
                KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
                    if macro_menu_selected < macro_menu_options.len() - 1 {
                        macro_menu_selected += 1
                    } else {
                        macro_menu_selected = 0
                    }
                }
                KeyCode::Left | KeyCode::Right => {
                    if path_stack.len() > 1 {
                        path_stack.pop();
                        current_dir = path_stack.last().unwrap().clone();
                        refresh_macro_menu(
                            &mut macro_menu_options,
                            &mut macro_menu_selected,
                            &current_dir,
                        );
                        macro_menu_selected = 0;
                    }
                }
                KeyCode::Tab | KeyCode::Char('d') | KeyCode::Char('D') => {
                    macro_tool_settings(&"macro".to_string())
                }
                KeyCode::BackTab | KeyCode::Char('a') | KeyCode::Char('A') => return,
                KeyCode::Char('q') | KeyCode::Char('Q') => return,
                KeyCode::Esc => process::exit(0),
                KeyCode::Delete | KeyCode::Backspace => match macro_menu_selected {
                    0 | 1 => {}
                    _ => {
                        let selected_item = &macro_menu_options[macro_menu_selected];
                        let file_candidate = current_dir.join(format!("{}.txt", selected_item));
                        let folder_candidate = current_dir.join(selected_item);
                        if file_candidate.is_file() {
                            match fs::remove_file(&file_candidate) {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!("Failed to delete file {}: {}", selected_item, e);
                                }
                            }
                        } else if folder_candidate.is_dir() {
                            match fs::remove_dir_all(&folder_candidate) {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!("Failed to delete folder {}: {}", selected_item, e);
                                }
                            }
                        }
                        refresh_macro_menu(
                            &mut macro_menu_options,
                            &mut macro_menu_selected,
                            &current_dir,
                        );
                    }
                },
                KeyCode::Enter => match macro_menu_selected {
                    0 => {
                        execute!(stdout, cursor::MoveUp(1)).unwrap();
                        execute!(stdout, cursor::MoveToColumn(2)).unwrap();
                        if macro_menu_options.len() > 10 {
                            execute!(stdout, cursor::MoveLeft(1)).unwrap();
                        }
                        print!(
                            "{}{} {}|{} ",
                            SetForegroundColor(get_color("main")),
                            macro_menu_options.len() - 2,
                            SetForegroundColor(Color::DarkGrey),
                            SetForegroundColor(get_color("theme"))
                        );
                        stdout.flush().unwrap();
                        let mut name = String::new();
                        io::stdin().read_line(&mut name).unwrap();
                        let name = name.trim().replace(" ", "_");
                        if !name.is_empty() {
                            let new_file_path = current_dir.join(format!("{}.txt", name));
                            match File::create(&new_file_path) {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!("Failed to create file: {}", e);
                                }
                            }
                        }
                        refresh_macro_menu(
                            &mut macro_menu_options,
                            &mut macro_menu_selected,
                            &current_dir,
                        );
                        macro_menu_selected = 0;
                    }
                    1 => {
                        execute!(stdout, cursor::MoveUp(1)).unwrap();
                        execute!(stdout, cursor::MoveToColumn(2)).unwrap();
                        if macro_menu_options.len() > 10 {
                            execute!(stdout, cursor::MoveLeft(1)).unwrap();
                        }
                        print!(
                            "{}{} {}•{} ",
                            SetForegroundColor(get_color("main")),
                            macro_menu_options.len() - 2,
                            SetForegroundColor(Color::DarkGrey),
                            SetForegroundColor(get_color("theme"))
                        );
                        stdout.flush().unwrap();
                        let mut folder_name = String::new();
                        io::stdin().read_line(&mut folder_name).unwrap();
                        let folder_name = folder_name.trim().replace(" ", "_");
                        if !folder_name.is_empty() {
                            let new_folder_path = current_dir.join(&folder_name);
                            match fs::create_dir_all(&new_folder_path) {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!("Failed to create folder: {}", e);
                                }
                            }
                        }
                        refresh_macro_menu(
                            &mut macro_menu_options,
                            &mut macro_menu_selected,
                            &current_dir,
                        );
                        macro_menu_selected = 0;
                    }
                    _ => {
                        let selected_item = &macro_menu_options[macro_menu_selected];
                        let selected_path = current_dir.join(selected_item);
                        if selected_path.is_dir() {
                            path_stack.push(selected_path.clone());
                            current_dir = selected_path;
                            refresh_macro_menu(
                                &mut macro_menu_options,
                                &mut macro_menu_selected,
                                &current_dir,
                            );
                            macro_menu_selected = 0;
                        } else {
                            macro_tool_macro(selected_item, &current_dir);
                        }
                    }
                },
                KeyCode::Char(' ') => match macro_menu_selected {
                    0 | 1 => {}
                    _ => {
                        let selected_item = &macro_menu_options[macro_menu_selected];
                        let selected_path = current_dir.join(format!("{}.txt", selected_item));
                        if selected_path.is_file() {
                            match run_file(selected_path.to_str().unwrap()) {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!("Failed to open file {}: {}", selected_item, e);
                                }
                            }
                        }
                    }
                },
                KeyCode::Char(c) if c.is_digit(10) => {
                    let num = c.to_digit(10).unwrap() as usize;
                    if num < macro_menu_options.len() - 2 {
                        macro_menu_selected = num + 2;
                        let selected_item = &macro_menu_options[macro_menu_selected];
                        let selected_path = current_dir.join(selected_item);
                        if selected_path.is_dir() {
                            path_stack.push(selected_path.clone());
                            current_dir = selected_path;
                            refresh_macro_menu(
                                &mut macro_menu_options,
                                &mut macro_menu_selected,
                                &current_dir,
                            );
                            macro_menu_selected = 0;
                        } else {
                            macro_tool_macro(selected_item, &current_dir);
                        }
                    }
                }
                _ => {}
            }
        }
        let current_time = get_time();
        let (width, height) = terminal::size().unwrap();
        if width != last_width
            || height != last_height
            || current_time != last_render_time
            || needs_rendering
        {
            render_macro_tool_menu(
                macro_menu_selected,
                &macro_menu_options
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<&str>>(),
                &current_dir,
            );
            last_render_time = current_time;
            last_width = width;
            last_height = height;
            needs_rendering = false;
        }
    }
}

fn tetris() {
    fn render_tetris() {
        let mut stdout = io::stdout();
        let help_string = String::from(
            "| quit: $[esc]$ | change tab: $[a]/[d]$ | move: $[←]/[→]/[↓/s]$ | rotate: $[↑/w]$ |",
        );
        let help_more_string = String::from(
            r#"| return: $[q]$ | change tab: $[backtab]/[tab]$ | drop: $[space]$ | hold: $[c]$ | pause: $[p]$ |"#,
        );
        let mut output = String::new();
        output.push_str(&render_top("tetris", Some("tetris_settings"), false));
        output.push_str(&render_bottom(0, help_string, help_more_string));
        execute!(stdout, crossterm::terminal::BeginSynchronizedUpdate).unwrap();
        clear();
        print!("{}", output);
        execute!(stdout, crossterm::terminal::EndSynchronizedUpdate).unwrap();
        stdout.flush().unwrap();
    }
    fn tetris_settings(speed_multiplier_changed: &mut bool) {
        let mut settings = Settings::load();
        fn render_tetris_settings(menu_selected: usize, menu_options: &[&str]) {
            let settings = Settings::load();
            let mut stdout = io::stdout();
            let help_string = String::from("| quit: $[esc]$ | change tab: $[a]/[d]$ | scroll: $[w]/[s]$ | change setting: $[←]/[→]$ |");
            let help_more_string = String::from(
                r#"| change setting: $[ent]$ | select: $[0-9]$ |
| return: $[q]$ | change tab: $[backtab]/[tab]$ | scroll: $[↑]/[↓]$ |"#,
            );
            let (width, _) = terminal::size().unwrap();
            let mut output = String::new();
            output.push_str(&render_top("tetris_settings", Some("tetris"), true));
            for i in 0..menu_options.len() {
                if i == menu_selected {
                    output.push_str(&format!(
                        "│{}{} {} › {} {}{}{}{}│\n",
                        SetBackgroundColor(get_color("main")),
                        SetForegroundColor(Color::Black),
                        i,
                        menu_options[i],
                        if menu_options[i] == "use_colors" {
                            if settings.tetris_use_colors {
                                "1 ".to_string()
                            } else {
                                "0 ".to_string()
                            }
                        } else if menu_options[i] == "show_ghost" {
                            if settings.tetris_show_ghost {
                                "1 ".to_string()
                            } else {
                                "0 ".to_string()
                            }
                        } else if menu_options[i] == "speed_multiplier" {
                            settings.tetris_speed_multiplier.to_string() + " "
                        } else {
                            " ".to_string()
                        },
                        SetForegroundColor(get_color("theme")),
                        SetBackgroundColor(Color::Reset),
                        cursor::MoveToColumn(width)
                    ));
                } else {
                    output.push_str(&format!(
                        "│{} {} {}| {}{}{}│\n",
                        SetForegroundColor(get_color("main")),
                        i,
                        SetForegroundColor(Color::DarkGrey),
                        SetForegroundColor(get_color("theme")),
                        menu_options[i],
                        cursor::MoveToColumn(width)
                    ));
                }
            }
            output.push_str(&render_bottom(
                menu_options.len() as u16,
                help_string,
                help_more_string,
            ));
            execute!(stdout, crossterm::terminal::BeginSynchronizedUpdate).unwrap();
            clear();
            print!("{}", output);
            execute!(stdout, crossterm::terminal::EndSynchronizedUpdate).unwrap();
            stdout.flush().unwrap();
        }
        let tetris_settings_menu_options = ["use_colors", "show_ghost", "speed_multiplier"];
        let mut tetris_settings_menu_selected = 0;
        let tetris_speed_multipliers = [0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 1.75, 2.0, 2.5, 3.0];
        let mut tetris_speed_multiplier_index = tetris_speed_multipliers
            .iter()
            .position(|&c| c == settings.tetris_speed_multiplier)
            .unwrap_or(0);
        let mut last_render_time = get_time();
        let (mut last_width, mut last_height) = terminal::size().unwrap();
        let mut needs_rendering = true;
        loop {
            if let Some((code, _)) = get_key() {
                needs_rendering = true;
                match code {
                    KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                        if tetris_settings_menu_selected > 0 {
                            tetris_settings_menu_selected -= 1
                        } else {
                            tetris_settings_menu_selected = tetris_settings_menu_options.len() - 1
                        }
                    }
                    KeyCode::Left => match tetris_settings_menu_selected {
                        0 => settings.set_tetris_use_colors(!settings.tetris_use_colors),
                        1 => settings.set_tetris_show_ghost(!settings.tetris_show_ghost),
                        2 => {
                            *speed_multiplier_changed = true;
                            if tetris_speed_multiplier_index > 0 {
                                settings.set_tetris_speed_multiplier(
                                    tetris_speed_multipliers[tetris_speed_multiplier_index - 1],
                                )
                            } else {
                                settings.set_tetris_speed_multiplier(
                                    tetris_speed_multipliers[tetris_speed_multipliers.len() - 1],
                                )
                            };
                            if tetris_speed_multiplier_index > 0 {
                                tetris_speed_multiplier_index -= 1
                            } else {
                                tetris_speed_multiplier_index = tetris_speed_multipliers.len() - 1
                            }
                        }
                        _ => {}
                    },
                    KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
                        if tetris_settings_menu_selected < tetris_settings_menu_options.len() - 1 {
                            tetris_settings_menu_selected += 1
                        } else {
                            tetris_settings_menu_selected = 0
                        }
                    }
                    KeyCode::Right | KeyCode::Enter => match tetris_settings_menu_selected {
                        0 => settings.set_tetris_use_colors(!settings.tetris_use_colors),
                        1 => settings.set_tetris_show_ghost(!settings.tetris_show_ghost),
                        2 => {
                            *speed_multiplier_changed = true;
                            settings.set_tetris_speed_multiplier(
                                tetris_speed_multipliers[(tetris_speed_multiplier_index + 1)
                                    % tetris_speed_multipliers.len()],
                            );
                            tetris_speed_multiplier_index =
                                (tetris_speed_multiplier_index + 1) % tetris_speed_multipliers.len()
                        }
                        _ => {}
                    },
                    KeyCode::Tab | KeyCode::Char('d') | KeyCode::Char('D') => settings_menu(),
                    KeyCode::BackTab | KeyCode::Char('a') | KeyCode::Char('A') => return,
                    KeyCode::Char('q') | KeyCode::Char('Q') => return,
                    KeyCode::Esc => process::exit(0),
                    KeyCode::Char(c) if c.is_digit(10) => {
                        let num = c.to_digit(10).unwrap() as usize;
                        if num < tetris_settings_menu_options.len() {
                            tetris_settings_menu_selected = num;
                        };
                    }
                    _ => {}
                }
            }
            let current_time = get_time();
            let (width, height) = terminal::size().unwrap();
            if width != last_width
                || height != last_height
                || current_time != last_render_time
                || needs_rendering
            {
                render_tetris_settings(
                    tetris_settings_menu_selected,
                    &tetris_settings_menu_options,
                );
                last_render_time = current_time;
                last_width = width;
                last_height = height;
                needs_rendering = false;
            }
        }
    }
    fn print_table(
        mut table: Vec<Vec<u8>>,
        hold_piece: &Option<Tetromino>,
        next_piece: &Tetromino,
        current_piece: &Tetromino,
        score: u32,
        level: u32,
        speed_multiplier_changed: bool,
        paused: &mut bool,
        game_over: bool,
    ) {
        let settings = Settings::load();
        let (width, height) = terminal::size().unwrap();
        let (logo_0, _, _) = logo();
        let logo_0_lines: Vec<&str> = logo_0.lines().collect();
        let mut stdout = io::stdout();
        let mut output = String::new();
        if height
            .saturating_sub(logo_0_lines.len() as u16)
            .saturating_sub(if !settings.hide_help {
                1 + if *HELP_OPEN.lock().unwrap() && !settings.hide_help {
                    1
                } else {
                    0
                }
            } else {
                0
            })
            > 22
        {
            output.push_str(&format!(
                "{}score {}{}level {}",
                cursor::MoveTo(
                    width - score.to_string().len() as u16 - 7,
                    logo_0_lines.len() as u16 + 2
                ),
                score,
                cursor::MoveTo(
                    width - level.to_string().len() as u16 - 7,
                    logo_0_lines.len() as u16 + 3
                ),
                level
            ));
            output.push_str(&format!(
                "{}",
                cursor::MoveTo(0, logo_0_lines.len() as u16 + 1)
            ));
            clear_piece_from_board(&mut table, current_piece);
            place_ghost_piece_on_board(&mut table, current_piece);
            place_piece_on_board(&mut table, current_piece);
            let board_width = table[0].len();
            let mut board_lines: Vec<String> = Vec::new();
            if height
                .saturating_sub(logo_0_lines.len() as u16)
                .saturating_sub(if !settings.hide_help {
                    1 + if *HELP_OPEN.lock().unwrap() && !settings.hide_help {
                        1
                    } else {
                        0
                    }
                } else {
                    0
                })
                .saturating_sub(1)
                > 22
            {
                board_lines.push(format!("│┌{}┐", "──".repeat(board_width)));
            } else {
                board_lines.push(format!("{}╭┼──────┴─╯        ╰─┴┬", cursor::MoveUp(1)));
            }
            for (y, row) in table.iter().enumerate() {
                let mut line = String::from("││");
                if settings.tetris_use_colors {
                    let mut last_color: Option<Color> = None;
                    for &cell in row.iter() {
                        let color = match cell {
                            1 => Some(Color::DarkCyan),
                            2 => Some(Color::DarkYellow),
                            3 => Some(Color::DarkMagenta),
                            4 => Some(Color::DarkBlue),
                            5 => Some(Color::Blue),
                            6 => Some(Color::DarkGreen),
                            7 => Some(Color::DarkRed),
                            _ => Some(get_color("theme")),
                        };
                        if color != last_color {
                            line.push_str(&format!("{}", SetForegroundColor(color.unwrap())));
                            last_color = color;
                        }
                        match cell {
                            0 => line.push_str("  "),
                            9 => {
                                if settings.tetris_show_ghost {
                                    line.push_str("▒▒")
                                } else {
                                    line.push_str("  ")
                                }
                            }
                            _ => line.push_str("██"),
                        }
                    }
                    line.push_str(&format!("{}", SetForegroundColor(get_color("theme"))));
                } else {
                    for (x, &cell) in row.iter().enumerate() {
                        let is_current_piece_block = current_piece.shape.iter().any(|(dx, dy)| {
                            let px = current_piece.x + dx;
                            let py = current_piece.y + dy;
                            px == x as i32 && py == y as i32
                        });
                        if is_current_piece_block {
                            line.push_str(&format!("{}", SetForegroundColor(get_color("main"))));
                        }
                        match cell {
                            0 => line.push_str("  "),
                            9 => {
                                if settings.tetris_show_ghost {
                                    line.push_str("▒▒")
                                } else {
                                    line.push_str("  ")
                                }
                            }
                            _ => line.push_str("██"),
                        }
                        if is_current_piece_block {
                            line.push_str(&format!("{}", SetForegroundColor(get_color("theme"))));
                        }
                    }
                }
                line.push('│');
                board_lines.push(line);
            }
            if height
                .saturating_sub(logo_0_lines.len() as u16)
                .saturating_sub(if !settings.hide_help {
                    1 + if *HELP_OPEN.lock().unwrap() && !settings.hide_help {
                        1
                    } else {
                        0
                    }
                } else {
                    0
                })
                .saturating_sub(2)
                > 22
            {
                board_lines.push(format!("│└{}┘", "──".repeat(board_width)));
            } else {
                board_lines.push(format!("╰┴{}┴", "──".repeat(board_width)));
            }
            let mut hold_box: Vec<String> = Vec::new();
            if height
                .saturating_sub(logo_0_lines.len() as u16)
                .saturating_sub(if !settings.hide_help {
                    1 + if *HELP_OPEN.lock().unwrap() && !settings.hide_help {
                        1
                    } else {
                        0
                    }
                } else {
                    0
                })
                .saturating_sub(1)
                <= 22
            {
                hold_box.push("".to_string());
            }
            hold_box.push("┌─ hold ─┐".to_string());
            for y in 0..4 {
                let mut line = String::from("│");
                for x in 0..4 {
                    let block = match hold_piece {
                        Some(p) => {
                            let min_x = p.shape.iter().map(|(x, _)| *x).min().unwrap_or(0);
                            let max_x = p.shape.iter().map(|(x, _)| *x).max().unwrap_or(0);
                            let min_y = p.shape.iter().map(|(_, y)| *y).min().unwrap_or(0);
                            let max_y = p.shape.iter().map(|(_, y)| *y).max().unwrap_or(0);
                            let offset_x = (4 - (max_x - min_x + 1)) / 2 - min_x;
                            let offset_y = (4 - (max_y - min_y + 1)) / 2 - min_y;
                            p.shape
                                .iter()
                                .any(|(dx, dy)| *dx + offset_x == x && *dy + offset_y == y)
                        }
                        None => false,
                    };
                    if block {
                        if settings.tetris_use_colors {
                            let piece_id = hold_piece.as_ref().map(|p| p.color).unwrap_or(0);
                            let color = match piece_id {
                                1 => Color::DarkCyan,
                                2 => Color::DarkYellow,
                                3 => Color::DarkMagenta,
                                4 => Color::DarkBlue,
                                5 => Color::Blue,
                                6 => Color::DarkGreen,
                                7 => Color::DarkRed,
                                _ => get_color("theme"),
                            };
                            line.push_str(&format!(
                                "{}██{}",
                                SetForegroundColor(color),
                                SetForegroundColor(get_color("theme"))
                            ));
                        } else {
                            line.push_str(&format!(
                                "{}██{}",
                                SetForegroundColor(get_color("main")),
                                SetForegroundColor(get_color("theme"))
                            ));
                        }
                    } else {
                        line.push_str("  ");
                    }
                }
                line.push_str("│");
                hold_box.push(line);
            }
            hold_box.push("└────────┘".to_string());
            let mut next_box: Vec<String> = Vec::new();
            next_box.push("┌─ next ─┐".to_string());
            for y in 0..4 {
                let mut line = String::from("│");
                for x in 0..4 {
                    let min_x = next_piece.shape.iter().map(|(x, _)| *x).min().unwrap_or(0);
                    let max_x = next_piece.shape.iter().map(|(x, _)| *x).max().unwrap_or(0);
                    let min_y = next_piece.shape.iter().map(|(_, y)| *y).min().unwrap_or(0);
                    let max_y = next_piece.shape.iter().map(|(_, y)| *y).max().unwrap_or(0);
                    let offset_x = (4 - (max_x - min_x + 1)) / 2 - min_x;
                    let offset_y = (4 - (max_y - min_y + 1)) / 2 - min_y;
                    let block = next_piece
                        .shape
                        .iter()
                        .any(|(dx, dy)| *dx + offset_x == x && *dy + offset_y == y);
                    if block {
                        if settings.tetris_use_colors {
                            let piece_id = next_piece.color;
                            let color = match piece_id {
                                1 => Color::DarkCyan,
                                2 => Color::DarkYellow,
                                3 => Color::DarkMagenta,
                                4 => Color::DarkBlue,
                                5 => Color::Blue,
                                6 => Color::DarkGreen,
                                7 => Color::DarkRed,
                                _ => get_color("theme"),
                            };
                            line.push_str(&format!(
                                "{}██{}",
                                SetForegroundColor(color),
                                SetForegroundColor(get_color("theme"))
                            ));
                        } else {
                            line.push_str(&format!(
                                "{}██{}",
                                SetForegroundColor(get_color("main")),
                                SetForegroundColor(get_color("theme"))
                            ));
                        }
                    } else {
                        line.push_str("  ");
                    }
                }
                line.push_str("│");
                next_box.push(line);
            }
            next_box.push("└────────┘".to_string());
            let total_lines = board_lines.len().max(hold_box.len() + next_box.len());
            for i in 0..total_lines {
                let board_line = board_lines.get(i).map(|s| s.as_str()).unwrap_or("");
                let right_line = if i < hold_box.len() {
                    hold_box[i].clone()
                } else if i < hold_box.len() + next_box.len() {
                    next_box[i - hold_box.len()].clone()
                } else {
                    "".to_string()
                };
                output.push_str(&format!(
                    "\n{:<width$}{}",
                    board_line,
                    right_line,
                    width = board_width * 2 + 2
                ));
            }
            if *paused {
                let score_text = format!("score {}", score);
                let level_text = format!("level {}", level);
                let speed_text = format!(
                    "speed {}",
                    if !speed_multiplier_changed {
                        settings.tetris_speed_multiplier.to_string()
                    } else {
                        format!("{} (varied)", settings.tetris_speed_multiplier)
                    }
                );
                let score_padding_left = (22 - score_text.len()) / 2;
                let score_padding_right = 22 - score_padding_left - score_text.len();
                let level_padding_left = (22 - level_text.len()) / 2;
                let level_padding_right = 22 - level_padding_left - level_text.len();
                let speed_padding_left = (22 - speed_text.len()) / 2;
                let speed_padding_right = 22 - speed_padding_left - speed_text.len();
                let score_line = format!(
                    "│{}{}{}{}{}│",
                    " ".repeat(score_padding_left),
                    SetForegroundColor(get_color("main")),
                    score_text,
                    SetForegroundColor(get_color("theme")),
                    " ".repeat(score_padding_right)
                );
                let level_line = format!(
                    "│{}{}{}{}{}│",
                    " ".repeat(level_padding_left),
                    SetForegroundColor(get_color("main")),
                    level_text,
                    SetForegroundColor(get_color("theme")),
                    " ".repeat(level_padding_right)
                );
                let speed_line = format!(
                    "│{}{}{}{}{}│",
                    " ".repeat(speed_padding_left),
                    SetForegroundColor(get_color("main")),
                    speed_text,
                    SetForegroundColor(get_color("theme")),
                    " ".repeat(speed_padding_right)
                );
                let center_x = (width - 24) / 2;
                let center_y = (height.saturating_sub(3) + logo_0_lines.len() as u16) / 2;
                output.push_str(&format!(
                    "{}{}{}{}{}{}{}{}{}{}",
                    cursor::MoveTo(center_x, center_y),
                    "┌─────── paused ───────┐",
                    cursor::MoveTo(center_x, center_y + 1),
                    score_line,
                    cursor::MoveTo(center_x, center_y + 2),
                    level_line,
                    cursor::MoveTo(center_x, center_y + 3),
                    speed_line,
                    cursor::MoveTo(center_x, center_y + 4),
                    "└──────────────────────┘"
                ));
            }
            if game_over {
                let score_text = format!("score {}", score);
                let level_text = format!("level {}", level);
                let speed_text = format!(
                    "speed {}",
                    if !speed_multiplier_changed {
                        settings.tetris_speed_multiplier.to_string()
                    } else {
                        format!("{} (varied)", settings.tetris_speed_multiplier)
                    }
                );
                let score_padding_left = (21 - score_text.len()) / 2;
                let score_padding_right = 21 - score_padding_left - score_text.len();
                let level_padding_left = (21 - level_text.len()) / 2;
                let level_padding_right = 21 - level_padding_left - level_text.len();
                let speed_padding_left = (21 - speed_text.len()) / 2;
                let speed_padding_right = 21 - speed_padding_left - speed_text.len();
                let score_line = format!(
                    "│{}{}{}{}{}│",
                    " ".repeat(score_padding_left),
                    SetForegroundColor(get_color("main")),
                    score_text,
                    SetForegroundColor(get_color("theme")),
                    " ".repeat(score_padding_right)
                );
                let level_line = format!(
                    "│{}{}{}{}{}│",
                    " ".repeat(level_padding_left),
                    SetForegroundColor(get_color("main")),
                    level_text,
                    SetForegroundColor(get_color("theme")),
                    " ".repeat(level_padding_right)
                );
                let speed_line = format!(
                    "│{}{}{}{}{}│",
                    " ".repeat(speed_padding_left),
                    SetForegroundColor(get_color("main")),
                    speed_text,
                    SetForegroundColor(get_color("theme")),
                    " ".repeat(speed_padding_right)
                );
                let center_x = (width - 23) / 2;
                let center_y = (height.saturating_sub(3) + logo_0_lines.len() as u16) / 2;
                output.push_str(&format!(
                    "{}{}{}{}{}{}{}{}{}{}",
                    cursor::MoveTo(center_x, center_y),
                    "┌───── game over ─────┐",
                    cursor::MoveTo(center_x, center_y + 1),
                    score_line,
                    cursor::MoveTo(center_x, center_y + 2),
                    level_line,
                    cursor::MoveTo(center_x, center_y + 3),
                    speed_line,
                    cursor::MoveTo(center_x, center_y + 4),
                    "└─────────────────────┘"
                ));
            }
        } else {
            *paused = true;
            let text = format!(
                "│ {}{}{} │",
                SetForegroundColor(get_color("main")),
                "window is too small",
                SetForegroundColor(get_color("theme")),
            );
            let center_x = (width - 23) / 2;
            let center_y = (height.saturating_sub(3) + logo_0_lines.len() as u16) / 2;
            output.push_str(&format!(
                "{}{}{}{}{}{}",
                cursor::MoveTo(center_x, center_y),
                "┌─────── error ───────┐",
                cursor::MoveTo(center_x, center_y + 1),
                text,
                cursor::MoveTo(center_x, center_y + 2),
                "└─────────────────────┘"
            ));
        }
        print!("{}", output);
        stdout.flush().unwrap();
    }
    #[derive(Clone)]
    struct Tetromino {
        shape: Vec<(i32, i32)>,
        pivot: (i32, i32),
        x: i32,
        y: i32,
        color: u8,
    }
    impl Tetromino {
        fn new(shape: Vec<(i32, i32)>, pivot: (i32, i32), color: u8) -> Self {
            Self {
                shape,
                pivot,
                x: 4,
                y: 0,
                color,
            }
        }
        fn with_position(mut self, x: i32, y: i32) -> Self {
            self.x = x;
            self.y = y;
            self
        }
        fn rotate(&mut self) {
            if self.color == 2 {
                return;
            }
            let (px, py) = self.pivot;
            self.shape = self
                .shape
                .iter()
                .map(|&(x, y)| {
                    let rel_x = x - px;
                    let rel_y = y - py;
                    let rot_x = -rel_y;
                    let rot_y = rel_x;
                    (rot_x + px, rot_y + py)
                })
                .collect();
        }
    }
    fn spawn_piece() -> Tetromino {
        let (shape, pivot, color, spawn_y) = match rand::random::<usize>() % 7 {
            0 => (vec![(0, 1), (1, 1), (2, 1), (3, 1)], (1, 1), 1, 0),
            1 => (vec![(0, 0), (1, 0), (0, 1), (1, 1)], (0, 0), 2, 0),
            2 => (vec![(0, 0), (1, 0), (2, 0), (1, 1)], (1, 0), 3, 0),
            3 => (vec![(0, 0), (1, 0), (2, 0), (2, 1)], (1, 0), 4, 0),
            4 => (vec![(0, 0), (1, 0), (2, 0), (0, 1)], (1, 0), 5, 0),
            5 => (vec![(1, 0), (2, 0), (0, 1), (1, 1)], (1, 1), 6, 0),
            _ => (vec![(0, 0), (1, 0), (1, 1), (2, 1)], (1, 1), 7, 0),
        };
        Tetromino::new(shape, pivot, color).with_position(4 - pivot.0, spawn_y - pivot.1)
    }
    fn place_ghost_piece_on_board(table: &mut Vec<Vec<u8>>, piece: &Tetromino) {
        let mut ghost = piece.clone();
        for row in table.iter_mut() {
            for cell in row.iter_mut() {
                if *cell == 9 {
                    *cell = 0;
                }
            }
        }
        while can_move(&ghost, table, 0, 1) {
            ghost.y += 1;
        }
        for &(dx, dy) in &ghost.shape {
            let x = ghost.x + dx;
            let y = ghost.y + dy;
            if y >= 0 && y < table.len() as i32 && x >= 0 && x < table[0].len() as i32 {
                let ux = x as usize;
                let uy = y as usize;
                if table[uy][ux] == 0 {
                    table[uy][ux] = 9;
                }
            }
        }
    }
    fn place_piece_on_board(table: &mut Vec<Vec<u8>>, piece: &Tetromino) {
        for &(dx, dy) in &piece.shape {
            let x_pos = (piece.x + dx) as usize;
            let y_pos = (piece.y + dy) as usize;
            if x_pos < table[0].len() && y_pos < table.len() {
                table[y_pos][x_pos] = piece.color;
            }
        }
    }
    fn clear_piece_from_board(table: &mut Vec<Vec<u8>>, piece: &Tetromino) {
        for &(dx, dy) in &piece.shape {
            let x = (piece.x + dx) as usize;
            let y = (piece.y + dy) as usize;
            if x < table[0].len() && y < table.len() {
                table[y][x] = 0;
            }
        }
    }
    fn lock_piece(piece: &Tetromino, table: &mut Vec<Vec<u8>>) {
        for &(dx, dy) in &piece.shape {
            let x = piece.x + dx;
            let y = piece.y + dy;
            if x >= 0 && x < table[0].len() as i32 && y >= 0 && y < table.len() as i32 {
                table[y as usize][x as usize] = piece.color;
            }
        }
    }
    fn clear_lines(
        table: &mut Vec<Vec<u8>>,
        score: &mut u32,
        level: &mut u32,
        lines_for_next_level: &mut u32,
    ) -> u32 {
        let mut cleared_lines = 0;
        let mut new_table: Vec<Vec<u8>> = Vec::new();
        for row in table.iter() {
            if row.iter().all(|&cell| cell != 0) {
                cleared_lines += 1;
                continue;
            }
            new_table.push(row.clone());
        }
        while new_table.len() < 20 {
            new_table.insert(0, vec![0; 10]);
        }
        *table = new_table;
        let points = match cleared_lines {
            1 => 40 * (*level + 1),
            2 => 100 * (*level + 1),
            3 => 300 * (*level + 1),
            4 => 1200 * (*level + 1),
            _ => 0,
        };
        *score += points;
        *lines_for_next_level = lines_for_next_level.saturating_sub(cleared_lines);
        if *lines_for_next_level == 0 {
            *lines_for_next_level = 10;
            *level += 1;
        }
        cleared_lines
    }
    fn can_move(piece: &Tetromino, table: &Vec<Vec<u8>>, dx: i32, dy: i32) -> bool {
        for &(bx, by) in &piece.shape {
            let new_x = piece.x + bx + dx;
            let new_y = piece.y + by + dy;
            if new_x < 0 || new_x >= table[0].len() as i32 {
                return false;
            }
            if new_y >= table.len() as i32 {
                return false;
            }
            if new_y >= 0 {
                let cell = table[new_y as usize][new_x as usize];
                if cell != 0 && cell != 9 {
                    return false;
                }
            }
        }
        true
    }
    fn move_piece_right(piece: &mut Tetromino, table: &mut Vec<Vec<u8>>) {
        clear_piece_from_board(table, piece);
        if can_move(piece, table, 1, 0) {
            piece.x += 1;
        }
        place_piece_on_board(table, piece);
    }
    fn move_piece_left(piece: &mut Tetromino, table: &mut Vec<Vec<u8>>) {
        clear_piece_from_board(table, piece);
        if can_move(piece, table, -1, 0) {
            piece.x -= 1;
        }
        place_piece_on_board(table, piece);
    }
    fn move_piece_down(
        piece: &mut Tetromino,
        table: &mut Vec<Vec<u8>>,
        score: &mut u32,
        count_score: bool,
    ) -> bool {
        clear_piece_from_board(table, piece);
        if can_move(piece, table, 0, 1) {
            piece.y += 1;
            if count_score {
                *score += 1;
            }
            place_piece_on_board(table, piece);
            return true;
        } else {
            lock_piece(piece, table);
            return false;
        }
    }
    fn hard_drop(piece: &mut Tetromino, table: &mut Vec<Vec<u8>>, score: &mut u32) {
        let mut rows_fallen = 0;
        clear_piece_from_board(table, piece);
        while can_move(piece, table, 0, 1) {
            piece.y += 1;
            rows_fallen += 1;
        }
        *score += rows_fallen * 2;
        lock_piece(piece, table);
    }
    fn rotate_piece(piece: &mut Tetromino, table: &mut Vec<Vec<u8>>) {
        let original_shape = piece.shape.clone();
        let original_x = piece.x;
        let original_y = piece.y;
        clear_piece_from_board(table, piece);
        piece.rotate();
        let kicks = [(0, 0), (-1, 0), (1, 0), (0, -1), (0, -2), (-2, 0), (2, 0)];
        for (dx, dy) in kicks {
            if can_move(piece, table, dx, dy) {
                piece.x += dx;
                piece.y += dy;
                place_piece_on_board(table, piece);
                return;
            }
        }
        piece.shape = original_shape;
        piece.x = original_x;
        piece.y = original_y;
        place_piece_on_board(table, piece);
    }
    fn is_game_over(piece: &Tetromino, table: &Vec<Vec<u8>>) -> bool {
        for (dx, dy) in &piece.shape {
            let x = piece.x + dx;
            let y = piece.y + dy;
            if y < 0 {
                continue;
            }
            if y as usize >= table.len() || x < 0 || x as usize >= table[0].len() {
                return true;
            }
            if table[y as usize][x as usize] != 0 {
                return true;
            }
        }
        false
    }
    fn reset_game(
        table: &mut Vec<Vec<u8>>,
        hold_piece: &mut Option<Tetromino>,
        current_piece: &mut Tetromino,
        next_piece: &mut Tetromino,
        score: &mut u32,
        level: &mut u32,
        speed_multiplier_changed: &mut bool,
        lines_for_next_level: &mut u32,
        can_hold: &mut bool,
    ) {
        *table = vec![vec![0; 10]; 20];
        *hold_piece = None;
        *score = 0;
        *level = 1;
        *speed_multiplier_changed = false;
        *lines_for_next_level = 10;
        *can_hold = true;
        *current_piece = spawn_piece();
        *next_piece = spawn_piece();
        place_piece_on_board(table, current_piece);
    }
    let mut table: Vec<Vec<u8>> = vec![vec![0; 10]; 20];
    let mut current_piece = spawn_piece();
    place_piece_on_board(&mut table, &current_piece);
    let mut hold_piece: Option<Tetromino> = None;
    let mut can_hold = true;
    let mut next_piece = spawn_piece();
    let mut last_render_time = get_time();
    let mut score = 0;
    let mut level = 1;
    let mut lines_for_next_level = 10;
    let mut game_over = false;
    let mut speed_multiplier_changed = false;
    let mut paused = false;
    let mut game_over_delay = Instant::now();
    let mut last_gravity_tick = Instant::now();
    let (mut last_width, mut last_height) = terminal::size().unwrap();
    let mut needs_rendering = true;
    loop {
        let settings = Settings::load();
        if let Some((code, _)) = get_key() {
            if game_over {
                needs_rendering = true;
                match code {
                    KeyCode::Tab | KeyCode::Char('d') | KeyCode::Char('D') => {
                        tetris_settings(&mut speed_multiplier_changed);
                    }
                    KeyCode::BackTab | KeyCode::Char('a') | KeyCode::Char('A') => return,
                    KeyCode::Char('q') | KeyCode::Char('Q') => return,
                    KeyCode::Esc => process::exit(0),
                    _ => {
                        if game_over_delay.elapsed() >= Duration::from_millis(2000) {
                            reset_game(
                                &mut table,
                                &mut hold_piece,
                                &mut current_piece,
                                &mut next_piece,
                                &mut score,
                                &mut level,
                                &mut speed_multiplier_changed,
                                &mut lines_for_next_level,
                                &mut can_hold,
                            );
                            game_over = false;
                        }
                    }
                }
            } else {
                match code {
                    KeyCode::Char('p') | KeyCode::Char('P') => {
                        paused = !paused;
                        needs_rendering = true
                    }
                    KeyCode::Enter => {
                        if paused {
                            paused = !paused
                        }
                        needs_rendering = true
                    }
                    KeyCode::Tab | KeyCode::Char('d') | KeyCode::Char('D') => {
                        needs_rendering = true;
                        tetris_settings(&mut speed_multiplier_changed);
                    }
                    KeyCode::BackTab | KeyCode::Char('a') | KeyCode::Char('A') => return,
                    KeyCode::Char('q') | KeyCode::Char('Q') => return,
                    KeyCode::Esc => process::exit(0),
                    _ => {
                        if !paused {
                            match code {
                                KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                                    rotate_piece(&mut current_piece, &mut table);
                                    print_table(
                                        table.clone(),
                                        &hold_piece,
                                        &next_piece,
                                        &current_piece,
                                        score,
                                        level,
                                        speed_multiplier_changed,
                                        &mut paused,
                                        game_over,
                                    );
                                }
                                KeyCode::Left => {
                                    move_piece_left(&mut current_piece, &mut table);
                                    print_table(
                                        table.clone(),
                                        &hold_piece,
                                        &next_piece,
                                        &current_piece,
                                        score,
                                        level,
                                        speed_multiplier_changed,
                                        &mut paused,
                                        game_over,
                                    );
                                }
                                KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
                                    if !move_piece_down(
                                        &mut current_piece,
                                        &mut table,
                                        &mut score,
                                        true,
                                    ) {
                                        clear_lines(
                                            &mut table,
                                            &mut score,
                                            &mut level,
                                            &mut lines_for_next_level,
                                        );
                                        current_piece = next_piece;
                                        next_piece = spawn_piece();
                                        if is_game_over(&current_piece, &table) {
                                            game_over = true;
                                            game_over_delay = Instant::now()
                                        }
                                        place_piece_on_board(&mut table, &current_piece);
                                        can_hold = true;
                                    }
                                    print_table(
                                        table.clone(),
                                        &hold_piece,
                                        &next_piece,
                                        &current_piece,
                                        score,
                                        level,
                                        speed_multiplier_changed,
                                        &mut paused,
                                        game_over,
                                    );
                                }
                                KeyCode::Right => {
                                    move_piece_right(&mut current_piece, &mut table);
                                    print_table(
                                        table.clone(),
                                        &hold_piece,
                                        &next_piece,
                                        &current_piece,
                                        score,
                                        level,
                                        speed_multiplier_changed,
                                        &mut paused,
                                        game_over,
                                    );
                                }
                                KeyCode::Char(' ') => {
                                    hard_drop(&mut current_piece, &mut table, &mut score);
                                    clear_lines(
                                        &mut table,
                                        &mut score,
                                        &mut level,
                                        &mut lines_for_next_level,
                                    );
                                    current_piece = next_piece;
                                    next_piece = spawn_piece();
                                    if is_game_over(&current_piece, &table) {
                                        game_over = true;
                                        game_over_delay = Instant::now()
                                    }
                                    place_piece_on_board(&mut table, &current_piece);
                                    can_hold = true;
                                    print_table(
                                        table.clone(),
                                        &hold_piece,
                                        &next_piece,
                                        &current_piece,
                                        score,
                                        level,
                                        speed_multiplier_changed,
                                        &mut paused,
                                        game_over,
                                    );
                                }
                                KeyCode::Char('c') | KeyCode::Char('C') => {
                                    if can_hold {
                                        clear_piece_from_board(&mut table, &current_piece);
                                        if let Some(mut held) = hold_piece.take() {
                                            std::mem::swap(&mut held, &mut current_piece);
                                            hold_piece = Some(held);
                                            current_piece = current_piece.clone().with_position(
                                                4 - current_piece.pivot.0,
                                                0 - current_piece.pivot.1,
                                            );
                                        } else {
                                            hold_piece = Some(current_piece.clone());
                                            current_piece = next_piece;
                                            next_piece = spawn_piece();
                                        }
                                        place_piece_on_board(&mut table, &current_piece);
                                        can_hold = false;
                                        print_table(
                                            table.clone(),
                                            &hold_piece,
                                            &next_piece,
                                            &current_piece,
                                            score,
                                            level,
                                            speed_multiplier_changed,
                                            &mut paused,
                                            game_over,
                                        );
                                    }
                                }
                                _ => needs_rendering = true,
                            }
                        } else {
                            needs_rendering = true;
                        }
                    }
                }
            }
        }
        if last_gravity_tick.elapsed()
            >= Duration::from_millis(
                ((1000 - (level * 50).min(800)) as f64 / settings.tetris_speed_multiplier) as u64,
            )
            && !paused
            && !game_over
        {
            if !move_piece_down(&mut current_piece, &mut table, &mut score, false) {
                clear_lines(
                    &mut table,
                    &mut score,
                    &mut level,
                    &mut lines_for_next_level,
                );
                current_piece = next_piece;
                next_piece = spawn_piece();
                if is_game_over(&current_piece, &table) {
                    game_over = true;
                    game_over_delay = Instant::now()
                }
                place_piece_on_board(&mut table, &current_piece);
                can_hold = true;
            }
            print_table(
                table.clone(),
                &hold_piece,
                &next_piece,
                &current_piece,
                score,
                level,
                speed_multiplier_changed,
                &mut paused,
                game_over,
            );
            last_gravity_tick = Instant::now();
        }
        let current_time = get_time();
        let (width, height) = terminal::size().unwrap();
        if width != last_width
            || height != last_height
            || current_time != last_render_time
            || needs_rendering
        {
            render_tetris();
            print_table(
                table.clone(),
                &hold_piece,
                &next_piece,
                &current_piece,
                score,
                level,
                speed_multiplier_changed,
                &mut paused,
                game_over,
            );
            last_render_time = current_time;
            last_width = width;
            last_height = height;
            needs_rendering = false;
        }
    }
}

fn game_of_life() {
    let help_string = String::from(
        "| quit: $[esc]$ | change tab: $[a]/[d]$ | select: $[space]$ | play: $[ent]$ |",
    );
    let help_more_string = String::from(
        r#"| return: $[q]$ | change tab: $[backtab]/[tab]$ | move: $[←]/[→]/[↑/w]/[↓/s]$ |"#,
    );
    let help_string_simulating = String::from(
        "| quit: $[esc]$ | change tab: $[a]/[d]$ | pause: $[space]$ | stop: $[ent]$ |",
    );
    let help_more_string_simulating = String::from(
        r#"| return: $[q]$ | change tab: $[backtab]/[tab]$ | move one gen: $[↑/w]/[↓/s]$ | change delay: $[←]/[→]$ |"#,
    );
    let help_line_count = help_more_string.lines().count() as u16;
    fn render_game_of_life(help_string: &String, help_more_string: &String) {
        let mut stdout = io::stdout();
        let mut output = String::new();
        output.push_str(&render_top(
            "game_of_life",
            Some("game_of_life_settings"),
            false,
        ));
        output.push_str(&render_bottom(
            0,
            help_string.clone(),
            help_more_string.clone(),
        ));
        execute!(stdout, crossterm::terminal::BeginSynchronizedUpdate).unwrap();
        clear();
        print!("{}", output);
        execute!(stdout, crossterm::terminal::EndSynchronizedUpdate).unwrap();
        stdout.flush().unwrap();
    }
    fn game_of_life_settings() {
        fn render_game_of_life_settings(menu_selected: usize, menu_options: &[&str]) {
            let settings = Settings::load();
            let mut stdout = io::stdout();
            let help_string = String::from("| quit: $[esc]$ | change tab: $[a]/[d]$ | scroll: $[w]/[s]$ | change setting: $[←]/[→]$ |");
            let help_more_string = String::from(
                r#"| change setting: $[ent]$ | select: $[0-9]$ |
| return: $[q]$ | change tab: $[backtab]/[tab]$ | scroll: $[↑]/[↓]$ |"#,
            );
            let (width, _) = terminal::size().unwrap();
            let mut output = String::new();
            output.push_str(&render_top(
                "game_of_life_settings",
                Some("game_of_life"),
                true,
            ));
            for i in 0..menu_options.len() {
                if i == menu_selected {
                    output.push_str(&format!(
                        "│{}{} {} › {} {}{}{}{}│\n",
                        SetBackgroundColor(get_color("main")),
                        SetForegroundColor(Color::Black),
                        i,
                        menu_options[i],
                        if menu_options[i] == "simulate_delay" {
                            settings.game_of_life_simulate_delay.to_string() + "ms "
                        } else if menu_options[i] == "save_input" {
                            if settings.game_of_life_save_input {
                                "1 ".to_string()
                            } else {
                                "0 ".to_string()
                            }
                        } else if menu_options[i] == "show_generation" {
                            if settings.game_of_life_show_generation {
                                "1 ".to_string()
                            } else {
                                "0 ".to_string()
                            }
                        } else {
                            " ".to_string()
                        },
                        SetForegroundColor(get_color("theme")),
                        SetBackgroundColor(Color::Reset),
                        cursor::MoveToColumn(width)
                    ));
                } else {
                    output.push_str(&format!(
                        "│{} {} {}| {}{}{}│\n",
                        SetForegroundColor(get_color("main")),
                        i,
                        SetForegroundColor(Color::DarkGrey),
                        SetForegroundColor(get_color("theme")),
                        menu_options[i],
                        cursor::MoveToColumn(width)
                    ));
                }
            }
            output.push_str(&render_bottom(
                menu_options.len() as u16,
                help_string,
                help_more_string,
            ));
            execute!(stdout, crossterm::terminal::BeginSynchronizedUpdate).unwrap();
            clear();
            print!("{}", output);
            execute!(stdout, crossterm::terminal::EndSynchronizedUpdate).unwrap();
            stdout.flush().unwrap();
        }
        let mut settings = Settings::load();
        let game_of_life_settings_menu_options =
            ["simulate_delay", "save_input", "show_generation"];
        let mut game_of_life_settings_menu_selected = 0;
        let game_of_life_simulate_delays = [5, 10, 25, 50, 75, 100, 200, 500, 1000];
        let mut game_of_life_simulate_delay_index = game_of_life_simulate_delays
            .iter()
            .position(|&c| c == settings.game_of_life_simulate_delay)
            .unwrap_or(0);
        let mut last_render_time = get_time();
        let (mut last_width, mut last_height) = terminal::size().unwrap();
        let mut needs_rendering = true;
        loop {
            if let Some((code, _)) = get_key() {
                needs_rendering = true;
                match code {
                    KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                        if game_of_life_settings_menu_selected > 0 {
                            game_of_life_settings_menu_selected -= 1
                        } else {
                            game_of_life_settings_menu_selected =
                                game_of_life_settings_menu_options.len() - 1
                        }
                    }
                    KeyCode::Left => match game_of_life_settings_menu_selected {
                        0 => {
                            if game_of_life_simulate_delay_index > 0 {
                                settings.set_game_of_life_simulate_delay(
                                    game_of_life_simulate_delays
                                        [game_of_life_simulate_delay_index - 1],
                                )
                            } else {
                                settings.set_game_of_life_simulate_delay(
                                    game_of_life_simulate_delays
                                        [game_of_life_simulate_delays.len() - 1],
                                )
                            };
                            if game_of_life_simulate_delay_index > 0 {
                                game_of_life_simulate_delay_index -= 1
                            } else {
                                game_of_life_simulate_delay_index =
                                    game_of_life_simulate_delays.len() - 1
                            }
                        }
                        1 => {
                            settings.set_game_of_life_save_input(!settings.game_of_life_save_input)
                        }
                        2 => settings.set_game_of_life_show_generation(
                            !settings.game_of_life_show_generation,
                        ),
                        _ => {}
                    },
                    KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
                        if game_of_life_settings_menu_selected
                            < game_of_life_settings_menu_options.len() - 1
                        {
                            game_of_life_settings_menu_selected += 1
                        } else {
                            game_of_life_settings_menu_selected = 0
                        }
                    }
                    KeyCode::Right | KeyCode::Enter => match game_of_life_settings_menu_selected {
                        0 => {
                            settings.set_game_of_life_simulate_delay(
                                game_of_life_simulate_delays[(game_of_life_simulate_delay_index
                                    + 1)
                                    % game_of_life_simulate_delays.len()],
                            );
                            game_of_life_simulate_delay_index = (game_of_life_simulate_delay_index
                                + 1)
                                % game_of_life_simulate_delays.len()
                        }
                        1 => {
                            settings.set_game_of_life_save_input(!settings.game_of_life_save_input)
                        }
                        2 => settings.set_game_of_life_show_generation(
                            !settings.game_of_life_show_generation,
                        ),
                        _ => {}
                    },
                    KeyCode::Tab | KeyCode::Char('d') | KeyCode::Char('D') => settings_menu(),
                    KeyCode::BackTab | KeyCode::Char('a') | KeyCode::Char('A') => return,
                    KeyCode::Char('q') | KeyCode::Char('Q') => return,
                    KeyCode::Esc => process::exit(0),
                    KeyCode::Char(c) if c.is_digit(10) => {
                        let num = c.to_digit(10).unwrap() as usize;
                        if num < game_of_life_settings_menu_options.len() {
                            game_of_life_settings_menu_selected = num;
                        };
                    }
                    _ => {}
                }
            }
            let current_time = get_time();
            let (width, height) = terminal::size().unwrap();
            if width != last_width
                || height != last_height
                || current_time != last_render_time
                || needs_rendering
            {
                render_game_of_life_settings(
                    game_of_life_settings_menu_selected,
                    &game_of_life_settings_menu_options,
                );
                last_render_time = current_time;
                last_width = width;
                last_height = height;
                needs_rendering = false;
            }
        }
    }
    fn simulate(table: &mut Vec<Vec<i32>>) {
        let mut next_table = table.clone();
        for i in 0..table.len() {
            for j in 0..table[i].len() {
                let mut neighbors = 0;
                for k in -1..=1 {
                    for l in -1..=1 {
                        if k == 0 && l == 0 {
                            continue;
                        }
                        let ni = i as isize + k;
                        let nj = j as isize + l;
                        if ni >= 0
                            && ni < table.len() as isize
                            && nj >= 0
                            && nj < table[i].len() as isize
                        {
                            if table[ni as usize][nj as usize] == 1 {
                                neighbors += 1;
                            }
                        }
                    }
                }
                if table[i][j] == 1 && (neighbors < 2 || neighbors > 3) {
                    next_table[i][j] = 0;
                } else if table[i][j] == 0 && neighbors == 3 {
                    next_table[i][j] = 1;
                }
            }
        }
        *table = next_table;
    }
    fn refresh_table(
        table: &mut Vec<Vec<i32>>,
        cursor_row: &mut usize,
        cursor_col: &mut usize,
        help_more_string_lines: u16,
    ) {
        let settings = Settings::load();
        let (width, height) = terminal::size().unwrap();
        let (logo_0, _, _) = logo();
        let logo_0_lines: Vec<&str> = logo_0.lines().collect();
        let mut help_length = 0;
        if !settings.hide_help {
            help_length += 1;
            let help_open = HELP_OPEN.lock().unwrap();
            if *help_open {
                help_length += help_more_string_lines
            }
        }
        let adjusted_height = (height as usize)
            .saturating_sub(logo_0_lines.len())
            .saturating_sub(3)
            .saturating_sub(help_length as usize);
        let new_height = adjusted_height * 2;
        let new_width = width as usize - 2;
        let mut new_table = vec![vec![0; new_width]; new_height];
        for i in 0..new_height.min(table.len()) {
            for j in 0..new_width.min(table[i].len()) {
                new_table[i][j] = table[i][j];
            }
        }
        *cursor_row = (*cursor_row).min(new_height.saturating_sub(1));
        *cursor_col = (*cursor_col).min(new_width.saturating_sub(1));
        *table = new_table;
    }
    fn render_table(
        table: &Vec<Vec<i32>>,
        cursor_row: usize,
        cursor_col: usize,
        generation: u32,
        last_delay_render: Instant,
        delay_changed: bool,
        simulating: bool,
        paused: bool,
    ) {
        let settings = Settings::load();
        let mut stdout = io::stdout();
        let mut output = String::new();
        let (logo_0, _, _) = logo();
        let logo_0_lines = logo_0.lines().count();
        let mut iter = table.chunks(2);
        let mut row_idx = 0;
        while let Some(rows) = iter.next() {
            let upper = &rows[0];
            let default_row = vec![0; upper.len()];
            let lower = rows.get(1).unwrap_or(&default_row);
            let row_str: String = upper
                .iter()
                .zip(lower.iter())
                .enumerate()
                .map(|(col_idx, (&u, &l))| {
                    let symbol = match (u, l) {
                        (1, 1) => "█",
                        (1, 0) => "▀",
                        (0, 1) => "▄",
                        _ => " ",
                    };
                    let cursor_on_upper = cursor_row % 2 == 0;
                    let cursor_matches = row_idx == cursor_row / 2 && col_idx == cursor_col;
                    if cursor_matches && !simulating {
                        let mut _highlighted_square = String::new();
                        match cursor_on_upper {
                            true => _highlighted_square = "▀".to_string(),
                            false => _highlighted_square = "▄".to_string(),
                        }
                        let highlighted_symbol = match (u, l) {
                            (1, 1) => format!(
                                "{}{}{}{}{}",
                                SetForegroundColor(get_color("main")),
                                SetBackgroundColor(get_color("theme")),
                                _highlighted_square,
                                SetForegroundColor(get_color("theme")),
                                SetBackgroundColor(Color::Reset)
                            ),
                            (1, 0) => match cursor_on_upper {
                                true => format!(
                                    "{}{}{}",
                                    SetForegroundColor(get_color("main")),
                                    _highlighted_square,
                                    SetForegroundColor(get_color("theme"))
                                ),
                                false => format!(
                                    "{}{}{}{}{}",
                                    SetForegroundColor(get_color("main")),
                                    SetBackgroundColor(get_color("theme")),
                                    _highlighted_square,
                                    SetForegroundColor(get_color("theme")),
                                    SetBackgroundColor(Color::Reset)
                                ),
                            },
                            (0, 1) => match cursor_on_upper {
                                true => format!(
                                    "{}{}{}{}{}",
                                    SetForegroundColor(get_color("main")),
                                    SetBackgroundColor(get_color("theme")),
                                    _highlighted_square,
                                    SetForegroundColor(get_color("theme")),
                                    SetBackgroundColor(Color::Reset)
                                ),
                                false => format!(
                                    "{}{}{}",
                                    SetForegroundColor(get_color("main")),
                                    _highlighted_square,
                                    SetForegroundColor(get_color("theme"))
                                ),
                            },
                            (0, 0) => format!(
                                "{}{}{}",
                                SetForegroundColor(get_color("main")),
                                _highlighted_square,
                                SetForegroundColor(get_color("theme"))
                            ),
                            (_, _) => "".to_string(),
                        };
                        highlighted_symbol
                    } else {
                        symbol.to_string()
                    }
                })
                .collect();
            output.push_str(&format!("│{}│", row_str));
            row_idx += 1;
        }
        if simulating {
            if settings.game_of_life_show_generation {
                let gen_str = format!("│gen {}", generation);
                let mut new_output = String::new();
                new_output.push_str(&gen_str);
                new_output.push_str(
                    &output
                        .chars()
                        .skip(gen_str.chars().count())
                        .collect::<String>(),
                );
                output = new_output;
            }
            if paused {
                let paused_str = format!("paused");
                if let Some((second_pipe_index, _)) = output.match_indices('│').nth(1) {
                    let start_index = output[..second_pipe_index]
                        .char_indices()
                        .rev()
                        .nth(paused_str.chars().count() - 1)
                        .map_or(0, |(i, _)| i);
                    output = format!(
                        "{}{}{}",
                        &output[..start_index],
                        paused_str,
                        &output[second_pipe_index..]
                    );
                }
            }
            if delay_changed && last_delay_render.elapsed() < Duration::from_millis(500) {
                let delay_str = format!("delay {}", settings.game_of_life_simulate_delay);
                if let Some((second_pipe_index, _)) = output.match_indices('│').nth(1) {
                    let start_index = output[..second_pipe_index]
                        .char_indices()
                        .rev()
                        .nth(delay_str.chars().count() - 1)
                        .map_or(0, |(i, _)| i);
                    output = format!(
                        "{}{}{}",
                        &output[..start_index],
                        delay_str,
                        &output[second_pipe_index..]
                    );
                }
            }
        }
        execute!(stdout, cursor::MoveTo(0, logo_0_lines as u16 + 2),).unwrap();
        execute!(stdout, crossterm::terminal::BeginSynchronizedUpdate).unwrap();
        print!("{}", output);
        execute!(stdout, crossterm::terminal::EndSynchronizedUpdate).unwrap();
        stdout.flush().unwrap();
    }
    let mut table: Vec<Vec<i32>> = vec![vec![0; 0]; 0];
    let mut saved_input = table.clone();
    let mut generation: u32 = 0;
    let mut simulating = false;
    let mut paused = false;
    let mut last_simulate = Instant::now();
    let mut last_delay_render = Instant::now();
    let mut delay_changed = false;
    let mut cursor_row = 0;
    let mut cursor_col = 0;
    refresh_table(
        &mut table,
        &mut cursor_row,
        &mut cursor_col,
        help_line_count,
    );
    let mut last_render_time = get_time();
    let (mut last_width, mut last_height) = terminal::size().unwrap();
    let mut needs_rendering = true;
    loop {
        let mut settings = Settings::load();
        if !simulating {
            saved_input = table.clone()
        }
        if let Some((code, _)) = get_key() {
            if !simulating {
                match code {
                    KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                        if cursor_row > 0 {
                            cursor_row -= 1;
                        } else {
                            cursor_row = table.len() - 1;
                        }
                    }
                    KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
                        if cursor_row < table.len() - 1 {
                            cursor_row += 1;
                        } else {
                            cursor_row = 0;
                        }
                    }
                    KeyCode::Left => {
                        if cursor_col > 0 {
                            cursor_col -= 1;
                        } else {
                            cursor_col = table[0].len() - 1;
                        }
                    }
                    KeyCode::Right => {
                        if cursor_col < table[0].len() - 1 {
                            cursor_col += 1;
                        } else {
                            cursor_col = 0;
                        }
                    }
                    KeyCode::Char(' ') => {
                        if cursor_row < table.len() && cursor_col < table[0].len() {
                            table[cursor_row][cursor_col] = if table[cursor_row][cursor_col] == 0 {
                                1
                            } else {
                                0
                            };
                        }
                    }
                    KeyCode::Tab | KeyCode::Char('d') | KeyCode::Char('D') => {
                        needs_rendering = true;
                        game_of_life_settings()
                    }
                    KeyCode::BackTab | KeyCode::Char('a') | KeyCode::Char('A') => return,
                    KeyCode::Char('q') | KeyCode::Char('Q') => return,
                    KeyCode::Esc => process::exit(0),
                    KeyCode::Enter => {
                        needs_rendering = true;
                        simulating = !simulating;
                        last_simulate = Instant::now()
                    }
                    _ => needs_rendering = true,
                }
                render_table(
                    &table,
                    cursor_row,
                    cursor_col,
                    generation,
                    last_delay_render,
                    delay_changed,
                    simulating,
                    paused,
                )
            } else {
                match code {
                    KeyCode::Up
                    | KeyCode::Char('w')
                    | KeyCode::Char('W')
                    | KeyCode::Down
                    | KeyCode::Char('s')
                    | KeyCode::Char('S') => {
                        generation += 1;
                        simulate(&mut table);
                        render_table(
                            &table,
                            cursor_row,
                            cursor_col,
                            generation,
                            last_delay_render,
                            delay_changed,
                            simulating,
                            paused,
                        );
                        last_simulate = Instant::now();
                    }
                    KeyCode::Left => {
                        let game_of_life_simulate_delays = [5, 10, 25, 50, 75, 100, 200, 500, 1000];
                        let game_of_life_simulate_delay_index = game_of_life_simulate_delays
                            .iter()
                            .position(|&c| c == settings.game_of_life_simulate_delay)
                            .unwrap_or(0);
                        if game_of_life_simulate_delay_index > 0 {
                            settings.set_game_of_life_simulate_delay(
                                game_of_life_simulate_delays[game_of_life_simulate_delay_index - 1],
                            )
                        } else {
                            settings.set_game_of_life_simulate_delay(
                                game_of_life_simulate_delays
                                    [game_of_life_simulate_delays.len() - 1],
                            )
                        };
                        last_delay_render = Instant::now();
                        delay_changed = true;
                        render_table(
                            &table,
                            cursor_row,
                            cursor_col,
                            generation,
                            last_delay_render,
                            delay_changed,
                            simulating,
                            paused,
                        )
                    }
                    KeyCode::Right => {
                        let game_of_life_simulate_delays = [5, 10, 25, 50, 75, 100, 200, 500, 1000];
                        let game_of_life_simulate_delay_index = game_of_life_simulate_delays
                            .iter()
                            .position(|&c| c == settings.game_of_life_simulate_delay)
                            .unwrap_or(0);
                        settings.set_game_of_life_simulate_delay(
                            game_of_life_simulate_delays[(game_of_life_simulate_delay_index + 1)
                                % game_of_life_simulate_delays.len()],
                        );
                        last_delay_render = Instant::now();
                        delay_changed = true;
                        render_table(
                            &table,
                            cursor_row,
                            cursor_col,
                            generation,
                            last_delay_render,
                            delay_changed,
                            simulating,
                            paused,
                        )
                    }
                    KeyCode::Char(' ') => {
                        paused = !paused;
                        render_table(
                            &table,
                            cursor_row,
                            cursor_col,
                            generation,
                            last_delay_render,
                            delay_changed,
                            simulating,
                            paused,
                        )
                    }
                    KeyCode::Tab | KeyCode::Char('d') | KeyCode::Char('D') => {
                        needs_rendering = true;
                        game_of_life_settings()
                    }
                    KeyCode::BackTab | KeyCode::Char('a') | KeyCode::Char('A') => return,
                    KeyCode::Char('q') | KeyCode::Char('Q') => return,
                    KeyCode::Esc => process::exit(0),
                    KeyCode::Enter => {
                        needs_rendering = true;
                        simulating = !simulating;
                        if settings.game_of_life_save_input {
                            table = saved_input.clone();
                            generation = 0
                        }
                    }
                    _ => needs_rendering = true,
                }
            }
        }
        if simulating
            && !paused
            && last_simulate.elapsed()
                >= Duration::from_millis(settings.game_of_life_simulate_delay)
        {
            generation += 1;
            simulate(&mut table);
            render_table(
                &table,
                cursor_row,
                cursor_col,
                generation,
                last_delay_render,
                delay_changed,
                simulating,
                paused,
            );
            last_simulate = Instant::now();
        }
        if simulating && delay_changed && last_delay_render.elapsed() >= Duration::from_millis(500)
        {
            render_table(
                &table,
                cursor_row,
                cursor_col,
                generation,
                last_delay_render,
                delay_changed,
                simulating,
                paused,
            );
            delay_changed = false;
        }
        let current_time = get_time();
        let (width, height) = terminal::size().unwrap();
        if width != last_width
            || height != last_height
            || current_time != last_render_time
            || needs_rendering
        {
            if !simulating {
                render_game_of_life(&help_string, &help_more_string)
            } else {
                render_game_of_life(&help_string_simulating, &help_more_string_simulating)
            }
            refresh_table(
                &mut table,
                &mut cursor_row,
                &mut cursor_col,
                help_line_count,
            );
            render_table(
                &table,
                cursor_row,
                cursor_col,
                generation,
                last_delay_render,
                delay_changed,
                simulating,
                paused,
            );
            last_render_time = current_time;
            last_width = width;
            last_height = height;
            needs_rendering = false;
        }
    }
}

fn sys_fetch() {
    let mut stdout = io::stdout();
    let help_string = String::from("| quit: $[esc]$ | change tab: $[a]/[d]$ |");
    let help_more_string = String::from(r#"| return: $[q]$ | change tab: $[backtab]/[tab]$ |"#);
    let user_name = whoami::username();
    fn get_machine_name() -> String {
        {
            let output = Command::new("hostname")
                .output()
                .expect("Failed to get hostname");
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
    }
    let machine_name = get_machine_name();
    let os = os_info::get();
    let os_string = os.to_string();
    let kernel = os.version().to_string();
    let os_name = os_string.replace(&format!(" {}", &kernel), "");
    let (days, hours, mins) = match uptime_lib::get() {
        Ok(duration) => {
            let secs = duration.as_secs();
            let days = secs / 86400;
            let hours = (secs % 86400) / 3600;
            let mins = (secs % 3600) / 60;
            (days, hours, mins)
        }
        Err(_) => (0, 0, 0),
    };
    let current_dir = match env::current_dir() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error getting current directory: {}", e);
            process::exit(1);
        }
    };
    let resolution = match resolution::current_resolution() {
        Ok((width, height)) => format!("{}x{}", width, height),
        Err(_) => "Unknown resolution".to_string(),
    };
    let uptime = format!("{} days {} hours {} mins", days, hours, mins);
    #[derive(Deserialize, Debug)]
    #[serde(rename = "Win32_Processor")]
    #[serde(rename_all = "PascalCase")]
    struct Processor {
        name: String,
    }
    fn get_cpu_name() -> Result<String, Box<dyn Error>> {
        let com_con = COMLibrary::new()?;
        let wmi_con = WMIConnection::new(com_con.into())?;
        let results: Vec<Processor> = wmi_con.query()?;
        Ok(results
            .first()
            .map(|cpu| cpu.name.clone())
            .unwrap_or_else(|| "Unknown CPU".to_string()))
    }
    let cpu_name = get_cpu_name().unwrap_or_else(|_| "Unknown CPU".to_string());
    #[derive(Deserialize, Debug)]
    #[serde(rename = "Win32_VideoController")]
    #[serde(rename_all = "PascalCase")]
    struct VideoController {
        name: String,
    }
    fn get_gpu_name() -> Result<String, Box<dyn Error>> {
        let com_con = COMLibrary::new()?;
        let wmi_con = WMIConnection::new(com_con.into())?;
        let results: Vec<VideoController> = wmi_con.query()?;
        Ok(results
            .first()
            .map(|vc| vc.name.clone())
            .unwrap_or_else(|| "Unknown GPU".to_string()))
    }
    let gpu_name = get_gpu_name().unwrap_or_else(|_| "Unknown GPU".to_string());
    fn get_ram_info() -> (u64, u64, f64) {
        let mut sys = System::new_all();
        sys.refresh_memory();
        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        let total_memory_mb = total_memory as f64 / (1024.0 * 1024.0);
        let used_memory_mb = used_memory as f64 / (1024.0 * 1024.0);
        let usage_percentage = (used_memory as f64 / total_memory as f64) * 100.0;
        (
            used_memory_mb as u64,
            total_memory_mb as u64,
            usage_percentage,
        )
    }
    let (used_memory, total_memory, ram_usage) = get_ram_info();
    let ram_info = format!(
        "{} MB / {} MB ({}%)",
        used_memory,
        total_memory,
        format!("{:.0}", ram_usage)
    );
    #[derive(Deserialize, Debug)]
    #[serde(rename = "Win32_LogicalDisk")]
    #[serde(rename_all = "PascalCase")]
    struct LogicalDisk {
        free_space: Option<u64>,
        size: Option<u64>,
    }
    fn get_disk_info() -> Result<String, Box<dyn Error>> {
        let com_con = COMLibrary::new()?;
        let wmi_con = WMIConnection::new(com_con.into())?;
        let results: Vec<LogicalDisk> = wmi_con.query()?;
        let mut total_used_space = 0u64;
        let mut total_size = 0u64;
        for disk in results {
            if let (Some(free_space), Some(size)) = (disk.free_space, disk.size) {
                if free_space < size {
                    total_used_space += size - free_space;
                    total_size += size;
                }
            }
        }
        if total_size == 0 {
            return Ok("Unknown Disk Info".to_string());
        }
        let total_used_gb = total_used_space / 1024 / 1024 / 1024;
        let total_size_gb = total_size / 1024 / 1024 / 1024;
        let usage_percentage = (total_used_space as f64 / total_size as f64) * 100.0;
        Ok(format!(
            "{} GB / {} GB ({}%)",
            total_used_gb,
            total_size_gb,
            usage_percentage.round()
        ))
    }
    let disk_info = get_disk_info().unwrap_or_else(|_| "Unknown Disk Info".to_string());
    let sys_fetch_logo = vec![
        "         \x1b[91m,.=:!!t3Z3z.\x1b[92m                    ",
        "        \x1b[91m:tt:::tt333EE3\x1b[92m                   ",
        "        \x1b[91mEt:::ztt33EEEL\x1b[92m @Ee.,      ..,    ",
        "       \x1b[91m;tt:::tt333EE7\x1b[92m ;EEEEEEttttt33#    ",
        "      \x1b[91m:Et:::zt333EEQ.\x1b[92m $EEEEEttttt33QL    ",
        "      \x1b[91mit::::tt333EEF\x1b[92m @EEEEEEttttt33F     ",
        "     \x1b[91m;3=*^```\"*4EEV\x1b[92m :EEEEEEttttt33@.     ",
        "     \x1b[36m,.=::::!t=.,\x1b[91m `\x1b[92m @EEEEEEtttz33QF      ",
        "    \x1b[36m;::::::::zt33)\x1b[92m   \"4EEEtttji3P*       ",
        "   \x1b[36m:t::::::::tt33.\x1b[33m:Z3z..\x1b[92m  ``\x1b[33m ,..g.       ",
        "   \x1b[36mi::::::::zt33F\x1b[33m AEEEtttt::::ztF        ",
        "  \x1b[36m;:::::::::t33V\x1b[33m ;EEEttttt::::t3         ",
        "  \x1b[36mE::::::::zt33L\x1b[33m @EEEtttt::::Z3F         ",
        " \x1b[36m{3=*^```\"*4E3)\x1b[33m ;EEEtttt:::::tZ`         ",
        "                \x1b[33m:EEEEtttt::::z7\x1b[33m          ",
        "                  \x1b[33m\"VEzjt:;;z>*`\x1b[33m          ",
    ];
    let (width, _) = terminal::size().unwrap();
    let last_render_time = get_time();
    let (last_width, last_height) = terminal::size().unwrap();
    let mut needs_rendering = false;
    let mut output = String::new();
    output.push_str(&render_top("fetch", None, false));
    for (index, line) in sys_fetch_logo.iter().enumerate() {
        output.push_str("│");
        let mut line_content = String::from(*line);
        SetForegroundColor(get_color("theme"));
        if index == 0 {
            line_content.push_str(&format!(
                "{}{}@{}",
                SetForegroundColor(get_color("main")),
                user_name,
                machine_name
            ));
        } else if index == 1 {
            let dashes = "-".repeat((user_name.len() + machine_name.len() + 1) as usize);
            line_content.push_str(&format!(
                "{}{}",
                SetForegroundColor(get_color("theme")),
                dashes
            ));
        } else if index == 2 {
            line_content.push_str(&format!(
                "{}OS{}: {}",
                SetForegroundColor(get_color("main")),
                SetForegroundColor(get_color("theme")),
                os_name
            ));
        } else if index == 3 {
            line_content.push_str(&format!(
                "{}Host{}: {}",
                SetForegroundColor(get_color("main")),
                SetForegroundColor(get_color("theme")),
                machine_name
            ));
        } else if index == 4 {
            line_content.push_str(&format!(
                "{}Kernel{}: {}",
                SetForegroundColor(get_color("main")),
                SetForegroundColor(get_color("theme")),
                kernel
            ));
        } else if index == 5 {
            line_content.push_str(&format!(
                "{}Uptime{}: {}",
                SetForegroundColor(get_color("main")),
                SetForegroundColor(get_color("theme")),
                uptime
            ));
        } else if index == 6 {
            line_content.push_str(&format!(
                "{}Dir{}: {}",
                SetForegroundColor(get_color("main")),
                SetForegroundColor(get_color("theme")),
                current_dir.display()
            ));
        } else if index == 7 {
            line_content.push_str(&format!(
                "{}Resolution{}: {}",
                SetForegroundColor(get_color("main")),
                SetForegroundColor(get_color("theme")),
                resolution
            ));
        } else if index == 8 {
            line_content.push_str(&format!(
                "{}CPU{}: {}",
                SetForegroundColor(get_color("main")),
                SetForegroundColor(get_color("theme")),
                cpu_name
            ));
        } else if index == 9 {
            line_content.push_str(&format!(
                "{}GPU{}: {}",
                SetForegroundColor(get_color("main")),
                SetForegroundColor(get_color("theme")),
                gpu_name
            ));
        } else if index == 10 {
            line_content.push_str(&format!(
                "{}RAM{}: {}",
                SetForegroundColor(get_color("main")),
                SetForegroundColor(get_color("theme")),
                ram_info
            ));
        } else if index == 11 {
            line_content.push_str(&format!(
                "{}Storage{}: {}",
                SetForegroundColor(get_color("main")),
                SetForegroundColor(get_color("theme")),
                disk_info
            ));
        } else if index == 13 {
            line_content.push_str(&format!(
                "{}{}   {}   {}   {}   {}   {}   {}   {}   {}",
                SetForegroundColor(get_color("theme")),
                SetBackgroundColor(Color::Reset),
                SetBackgroundColor(Color::DarkRed),
                SetBackgroundColor(Color::DarkYellow),
                SetBackgroundColor(Color::DarkGreen),
                SetBackgroundColor(Color::DarkCyan),
                SetBackgroundColor(Color::DarkBlue),
                SetBackgroundColor(Color::Magenta),
                SetBackgroundColor(Color::Grey),
                SetBackgroundColor(Color::Reset)
            ));
        }
        output.push_str(&format!(
            "{}{}{}│\n",
            line_content,
            SetForegroundColor(get_color("theme")),
            cursor::MoveToColumn(width)
        ));
    }
    output.push_str(&render_bottom(
        sys_fetch_logo.len() as u16,
        help_string,
        help_more_string,
    ));
    execute!(stdout, crossterm::terminal::BeginSynchronizedUpdate).unwrap();
    clear();
    print!("{}", output);
    execute!(stdout, crossterm::terminal::EndSynchronizedUpdate).unwrap();
    stdout.flush().unwrap();
    loop {
        if let Some((code, _)) = get_key() {
            needs_rendering = true;
            match code {
                KeyCode::Tab | KeyCode::Char('d') | KeyCode::Char('D') => {
                    main();
                    return;
                }
                KeyCode::BackTab | KeyCode::Char('a') | KeyCode::Char('A') => settings_menu(),
                KeyCode::Char('q') | KeyCode::Char('Q') => return,
                KeyCode::Esc => process::exit(0),
                _ => {}
            }
        }
        let current_time = get_time();
        let (width, height) = terminal::size().unwrap();
        if width != last_width
            || height != last_height
            || current_time != last_render_time
            || needs_rendering
        {
            sys_fetch();
            return;
        }
    }
}

fn run_settings_menu_selected(settings_menu_selected: usize, direction: &str) {
    let mut settings = Settings::load();
    let colors = ["grey", "red", "yellow", "green", "cyan", "blue", "magenta"];
    let color_index = colors
        .iter()
        .position(|&c| c == settings.color)
        .unwrap_or(0);
    let ping_delays = [10, 50, 100, 200, 350, 500, 1000];
    let ping_delay_index = ping_delays
        .iter()
        .position(|&c| c == settings.ping_delay)
        .unwrap_or(0);
    let port_scan_timeouts = [10, 25, 50, 75, 100, 150, 200, 350, 500, 750, 1000];
    let port_scan_timeout_index = port_scan_timeouts
        .iter()
        .position(|&c| c == settings.port_scan_timeout)
        .unwrap_or(0);
    let micro_macro_hotkeys = [
        "None", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "X1Mouse", "X2Mouse",
    ];
    let micro_macro_hotkey_index = micro_macro_hotkeys
        .iter()
        .position(|&c| c == settings.micro_macro_hotkey)
        .unwrap_or(0);
    let macro_hotkeys = [
        "None", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "X1Mouse", "X2Mouse",
    ];
    let macro_hotkey_index = macro_hotkeys
        .iter()
        .position(|&c| c == settings.macro_hotkey)
        .unwrap_or(0);
    match direction {
        "left" => match settings_menu_selected {
            0 => {
                if color_index > 0 {
                    settings.set_color(colors[color_index - 1])
                } else {
                    settings.set_color(colors[colors.len() - 1])
                }
            }
            1 => settings.set_dark_theme(!settings.dark_theme),
            2 => {
                if ping_delay_index > 0 {
                    settings.set_ping_delay(ping_delays[ping_delay_index - 1])
                } else {
                    settings.set_ping_delay(ping_delays[ping_delays.len() - 1])
                }
            }
            3 => {
                if port_scan_timeout_index > 0 {
                    settings.set_port_scan_timeout(port_scan_timeouts[port_scan_timeout_index - 1])
                } else {
                    settings.set_port_scan_timeout(port_scan_timeouts[port_scan_timeouts.len() - 1])
                }
            }
            4 => {
                if micro_macro_hotkey_index > 0 {
                    settings
                        .set_micro_macro_hotkey(micro_macro_hotkeys[micro_macro_hotkey_index - 1])
                } else {
                    settings
                        .set_micro_macro_hotkey(micro_macro_hotkeys[micro_macro_hotkeys.len() - 1])
                }
            }
            5 => {
                if macro_hotkey_index > 0 {
                    settings.set_macro_hotkey(macro_hotkeys[macro_hotkey_index - 1])
                } else {
                    settings.set_macro_hotkey(macro_hotkeys[macro_hotkeys.len() - 1])
                }
            }
            6 => settings.set_hide_help(!settings.hide_help),
            7 => {
                {
                    let dir = "NUUI_config";
                    if settings.show_config_files {
                        Command::new("attrib")
                            .args(&["+H", dir])
                            .status()
                            .expect("Failed to hide NUUI_config");
                    } else {
                        Command::new("attrib")
                            .args(&["-H", dir])
                            .status()
                            .expect("Failed to unhide NUUI_config");
                    }
                }
                settings.set_show_config_files(!settings.show_config_files);
            }
            8 => settings.set_show_clock(!settings.show_clock),
            9 => settings.set_show_size(!settings.show_size),
            _ => {}
        },
        "right" => match settings_menu_selected {
            0 => settings.set_color(colors[(color_index + 1) % colors.len()]),
            1 => settings.set_dark_theme(!settings.dark_theme),
            2 => settings.set_ping_delay(ping_delays[(ping_delay_index + 1) % ping_delays.len()]),
            3 => settings.set_port_scan_timeout(
                port_scan_timeouts[(port_scan_timeout_index + 1) % port_scan_timeouts.len()],
            ),
            4 => settings.set_micro_macro_hotkey(
                micro_macro_hotkeys[(micro_macro_hotkey_index + 1) % micro_macro_hotkeys.len()],
            ),
            5 => settings
                .set_macro_hotkey(macro_hotkeys[(macro_hotkey_index + 1) % macro_hotkeys.len()]),
            6 => settings.set_hide_help(!settings.hide_help),
            7 => {
                {
                    let dir = "NUUI_config";
                    if settings.show_config_files {
                        Command::new("attrib")
                            .args(&["+H", dir])
                            .status()
                            .expect("Failed to hide NUUI_config");
                    } else {
                        Command::new("attrib")
                            .args(&["-H", dir])
                            .status()
                            .expect("Failed to unhide NUUI_config");
                    }
                }
                settings.set_show_config_files(!settings.show_config_files);
            }
            8 => settings.set_show_clock(!settings.show_clock),
            9 => settings.set_show_size(!settings.show_size),
            _ => {}
        },
        _ => {}
    }
}

fn render_settings_menu(menu_selected: usize, menu_options: &[&str]) {
    let settings = Settings::load();
    let help_string = String::from(
        "| quit: $[esc]$ | change tab: $[a]/[d]$ | scroll: $[w]/[s]$ | change setting: $[←]/[→]$ |",
    );
    let help_more_string = String::from(
        r#"| change setting: $[ent]$ | select: $[0-9]$ |
| return: $[q]$ | change tab: $[backtab]/[tab]$ | scroll: $[↑]/[↓]$ |"#,
    );
    let mut stdout = io::stdout();
    let (width, _) = terminal::size().unwrap();
    let mut output = String::new();
    output.push_str(&render_top("settings", None, false));
    for i in 0..menu_options.len() {
        if i == menu_selected {
            output.push_str(&format!(
                "│{}{} {} › {} {}{}{}{}│\n",
                SetBackgroundColor(get_color("main")),
                SetForegroundColor(Color::Black),
                i,
                menu_options[i],
                if menu_options[i] == "dark_theme" {
                    if settings.dark_theme {
                        "1 ".to_string()
                    } else {
                        "0 ".to_string()
                    }
                } else if menu_options[i] == "ping_delay" {
                    settings.ping_delay.to_string() + "ms "
                } else if menu_options[i] == "port_scan_timeout" {
                    settings.port_scan_timeout.to_string() + "ms "
                } else if menu_options[i] == "micro_macro_hotkey" {
                    settings.micro_macro_hotkey.to_string() + " "
                } else if menu_options[i] == "macro_hotkey" {
                    settings.macro_hotkey.to_string() + " "
                } else if menu_options[i] == "hide_help" {
                    if settings.hide_help {
                        "1 ".to_string()
                    } else {
                        "0 ".to_string()
                    }
                } else if menu_options[i] == "show_config_files" {
                    if settings.show_config_files {
                        "1 ".to_string()
                    } else {
                        "0 ".to_string()
                    }
                } else if menu_options[i] == "show_clock" {
                    if settings.show_clock {
                        "1 ".to_string()
                    } else {
                        "0 ".to_string()
                    }
                } else if menu_options[i] == "show_size" {
                    if settings.show_size {
                        "1 ".to_string()
                    } else {
                        "0 ".to_string()
                    }
                } else {
                    " ".to_string()
                },
                SetForegroundColor(get_color("theme")),
                SetBackgroundColor(Color::Reset),
                cursor::MoveToColumn(width)
            ));
        } else {
            output.push_str(&format!(
                "│{} {} {}| {}{}{}│\n",
                SetForegroundColor(get_color("main")),
                i,
                SetForegroundColor(Color::DarkGrey),
                SetForegroundColor(get_color("theme")),
                menu_options[i],
                cursor::MoveToColumn(width)
            ));
        }
    }
    output.push_str(&&render_bottom(
        menu_options.len() as u16,
        help_string,
        help_more_string,
    ));
    execute!(stdout, crossterm::terminal::BeginSynchronizedUpdate).unwrap();
    clear();
    print!("{}", output);
    execute!(stdout, crossterm::terminal::EndSynchronizedUpdate).unwrap();
    stdout.flush().unwrap();
}

fn settings_menu() {
    let settings_menu_options = [
        "color",
        "dark_theme",
        "ping_delay",
        "port_scan_timeout",
        "micro_macro_hotkey",
        "macro_hotkey",
        "hide_help",
        "show_config_files",
        "show_clock",
        "show_size",
    ];
    let mut settings_menu_selected = 0;
    let mut last_render_time = get_time();
    let (mut last_width, mut last_height) = terminal::size().unwrap();
    let mut needs_rendering = true;
    loop {
        if let Some((code, _)) = get_key() {
            needs_rendering = true;
            match code {
                KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                    if settings_menu_selected > 0 {
                        settings_menu_selected -= 1
                    } else {
                        settings_menu_selected = settings_menu_options.len() - 1
                    }
                }
                KeyCode::Left => run_settings_menu_selected(settings_menu_selected, "left"),
                KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
                    if settings_menu_selected < settings_menu_options.len() - 1 {
                        settings_menu_selected += 1
                    } else {
                        settings_menu_selected = 0
                    }
                }
                KeyCode::Right | KeyCode::Enter => {
                    run_settings_menu_selected(settings_menu_selected, "right")
                }
                KeyCode::Tab | KeyCode::Char('d') | KeyCode::Char('D') => sys_fetch(),
                KeyCode::BackTab | KeyCode::Char('a') | KeyCode::Char('A') => {
                    main();
                    return;
                }
                KeyCode::Char('q') | KeyCode::Char('Q') => return,
                KeyCode::Esc => process::exit(0),
                KeyCode::Char(c) if c.is_digit(10) => {
                    let num = c.to_digit(10).unwrap() as usize;
                    if num < settings_menu_options.len() {
                        settings_menu_selected = num;
                    };
                }
                _ => {}
            }
        }
        let current_time = get_time();
        let (width, height) = terminal::size().unwrap();
        if width != last_width
            || height != last_height
            || current_time != last_render_time
            || needs_rendering
        {
            render_settings_menu(settings_menu_selected, &settings_menu_options);
            last_render_time = current_time;
            last_width = width;
            last_height = height;
            needs_rendering = false;
        }
    }
}

fn run_menu_selected(menu_selected: usize, menu_options: &[String]) {
    match menu_options[menu_selected].as_str() {
        "ping_tool" => ping_tool(),
        "port_scan" => port_scan(),
        "micro_macro" => micro_macro(),
        "macro" => macro_tool(),
        "tetris" => tetris(),
        "game_of_life" => game_of_life(),
        path => {
            let _ = run_file(path);
        }
    }
}

fn run_file(path: &str) -> std::io::Result<()> {
    Command::new("cmd")
        .args(&["/C", "start", path])
        .spawn()?
        .wait()?;
    Ok(())
}

fn render_menu(menu_selected: usize, shift_selected: bool) {
    let settings = Settings::load();
    let help_string = String::from(
        "| quit: $[esc]$ | change tab: $[a]/[d]$ | scroll: $[w]/[s]$ | select: $[ent]$ |",
    );
    let help_more_string = String::from(
        r#"| rearrange: $[shift]+[↑/w]/[↓/s]$ | add_option: $[c]$ | remove_option: $[del]/[backspace]$ |
| return: $[q]$ | change tab: $[backtab]/[tab]$ | scroll: $[↑]/[←]/[↓]/[→]$ | select: $[0-9]$ |"#,
    );
    let mut stdout = io::stdout();
    let (width, _) = terminal::size().unwrap();
    let mut output = String::new();
    output.push_str(&render_top("menu", None, false));
    for i in 0..settings.options.len() {
        let mut spaces = " ";
        if i >= 10 {
            spaces = "";
        }
        if i == menu_selected && shift_selected {
            spaces.to_string().push_str(" ");
        }
        let shift_indent = if i == menu_selected && shift_selected {
            " "
        } else {
            ""
        };
        let path = Path::new(&settings.options[i]);
        let filename = path.file_name().unwrap_or_default().to_string_lossy();
        let prefix = if path.is_dir() { "•" } else { "|" };
        if i == menu_selected {
            output.push_str(&format!(
                "│{}{}{}{}{} › {}  {}{}{}│\n",
                SetForegroundColor(Color::Black),
                SetBackgroundColor(get_color("main")),
                spaces,
                shift_indent,
                i,
                filename,
                SetForegroundColor(get_color("theme")),
                SetBackgroundColor(Color::Reset),
                cursor::MoveToColumn(width)
            ));
        } else {
            output.push_str(&format!(
                "│{}{}{} {}{} {}{}{}│\n",
                SetForegroundColor(get_color("main")),
                spaces,
                i,
                SetForegroundColor(Color::DarkGrey),
                prefix,
                SetForegroundColor(get_color("theme")),
                filename,
                cursor::MoveToColumn(width)
            ));
        }
    }
    output.push_str(&render_bottom(
        settings.options.len() as u16,
        help_string,
        help_more_string,
    ));
    execute!(stdout, crossterm::terminal::BeginSynchronizedUpdate).unwrap();
    clear();
    print!("{}", output);
    execute!(stdout, crossterm::terminal::EndSynchronizedUpdate).unwrap();
    stdout.flush().unwrap();
}

fn main() {
    terminal::enable_raw_mode().unwrap();
    let mut settings = Settings::load();
    let mut menu_selected = 0;
    let mut shift_held = false;
    let mut last_render_time = get_time();
    let (mut last_width, mut last_height) = terminal::size().unwrap();
    let mut needs_rendering = true;
    loop {
        if let Some((code, modifiers)) = get_key() {
            shift_held = modifiers.contains(KeyModifiers::SHIFT);
            needs_rendering = true;
            match (code, modifiers) {
                (KeyCode::Up, KeyModifiers::SHIFT) => {
                    if menu_selected > 0 {
                        settings.options.swap(menu_selected, menu_selected - 1);
                        menu_selected -= 1;
                        settings.save();
                    }
                }
                (KeyCode::Down, KeyModifiers::SHIFT) => {
                    if menu_selected < settings.options.len() - 1 {
                        settings.options.swap(menu_selected, menu_selected + 1);
                        menu_selected += 1;
                        settings.save();
                    }
                }
                (KeyCode::Up, _) | (KeyCode::Char('w'), _) | (KeyCode::Char('W'), _) => {
                    if menu_selected > 0 {
                        menu_selected -= 1;
                    } else {
                        menu_selected = settings.options.len() - 1;
                    }
                }
                (KeyCode::Left, _) => {
                    if menu_selected > 0 {
                        menu_selected -= 1;
                    } else {
                        menu_selected = settings.options.len() - 1;
                    }
                }
                (KeyCode::Down, _) | (KeyCode::Char('s'), _) | (KeyCode::Char('S'), _) => {
                    if menu_selected < settings.options.len() - 1 {
                        menu_selected += 1;
                    } else {
                        menu_selected = 0;
                    }
                }
                (KeyCode::Right, _) => {
                    if menu_selected < settings.options.len() - 1 {
                        menu_selected += 1;
                    } else {
                        menu_selected = 0;
                    }
                }
                (KeyCode::Char('c'), _) | (KeyCode::Char('C'), _) => {
                    execute!(io::stdout(), cursor::MoveUp(1), cursor::MoveToColumn(2)).unwrap();
                    if settings.options.len() >= 10 {
                        execute!(io::stdout(), cursor::MoveLeft(1)).unwrap();
                    }
                    print!(
                        "{}{} {}|{} ",
                        SetForegroundColor(get_color("main")),
                        settings.options.len(),
                        SetForegroundColor(Color::DarkGrey),
                        SetForegroundColor(get_color("theme"))
                    );
                    let mut custom_option_path = String::new();
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut custom_option_path).unwrap();
                    let custom_option_path = custom_option_path.trim();
                    if !custom_option_path.is_empty() {
                        settings.add_option(&custom_option_path.to_string());
                    }
                }
                (KeyCode::Delete, _) | (KeyCode::Backspace, _) => {
                    settings.remove_option(menu_selected);
                    if menu_selected > settings.options.len() - 1 {
                        menu_selected -= 1;
                    }
                }
                (KeyCode::Tab, _) | (KeyCode::Char('d'), _) | (KeyCode::Char('D'), _) => {
                    settings_menu()
                }
                (KeyCode::BackTab, _) | (KeyCode::Char('a'), _) | (KeyCode::Char('A'), _) => {
                    sys_fetch()
                }
                (KeyCode::Esc, _) => process::exit(0),
                (KeyCode::Enter, _) => run_menu_selected(menu_selected, &settings.options),
                (KeyCode::Char(c), _) if c.is_digit(10) => {
                    let num = c.to_digit(10).unwrap() as usize;
                    if num < settings.options.len() {
                        menu_selected = num;
                    }
                    run_menu_selected(menu_selected, &settings.options);
                }
                _ => {}
            }
        }
        let current_time = get_time();
        let (width, height) = terminal::size().unwrap();
        if width != last_width
            || height != last_height
            || current_time != last_render_time
            || needs_rendering
        {
            render_menu(menu_selected, shift_held);
            last_render_time = current_time;
            last_width = width;
            last_height = height;
            needs_rendering = false;
        }
    }
}
