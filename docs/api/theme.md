# API Reference - Theme

Theming system for cTUI applications.

## Overview

cTUI provides a comprehensive theming system with preset themes, custom tokens, and accessibility validation.

```rust
use ctui_theme::{Theme, ThemeLoader, Style, Color};

// Use a preset theme
let theme = Theme::dracula();
let theme = Theme::nord();
let theme = Theme::tokyo_night();

// Or a built-in theme
let theme = Theme::dark();
let theme = Theme::light();

// Load from TOML
let theme = ThemeLoader::from_str(r#"
    name = "my_theme"
    [colors]
    primary = { r = 100, g = 149, b = 237 }
"#).unwrap();
```

---

## Theme

Main theme structure.

### Preset Themes

```rust
// Dark themes
Theme::dark()           // Default dark
Theme::dracula()        // Dracula
Theme::nord()           // Nord
Theme::tokyo_night()    // Tokyo Night
Theme::gruvbox()        // Gruvbox
Theme::catppuccin()     // Catppuccin Mocha
Theme::one_dark()       // One Dark

// Light themes
Theme::light()          // Default light
Theme::gruvbox_light()  // Gruvbox Light
Theme::catppuccin_latte() // Catppuccin Latte
```

### Theme Fields

```rust
pub struct Theme {
    pub name: String,
    pub colors: ColorPalette,
    pub spacing: SpacingTokens,
    pub typography: Typography,
    pub components: ComponentStyles,
    pub elevation: ElevationTokens,
    pub custom_colors: HashMap<String, Color>,
}
```

### ColorPalette

```rust
pub struct ColorPalette {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub background: Color,
    pub surface: Color,
    pub text: Color,
    pub text_muted: Color,
    pub border: Color,
    pub error: Color,
    pub warning: Color,
    pub success: Color,
    pub info: Color,
}
```

### Methods

```rust
let theme = Theme::dark();

// Access colors
let primary = theme.colors.primary;
let bg = theme.colors.background;

// Custom colors
let brand = theme.color("brand");

// Available themes
for name in Theme::available_themes() {
    println!("Theme: {}", name);
}

// Load by name
let theme = Theme::by_name("nord").unwrap();
```

---

## Color Types

### Color

```rust
pub enum Color {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Gray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    White,
    Indexed(u8),         // 256-color
    Rgb(u8, u8, u8),     // True color
}
```

### Creating Colors

```rust
use ctui_theme::Color;

// Named
let red = Color::Red;
let cyan = Color::Cyan;

// 256-color
let orange = Color::Indexed(208);

// RGB
let purple = Color::Rgb(128, 0, 255);
let purple = Color::rgb(128, 0, 255);  // Helper

// From hex
let blue = Color::from_hex("#1E90FF").unwrap();
```

### NamedColor

```rust
pub enum NamedColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Gray,
    // ... 256 colors
}
```

---

## Style

Apply styling to widgets.

```rust
use ctui_theme::{Style, Modifier, Spacing, BorderStyle};

let style = Style::new()
    .fg(Color::Cyan)
    .bg(Color::Rgb(30, 30, 40))
    .add_modifier(Modifier::BOLD)
    .padding(Spacing::all(2))
    .border(BorderStyle::Rounded);
```

### Style Fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `fg` | `Color` | Reset | Foreground color |
| `bg` | `Color` | Reset | Background color |
| `modifiers` | `Modifier` | None | Text modifiers |
| `padding` | `Spacing` | zero | Padding |
| `margin` | `Spacing` | zero | Margin |
| `border_style` | `BorderStyle` | Plain | Border style |

### Modifier

```rust
bitflags! {
    pub struct Modifier: u16 {
        const BOLD = 0b000000001;
        const DIM = 0b000000010;
        const ITALIC = 0b000000100;
        const UNDERLINED = 0b000001000;
        const SLOW_BLINK = 0b000010000;
        const RAPID_BLINK = 0b000100000;
        const REVERSED = 0b001000000;
        const HIDDEN = 0b010000000;
        const CROSSED_OUT = 0b100000000;
    }
}
```

