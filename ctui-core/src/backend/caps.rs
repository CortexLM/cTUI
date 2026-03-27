//! Terminal capability detection
//!
//! This module provides detection of terminal capabilities including
//! synchronized output (DEC mode 2026) support for flicker-free rendering.

/// Terminal capabilities detected from the environment
#[derive(Debug, Clone)]
pub struct Capabilities {
    supports_synchronized_output: bool,
}

impl Capabilities {
    /// Detects terminal capabilities from the current environment
    ///
    /// This examines environment variables to determine which features
    /// the current terminal emulator supports.
    pub fn detect() -> Self {
        let supports_sync = Self::detect_synchronized_output();
        Self {
            supports_synchronized_output: supports_sync,
        }
    }

    /// Creates a Capabilities struct with all features disabled
    ///
    /// Useful for testing or when running in a known-limited environment.
    pub fn none() -> Self {
        Self {
            supports_synchronized_output: false,
        }
    }

    /// Creates a Capabilities struct with all features enabled
    ///
    /// Useful for testing when you want to assume full terminal support.
    pub fn all() -> Self {
        Self {
            supports_synchronized_output: true,
        }
    }

    /// Detects if synchronized output (DEC mode 2026) is supported
    ///
    /// Synchronized output prevents flickering by buffering all updates
    /// and presenting them atomically to the user.
    ///
    /// # Supported Terminals
    /// - Alacritty (version 0.13+)
    /// - Kitty
    /// - WezTerm
    /// - iTerm2
    /// - Windows Terminal
    /// - Foot
    fn detect_synchronized_output() -> bool {
        // Check terminal-specific environment variables first
        // These are more reliable than TERM matching
        if Self::check_terminal_env_vars() {
            return true;
        }

        // Fall back to TERM-based detection
        if let Ok(term) = std::env::var("TERM") {
            match term.as_str() {
                "xterm-kitty" => return true,
                "wezterm" => return true,
                "foot" | "foot-extra" => return true,
                "alacritty" => return true,
                _ => {}
            }
        }

        // Check for xterm-256color which is commonly set
        // This is less reliable but covers most modern terminals
        if Self::is_modern_terminal() {
            return true;
        }

        false
    }

    /// Checks for known terminal-specific environment variables
    fn check_terminal_env_vars() -> bool {
        // Alacritty
        if std::env::var("ALACRITTY_WINDOW_ID").is_ok() {
            return true;
        }

        // Kitty
        if std::env::var("KITTY_WINDOW_ID").is_ok() {
            return true;
        }

        // WezTerm
        if std::env::var("WEZTERM_EXECUTABLE").is_ok() {
            return true;
        }

        // Windows Terminal
        if std::env::var("WT_SESSION").is_ok() {
            return true;
        }

        // iTerm2
        if std::env::var("ITERM_SESSION_ID").is_ok() {
            return true;
        }

        // Kitty (alternate check)
        if std::env::var("KITTY_PID").is_ok() {
            return true;
        }

        // Generic terminal that might support DEC private mode 2026
        if std::env::var("TERM_PROGRAM").is_ok() {
            if let Ok(term_program) = std::env::var("TERM_PROGRAM") {
                // iTerm2 or other macOS terminals
                if matches!(term_program.as_str(), "iTerm.app" | "WezTerm") {
                    return true;
                }
            }
        }

        false
    }

    /// Additional heuristics for modern terminals that might support sync
    fn is_modern_terminal() -> bool {
        // If COLORTERM is set to truecolor or 24bit, it's likely a modern terminal
        if let Ok(colorterm) = std::env::var("COLORTERM") {
            if matches!(colorterm.as_str(), "truecolor" | "24bit") {
                // Check if TERM is also set to a known good value
                if let Ok(term) = std::env::var("TERM") {
                    if term.contains("256color") {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Returns true if synchronized output is supported
    ///
    /// When true, you can use begin_synchronized_output/end_synchronized_output
    /// to prevent flickering during screen updates.
    pub fn supports_synchronized_output(&self) -> bool {
        self.supports_synchronized_output
    }
}

impl Default for Capabilities {
    fn default() -> Self {
        Self::detect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capabilities_detection() {
        // Smoke test - should not panic
        let caps = Capabilities::detect();
        // Result depends on environment, just verify it runs
        let _ = caps.supports_synchronized_output();
    }

    #[test]
    fn test_capabilities_none() {
        let caps = Capabilities::none();
        assert!(!caps.supports_synchronized_output());
    }

    #[test]
    fn test_capabilities_all() {
        let caps = Capabilities::all();
        assert!(caps.supports_synchronized_output());
    }

    #[test]
    fn test_capabilities_default() {
        // Default should call detect()
        let caps = Capabilities::default();
        let detect_caps = Capabilities::detect();
        assert_eq!(
            caps.supports_synchronized_output(),
            detect_caps.supports_synchronized_output()
        );
    }
}
