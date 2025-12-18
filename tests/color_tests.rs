#[cfg(test)]
mod color_conversion_tests {
    use cascolor::color::Color;

    #[test]
    fn test_hex_to_rgb() {
        let color = Color::from_hex("#FF5733").unwrap();
        let (r, g, b) = color.to_rgb();
        assert_eq!(r, 255);
        assert_eq!(g, 87);
        assert_eq!(b, 51);
    }

    #[test]
    fn test_hex_to_hex() {
        let color = Color::from_hex("#FF5733").unwrap();
        assert_eq!(color.to_hex(), "#FF5733");
    }

    #[test]
    fn test_short_hex() {
        let color = Color::from_hex("#F53").unwrap();
        assert_eq!(color.to_hex(), "#FF5533");
    }

    #[test]
    fn test_hex_without_hash() {
        let color = Color::from_hex("FF5733").unwrap();
        assert_eq!(color.to_hex(), "#FF5733");
    }

    #[test]
    fn test_rgb_to_hsl() {
        let color = Color::from_rgb(255, 0, 0); // Pure red
        let (h, s, l) = color.to_hsl();
        assert!((h - 0.0).abs() < 1.0);
        assert!((s - 100.0).abs() < 1.0);
        assert!((l - 50.0).abs() < 1.0);
    }

    #[test]
    fn test_hsl_to_rgb() {
        let color = Color::from_hsl(0.0, 100.0, 50.0); // Red
        let (r, g, b) = color.to_rgb();
        assert_eq!(r, 255);
        assert!(g < 5);
        assert!(b < 5);
    }

    #[test]
    fn test_rgb_to_hsv() {
        let color = Color::from_rgb(255, 0, 0); // Pure red
        let (h, s, v) = color.to_hsv();
        assert!((h - 0.0).abs() < 1.0);
        assert!((s - 100.0).abs() < 1.0);
        assert!((v - 100.0).abs() < 1.0);
    }

    #[test]
    fn test_white_color() {
        let color = Color::from_rgb(255, 255, 255);
        assert_eq!(color.to_hex(), "#FFFFFF");
        let (_, _, l) = color.to_hsl();
        assert!((l - 100.0).abs() < 1.0);
    }

    #[test]
    fn test_black_color() {
        let color = Color::from_rgb(0, 0, 0);
        assert_eq!(color.to_hex(), "#000000");
        let (_, _, l) = color.to_hsl();
        assert!((l - 0.0).abs() < 1.0);
    }

    #[test]
    fn test_gray_color() {
        let color = Color::from_rgb(128, 128, 128);
        let (_, s, _) = color.to_hsl();
        assert!(s < 1.0); // Gray should have low saturation
    }

    #[test]
    fn test_cmyk_conversion() {
        let color = Color::from_rgb(255, 0, 0); // Red
        let (c, m, y, k) = color.to_cmyk();
        assert!((c - 0.0).abs() < 1.0);
        assert!((m - 100.0).abs() < 1.0);
        assert!((y - 100.0).abs() < 1.0);
        assert!((k - 0.0).abs() < 1.0);
    }

    #[test]
    fn test_cmyk_black() {
        let color = Color::from_rgb(0, 0, 0);
        let (_c, _m, _y, k) = color.to_cmyk();
        assert_eq!(k, 100.0);
    }

    #[test]
    fn test_lab_conversion() {
        let color = Color::from_rgb(255, 255, 255);
        let (l, _, _) = color.to_lab();
        assert!(l > 99.0); // White should have high L value
    }

    #[test]
    fn test_oklab_conversion() {
        let color = Color::from_rgb(128, 128, 128);
        let (l, a, b) = color.to_oklab();
        assert!(l > 0.0 && l < 1.0);
        assert!(a.abs() < 0.1); // Gray should have near-zero chroma
        assert!(b.abs() < 0.1);
    }

