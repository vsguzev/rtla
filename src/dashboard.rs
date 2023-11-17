use crate::log_processor::LogEntry;
use std::collections::VecDeque;

pub struct Dashboard {
    log_entries: VecDeque<LogEntry>,
}

impl Dashboard {
    // Constructor for Dashboard
    pub fn new() -> Self {
        Dashboard {
            log_entries: VecDeque::new(),
        }
    }

    // Update the dashboard with new log entries
    pub fn update(&mut self, new_logs: &[LogEntry]) {
        for log in new_logs {
            self.log_entries.push_back(log.clone());
            // Keep the log buffer to a reasonable size
            if self.log_entries.len() > 100 {
                self.log_entries.pop_front();
            }
        }

        // Display the updated log entries
        self.display();
    }

    // Display the log entries on the dashboard
    fn display(&self) {
        println!("----- Dashboard -----");
        for log in &self.log_entries {
            println!("[{}] [{}] {}", log.timestamp, log.level, log.message);
        }
        println!("---------------------\n");
    }

    // Additional methods for interactive features can be added here
}
