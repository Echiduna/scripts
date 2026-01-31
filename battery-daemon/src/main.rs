use battery_daemon::config::Config;
use battery_daemon::run_daemon;

fn main() {
    let config = Config::load();
    run_daemon(config);
}
