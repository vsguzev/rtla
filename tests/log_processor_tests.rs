#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    // Helper function to create a sample log entry
    fn create_sample_log_entry(level: &str, message: &str) -> LogEntry {
        LogEntry {
            timestamp: "2023-11-17T12:00:00".to_string(),
            level: level.to_string(),
            message: message.to_string(),
        }
    }

    #[test]
    fn test_process_logs() {
        let mut processor = LogProcessor::new();
        
        // Simulate adding log entries
        processor.log_buffer.push_back(create_sample_log_entry("INFO", "First log entry"));
        processor.log_buffer.push_back(create_sample_log_entry("ERROR", "Second log entry"));

        let processed_logs = processor.process_logs();

        // Check if the log entries are processed correctly
        assert_eq!(processed_logs.len(), 2);
        assert_eq!(processed_logs[0].level, "INFO");
        assert_eq!(processed_logs[1].level, "ERROR");
    }

    #[test]
    fn test_log_filtering() {
        let mut processor = LogProcessor::new();

        // Add various log entries
        processor.log_buffer.push_back(create_sample_log_entry("INFO", "Info log"));
        processor.log_buffer.push_back(create_sample_log_entry("DEBUG", "Debug log"));
        processor.log_buffer.push_back(create_sample_log_entry("ERROR", "Error log"));

        // Implement a filtering logic in `LogProcessor` and test it
        let filtered_logs = processor.filter_logs("ERROR");

        // Check if only ERROR logs are filtered
        assert_eq!(filtered_logs.len(), 1);
        assert_eq!(filtered_logs[0].level, "ERROR");
        assert_eq!(filtered_logs[0].message, "Error log");
    }

    // Additional tests for other functionalities of `LogProcessor` can be added here
}
