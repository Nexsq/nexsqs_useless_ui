use std::process::Command;
use std::process;
use std::fs::{self, File};
use std::io::{self, Write, Read};
use std::path::Path;
use std::env;
use std::time::{Duration, Instant};
use std::sync::{Arc, LazyLock, Mutex};
use std::thread;
use std::net::{TcpStream, ToSocketAddrs};
use std::error::Error;
use crossterm::cursor;
use crossterm::event::{self, KeyCode, Event, KeyEvent, KeyEventKind};
use crossterm::{style::{Color, SetForegroundColor, SetBackgroundColor}, terminal};
use crossterm::execute;
use chrono::Local;
use serde_derive::{Serialize, Deserialize};
use toml;
use sysinfo::System;
use wmi::{COMLibrary, WMIConnection};

fn version() -> String {
    let version = "v3.0";
    version.to_string()
}

fn logo() -> (String, String, String) {
    let logo_0 = String::from(r#"
  █▀█ █▀▀ █ █ █▀▀ █▀█ █ █▀▀
  █ █ █▀▀ ▄▀▄ ▀▀█ ▀▀█   ▀▀█
  ▀ ▀ ▀▀▀ ▀ ▀ ▀▀▀   ▀   ▀▀▀
    "#);
    let logo_1 = String::from(r#"
 █ █ █▀▀ █▀▀ █   █▀▀ █▀▀ █▀▀
 █ █ ▀▀█ █▀▀ █   █▀▀ ▀▀█ ▀▀█
 ▀▀▀ ▀▀▀ ▀▀▀ ▀▀▀ ▀▀▀ ▀▀▀ ▀▀▀
    "#);
    let logo_2 = format!(r#"
 █ █ ▀█▀  
 █ █  █   
 ▀▀▀ ▀▀▀  {}
    "#, version());
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
    ping_delay: u32,
    port_scan_timeout: u32,
    micro_macro_key: String,
    micro_macro_delay: u32,
    hide_help: bool,
    show_config_files: bool,
    custom_options: Vec<String>
}
impl Settings {
    fn new() -> Self {
        Settings {
            color: "grey".to_string(),
            dark_theme: false,
            ping_delay: 500,
            port_scan_timeout: 500,
            micro_macro_key: "F15".to_string(),
            micro_macro_delay: 30000,
            hide_help: false,
            show_config_files: false,
            custom_options: Vec::new()
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
        file.read_to_string(&mut contents).expect("Failed to read settings.toml");
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
    fn set_ping_delay(&mut self, new_delay: u32) {
        self.ping_delay = new_delay.clamp(0, 4294967295);
        self.save();
    }
    fn set_port_scan_timeout(&mut self, new_delay: u32) {
        self.port_scan_timeout = new_delay.clamp(0, 4294967295);
        self.save();
    }
    fn set_micro_macro_key(&mut self, new_key: &str) {
        self.micro_macro_key = new_key.to_string();
        self.save();
    }
    fn set_micro_macro_delay(&mut self, new_delay: u32) {
        self.micro_macro_delay = new_delay.clamp(0, 4294967295);
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
    fn add_custom_option(&mut self, path: &str) {
        self.custom_options.push(path.to_string());
        self.save();
    }
    fn clear_custom_options(&mut self) {
        self.custom_options.pop();
        self.save();
    }
}

fn get_key() -> Option<KeyCode> {
    if event::poll(Duration::ZERO).unwrap() {
        if let Event::Key(KeyEvent { code, kind,.. }) = event::read().unwrap() {
            if kind == KeyEventKind::Press {
                if code == KeyCode::Char('h') || code == KeyCode::Char('H') {
                    let mut help_open = HELP_OPEN.lock().unwrap();
                    *help_open = !*help_open;
                }
                return Some(code);
            }
        }
    }
    None
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
        },
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
                    _ => Color::DarkGrey
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
                    _ => Color::Grey
                }
            }
        },
        _ => { Color::Reset }
    }
}

fn clear() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", "cls"]).status().expect("Failed to clear the terminal");
    } else {
        Command::new("clear").status().expect("Failed to clear the terminal");
    }
}

