pub mod battery;
pub mod config;
pub mod notifier;
pub mod traits;

use crate::battery::sysfs::SysfsBatteryMonitor;
use crate::config::Config;
use crate::notifier::notify_send::NotifySendNotifier;
use crate::traits::{BatteryMonitor, BatteryStatus, Notifier};
use std::thread;
use std::time::Duration;

/// Starts the battery daemon.
/// This function contains the main loop of the application.
pub fn run_daemon(config: Config) {
    println!("Starting Battery Daemon...");
    println!("Monitoring battery level.");
    println!("  Check Interval: {}s", config.check_interval);
    println!("  Warning Threshold: {}%", config.threshold);

    // Initialize the components
    let battery_monitor = SysfsBatteryMonitor::new();
    let notifier = NotifySendNotifier::new();

    loop {
        match battery_monitor.get_info() {
            Ok(info) => {
                println!("Current Status: {:?}, Capacity: {}%", info.status, info.capacity);

                if info.status == BatteryStatus::Discharging && info.capacity <= config.threshold {
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
        thread::sleep(Duration::from_secs(config.check_interval));
    }
}
