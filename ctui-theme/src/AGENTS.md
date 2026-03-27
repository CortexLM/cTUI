# ctui-theme - AGENTS.md

## OVERVIEW

Theming system with 8 built-in presets. Color types, component themes, elevation/shadows, accessibility validation, theme transitions.

## WHERE TO LOOK

| Need | File |
|------|------|
| Preset themes | `theme.rs` |
| Color types | `color.rs` |
| Component themes | `component.rs` |
| Elevation/shadows | `elevation.rs` |
| Theme loading | `loader.rs` |
| Style + modifiers | `style.rs` |
| Transitions | `transition.rs` |
| Accessibility | `validation.rs` |

## KEY TYPES

```rust
Theme                              // Full theme definition
Color, NamedColor, Rgb            // Color types
Style, Modifier, Spacing          // Styling primitives
ThemeLoader                        // TOML loading
AccessibilityAudit                 // WCAG contrast checking
Elevation, Shadow                  // Depth system

// Presets: Theme::dark(), Theme::dracula(), Theme::tokyo_night(), 
//          Theme::catppuccin(), Theme::nord(), Theme::gruvbox()
```

## CONVENTIONS

- Themes load from TOML files
- Component themes: BlockTheme, ButtonTheme, InputTheme...
- Accessibility audit returns score + issues
- Theme transitions for dark/light mode switching
