use std::env;
use std::fs;
use std::path::PathBuf;

// Default constants
const DEFAULT_CHECK_INTERVAL_SECS: u64 = 60;
const DEFAULT_LOW_BATTERY_THRESHOLD: u8 = 30;

#[derive(Debug)]
pub struct Config {
    pub check_interval: u64,
    pub threshold: u8,
}

impl Config {
    /// Loads configuration with precedence: CLI > Env > Config File > Default
    pub fn load() -> Self {
        // 1. Start with defaults
        let mut check_interval = DEFAULT_CHECK_INTERVAL_SECS;
        let mut threshold = DEFAULT_LOW_BATTERY_THRESHOLD;

        // 2. Load from Config File (lowest precedence of user inputs)
        if let Some(config_path) = get_xdg_config_path() {
            if config_path.exists() {
                if let Ok(content) = fs::read_to_string(config_path) {
                    parse_config_file(&content, &mut check_interval, &mut threshold);
                }
            }
        }

        // 3. Load from Environment Variables
        if let Ok(val) = env::var("BATTERY_DAEMON_INTERVAL") {
            if let Ok(parsed) = val.parse() {
                check_interval = parsed;
            }
        }
        if let Ok(val) = env::var("BATTERY_DAEMON_THRESHOLD") {
            if let Ok(parsed) = val.parse() {
                threshold = parsed;
            }
        }

        // 4. Load from CLI Arguments (highest precedence)
        // We do a simple manual parse since no external crates are allowed.
        let args: Vec<String> = env::args().collect();
        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--interval" | "-i" => {
                    if i + 1 < args.len() {
                        if let Ok(parsed) = args[i + 1].parse() {
                            check_interval = parsed;
                        }
                        i += 1;
                    }
                }
                "--threshold" | "-t" => {
                    if i + 1 < args.len() {
                        if let Ok(parsed) = args[i + 1].parse() {
                            threshold = parsed;
                        }
                        i += 1;
                    }
                }
                _ => {}
            }
            i += 1;
        }

        Config {
            check_interval,
            threshold,
        }
    }
}

/// simple key=value parser
fn parse_config_file(content: &str, interval: &mut u64, threshold: &mut u8) {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            match key {
                "interval" => {
                    if let Ok(parsed) = value.parse() {
                        *interval = parsed;
                    }
                }
                "threshold" => {
                    if let Ok(parsed) = value.parse() {
                        *threshold = parsed;
                    }
                }
                _ => {}
            }
        }
    }
}

fn get_xdg_config_path() -> Option<PathBuf> {
    let xdg_config_home = env::var("XDG_CONFIG_HOME").ok().map(PathBuf::from);

    let config_dir = xdg_config_home.or_else(|| {
        env::var("HOME").ok().map(|home| PathBuf::from(home).join(".config"))
    })?;

    Some(config_dir.join("battery-daemon").join("config"))
}