### Spacing

```rust
let spacing = Spacing::all(4);                 // All sides
let spacing = Spacing::horizontal_vertical(2, 1); // H/V
let spacing = Spacing {
    top: 1,
    right: 2,
    bottom: 1,
    left: 2,
};
```

### BorderStyle

```rust
pub enum BorderStyle {
    None,
    Plain,
    Rounded,
    Double,
    Thick,
    Custom(BorderChars),
}

let chars = BorderChars {
    top_left: '╭',
    top_right: '╮',
    bottom_left: '╰',
    bottom_right: '╯',
    horizontal: '─',
    vertical: '│',
};
```

---

## Typography

Font and text settings.

```rust
use ctui_theme::{Typography, FontWeight, FontRendering};

let typo = Typography::new("Fira Code", 14, 20)
    .weight(FontWeight::Medium)
    .letter_spacing(1)
    .rendering(FontRendering::Crisp);
```

### FontWeight

```rust
pub enum FontWeight {
    Thin,       // 100
    ExtraLight, // 200
    Light,      // 300
    Regular,    // 400
    Medium,     // 500
    SemiBold,   // 600
    Bold,       // 700
    ExtraBold,  // 800
    Black,      // 900
}
```

### FontRendering

```rust
pub enum FontRendering {
    Auto,
    Crisp,
    Smooth,
}
```

---

## Spacing Tokens

Semantic spacing values.

```rust
pub struct SpacingTokens {
    pub xs: u16,    // 2
    pub sm: u16,    // 4
    pub md: u16,    // 8
    pub lg: u16,    // 12
    pub xl: u16,    // 16
    pub xxl: u16,   // 24
}

let theme = Theme::dark();
let padding = theme.spacing.md;  // 8
```

---

## Elevation

Z-index and shadow system.

### Elevation Levels

```rust
pub enum Elevation {
    Level0,  // Ground level
    Level1,  // Slightly raised
    Level2,  // Cards
    Level3,  // Modals
    Level4,  // Popovers
    Level5,  // Tooltips
}

let elevation = Elevation::Level2;
let z = elevation.z_index();     // Z-index value
let shadow = elevation.shadow(); // Shadow config
```

### Shadow

```rust
pub struct Shadow {
    pub offset_x: i16,
    pub offset_y: i16,
    pub blur: u8,
    pub color: Color,
    pub opacity: f32,
}
```

### Z-Index

```rust
use ctui_theme::z_index;

let z = z_index::BASE;      // 0
let z = z_index::NORMAL;    // 1
let z = z_index::ELEVATED;  // 10
let z = z_index::MODAL;     // 100
let z = z_index::TOOLTIP;   // 200
```

---

## Component Themes

Each component has its own theme.

```rust
use ctui_theme::{
    BlockTheme, ButtonTheme, InputTheme, ListTheme,
    ModalTheme, ParagraphTheme, ProgressTheme, ScrollbarTheme,
    TableTheme, TabsTheme, TextAlignment,
};

// Get component theme
let theme = Theme::dark();
let block_theme = BlockTheme::default();

// Apply component theme
let style = block_theme.style(&theme);
```

### ButtonTheme

```rust
let button_theme = ButtonTheme {
    sizes: ButtonSizes {
        sm: Style::new().padding(Spacing::all(2)),
        md: Style::new().padding(Spacing::all(4)),
        lg: Style::new().padding(Spacing::all(6)),
    },
    ..Default::default()
};
```

---

## Theme Loader

Load and save themes.

### Loading

```rust
use ctui_theme::ThemeLoader;

// From string
let theme = ThemeLoader::from_str(toml_string)?;

// From file
let theme = ThemeLoader::from_file("theme.toml")?;

// From reader
let theme = ThemeLoader::from_reader(reader)?;
```

### Saving

```rust
// To string
let toml = ThemeLoader::to_string(&theme)?;

// To file
ThemeLoader::to_file(&theme, "theme.toml")?;
```

### Theme File Format

