#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    // Helper function to create a temporary config file
    fn create_temp_config_file(contents: &str) -> String {
        let temp_file = tempfile::NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_str().unwrap().to_string();
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "{}", contents).unwrap();
        file_path
    }

    #[test]
    fn test_load_valid_config() {
        let config_contents = r#"
            [log_config]
            file_log_path = "/var/log/myapp.log"

            [alert_config]
            error_threshold = 5
            critical_message_keyword = "CRITICAL"
        "#;

        let file_path = create_temp_config_file(config_contents);
        let config = load_config(&file_path).unwrap();

        assert_eq!(config.log_config.file_log_path, "/var/log/myapp.log");
        assert_eq!(config.alert_config.error_threshold, 5);
        assert_eq!(config.alert_config.critical_message_keyword, "CRITICAL");

        // Clean up the temporary file
        std::fs::remove_file(Path::new(&file_path)).unwrap();
    }

    #[test]
    fn test_load_invalid_config() {
        let invalid_config_contents = r#"
            [invalid_config]
            unknown_field = "value"
        "#;

        let file_path = create_temp_config_file(invalid_config_contents);
        let result = load_config(&file_path);

        assert!(result.is_err());

        // Clean up the temporary file
        std::fs::remove_file(Path::new(&file_path)).unwrap();
    }
}
