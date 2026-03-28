//! Core error types for cTUI
//!
//! This module defines the error types used throughout the cTUI core library.

use std::fmt;

/// Errors that can occur in cTUI core operations
#[derive(Debug)]
pub enum CoreError {
    /// Buffer operation error
    BufferError(String),
    /// Component lifecycle error
    ComponentError(String),
    /// Rendering error
    RenderError(String),
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BufferError(msg) => write!(f, "Buffer error: {msg}"),
            Self::ComponentError(msg) => write!(f, "Component error: {msg}"),
            Self::RenderError(msg) => write!(f, "Render error: {msg}"),
        }
    }
}

impl std::error::Error for CoreError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = CoreError::BufferError("test".to_string());
        assert_eq!(format!("{err}"), "Buffer error: test");

        let err = CoreError::ComponentError("failed".to_string());
        assert_eq!(format!("{err}"), "Component error: failed");

        let err = CoreError::RenderError("bad render".to_string());
        assert_eq!(format!("{err}"), "Render error: bad render");
    }

    #[test]
    fn test_error_is_error() {
        let err = CoreError::BufferError("test".to_string());
        let _: &dyn std::error::Error = &err;
    }
}
