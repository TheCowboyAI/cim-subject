// Copyright 2025 Cowboy AI, LLC.

//! Error types for subject operations

use thiserror::Error;

/// Result type alias for subject operations
pub type Result<T> = std::result::Result<T, SubjectError>;

/// Errors that can occur during subject operations
#[derive(Error, Debug, Clone, PartialEq)]
pub enum SubjectError {
    /// Invalid subject format
    #[error("Invalid subject format: {0}")]
    InvalidFormat(String),

    /// Invalid pattern
    #[error("Invalid pattern: {0}")]
    InvalidPattern(String),

    /// Parse error
    #[error("Parse error: {0}")]
    ParseError(String),

    /// Permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// Translation error
    #[error("Translation error: {0}")]
    TranslationError(String),

    /// Composition error
    #[error("Composition error: {0}")]
    CompositionError(String),

    /// Validation error
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Not found
    #[error("Not found: {0}")]
    NotFound(String),
}

impl SubjectError {
    /// Create an invalid format error
    pub fn invalid_format(msg: impl Into<String>) -> Self {
        Self::InvalidFormat(msg.into())
    }

    /// Create an invalid pattern error
    pub fn invalid_pattern(msg: impl Into<String>) -> Self {
        Self::InvalidPattern(msg.into())
    }

    /// Create a parse error
    pub fn parse_error(msg: impl Into<String>) -> Self {
        Self::ParseError(msg.into())
    }

    /// Create a permission denied error
    pub fn permission_denied(msg: impl Into<String>) -> Self {
        Self::PermissionDenied(msg.into())
    }

    /// Create a translation error
    pub fn translation_error(msg: impl Into<String>) -> Self {
        Self::TranslationError(msg.into())
    }

    /// Create a composition error
    pub fn composition_error(msg: impl Into<String>) -> Self {
        Self::CompositionError(msg.into())
    }

    /// Create a validation error
    pub fn validation_error(msg: impl Into<String>) -> Self {
        Self::ValidationError(msg.into())
    }

    /// Create a not found error
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation_helpers() {
        // Test invalid_format
        let err = SubjectError::invalid_format("bad format");
        assert_eq!(err.to_string(), "Invalid subject format: bad format");
        assert!(matches!(err, SubjectError::InvalidFormat(_)));

        // Test invalid_pattern
        let err = SubjectError::invalid_pattern("bad pattern");
        assert_eq!(err.to_string(), "Invalid pattern: bad pattern");
        assert!(matches!(err, SubjectError::InvalidPattern(_)));

        // Test parse_error
        let err = SubjectError::parse_error("parse failed");
        assert_eq!(err.to_string(), "Parse error: parse failed");
        assert!(matches!(err, SubjectError::ParseError(_)));

        // Test permission_denied
        let err = SubjectError::permission_denied("access denied");
        assert_eq!(err.to_string(), "Permission denied: access denied");
        assert!(matches!(err, SubjectError::PermissionDenied(_)));

        // Test translation_error
        let err = SubjectError::translation_error("translation failed");
        assert_eq!(err.to_string(), "Translation error: translation failed");
        assert!(matches!(err, SubjectError::TranslationError(_)));

        // Test composition_error
        let err = SubjectError::composition_error("composition failed");
        assert_eq!(err.to_string(), "Composition error: composition failed");
        assert!(matches!(err, SubjectError::CompositionError(_)));

        // Test validation_error
        let err = SubjectError::validation_error("validation failed");
        assert_eq!(err.to_string(), "Validation error: validation failed");
        assert!(matches!(err, SubjectError::ValidationError(_)));

        // Test not_found
        let err = SubjectError::not_found("item not found");
        assert_eq!(err.to_string(), "Not found: item not found");
        assert!(matches!(err, SubjectError::NotFound(_)));
    }

    #[test]
    fn test_error_with_string_type() {
        // Test with String instead of &str
        let msg = String::from("dynamic message");
        let err = SubjectError::invalid_format(msg);
        assert_eq!(err.to_string(), "Invalid subject format: dynamic message");
    }

    #[test]
    fn test_error_equality() {
        let err1 = SubjectError::invalid_format("test");
        let err2 = SubjectError::invalid_format("test");
        let err3 = SubjectError::invalid_format("different");
        
        assert_eq!(err1, err2);
        assert_ne!(err1, err3);
    }

    #[test]
    fn test_error_clone() {
        let err1 = SubjectError::parse_error("original");
        let err2 = err1.clone();
        
        assert_eq!(err1, err2);
    }

    #[test]
    fn test_result_type_alias() {
        fn test_function() -> Result<String> {
            Err(SubjectError::not_found("test"))
        }
        
        let result = test_function();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Not found: test");
    }
}
