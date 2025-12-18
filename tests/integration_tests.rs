// Integration tests for cascolor
// These tests verify the application as a whole

#[cfg(test)]
mod integration_tests {
    use cascolor::color::Color;
    use cascolor::config::Config;

    #[test]
    fn test_color_workflow() {
        // Simulate a typical user workflow
        let color = Color::from_hex("#FF5733").unwrap();
        
        // User can convert to different formats
        let hex = color.to_hex();
        let rgb = color.to_rgb_string();
        let hsl = color.to_hsl_string();
        
        assert_eq!(hex, "#FF5733");
        assert!(rgb.contains("255"));
        assert!(hsl.contains("hsl"));
    }

    #[test]
    fn test_config_and_history() {
        let mut config = Config::default();
        
        // Simulate adding colors to history
        config.color_history.push("#FF5733".to_string());
        config.color_history.push("#33FF57".to_string());
        
        assert_eq!(config.color_history.len(), 2);
        
        // Verify history doesn't exceed limit
        for i in 0..25 {
            config.color_history.push(format!("#00000{:01X}", i % 16));
        }
        
        config.color_history.truncate(config.general.history_size);
        assert_eq!(config.color_history.len(), 20);
    }

    #[test]
    fn test_color_picker_simulation() {
        // Simulate picking a color and converting it
        let picked_color = Color::from_rgb(128, 64, 192);
        
        // Get all formats
        let formats = picked_color.to_all_formats();
        
        // Verify all formats are generated
        assert!(!formats.hex.is_empty());
        assert!(!formats.rgb.is_empty());
        assert!(!formats.hsl.is_empty());
        assert!(!formats.hsv.is_empty());
        assert!(!formats.cmyk.is_empty());
        assert!(!formats.lab.is_empty());
        assert!(!formats.lch.is_empty());
        assert!(!formats.oklab.is_empty());
        assert!(!formats.oklch.is_empty());
    }

    #[test]
    fn test_theme_switching() {
        let mut config = Config::default();
        assert_eq!(config.general.theme, "dark");
        
        // Simulate theme toggle
        config.general.theme = "light".to_string();
        assert_eq!(config.general.theme, "light");
        
        config.general.theme = "dark".to_string();
        assert_eq!(config.general.theme, "dark");
    }

    #[test]
    fn test_update_channel_switching() {
        let mut config = Config::default();
        assert_eq!(config.updates.channel, "stable");
        
        config.updates.channel = "beta".to_string();
        assert_eq!(config.updates.channel, "beta");
        
        config.updates.channel = "daily".to_string();
        assert_eq!(config.updates.channel, "daily");
    }
}