    #[test]
    fn test_all_formats() {
        let color = Color::from_hex("#FF5733").unwrap();
        let formats = color.to_all_formats();
        
        assert_eq!(formats.hex, "#FF5733");
        assert!(formats.rgb.starts_with("rgb("));
        assert!(formats.hsl.starts_with("hsl("));
        assert!(formats.hsv.starts_with("hsv("));
        assert!(formats.cmyk.starts_with("cmyk("));
        assert!(formats.lab.starts_with("lab("));
        assert!(formats.lch.starts_with("lch("));
        assert!(formats.oklab.starts_with("oklab("));
        assert!(formats.oklch.starts_with("oklch("));
    }

    #[test]
    fn test_color_display() {
        let color = Color::from_rgb(255, 87, 51);
        assert_eq!(format!("{}", color), "#FF5733");
    }

    #[test]
    fn test_color_default() {
        let color = Color::default();
        assert_eq!(color.to_hex(), "#000000");
    }

    #[test]
    fn test_invalid_hex() {
        assert!(Color::from_hex("#ZZZ").is_err());
        assert!(Color::from_hex("#12345").is_err());
        assert!(Color::from_hex("").is_err());
    }

    #[test]
    fn test_rgb_clamping() {
        // Test with valid RGB values
        let color = Color::from_rgb(255, 0, 128);
        let (r, g, b) = color.to_rgb();
        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 128);
    }

    #[test]
    fn test_roundtrip_hex_rgb() {
        let original = Color::from_hex("#ABCDEF").unwrap();
        let (r, g, b) = original.to_rgb();
        let roundtrip = Color::from_rgb(r, g, b);
        assert_eq!(original.to_hex(), roundtrip.to_hex());
    }

    #[test]
    fn test_roundtrip_rgb_hsl() {
        let original = Color::from_rgb(200, 100, 50);
        let (h, s, l) = original.to_hsl();
        let roundtrip = Color::from_hsl(h, s, l);
        let (r1, g1, b1) = original.to_rgb();
        let (r2, g2, b2) = roundtrip.to_rgb();
        
        // Allow small rounding differences
        assert!((r1 as i32 - r2 as i32).abs() <= 2);
        assert!((g1 as i32 - g2 as i32).abs() <= 2);
        assert!((b1 as i32 - b2 as i32).abs() <= 2);
    }
}

#[cfg(test)]
mod css_color_tests {
    use cascolor::color::formats::{find_css_color_by_name, find_css_color_by_hex};

    #[test]
    fn test_find_css_color_red() {
        assert_eq!(find_css_color_by_name("red"), Some("#FF0000"));
    }

    #[test]
    fn test_find_css_color_case_insensitive() {
        assert_eq!(find_css_color_by_name("RED"), Some("#FF0000"));
        assert_eq!(find_css_color_by_name("Red"), Some("#FF0000"));
        assert_eq!(find_css_color_by_name("rEd"), Some("#FF0000"));
    }

    #[test]
    fn test_find_css_color_by_hex() {
        assert_eq!(find_css_color_by_hex("#FF0000"), Some("red"));
        assert_eq!(find_css_color_by_hex("#ff0000"), Some("red"));
    }

    #[test]
    fn test_find_invalid_css_color() {
        assert_eq!(find_css_color_by_name("notacolor"), None);
    }

    #[test]
    fn test_find_invalid_hex() {
        assert_eq!(find_css_color_by_hex("#123456"), None);
    }

    #[test]
    fn test_common_colors() {
        assert_eq!(find_css_color_by_name("blue"), Some("#0000FF"));
        assert_eq!(find_css_color_by_name("green"), Some("#008000"));
        assert_eq!(find_css_color_by_name("white"), Some("#FFFFFF"));
        assert_eq!(find_css_color_by_name("black"), Some("#000000"));
    }

    #[test]
    fn test_color_count() {
        use cascolor::color::formats::CSS_COLORS;
        assert_eq!(CSS_COLORS.len(), 147);
    }
}
