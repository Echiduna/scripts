mod battery;
mod notifier;
mod traits;

use crate::battery::sysfs::SysfsBatteryMonitor;
use crate::notifier::notify_send::NotifySendNotifier;
use crate::traits::{BatteryMonitor, BatteryStatus, Notifier};
use std::thread;
use std::time::Duration;

// Configuration constants
const CHECK_INTERVAL_SECS: u64 = 60;
const LOW_BATTERY_THRESHOLD: u8 = 30;

fn main() {
    println!("Starting Battery Daemon...");
    println!("Monitoring battery level. Warning threshold: {}%", LOW_BATTERY_THRESHOLD);

    // Initialize the components
    let battery_monitor = SysfsBatteryMonitor::new();
    let notifier = NotifySendNotifier::new();

    // State to track if we have already warned about the current low battery event.
    // This prevents spamming notifications every check interval.
    let mut has_notified_low = false;

    loop {
        match battery_monitor.get_info() {
            Ok(info) => {
                println!("Current Status: {:?}, Capacity: {}%", info.status, info.capacity);

                if info.status == BatteryStatus::Discharging {
                    if info.capacity <= LOW_BATTERY_THRESHOLD {
                        if !has_notified_low {
                            let summary = "Battery Low";
                            let body = format!("Battery level is at {}%. Please plug in charger.", info.capacity);

                            match notifier.notify(summary, &body) {
                                Ok(_) => {
                                    println!("Notification sent: {}", body);
                                    has_notified_low = true;
                                }
                                Err(e) => eprintln!("Failed to send notification: {}", e),
                            }
                        }
                    } else {
                        // Reset the flag if capacity goes above threshold (e.g. weird fluctuation or partial charge without status change?)
                        // Usually status would change to Charging, but safe to reset here too.
                        has_notified_low = false;
                    }
                } else {
                    // If we are charging or full, reset the notification flag.
                    has_notified_low = false;
                }
            }
            Err(e) => {
                eprintln!("Error reading battery info: {}", e);
            }
        }

        // Sleep before the next check
        thread::sleep(Duration::from_secs(CHECK_INTERVAL_SECS));
    }
}