fn render_top(current_option: &str, new_option: Option<&str>, new_option_selected: bool) -> String {
    let (width, _) = terminal::size().unwrap();
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
    let main_options_length: u16 = main_options.iter().map(|option| option.len() as u16 + 5).sum();
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
        current_time
    ));
    output.push_str(&format!(
        "{}{}",
        cursor::MoveDown(logo_0_lines.len() as u16 - 2),
        cursor::MoveToColumn(0)
    ));
    for (index, option) in main_options.iter().enumerate() {
        if index == 1 + new_options_count {
            if width > main_options_length + 1 { output.push_str(&format!("{}", cursor::MoveRight(width - main_options_length - 1))) }
        }
        let dashes = "─".repeat(option.len());
        output.push_str(" ╭─");
        output.push_str(&dashes);
        output.push_str("─╮");
    }
    output.push_str(" ");
    for (index, option) in main_options.iter().enumerate() {
        if index == 1 + new_options_count {
            if width > main_options_length + 1 { output.push_str(&format!("{}", cursor::MoveRight(width - main_options_length - 1))) }
        }
        output.push_str(" │ ");
        output.push_str(option);
        output.push_str(" │");
    }
    output.push_str(&format!("{}{}", cursor::MoveToColumn(0), cursor::MoveDown(1)));
    output.push_str("╭");
    for (index, option) in main_options.iter().enumerate() {
        if index == 1 + new_options_count {
            if width > main_options_length + 1 { let dashes = "─".repeat(width as usize - main_options_length as usize - 1); output.push_str(&dashes) }
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
    if *help_open { help_string += " less: $[h]$ |" } else { help_string += " more: $[h]$ |" };
    let mut help_more_height = 0;
    let help_more_string_lines: Vec<&str> = help_more_string.lines().collect();
    if !settings.hide_help && *help_open { help_more_height = help_more_string.lines().count() as u16 }
    let mut output = String::new();
    if !settings.hide_help { help_more_height += 1 };
    if height > logo_0_lines.len() as u16 + mid_length + 3 && height > 8 + help_more_height + mid_length {
        for _ in 0..height - 8 - help_more_height - mid_length {
            output.push_str("│");
            output.push_str(&format!("{}", cursor::MoveToColumn(width)));
            output.push_str("│\n");
        }
    }
    output.push_str("╰");
    output.push_str(&dashes);
    output.push_str("╯");
    if !settings.hide_help {
        if *help_open {
            for i in 0..help_more_string.lines().count() {
                output.push_str(&format!("{}", cursor::MoveDown(1)));
                output.push_str(&format!("{}", cursor::MoveToColumn(width / 2 - help_more_string_lines[i].chars().filter(|&c| c != '$').count() as u16 / 2)));
                let help_more_string_lines_parts: Vec<&str> = help_more_string_lines[i].split('$').collect();
                for (i, part) in help_more_string_lines_parts.iter().enumerate() {
                    if i % 2 == 1 {
                        output.push_str(&format!("{}", SetForegroundColor(Color::Black)));
                        output.push_str(&format!("{}", SetBackgroundColor(get_color("main"))));
                        output.push_str(part);
                        output.push_str(&format!("{}", SetForegroundColor(get_color("theme"))));
                        output.push_str(&format!("{}", SetBackgroundColor(Color::Black)));
                    } else {
                        output.push_str(part);
                    }
                }
            }
        }
        output.push_str(&format!("{}", cursor::MoveToRow(height)));
        if width / 2 > help_string.chars().filter(|&c| c != '$').count() as u16 / 2 { output.push_str(&format!("{}", cursor::MoveToColumn(width / 2 - help_string.chars().filter(|&c| c != '$').count() as u16 / 2))) }
        let help_string_parts: Vec<&str> = help_string.split('$').collect();
        for (i, part) in help_string_parts.iter().enumerate() {
            if i % 2 == 1 {
                output.push_str(&format!("{}", SetForegroundColor(Color::Black)));
                output.push_str(&format!("{}", SetBackgroundColor(get_color("main"))));
                output.push_str(part);
                output.push_str(&format!("{}", SetForegroundColor(get_color("theme"))));
                output.push_str(&format!("{}", SetBackgroundColor(Color::Black)));
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
    static PINGS: LazyLock<Arc<Mutex<Vec<String>>>> = LazyLock::new(|| { Arc::new(Mutex::new(Vec::new())) });
    static PING_SEQ: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(1));
    static PING_IP: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::new()));
    fn add_ping(ping: String, help_more_string_lines: u16) {
        let settings = Settings::load();
        let (_, height) = terminal::size().unwrap();
        let mut help_length = 0;
        if !settings.hide_help { help_length += 1 };
        let help_open = HELP_OPEN.lock().unwrap();
        if *help_open { help_length += help_more_string_lines }
        let max_pings = height.saturating_sub(12 + help_length).max(1) as usize;
        let mut pings = PINGS.lock().unwrap();
        while pings.len() > max_pings {
            if !pings.is_empty() {
                pings.remove(0);
            }
        }
        pings.push(ping);
    }
    fn print_pings() -> usize {
        let (width, _) = terminal::size().unwrap();
        let mut stdout = io::stdout();
        let start_y = 9;
        let pings = PINGS.lock().unwrap();
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
    fn clear_pings() { let mut pings = PINGS.lock().unwrap(); pings.clear() }
    fn clear_seqs() { let mut seq = PING_SEQ.lock().unwrap(); *seq = 1 }
    fn set_ip() {
        let mut ip = PING_IP.lock().unwrap();
        let mut new_ip = String::new();
        io::stdin().read_line(&mut new_ip).unwrap();
        let new_ip = new_ip.trim();
        *ip = new_ip.to_string()
    }
    fn get_ip() -> String { let ip = PING_IP.lock().unwrap(); ip.clone() }
    fn clear_ip() { let mut ip = PING_IP.lock().unwrap(); *ip = String::new() }
    let mut stdout = io::stdout();
    let help_string = String::from("| quit: $[esc]$ | change tab: $[a]/[d]$ | change ip: $[ent]$ |");
    let help_more_string = String::from(
r#"| quit: $[q]$ | change tab: $[backtab]/[tab]$ | change ip: $[space]$ |"#
    );
    let help_line_count = help_more_string.lines().count() as u16;
    let last_render_time = get_time();
    let (last_width, last_height) = terminal::size().unwrap();
    let mut needs_rendering = false;
    let mut output = String::new();
    output.push_str(&render_top("ping_tool", None, false));
    output.push_str(&render_bottom(0, help_string, help_more_string));
    clear();
    print!("{}", output);
    stdout.flush().unwrap();
    execute!(stdout, cursor::MoveUp(1)).unwrap();
    let mut ip = get_ip();
    if ip.is_empty() {
        print!("Pinging: ");
        stdout.flush().unwrap();
        set_ip();
        ip = get_ip()
    } else {
        print!("Pinging: {}", ip)
    }
    if ip.is_empty() {
        return
    }
    let ip = ip.trim();
    let mut last_ping = Instant::now();
    fn ping(ip: &str) -> Option<(f64, u32)> {
        let output = {
            Command::new("ping").arg("-n").arg("1").arg(ip).output().expect("Failed to execute ping")
        };
        if output.status.success() {
            let stdout = std::str::from_utf8(&output.stdout).unwrap();
            let time = stdout.lines().find(|line| line.contains("Average")).and_then(|line| line.split("=").last()).and_then(|avg| avg.trim().strip_suffix("ms")).and_then(|ms| ms.parse::<f64>().ok());
            let ttl = stdout.lines().find(|line| line.contains("TTL")).and_then(|line| line.split("TTL=").nth(1)).and_then(|ttl| ttl.split_whitespace().next()).and_then(|ttl| ttl.parse::<u32>().ok()).unwrap_or(64);
            if let Some(ms) = time {
                return Some((ms, ttl));
            }
        }
        None
    }
    print_pings();
    loop {
        if let Some(pressed_key) = get_key() {
            needs_rendering = true;
            match pressed_key {
                KeyCode::Tab | KeyCode::Char('d') | KeyCode::Char('D') => { clear_pings(); clear_seqs(); clear_ip(); settings_menu() },
                KeyCode::BackTab | KeyCode::Char('a') | KeyCode::Char('A') => { clear_pings(); clear_seqs(); clear_ip(); return },
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => process::exit(0),
                KeyCode::Enter => { clear_pings(); clear_seqs(); clear_ip(); ping_tool(); return }
                KeyCode::Char(c) if c == ' ' => { clear_pings(); clear_seqs(); clear_ip(); ping_tool(); return }
                _ => {}
            }
        }
        if last_ping.elapsed() >= Duration::from_millis(settings.ping_delay as u64) {
            match ping(ip) {
                Some((ms, ttl)) => {
                    let ping_status = format!("Ping: {:.0} ms (seq={}, ttl={})", ms, PING_SEQ.lock().unwrap(), ttl);
                    add_ping(ping_status, help_line_count);
                },
                None => {
                    let ping_status = format!("Ping to {} failed", ip);
                    add_ping(ping_status, help_line_count);
                }
            }
            let mut seq = PING_SEQ.lock().unwrap();
            *seq += 1;
            let num_pings = print_pings();
            execute!(stdout, cursor::MoveUp(num_pings as u16)).unwrap();
            last_ping = Instant::now();
        }
        let current_time = get_time();
        let (width, height) = terminal::size().unwrap();
        if width != last_width || height != last_height || current_time != last_render_time || needs_rendering {
            ping_tool();
            return;
        }
    }
}

fn port_scan() {
    let settings = Settings::load();
    static PORT_SCANS: LazyLock<Arc<Mutex<Vec<String>>>> = LazyLock::new(|| { Arc::new(Mutex::new(Vec::new())) });
    static OPEN_PORTS: LazyLock<Arc<Mutex<Vec<String>>>> = LazyLock::new(|| { Arc::new(Mutex::new(Vec::new())) });
    static PORT_NUM: LazyLock<Mutex<u16>> = LazyLock::new(|| Mutex::new(0));
    static PORT_SCAN_IP: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::new()));
    static PORT_RANGE: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::new()));
    fn add_port_scan(port_scan: String, help_more_string_lines: u16) {
        let settings = Settings::load();
        let (_, height) = terminal::size().unwrap();
        let mut help_length = 0;
        if !settings.hide_help { help_length += 1 };
        let help_open = HELP_OPEN.lock().unwrap();
        if *help_open { help_length += help_more_string_lines }
        let max_port_scans = height.saturating_sub(14 + help_length).max(1) as usize;
        let mut port_scans = PORT_SCANS.lock().unwrap();
        while port_scans.len() > max_port_scans {
            if !port_scans.is_empty() {
                port_scans.remove(0);
            }
        }
        port_scans.push(port_scan);
    }
    fn print_port_scans() -> usize {
        let (width, _) = terminal::size().unwrap();
        let mut stdout = io::stdout();
        let start_y = 10;
        let port_scans = PORT_SCANS.lock().unwrap();
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
    fn clear_port_scans() { let mut port_scans = PORT_SCANS.lock().unwrap(); port_scans.clear() }
    fn add_open_port(port_scan: String) {
        let mut open_ports = OPEN_PORTS.lock().unwrap();
        if !open_ports.contains(&port_scan) { open_ports.push(port_scan) }
    }
    fn print_open_ports(help_more_string_lines: u16) -> usize {
        let settings = Settings::load();
        let (_, height) = terminal::size().unwrap();
        let mut help_length = 0;
        if !settings.hide_help { help_length += 1 };
        let help_open = HELP_OPEN.lock().unwrap();
        if *help_open { help_length += help_more_string_lines }
        let mut stdout = io::stdout();
        let open_ports = OPEN_PORTS.lock().unwrap();
        execute!(stdout, cursor::MoveTo(2, height - 2 - help_length)).unwrap();
        print!("Open Ports: ");
        let mut first = true;
        for open_port in open_ports.iter() {
            if first { first = false } else { print!(", ") }
            print!("{}", open_port);
        }
        stdout.flush().unwrap();
        open_ports.len()
    }
    fn clear_open_ports() { let mut open_ports = OPEN_PORTS.lock().unwrap(); open_ports.clear() }
    fn set_port_num(new_port_num: u16) { let mut num = PORT_NUM.lock().unwrap(); *num = new_port_num }
    fn get_port_num() -> u16 { let num = PORT_NUM.lock().unwrap(); *num }
    fn clear_port_num() { let mut num = PORT_NUM.lock().unwrap(); *num = 0 }
    fn set_ip() {
        let mut ip = PORT_SCAN_IP.lock().unwrap();
        let mut new_ip = String::new();
        io::stdin().read_line(&mut new_ip).unwrap();
        let new_ip = new_ip.trim();
        *ip = new_ip.to_string()
    }
    fn get_ip() -> String { let ip = PORT_SCAN_IP.lock().unwrap(); ip.clone() }
    fn clear_ip() { let mut ip = PORT_SCAN_IP.lock().unwrap(); *ip = String::new() }
    fn set_port() {
        let mut port = PORT_RANGE.lock().unwrap();
        let mut new_port = String::new();
        io::stdin().read_line(&mut new_port).unwrap();
        let new_port = new_port.trim();
        *port = new_port.to_string()
    }
    fn get_port() -> String { let port = PORT_RANGE.lock().unwrap(); port.clone() }
    fn clear_port() { let mut port = PORT_RANGE.lock().unwrap(); *port = String::new() }
    let mut stdout = io::stdout();
    let help_string = String::from("| quit: $[esc]$ | change tab: $[a]/[d]$ | change ip: $[ent]$ |");
    let help_more_string = String::from(
r#"| quit: $[q]$ | change tab: $[backtab]/[tab]$ | change ip: $[space]$ |"#
    );
    let help_line_count = help_more_string.lines().count() as u16;
    let last_render_time = get_time();
    let (last_width, last_height) = terminal::size().unwrap();
    let mut needs_rendering = false;
    let mut output = String::new();
    output.push_str(&render_top("port_scan", None, false));
    output.push_str(&render_bottom(0, help_string, help_more_string));
    clear();
    print!("{}", output);
    stdout.flush().unwrap();
    execute!(stdout, cursor::MoveUp(1)).unwrap();
    let mut ip = get_ip();
    if ip.is_empty() {
        print!("Ip: ");
        stdout.flush().unwrap();
        set_ip();
        ip = get_ip()
    } else {
        print!("Ip: {}", ip);
        execute!(stdout, cursor::MoveDown(1)).unwrap()
    }
    if ip.is_empty() {
        return
    }
    let ip = ip.trim();
    execute!(stdout, cursor::MoveToColumn(2)).unwrap();
    let mut port = get_port();
    if port.is_empty() {
        print!("Starting port: ");
        stdout.flush().unwrap();
        set_port();
        port = get_port()
    } else {
        print!("Starting port: {}", port)
    }
    if port.is_empty() {
        clear_ip(); return
    }
    let port = port.trim();
    if get_port_num() == 0 {
        if let Ok(port) = port.parse::<u16>() {
            set_port_num(port);
        } else {
            execute!(stdout, cursor::MoveToColumn(2)).unwrap();
            eprintln!("Error: Invalid port '{}'", port);
        }
    }
    let mut last_port_scan = Instant::now();
    enum PortStatus { Open, Closed, Error(String) }
    fn check_port(ip: &str, port: u16) -> PortStatus {
        let settings = Settings::load();
        let address = format!("{}:{}", ip, port);
        match address.to_socket_addrs() {
            Ok(mut addrs) => {
                if let Some(addr) = addrs.next() {
                    match TcpStream::connect_timeout(&addr, Duration::from_millis(settings.port_scan_timeout as u64)) {
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
    print_port_scans();
    print_open_ports(help_line_count);
    let mut handle: Option<thread::JoinHandle<()>> = None;
    loop {
        if let Some(pressed_key) = get_key() {
            needs_rendering = true;
            match pressed_key {
                KeyCode::Tab | KeyCode::Char('d') | KeyCode::Char('D') => { if let Some(h) = handle.take() { h.join().unwrap(); }; clear_port_scans(); clear_open_ports(); clear_port_num(); clear_ip(); clear_port(); settings_menu() },
                KeyCode::BackTab | KeyCode::Char('a') | KeyCode::Char('A') => { if let Some(h) = handle.take() { h.join().unwrap(); }; clear_port_scans(); clear_open_ports(); clear_port_num(); clear_ip(); clear_port(); return },
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => process::exit(0),
                KeyCode::Enter => { if let Some(h) = handle.take() { h.join().unwrap(); }; clear_port_scans(); clear_open_ports(); clear_port_num(); clear_ip(); clear_port(); port_scan(); return }
                KeyCode::Char(c) if c == ' ' => { if let Some(h) = handle.take() { h.join().unwrap(); }; clear_port_scans(); clear_open_ports(); clear_port_num(); clear_ip(); clear_port(); port_scan(); return }
                _ => {}
            }
        }
        if last_port_scan.elapsed() >= Duration::from_millis(settings.port_scan_timeout as u64) {
            if let Some(h) = handle.take() { h.join().unwrap() }
            let port_num = get_port_num();
            let ip_clone = ip.to_string();
            handle = Some(thread::spawn(move || {
                match check_port(&ip_clone, port_num) {
                    PortStatus::Open => {
                        let port_status = format!("Port {} {}open{}", port_num, SetForegroundColor(get_color("main")), SetForegroundColor(get_color("theme")));
                        add_port_scan(port_status, help_line_count);
                        add_open_port(port_num.to_string());
                        if port_num < u16::MAX { set_port_num(port_num + 1) }
                    }
                    PortStatus::Closed => {
                        let port_status = format!("Port {} closed", port_num);
                        add_port_scan(port_status, help_line_count);
                        if port_num < u16::MAX { set_port_num(port_num + 1) }
                    }
                    PortStatus::Error(err) => {
                        add_port_scan(err, help_line_count);
                    }
                }
            }));
            let num_port_scans = print_port_scans();
            execute!(stdout, cursor::MoveUp(num_port_scans as u16)).unwrap();
            last_port_scan = Instant::now();
            print_open_ports(help_line_count);
        }
        let current_time = get_time();
        let (width, height) = terminal::size().unwrap();
        if width != last_width || height != last_height || current_time != last_render_time || needs_rendering {
            port_scan();
            return;
        }
    }
}

fn micro_macro() {
    println!("micro_macro using enigo")
}

fn macro_tool() {
    println!("macro_tool using enigo")
}

fn quick_start() {
    println!("quick_start")
}

fn quick_download() {
    println!("quick_download")
}

fn tetris() {
    println!("tetris")
}

fn game_of_life() {
    println!("game_of_life")
}

fn sys_fetch() {
    let mut stdout = io::stdout();
    let help_string = String::from("| quit: $[esc]$ | change tab: $[a]/[d]$ |");
    let help_more_string = String::from(
r#"| quit: $[q]$ | change tab: $[backtab]/[tab]$ |"#
    );
    let user_name = whoami::username();
    fn get_machine_name() -> String { { let output = Command::new("hostname").output().expect("Failed to get hostname"); String::from_utf8_lossy(&output.stdout).trim().to_string() } }
    let machine_name = get_machine_name();
    let os = os_info::get();
    let os_string = os.to_string();
    let kernel = os.version().to_string();
    let os_name = os_string.replace(&format!(" {}", &kernel), "");
    let (days, hours, mins) = match uptime_lib::get() { Ok(duration) => {
            let secs = duration.as_secs();
            let days = secs / 86400;
            let hours = (secs % 86400) / 3600;
            let mins = (secs % 3600) / 60;
            (days, hours, mins)
        }
        Err(_) => { (0, 0, 0) }
    };
    let current_dir = match env::current_dir() { Ok(path) => path, Err(e) => { eprintln!("Error getting current directory: {}", e); process::exit(1); } };
    let resolution = match resolution::current_resolution() { Ok((width, height)) => format!("{}x{}", width, height), Err(_) => "Unknown resolution".to_string() };
    let uptime = format!("{} days {} hours {} mins", days, hours, mins);
    #[derive(Deserialize, Debug)]
    #[serde(rename = "Win32_Processor")]
    #[serde(rename_all = "PascalCase")]
    struct Processor { name: String }
    fn get_cpu_name() -> Result<String, Box<dyn Error>> {
        let com_con = COMLibrary::new()?;
        let wmi_con = WMIConnection::new(com_con.into())?;
        let results: Vec<Processor> = wmi_con.query()?;
        Ok(results.first().map(|cpu| cpu.name.clone()).unwrap_or_else(|| "Unknown CPU".to_string()))
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
        Ok(results.first().map(|vc| vc.name.clone()).unwrap_or_else(|| "Unknown GPU".to_string()))
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
        (used_memory_mb as u64, total_memory_mb as u64, usage_percentage)
    }
    let (used_memory, total_memory, ram_usage) = get_ram_info();
    let ram_info = format!("{} MB / {} MB ({}%)", used_memory, total_memory, format!("{:.0}", ram_usage));
    #[derive(Deserialize, Debug)]
    #[serde(rename = "Win32_LogicalDisk")]
    #[serde(rename_all = "PascalCase")]
    struct LogicalDisk { free_space: Option<u64>, size: Option<u64> }
    fn get_disk_info() -> Result<String, Box<dyn Error>> {
        let com_con = COMLibrary::new()?;
        let wmi_con = WMIConnection::new(com_con.into())?;
        let results: Vec<LogicalDisk> = wmi_con.query()?;
        let mut total_used_space = 0u64;
        let mut total_size = 0u64;
        for disk in results {
            if let (Some(free_space), Some(size)) = (disk.free_space, disk.size) {
                total_used_space += size - free_space;
                total_size += size;
            }
        }
        if total_size == 0 { return Ok("Unknown Disk Info".to_string()) }
        let total_used_gb = total_used_space / 1024 / 1024 / 1024;
        let total_size_gb = total_size / 1024 / 1024 / 1024;
        let usage_percentage = (total_used_space as f64 / total_size as f64) * 100.0;
        Ok(format!(
            "{} GB / {} GB ({}%)",
            total_used_gb,
            total_size_gb,
            usage_percentage.round() as u64
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
                SetBackgroundColor(Color::Black),
                SetBackgroundColor(Color::DarkRed),
                SetBackgroundColor(Color::DarkYellow),
                SetBackgroundColor(Color::DarkGreen),
                SetBackgroundColor(Color::DarkCyan),
                SetBackgroundColor(Color::DarkBlue),
                SetBackgroundColor(Color::Magenta),
                SetBackgroundColor(Color::Grey),
                SetBackgroundColor(Color::Black)
            ));
        }
        output.push_str(&format!(
            "{}{}{}│\n",
            line_content,
            SetForegroundColor(get_color("theme")),
            cursor::MoveToColumn(width)
        ));
    }
    output.push_str(&render_bottom(sys_fetch_logo.len() as u16, help_string, help_more_string));
    clear();
    print!("{}", output);
    stdout.flush().unwrap();
    loop {
        if let Some(pressed_key) = get_key() {
            needs_rendering = true;
            match pressed_key {
                KeyCode::Tab | KeyCode::Char('d') | KeyCode::Char('D') => main(),
                KeyCode::BackTab | KeyCode::Char('a') | KeyCode::Char('A') => settings_menu(),
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => process::exit(0),
                _ => {}
            }
        }
        let current_time = get_time();
        let (width, height) = terminal::size().unwrap();
        if width != last_width || height != last_height || current_time != last_render_time || needs_rendering {
            sys_fetch();
            return;
        }
    }
}

fn run_settings_menu_selected(settings_menu_selected: usize, direction: &str) {
    let mut settings = Settings::load();
    let colors = ["grey", "red", "yellow", "green", "cyan", "blue", "magenta"];
    let color_index = colors.iter().position(|&c| c == settings.color ).unwrap_or(0);
    let ping_delays = [10, 50, 100, 200, 500, 1000];
    let ping_delay_index = ping_delays.iter().position(|&c| c == settings.ping_delay ).unwrap_or(0);
    let port_scan_timeouts = [10, 25, 50, 75, 100, 150, 200, 500, 750, 1000];
    let port_scan_timeout_index = port_scan_timeouts.iter().position(|&c| c == settings.port_scan_timeout ).unwrap_or(0);
    let micro_macro_keys = ["F15", "RandomNum", "Enter", "Space", "E", "F", "LMB", "RMB"];
    let micro_macro_key_index = micro_macro_keys.iter().position(|&c| c == settings.micro_macro_key ).unwrap_or(0);
    let micro_macro_delays = [200, 500, 1000, 5000, 10000, 30000, 60000, 120000, 300000, 600000];
    let micro_macro_delay_index = micro_macro_delays.iter().position(|&c| c == settings.micro_macro_delay ).unwrap_or(0);
    match direction {
        "left" => {
            match settings_menu_selected {
                0 => { if color_index > 0 { settings.set_color(colors[color_index - 1]) } else { settings.set_color(colors[colors.len() - 1]) } }
                1 => settings.set_dark_theme(!settings.dark_theme),
                2 => { if ping_delay_index > 0 { settings.set_ping_delay(ping_delays[ping_delay_index - 1]) } else { settings.set_ping_delay(ping_delays[ping_delays.len() - 1]) } }
                3 => { if port_scan_timeout_index > 0 { settings.set_port_scan_timeout(port_scan_timeouts[port_scan_timeout_index - 1]) } else { settings.set_port_scan_timeout(port_scan_timeouts[port_scan_timeouts.len() - 1]) } }
                4 => { if micro_macro_key_index > 0 { settings.set_micro_macro_key(micro_macro_keys[micro_macro_key_index - 1]) } else { settings.set_micro_macro_key(micro_macro_keys[micro_macro_keys.len() - 1]) } }
                5 => { if micro_macro_delay_index > 0 { settings.set_micro_macro_delay(micro_macro_delays[micro_macro_delay_index - 1]) } else { settings.set_micro_macro_delay(micro_macro_delays[micro_macro_delays.len() - 1]) } }
                6 => settings.set_hide_help(!settings.hide_help),
                7 => {
                    {
                        let dir = "NUUI_config";
                        if settings.show_config_files { Command::new("attrib").args(&["+H", dir]).status().expect("Failed to hide NUUI_config");
                        } else { Command::new("attrib").args(&["-H", dir]).status().expect("Failed to unhide NUUI_config"); }
                    }
                    settings.set_show_config_files(!settings.show_config_files);
                }
                8 => {
                    let mut custom_option_path = String::new();
                    print!("Enter file path: ");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut custom_option_path).unwrap();
                    let custom_option_path = custom_option_path.trim();
                    if custom_option_path != "" { settings.add_custom_option(&custom_option_path.to_string()) };
                }
                9 => settings.clear_custom_options(),
                _ => {}
            }
        },
        "right" => {
            match settings_menu_selected {
                0 => settings.set_color(colors[(color_index + 1) % colors.len()]),
                1 => settings.set_dark_theme(!settings.dark_theme),
                2 => settings.set_ping_delay(ping_delays[(ping_delay_index + 1) % ping_delays.len()]),
                3 => settings.set_port_scan_timeout(port_scan_timeouts[(port_scan_timeout_index + 1) % port_scan_timeouts.len()]),
                4 => settings.set_micro_macro_key(micro_macro_keys[(micro_macro_key_index + 1) % micro_macro_keys.len()]),
                5 => settings.set_micro_macro_delay(micro_macro_delays[(micro_macro_delay_index + 1) % micro_macro_delays.len()]),
                6 => settings.set_hide_help(!settings.hide_help),
                7 => {
                    {
                        let dir = "NUUI_config";
                        if settings.show_config_files { Command::new("attrib").args(&["+H", dir]).status().expect("Failed to hide NUUI_config");
                        } else { Command::new("attrib").args(&["-H", dir]).status().expect("Failed to unhide NUUI_config"); }
                    }
                    settings.set_show_config_files(!settings.show_config_files);
                }
                8 => {
                    let mut custom_option_path = String::new();
                    print!("Enter file path: ");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut custom_option_path).unwrap();
                    let custom_option_path = custom_option_path.trim();
                    if custom_option_path != "" { settings.add_custom_option(&custom_option_path.to_string()) };
                }
                9 => settings.clear_custom_options(),
                _ => {}
            }
        },
        _ => {}
    }
}

fn render_settings_menu(menu_selected: usize, menu_options: &[&str]) {
    let settings = Settings::load();
    let mut stdout = io::stdout();
    let help_string = String::from("| quit: $[esc]$ | change tab: $[a]/[d]$ | scroll: $[w]/[s]$ | change setting: $[←]/[→]$ |");
    let help_more_string = String::from(
r#"| change setting: $[ent]$ | select: $[0-9]$ |
| quit: $[q]$ | change tab: $[backtab]/[tab]$ | scroll: $[↑]/[↓]$ |"#
    );
    let (width, _) = terminal::size().unwrap();
    let mut output = String::new();
    output.push_str(&render_top("settings", None, false));
    for i in 0..menu_options.len() {
        if i == menu_selected {
            output.push_str("│");
            output.push_str(&format!(
                "{}{} {} {} {} {}{}{}{}",
                SetBackgroundColor(get_color("main")),
                SetForegroundColor(Color::Black),
                i,
                "›",
                menu_options[i],
                if menu_options[i] == "dark_theme" {
                    if settings.dark_theme { "1 ".to_string() } else { "0 ".to_string() }
                } else if menu_options[i] == "ping_delay" {
                    settings.ping_delay.to_string() + "ms "
                } else if menu_options[i] == "port_scan_timeout" {
                    settings.port_scan_timeout.to_string() + "ms "
                } else if menu_options[i] == "micro_macro_key" {
                    settings.micro_macro_key.to_string() + " "
                } else if menu_options[i] == "micro_macro_delay" {
                    let delay = settings.micro_macro_delay as usize; let (display_delay, delay_unit) = if delay <= 1000 { (delay, "ms") } else if delay > 60000 { (delay / 60000, "m") } else { (delay / 1000, "s") }; format!("{}{} ", display_delay, delay_unit)
                } else if menu_options[i] == "hide_help" {
                    if settings.hide_help { "1 ".to_string() } else { "0 ".to_string() }
                } else if menu_options[i] == "show_config_files" {
                    if settings.show_config_files { "1 ".to_string() } else { "0 ".to_string() }
                } else if menu_options[i] == "clear_custom" {
                    settings.custom_options.len().to_string() + " "
                } else {
                    " ".to_string()
                },
                SetForegroundColor(get_color("theme")),
                SetBackgroundColor(Color::Black),
                cursor::MoveToColumn(width)
            ));
            output.push_str("│\n");
        } else {
            output.push_str("│");
            output.push_str(&format!(
                "{} {} {}{} {}{}{}│\n",
                SetForegroundColor(get_color("main")),
                i,
                SetForegroundColor(Color::DarkGrey),
                "|",
                SetForegroundColor(get_color("theme")),
                menu_options[i],
                cursor::MoveToColumn(width)
            ));
        }
    }
    output.push_str(&&render_bottom(menu_options.len() as u16, help_string, help_more_string));
    clear();
    print!("{}", output);
    stdout.flush().unwrap();
}

fn settings_menu() {
    let settings_menu_options = ["color", "dark_theme", "ping_delay", "port_scan_timeout", "micro_macro_key", "micro_macro_delay", "hide_help", "show_config_files", "add_custom", "clear_custom"];
    let mut settings_menu_selected = 0;
    let mut last_render_time = get_time();
    let (mut last_width, mut last_height) = terminal::size().unwrap();
    let mut needs_rendering = true;
    loop {
        if let Some(pressed_key) = get_key() {
            needs_rendering = true;
            match pressed_key {
                KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => if settings_menu_selected > 0 { settings_menu_selected -= 1 } else { settings_menu_selected = settings_menu_options.len() - 1 }
                KeyCode::Left => run_settings_menu_selected(settings_menu_selected, "left"),
                KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => if settings_menu_selected < settings_menu_options.len() - 1 { settings_menu_selected += 1 } else { settings_menu_selected = 0 }
                KeyCode::Right => run_settings_menu_selected(settings_menu_selected, "right"),
                KeyCode::Tab | KeyCode::Char('d') | KeyCode::Char('D') => sys_fetch(),
                KeyCode::BackTab | KeyCode::Char('a') | KeyCode::Char('A') => main(),
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => process::exit(0),
                KeyCode::Enter => run_settings_menu_selected(settings_menu_selected, "right"),
                KeyCode::Char(c) if c.is_digit(10) => { 
                    let num = c.to_digit(10).unwrap() as usize;
                    if num < settings_menu_options.len() { 
                        settings_menu_selected = num;
                    };
                },
                _ => {}
            }
        }
        let current_time = get_time();
        let (width, height) = terminal::size().unwrap();
        if width != last_width || height != last_height || current_time != last_render_time || needs_rendering {
            render_settings_menu(settings_menu_selected, &settings_menu_options);
            last_render_time = current_time;
            last_width = width;
            last_height = height;
            needs_rendering = false;
        }
    }
}

fn run_menu_selected(menu_selected: usize, menu_options: &[&str]) {
    let settings = Settings::load();
    match menu_selected {
        0 => ping_tool(),
        1 => port_scan(),
        2 => micro_macro(),
        3 => macro_tool(),
        4 => quick_start(),
        5 => quick_download(),
        6 => tetris(),
        7 => game_of_life(),
        _ => {
            let _ = run_file(&settings.custom_options[menu_selected - (menu_options.len() - settings.custom_options.len())]);
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

fn render_menu(menu_selected: usize, menu_options: &[&str]) {
    let mut stdout = io::stdout();
    let help_string = String::from("| quit: $[esc]$ | change tab: $[a]/[d]$ | scroll: $[w]/[s]$ | select: $[ent]$ |");
    let help_more_string = String::from(
r#"| special select: $[space]$ | select: $[0-9]$ |
| quit: $[q]$ | change tab: $[backtab]/[tab]$ | scroll: $[↑]/[←]/[↓]/[→]$ |"#
    );
    let (width, _) = terminal::size().unwrap();
    let mut output = String::new();
    output.push_str(&render_top("menu", None, false));
    for i in 0..menu_options.len() {
        let mut spaces= " ";
        if i >= 10 { spaces = "" }
        if i == menu_selected {
            output.push_str("│");
            output.push_str(&format!(
                "{}{}{}{} {} {}  {}{}{}",
                SetForegroundColor(Color::Black),
                SetBackgroundColor(get_color("main")),
                spaces,
                i,
                "›",
                menu_options[i],
                SetForegroundColor(get_color("theme")),
                SetBackgroundColor(Color::Black),
                cursor::MoveToColumn(width)
            ));
            output.push_str("│\n");
        } else {
            output.push_str("│");
            output.push_str(&format!(
                "{}{}{} {}{} {}{}{}│\n",
                SetForegroundColor(get_color("main")),
                spaces,
                i,
                SetForegroundColor(Color::DarkGrey),
                "|",
                SetForegroundColor(get_color("theme")),
                menu_options[i],
                cursor::MoveToColumn(width)
            ));
        }
    }
    output.push_str(&&render_bottom(menu_options.len() as u16, help_string, help_more_string));
    clear();
    print!("{}", output);
    stdout.flush().unwrap();
}

fn main() {
    terminal::enable_raw_mode().unwrap();
    let settings = Settings::load();
    let mut menu_options: Vec<String> = vec![
        "ping_tool".to_string(),
        "port_scan".to_string(),
        "micro_macro".to_string(),
        "macro".to_string(),
        "quick_start".to_string(),
        "quick_download".to_string(),
        "tetris".to_string(),
        "game_of_life".to_string(),
    ];
    for path in &settings.custom_options {
        let path_obj = Path::new(path);
        if let Some(file_name) = path_obj.file_name() {
            if let Some(name_str) = file_name.to_str() {
                menu_options.push(name_str.to_string());
            }
        }
    }
    let mut menu_selected = 0;
    let mut last_render_time = get_time();
    let (mut last_width, mut last_height) = terminal::size().unwrap();
    let mut needs_rendering = true;
    loop {
        if let Some(pressed_key) = get_key() {
            needs_rendering = true;
            match pressed_key {
                KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => if menu_selected > 0 { menu_selected -= 1 } else { menu_selected = menu_options.len() - 1 }
                KeyCode::Left => if menu_selected > 0 { menu_selected -= 1 } else { menu_selected = menu_options.len() - 1 }
                KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => if menu_selected < menu_options.len() - 1 { menu_selected += 1 } else { menu_selected = 0 }
                KeyCode::Right => if menu_selected < menu_options.len() - 1 { menu_selected += 1 } else { menu_selected = 0 }
                KeyCode::Tab | KeyCode::Char('d') | KeyCode::Char('D') => settings_menu(),
                KeyCode::BackTab | KeyCode::Char('a') | KeyCode::Char('A') => sys_fetch(),
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => process::exit(0),
                KeyCode::Char(c) if c == ' ' => { println!("DODAJ TUTAJ TO CO MIALES DODAC IDIOTO WALONY NIE ZAPOMNIJ BO BEDZIE PRZYPAL") }
                KeyCode::Char(c) if c.is_digit(10) => { let num = c.to_digit(10).unwrap() as usize; if num < menu_options.len() { menu_selected = num } run_menu_selected(menu_selected, &menu_options.iter().map(|s| s.as_str()).collect::<Vec<&str>>()) }
                KeyCode::Enter => run_menu_selected(menu_selected,&menu_options.iter().map(|s| s.as_str()).collect::<Vec<&str>>()),
                _ => {}
            }
        }
        let current_time = get_time();
        let (width, height) = terminal::size().unwrap();
        if width != last_width || height != last_height || current_time != last_render_time || needs_rendering {
            render_menu(menu_selected, &menu_options.iter().map(|s| s.as_str()).collect::<Vec<&str>>());
            last_render_time = current_time;
            last_width = width;
            last_height = height;
            needs_rendering = false;
        }
    }
}