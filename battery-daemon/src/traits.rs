// This module defines the core traits for the battery daemon.
// These traits allow for a pluggable architecture where different implementations
// for battery monitoring and user notification can be swapped easily.

/// Represents the status of the battery.
#[derive(Debug, PartialEq)]
pub enum BatteryStatus {
    Charging,
    Discharging,
    Full,
    Unknown,
}

/// Holds information about the battery state.
#[derive(Debug)]
pub struct BatteryInfo {
    /// Battery capacity in percentage (0-100).
    pub capacity: u8,
    /// Current charging status.
    pub status: BatteryStatus,
}

/// Trait for retrieving battery information.
/// Implementations of this trait are responsible for querying the system
/// (e.g., via sysfs, upower, etc.) to get current battery details.
pub trait BatteryMonitor {
    /// Returns the current battery information.
    /// Returns an error string if retrieving the info fails.
    fn get_info(&self) -> Result<BatteryInfo, String>;
}

/// Trait for notifying the user.
/// Implementations of this trait are responsible for delivering messages
/// to the user (e.g., via desktop notifications, console output, etc.).
pub trait Notifier {
    /// Sends a notification with the given summary and body.
    /// Returns an error string if sending the notification fails.
    fn notify(&self, summary: &str, body: &str) -> Result<(), String>;
}
