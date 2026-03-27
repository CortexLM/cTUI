//! Integration tests for the theming system

use ctui_core::backend::test::TestBackend;
use ctui_core::{Buffer, Rect, Terminal, Widget};
use ctui_theme::{
    audit_accessibility, BorderStyle, Color, ColorMode, Easing, FontWeight, NamedColor, Rgb,
    Spacing, Style, Theme, ThemeInterpolator, ThemeLoader, ThemeTransition, ThemeValidator,
    Typography,
};

#[test]
fn test_theme_load_and_apply() {
    let theme = Theme::dark();
    assert_eq!(theme.name, "dark");

    let style = Style::new()
        .fg(theme.colors.primary)
        .bg(theme.colors.background);
    assert!(!style.fg.is_default());
}

#[test]
fn test_all_builtin_themes() {
    for name in Theme::available_themes() {
        let theme = Theme::by_name(name).unwrap_or_else(|| panic!("Theme {} should load", name));
        assert!(!theme.name.is_empty());
        assert!(theme.color("primary").is_some());
        assert!(theme.color("text").is_some());
    }
}

#[test]
fn test_theme_color_lookup() {
    let theme = Theme::nord();

    assert!(theme.color("primary").is_some());
    assert!(theme.color("secondary").is_some());
    assert!(theme.color("text").is_some());
    assert!(theme.color("error").is_some());
    assert!(theme.color("success").is_some());

    assert!(theme.color("nonexistent").is_none());
}

#[test]
fn test_theme_custom_colors() {
    let mut theme = Theme::new("custom");
    theme
        .custom_colors
        .insert("brand".to_string(), Color::rgb(255, 100, 50));

    assert!(theme.color("brand").is_some());
    let brand = theme.color("brand").unwrap();
    if let Color::Rgb(rgb) = brand {
        assert_eq!(rgb.r, 255);
        assert_eq!(rgb.g, 100);
        assert_eq!(rgb.b, 50);
    }
}

#[test]
fn test_theme_spacing() {
    let theme = Theme::default();

    let spacing_xs = theme.spacing("xs");
    assert_eq!(spacing_xs.top, 2);

    let spacing_md = theme.spacing("md");
    assert_eq!(spacing_md.top, 8);
}

#[test]
fn test_theme_style_lookup() {
    let theme = Theme::default();

    assert!(theme.style("button").is_some());
    assert!(theme.style("input").is_some());
    assert!(theme.style("label").is_some());
    assert!(theme.style("panel").is_some());

    assert!(theme.style("nonexistent").is_none());
}

#[test]
fn test_theme_roundtrip() {
    let original = Theme::nord();
    let toml = ThemeLoader::to_string(&original).unwrap();
    let loaded = ThemeLoader::from_str(&toml).unwrap();

    assert_eq!(original.name, loaded.name);
    assert_eq!(original.colors.primary, loaded.colors.primary);
}

#[test]
fn test_theme_interpolation() {
    let transition = ThemeTransition::fast();
    assert!(transition.duration.as_millis() < 200);

    let from = Theme::dark();
    let to = Theme::light();
    let interpolator = ThemeInterpolator::new(from, to, transition).with_progress(0.5);
    let interpolated = interpolator.interpolate();

    assert!(!interpolated.name.is_empty());
}

#[test]
fn test_color_mode_toggle() {
    let mode = ColorMode::Dark;
    let toggled = mode.toggle();
    assert!(toggled.is_light());

    let back = toggled.toggle();
    assert!(back.is_dark());
}

#[test]
fn test_accessibility_audit() {
    let theme = Theme::gruvbox();
    let audit = audit_accessibility(&theme);
    assert!(audit.score > 0);
}

#[test]
fn test_theme_validation() {
    let theme = Theme::nord();
    let validator = ThemeValidator::new().check_contrast(false);
    let result = validator.validate(&theme);
    assert!(result.is_valid());
}

#[test]
fn test_font_weight_values() {
    assert_eq!(FontWeight::Thin.weight(), 100);
    assert_eq!(FontWeight::ExtraLight.weight(), 200);
    assert_eq!(FontWeight::Light.weight(), 300);
    assert_eq!(FontWeight::Normal.weight(), 400);
    assert_eq!(FontWeight::Medium.weight(), 500);
    assert_eq!(FontWeight::SemiBold.weight(), 600);
    assert_eq!(FontWeight::Bold.weight(), 700);
    assert_eq!(FontWeight::ExtraBold.weight(), 800);
    assert_eq!(FontWeight::Black.weight(), 900);
}

