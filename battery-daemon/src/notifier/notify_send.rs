use crate::traits::Notifier;
use std::process::Command;

/// Implementation of Notifier that uses the `notify-send` command line utility.
/// This requires `libnotify-bin` (or equivalent) to be installed on the system.
pub struct NotifySendNotifier;

impl NotifySendNotifier {
    pub fn new() -> Self {
        NotifySendNotifier
    }
}

impl Notifier for NotifySendNotifier {
    fn notify(&self, summary: &str, body: &str) -> Result<(), String> {
        // Construct the command `notify-send "summary" "body"`
        // We set urgency to critical for low battery
        let output = Command::new("notify-send")
            .arg("--urgency=critical")
            .arg(summary)
            .arg(body)
            .output()
            .map_err(|e| format!("Failed to execute notify-send: {}", e))?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("notify-send failed: {}", stderr))
        }
    }
}
