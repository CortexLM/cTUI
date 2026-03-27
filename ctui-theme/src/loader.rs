//! Theme file loader for TOML format

use crate::theme::Theme;
use std::path::Path;
use thiserror::Error;

/// Errors that can occur during theme loading
#[derive(Debug, Error)]
pub enum ThemeLoadError {
    /// IO error reading the file
    #[error("Failed to read theme file: {0}")]
    Io(#[from] std::io::Error),

    /// TOML parsing error
    #[error("Failed to parse theme TOML: {0}")]
    Parse(#[from] toml::de::Error),

    /// Theme validation error
    #[error("Invalid theme: {0}")]
    Validation(String),
}

/// Theme loader for TOML files
pub struct ThemeLoader;

impl ThemeLoader {
    /// Loads a theme from a TOML file
    ///
    /// # Arguments
    /// * `path` - Path to the TOML theme file
    ///
    /// # Errors
    /// Returns an error if the file cannot be read or parsed
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Theme, ThemeLoadError> {
        let content = std::fs::read_to_string(path)?;
        Self::from_str(&content)
    }

    /// Loads a theme from a TOML string
    ///
    /// # Arguments
    /// * `content` - TOML content as a string
    ///
    /// # Errors
    /// Returns an error if the content cannot be parsed
    pub fn from_str(content: &str) -> Result<Theme, ThemeLoadError> {
        let theme: Theme = toml::from_str(content)?;
        Self::validate(&theme)?;
        Ok(theme)
    }

    /// Saves a theme to a TOML file
    ///
    /// # Arguments
    /// * `theme` - The theme to save
    /// * `path` - Path to save the theme file
    ///
    /// # Errors
    /// Returns an error if the file cannot be written
    pub fn to_file<P: AsRef<Path>>(theme: &Theme, path: P) -> Result<(), ThemeLoadError> {
        let content = Self::to_string(theme)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Converts a theme to a TOML string
    ///
    /// # Arguments
    /// * `theme` - The theme to convert
    ///
    /// # Errors
    /// Returns an error if serialization fails (should not happen with valid themes)
    pub fn to_string(theme: &Theme) -> Result<String, ThemeLoadError> {
        toml::to_string_pretty(theme).map_err(|e| ThemeLoadError::Validation(e.to_string()))
    }

    /// Validates a theme for common issues
    fn validate(theme: &Theme) -> Result<(), ThemeLoadError> {
        if theme.name.is_empty() {
            return Err(ThemeLoadError::Validation(
                "Theme name cannot be empty".to_string(),
            ));
        }
        Ok(())
    }

    /// Loads a theme by name from standard theme directories
    ///
    /// Searches in the following order:
    /// 1. Current directory: `./{name}.toml`
    /// 2. Config directory: `~/.config/ctui/themes/{name}.toml`
    /// 3. System directory: `/usr/share/ctui/themes/{name}.toml`
    ///
    /// # Arguments
    /// * `name` - Theme name (without .toml extension)
    ///
    /// # Errors
    /// Returns an error if the theme cannot be found or loaded
    pub fn load_by_name(name: &str) -> Result<Theme, ThemeLoadError> {
        let filename = format!("{name}.toml");

        let search_paths = vec![
            Path::new(&filename).to_path_buf(),
            dirs::config_dir()
                .map(|p| p.join("ctui").join("themes").join(&filename))
                .unwrap_or_default(),
            Path::new("/usr/share/ctui/themes").join(&filename),
        ];

        for path in search_paths {
            if path.exists() {
                return Self::from_file(&path);
            }
        }

        Err(ThemeLoadError::Validation(format!(
            "Theme '{name}' not found in any standard location"
        )))
    }

    /// Lists available themes from standard directories
    ///
    /// # Returns
    /// A vector of theme names found in standard directories
    #[must_use]
    pub fn list_available() -> Vec<String> {
        use std::path::PathBuf;
        let mut themes = std::collections::HashSet::new();

        let search_dirs: Vec<PathBuf> = vec![
            PathBuf::from("."),
            dirs::config_dir()
                .map(|p| p.join("ctui").join("themes"))
                .unwrap_or_default(),
            PathBuf::from("/usr/share/ctui/themes"),
        ];

        for dir in search_dirs {
            if dir.exists() {
                if let Ok(entries) = std::fs::read_dir(&dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.extension().is_some_and(|ext| ext == "toml") {
                            if let Some(stem) = path.file_stem() {
                                if let Some(name) = stem.to_str() {
                                    themes.insert(name.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }

        let mut themes: Vec<_> = themes.into_iter().collect();
        themes.sort();
        themes
    }
}

mod dirs {
    use std::path::PathBuf;

    pub fn config_dir() -> Option<PathBuf> {
        #[cfg(unix)]
        {
            std::env::var_os("XDG_CONFIG_HOME")
                .map(PathBuf::from)
                .or_else(|| {
                    std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".config"))
                })
        }
        #[cfg(windows)]
        {
            std::env::var_os("APPDATA")
                .map(PathBuf::from)
                .map(|p| p.join("ctui"))
        }
        #[cfg(not(any(unix, windows)))]
        {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;

    #[test]
    fn test_load_from_string() {
        let toml = r#"
            name = "test_theme"
            author = "Test Author"
            description = "A test theme"
            
            [colors]
            primary = "blue"
        "#;

        let result = ThemeLoader::from_str(toml);
        assert!(result.is_ok());
        let theme = result.unwrap();
        assert_eq!(theme.name, "test_theme");
        assert_eq!(theme.author, "Test Author");
    }

    #[test]
    fn test_load_empty_name_fails() {
        let toml = r#"
            name = ""
        "#;

        let result = ThemeLoader::from_str(toml);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ThemeLoadError::Validation(_)));
    }

    #[test]
    fn test_serialize_deserialize() {
        let original = Theme::dracula();
        let toml_str = ThemeLoader::to_string(&original).unwrap();
        let loaded = ThemeLoader::from_str(&toml_str).unwrap();
        assert_eq!(original.name, loaded.name);
    }

    #[test]
    fn test_complex_theme_parsing() {
        let toml = r#"
            name = "complex"
            author = "Test"
            
            [colors]
            primary = { r = 255, g = 128, b = 64 }
            background = { indexed = 235 }
            
            [spacing]
            xs = 2
            sm = 4
            md = 8
            lg = 16
            xl = 24
            x2 = 32
            x3 = 48
            
            [typography]
            family = "Fira Code"
            size = 14
            line_height = 18
            
            [borders]
            style = "rounded"
            radius = 4
            width = 2
            
            [components.button]
            fg = "white"
            bg = { r = 0, g = 128, b = 255 }
            [components.button.padding]
            top = 2
            right = 4
            bottom = 2
            left = 4
        "#;

        let theme = ThemeLoader::from_str(toml).unwrap();
        assert_eq!(theme.name, "complex");
        assert_eq!(theme.spacing.md, 8);
        assert_eq!(theme.typography.family, "Fira Code");
        assert_eq!(theme.borders.style, crate::style::BorderStyle::Rounded);
    }

    #[test]
    fn test_theme_to_string() {
        let theme = Theme {
            name: "minimal".to_string(),
            ..Theme::default()
        };

        let result = ThemeLoader::to_string(&theme);
        assert!(result.is_ok());
        let toml = result.unwrap();
        assert!(toml.contains("name = \"minimal\""));
    }

    #[test]
    fn test_parse_error() {
        let invalid_toml = "name = [";
        let result = ThemeLoader::from_str(invalid_toml);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ThemeLoadError::Parse(_)));
    }

    #[test]
    fn test_file_not_found() {
        let result = ThemeLoader::from_file("/nonexistent/path/theme.toml");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ThemeLoadError::Io(_)));
    }
}