#[test]
fn test_typography_builder() {
    let typo = Typography::new("Fira Code", 14, 20)
        .weight(FontWeight::Medium)
        .letter_spacing(1);

    assert_eq!(typo.family, "Fira Code");
    assert_eq!(typo.size, 14);
    assert_eq!(typo.line_height, 20);
    assert_eq!(typo.weight, FontWeight::Medium);
    assert_eq!(typo.letter_spacing, 1);
}

#[test]
fn test_style_creation() {
    let style = Style::new()
        .fg(Color::named(NamedColor::Red))
        .bg(Color::named(NamedColor::Black))
        .padding(Spacing::all(4));

    assert!(!style.fg.is_default());
    assert_eq!(style.padding.top, 4);
}

#[test]
fn test_style_with_border() {
    let style = Style::new()
        .border(BorderStyle::Rounded)
        .padding(Spacing::all(2));

    assert_eq!(style.border_style, BorderStyle::Rounded);
}

#[test]
fn test_color_creation() {
    let named = Color::named(NamedColor::Blue);
    assert!(matches!(named, Color::Named(_)));

    let rgb = Color::rgb(255, 128, 64);
    if let Color::Rgb(rgb) = rgb {
        assert_eq!(rgb.r, 255);
        assert_eq!(rgb.g, 128);
        assert_eq!(rgb.b, 64);
    }
}

#[test]
fn test_easing_variants() {
    let linear = Easing::Linear;
    let ease_in = Easing::EaseIn;
    let ease_out = Easing::EaseOut;

    assert!((linear.apply(0.5) - 0.5).abs() < 0.001);
    assert!(ease_in.apply(0.5) < 0.5);
    assert!(ease_out.apply(0.5) > 0.5);
}

#[test]
fn test_theme_transition_presets() {
    let fast = ThemeTransition::fast();
    assert!(fast.duration.as_millis() < 200);

    let default = ThemeTransition::default();
    assert!(default.duration.as_millis() >= 200);

    let slow = ThemeTransition::slow();
    assert!(slow.duration.as_millis() >= 300);
}

struct ThemedWidget {
    theme: Theme,
}

impl Widget for ThemedWidget {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let text = &self.theme.name;
        for (i, ch) in text.chars().take(area.width as usize).enumerate() {
            if let Some(cell) = buffer.get_mut(area.x + i as u16, area.y) {
                cell.symbol = ch.to_string();
            }
        }
    }
}

#[test]
fn test_theme_render_integration() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let theme = Theme::dracula();
    let widget = ThemedWidget { theme };

    terminal
        .draw(|f| {
            widget.render(Rect::new(0, 0, 10, 1), f.buffer_mut());
        })
        .unwrap();

    let backend = terminal.backend();
    assert_eq!(backend.buffer()[(0, 0)].symbol, "d");
}

#[test]
fn test_multiple_themes_in_sequence() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let themes = [
        Theme::dark(),
        Theme::light(),
        Theme::nord(),
        Theme::dracula(),
    ];

    for theme in themes {
        terminal.clear().unwrap();

        let widget = ThemedWidget { theme };
        terminal
            .draw(|f| {
                widget.render(Rect::new(0, 0, 10, 1), f.buffer_mut());
            })
            .unwrap();
    }
}

#[test]
fn test_style_presets_with_theme() {
    let theme = Theme::dracula();

    let button_style = Style::new()
        .fg(theme.colors.text)
        .bg(theme.colors.primary)
        .padding(Spacing::horizontal_vertical(4, 2))
        .border(BorderStyle::Rounded);

    assert!(!button_style.fg.is_default());
    assert_eq!(button_style.padding.left, 4);
    assert_eq!(button_style.padding.top, 2);
}

#[test]
fn test_theme_component_styles() {
    let mut theme = Theme::default();
    theme.components.button = Style::new().fg(Color::named(NamedColor::Yellow));
    theme.components.input = Style::new().fg(Color::named(NamedColor::Cyan));

    let button_style = theme.style("button").unwrap();
    let input_style = theme.style("input").unwrap();

    assert!(!button_style.fg.is_default());
    assert!(!input_style.fg.is_default());
}

#[test]
fn test_color_palette_semantic_colors() {
    let theme = Theme::tokyo_night();

    assert!(theme.color("success").is_some());
    assert!(theme.color("warning").is_some());
    assert!(theme.color("error").is_some());
    assert!(theme.color("info").is_some());
}

#[test]
fn test_theme_by_name_case_sensitive() {
    assert!(Theme::by_name("dracula").is_some());
    assert!(Theme::by_name("Dracula").is_none());
    assert!(Theme::by_name("DRACULA").is_none());
}

#[test]
fn test_empty_custom_colors() {
    let theme = Theme::dark();
    assert!(theme.custom_colors.is_empty());
    assert!(theme.color("brand").is_none());
}
