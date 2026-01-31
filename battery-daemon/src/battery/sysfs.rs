use crate::traits::{BatteryInfo, BatteryMonitor, BatteryStatus};
use std::fs;
use std::path::{Path, PathBuf};

/// Implementation of BatteryMonitor that reads from the Linux sysfs interface.
/// Usually located at /sys/class/power_supply/BAT*.
pub struct SysfsBatteryMonitor {
    /// Optional path to a specific battery directory.
    /// If None, it attempts to auto-discover a battery.
    battery_path: Option<PathBuf>,
}

impl SysfsBatteryMonitor {
    /// Creates a new SysfsBatteryMonitor.
    /// It tries to find a battery in /sys/class/power_supply.
    pub fn new() -> Self {
        let battery_path = Self::find_battery_path();
        SysfsBatteryMonitor { battery_path }
    }

    /// Helper to find a battery directory.
    /// Iterates over /sys/class/power_supply and looks for names starting with "BAT".
    fn find_battery_path() -> Option<PathBuf> {
        let power_supply_dir = Path::new("/sys/class/power_supply");
        if let Ok(entries) = fs::read_dir(power_supply_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    // Common convention for battery names on Linux
                    if name.starts_with("BAT") {
                        return Some(path);
                    }
                }
            }
        }
        None
    }
}

impl BatteryMonitor for SysfsBatteryMonitor {
    fn get_info(&self) -> Result<BatteryInfo, String> {
        let path = self
            .battery_path
            .as_ref()
            .ok_or("No battery found in /sys/class/power_supply".to_string())?;

        // Read capacity
        let capacity_path = path.join("capacity");
        let capacity_str = fs::read_to_string(&capacity_path)
            .map_err(|e| format!("Failed to read capacity from {:?}: {}", capacity_path, e))?;

        let capacity: u8 = capacity_str
            .trim()
            .parse()
            .map_err(|_| "Failed to parse capacity value".to_string())?;

        // Read status
        let status_path = path.join("status");
        let status_str = fs::read_to_string(&status_path)
            .map_err(|e| format!("Failed to read status from {:?}: {}", status_path, e))?;

        let status = match status_str.trim() {
            "Charging" => BatteryStatus::Charging,
            "Discharging" => BatteryStatus::Discharging,
            "Full" => BatteryStatus::Full,
            _ => BatteryStatus::Unknown,
        };

        Ok(BatteryInfo { capacity, status })
    }
}
