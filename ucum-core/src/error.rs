//! Enhanced error types for UCUM operations with detailed diagnostics.

#[cfg(feature = "std")]
extern crate std;

use crate::types::Dimension;

/// Source location information for parser errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    /// Start position in the source string
    pub start: usize,
    /// End position in the source string
    pub end: usize,
    /// The original source string for context
    pub source: String,
}

impl Span {
    /// Create a new span
    pub fn new(start: usize, end: usize, source: String) -> Self {
        Self { start, end, source }
    }
    
    /// Get the text content of this span
    pub fn text(&self) -> &str {
        &self.source[self.start..self.end.min(self.source.len())]
    }
    
    /// Get a visual representation of the error location
    pub fn display_error(&self, message: &str) -> String {
        let line_start = self.source[..self.start].rfind('\n').map(|pos| pos + 1).unwrap_or(0);
        let line_end = self.source[self.start..].find('\n').map(|pos| self.start + pos).unwrap_or(self.source.len());
        let line = &self.source[line_start..line_end];
        let column = self.start - line_start;
        
        format!("{}\n{}\n{}^{}", 
                message,
                line,
                " ".repeat(column),
                if self.end > self.start + 1 { "~".repeat(self.end - self.start - 1) } else { String::new() })
    }
}

/// Specific error kinds with detailed context
#[derive(Debug, Clone)]
pub enum ErrorKind {
    /// Parsing error with expected and found tokens
    ParseError { 
        expected: String, 
        found: String 
    },
    /// Dimension mismatch with specific dimensions
    DimensionMismatch { 
        expected: Dimension, 
        found: Dimension,
        operation: String,
    },
    /// Unit not found with similar suggestions
    UnitNotFound { 
        unit: String, 
        similar: Vec<String> 
    },
    /// Conversion error with detailed reason
    ConversionError { 
        from: String, 
        to: String, 
        reason: String 
    },
    /// Precision overflow during calculation
    PrecisionOverflow { 
        operation: String,
        value: String,
    },
    /// Invalid expression structure
    InvalidExpression { 
        reason: String 
    },
    /// Invalid property for unit validation
    InvalidProperty { 
        property: String, 
        available: Vec<String> 
    },
    /// Multiple slashes in expression
    MultipleSlash,
    /// Invalid percent placement
    InvalidPercentPlacement { 
        position: usize 
    },
    /// Special unit handling error
    SpecialUnitError { 
        unit: String, 
        reason: String 
    },
}

/// Enhanced UCUM error with detailed context and suggestions
#[derive(Debug, Clone)]
pub struct UcumError {
    /// The specific kind of error that occurred
    pub kind: ErrorKind,
    /// Primary error message
    pub message: String,
    /// Source location information (for parsing errors)
    pub span: Option<Span>,
    /// Suggested corrections or alternatives
    pub suggestions: Vec<String>,
    /// Additional context information
    pub context: Vec<String>,
}

impl UcumError {
    /// Create a new error with the given kind and message
    pub fn new(kind: ErrorKind, message: String) -> Self {
        Self {
            kind,
            message,
            span: None,
            suggestions: Vec::new(),
            context: Vec::new(),
        }
    }
    
    /// Add source location information
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }
    
    /// Add a suggestion for fixing the error
    pub fn with_suggestion<S: Into<String>>(mut self, suggestion: S) -> Self {
        self.suggestions.push(suggestion.into());
        self
    }
    
    /// Add multiple suggestions
    pub fn with_suggestions(mut self, suggestions: Vec<String>) -> Self {
        self.suggestions.extend(suggestions);
        self
    }
    
    /// Add contextual information
    pub fn with_context<S: Into<String>>(mut self, context: S) -> Self {
        self.context.push(context.into());
        self
    }
    
    /// Create a parse error
    pub fn parse_error(expected: &str, found: &str) -> Self {
        Self::new(
            ErrorKind::ParseError {
                expected: expected.to_string(),
                found: found.to_string(),
            },
            format!("Expected '{}', but found '{}'", expected, found),
        )
    }
    
    /// Create a dimension mismatch error
    pub fn dimension_mismatch(expected: Dimension, found: Dimension, operation: &str) -> Self {
        Self::new(
            ErrorKind::DimensionMismatch {
                expected,
                found,
                operation: operation.to_string(),
            },
            format!("Dimension mismatch in {}: expected {:?}, found {:?}", operation, expected, found),
        )
    }
    
    /// Create a unit not found error
    pub fn unit_not_found(unit: &str) -> Self {
        Self::new(
            ErrorKind::UnitNotFound {
                unit: unit.to_string(),
                similar: Vec::new(),
            },
            format!("Unknown unit: '{}'", unit),
        )
    }
    
    /// Create a conversion error
    pub fn conversion_error(from: &str, to: &str, reason: &str) -> Self {
        Self::new(
            ErrorKind::ConversionError {
                from: from.to_string(),
                to: to.to_string(),
                reason: reason.to_string(),
            },
            format!("Cannot convert from '{}' to '{}': {}", from, to, reason),
        )
    }
    
    /// Create a precision overflow error
    pub fn precision_overflow(operation: &str, value: &str) -> Self {
        Self::new(
            ErrorKind::PrecisionOverflow {
                operation: operation.to_string(),
                value: value.to_string(),
            },
            format!("Precision overflow during {}: {}", operation, value),
        )
    }
    
    /// Create an invalid expression error
    pub fn invalid_expression(reason: &str) -> Self {
        Self::new(
            ErrorKind::InvalidExpression {
                reason: reason.to_string(),
            },
            format!("Invalid UCUM expression: {}", reason),
        )
    }
    
    /// Create an invalid property error
    pub fn invalid_property(property: &str) -> Self {
        Self::new(
            ErrorKind::InvalidProperty {
                property: property.to_string(),
                available: Vec::new(),
            },
            format!("Invalid property: '{}'", property),
        )
    }
    
    /// Create a multiple slash error
    pub fn multiple_slash() -> Self {
        Self::new(
            ErrorKind::MultipleSlash,
            "Multiple slashes not allowed in expression".to_string(),
        )
    }
    
    /// Create an invalid percent placement error
    pub fn invalid_percent_placement(position: usize) -> Self {
        Self::new(
            ErrorKind::InvalidPercentPlacement { position },
            format!("Invalid percent placement at position {}", position),
        )
    }
    
    /// Create a special unit error
    pub fn special_unit_error(unit: &str, reason: &str) -> Self {
        Self::new(
            ErrorKind::SpecialUnitError {
                unit: unit.to_string(),
                reason: reason.to_string(),
            },
            format!("Special unit error for '{}': {}", unit, reason),
        )
    }
}

impl std::fmt::Display for UcumError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Display the primary error message
        write!(f, "{}", self.message)?;
        
        // Add span information if available
        if let Some(ref span) = self.span {
            write!(f, "\n{}", span.display_error(&self.message))?;
        }
        
        // Add suggestions if available
        if !self.suggestions.is_empty() {
            write!(f, "\n\nSuggestions:")?;
            for suggestion in &self.suggestions {
                write!(f, "\n  - {}", suggestion)?;
            }
        }
        
        // Add context information if available
        if !self.context.is_empty() {
            write!(f, "\n\nContext:")?;
            for context in &self.context {
                write!(f, "\n  {}", context)?;
            }
        }
        
        Ok(())
    }
}

impl std::error::Error for UcumError {}

// Maintain backward compatibility with the old error types
impl From<UcumError> for String {
    fn from(error: UcumError) -> Self {
        error.to_string()
    }
}