```toml
name = "my_custom_theme"

[colors]
primary = { r = 100, g = 149, b = 237 }
secondary = { r = 150, g = 100, b = 200 }
background = { r = 30, g = 30, b = 40 }
text = { r = 255, g = 255, b = 255 }

[spacing]
xs = 2
sm = 4
md = 8
lg = 12
xl = 16

[typography]
family = "Fira Code"
size = 14
line_height = 20

[components.button]
border_style = "Rounded"

[custom_colors]
brand = { r = 255, g = 100, b = 50 }
accent = { r = 0, g = 200, b = 150 }
```

---

## Theme Transitions

Animate between themes.

### ThemeTransition

```rust
use ctui_theme::{ThemeTransition, ThemeInterpolator, ColorMode};

let transition = ThemeTransition::fast();     // 100ms
let transition = ThemeTransition::normal();   // 300ms
let transition = ThemeTransition::slow();     // 500ms

let from = Theme::dark();
let to = Theme::light();

let interpolator = ThemeInterpolator::new(from, to, transition)
    .with_progress(0.5);  // 50% transition

let interpolated = interpolator.interpolate();
```

### ColorMode

```rust
let mode = ColorMode::Dark;
let toggled = mode.toggle();  // Light

let is_dark = mode.is_dark();
let is_light = mode.is_light();
```

---

## Theme Validation

Validate themes for accessibility.

### ThemeValidator

```rust
use ctui_theme::{ThemeValidator, ValidationResult};

let validator = ThemeValidator::new()
    .check_contrast(true)
    .check_color_blindness(false);

let result = validator.validate(&theme);

match result {
    ValidationResult::Valid => println!("Theme OK"),
    ValidationResult::Invalid(errors) => {
        for error in errors {
            eprintln!("Error: {:?}", error);
        }
    }
}
```

### AccessibilityAudit

```rust
use ctui_theme::{audit_accessibility, AccessibilityAudit};

let audit = audit_accessibility(&theme);
println!("Score: {}/100", audit.score);

for issue in audit.contrast_issues {
    println!("Contrast issue: {:?}", issue);
}
```

### Contrast Ratio

```rust
use ctui_theme::{contrast_ratio, relative_luminance, WcagLevel};

let ratio = contrast_ratio(Color::White, Color::Black);  // 21.0

// Check WCAG compliance
let level = WcagLevel::Aa;
let passes = ratio >= level.minimum_ratio();  // 4.5 for AA
```

---

## Examples

### Building a Custom Theme

```rust
use ctui_theme::{Theme, ColorPalette, SpacingTokens, Typography, FontWeight};

let mut theme = Theme::new("my_theme");
theme.colors = ColorPalette {
    primary: Color::rgb(100, 149, 237),
    secondary: Color::rgb(150, 100, 200),
    background: Color::rgb(30, 30, 40),
    surface: Color::rgb(40, 40, 50),
    text: Color::rgb(255, 255, 255),
    text_muted: Color::rgb(150, 150, 150),
    border: Color::rgb(60, 60, 70),
    error: Color::rgb(255, 100, 100),
    warning: Color::rgb(255, 200, 100),
    success: Color::rgb(100, 255, 150),
    info: Color::rgb(100, 200, 255),
    accent: Color::rgb(255, 100, 200),
};
theme.spacing = SpacingTokens {
    xs: 2,
    sm: 4,
    md: 8,
    lg: 16,
    xl: 24,
    xxl: 32,
};
theme.typography = Typography::new("JetBrains Mono", 14, 20)
    .weight(FontWeight::Regular);
```

### Using Theme in Components

```rust
impl Component for MyPanel {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let theme = Theme::nord();
        
        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme.colors.border))
            .style(Style::default().bg(theme.colors.surface));
        
        block.render(area, buf);
    }
}
```

### Theme Switching

```rust
struct App {
    theme: Theme,
    mode: ColorMode,
}

impl App {
    fn toggle_theme(&mut self) {
        self.mode = self.mode.toggle();
        self.theme = match self.mode {
            ColorMode::Dark => Theme::dark(),
            ColorMode::Light => Theme::light(),
        };
    }
}
```
