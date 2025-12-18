#[cfg(test)]
mod config_tests {
    use cascolor::config::Config;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn temp_config_path() -> (TempDir, PathBuf) {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        (temp_dir, config_path)
    }

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.general.theme, "dark");
        assert_eq!(config.general.history_size, 20);
        assert_eq!(config.updates.channel, "stable");
        assert!(config.updates.check_on_startup);
        assert!(config.updates.check_in_background);
        assert!(config.updates.prompt_before_update);
        assert!(config.ui.show_system_tray);
        assert!(config.ui.remember_window_position);
        assert_eq!(config.ui.default_color_format, "hex");
        assert!(config.color_history.is_empty());
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let toml_str = toml::to_string(&config).unwrap();
        assert!(toml_str.contains("theme"));
        assert!(toml_str.contains("dark"));
        assert!(toml_str.contains("history_size"));
    }

    #[test]
    fn test_config_deserialization() {
        let toml_str = r#"
            [general]
            theme = "light"
            history_size = 30

            [updates]
            channel = "beta"
            check_on_startup = false
            check_in_background = false
            prompt_before_update = false

            [ui]
            show_system_tray = false
            remember_window_position = false
            default_color_format = "rgb"
        "#;

        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.general.theme, "light");
        assert_eq!(config.general.history_size, 30);
        assert_eq!(config.updates.channel, "beta");
        assert!(!config.updates.check_on_startup);
        assert_eq!(config.ui.default_color_format, "rgb");
        assert!(!config.ui.show_system_tray);
        assert_eq!(config.general.history_size, 30);
        assert_eq!(config.updates.channel, "beta");
        assert!(!config.updates.check_on_startup);
        assert!(!config.updates.check_in_background);
        assert!(!config.updates.prompt_before_update);
        assert!(!config.ui.show_system_tray);
        assert!(!config.ui.remember_window_position);
        assert_eq!(config.ui.default_color_format, "rgb");
        assert_eq!(config.color_history.len(), 0); // No history in TOML, defaults to empty
    }

    #[test]
    fn test_config_with_history() {
        let mut config = Config::default();
        config.color_history.push("#FF5733".to_string());
        config.color_history.push("#33FF57".to_string());
        
        let toml_str = toml::to_string(&config).unwrap();
        let deserialized: Config = toml::from_str(&toml_str).unwrap();
        
        assert_eq!(deserialized.color_history.len(), 2);
        assert_eq!(deserialized.color_history[0], "#FF5733");
        assert_eq!(deserialized.color_history[1], "#33FF57");
    }

    #[test]
    fn test_partial_config() {
        // Test that missing fields use defaults
        let toml_str = r#"
            [general]
            theme = "light"
        "#;

        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.general.theme, "light");
        assert_eq!(config.general.history_size, 20); // Default
        assert_eq!(config.updates.channel, "stable"); // Default
    }

    #[test]
    fn test_update_channels() {
        let toml_str = r#"
            [updates]
            channel = "daily"
        "#;

        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.updates.channel, "daily");
    }

    #[test]
    fn test_theme_options() {
        for theme in &["dark", "light", "system", "auto"] {
            let toml_str = format!(r#"[general]
theme = "{}""#, theme);
            let config: Config = toml::from_str(&toml_str).unwrap();
            assert_eq!(config.general.theme, *theme);
        }
    }
}
