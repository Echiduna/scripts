# Battery Daemon Agents Guidelines

This document outlines the coding standards, architectural requirements, and constraints for the `battery-daemon` project.

## Project Scope
This is a simple utility to notify the user when the battery is too low (default 30%).
It currently targets **Linux** only.

## Constraints
1.  **No External Crates**: The project must be self-contained. Do not add any dependencies to `Cargo.toml`.
2.  **Platform**: Target Linux. Prefer `musl` build to avoid libc compatibility issues.
3.  **Documentation**: Code must be heavily commented. Explain *why* something is done, not just *what*.
4.  **Architecture**:
    *   **Pluggable Battery Interface**: The mechanism for retrieving battery status must be abstracted behind a trait/interface.
    *   **Pluggable Notification Interface**: The mechanism for notifying the user must be abstracted behind a trait/interface.

## Architecture

### Modules
*   `src/traits.rs`: Definitions of `BatteryMonitor` and `Notifier` traits.
*   `src/battery/`: Implementations for battery monitoring (e.g., via `/sys/class/power_supply` or `upower`).
*   `src/notifier/`: Implementations for user notification (e.g., via `notify-send` or direct DBus if feasible without crates).

### Implementation Details
*   **Battery Monitoring**: The default implementation should prefer reading from `/sys/class/power_supply` as it requires no external binaries.
*   **Notification**: The default implementation may wrap `notify-send` as a pragmatic solution given the "no external crates" constraint preventing easy DBus integration.

## Building
To build with musl (if available):
```bash
cargo build --target x86_64-unknown-linux-musl --release
```
Otherwise, standard `cargo build` is acceptable for development.
