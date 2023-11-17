use std::env;
use std::process;

mod config;
mod log_processor;
mod dashboard;
mod alert_system;

fn main() {
    // Set up logging
    env_logger::init();

    // Load configuration
    let config_path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: rtla <config_path>");
        process::exit(1);
    });

    let config = match config::load_config(&config_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            process::exit(1);
        }
    };

    // Initialize log processor
    let log_processor = log_processor::LogProcessor::new(config.log_config);

    // Initialize dashboard
    let dashboard = dashboard::Dashboard::new();

    // Initialize alert system
    let alert_system = alert_system::AlertSystem::new(config.alert_config);

    // Start the main application loop
    loop {
        // Process logs
        let logs = log_processor.process_logs();

        // Update dashboard
        dashboard.update(&logs);

        // Check and handle alerts
        alert_system.check_alerts(&logs);

        // Implement a sleep or a more sophisticated scheduling mechanism
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
