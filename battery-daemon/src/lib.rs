pub mod battery;
pub mod notifier;
pub mod traits;

use crate::battery::sysfs::SysfsBatteryMonitor;
use crate::notifier::notify_send::NotifySendNotifier;
use crate::traits::{BatteryMonitor, BatteryStatus, Notifier};
use std::thread;
use std::time::Duration;

// Configuration constants
const CHECK_INTERVAL_SECS: u64 = 60;
const LOW_BATTERY_THRESHOLD: u8 = 30;

/// Starts the battery daemon.
/// This function contains the main loop of the application.
pub fn run_daemon() {
    println!("Starting Battery Daemon...");
    println!("Monitoring battery level. Warning threshold: {}%", LOW_BATTERY_THRESHOLD);

    // Initialize the components
    let battery_monitor = SysfsBatteryMonitor::new();
    let notifier = NotifySendNotifier::new();

    loop {
        match battery_monitor.get_info() {
            Ok(info) => {
                println!("Current Status: {:?}, Capacity: {}%", info.status, info.capacity);

                if info.status == BatteryStatus::Discharging && info.capacity <= LOW_BATTERY_THRESHOLD {
                    let summary = "Battery Low";
                    let body = format!("Battery level is at {}%. Please plug in charger.", info.capacity);

                    match notifier.notify(summary, &body) {
                        Ok(_) => {
                            println!("Notification sent: {}", body);
                        }
                        Err(e) => eprintln!("Failed to send notification: {}", e),
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading battery info: {}", e);
            }
        }

        // Sleep before the next check.
        // This naturally implements the "notify every 60s" requirement if the condition persists,
        // because we check again after 60s and re-notify if still low and discharging.
        thread::sleep(Duration::from_secs(CHECK_INTERVAL_SECS));
    }
}
