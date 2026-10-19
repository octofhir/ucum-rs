//! Enhanced error types for UCUM operations with detailed diagnostics.

use crate::prelude::*;
use crate::types::Dimension;

/// Specific error kinds with detailed context
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ErrorKind {
    /// Dimension mismatch with specific dimensions
    DimensionMismatch {
        expected: Dimension,
        found: Dimension,
        operation: String,
    },
    /// Unit not found
    UnitNotFound { unit: String },
    /// Conversion error with detailed reason
    ConversionError {
        from: String,
        to: String,
        reason: String,
    },
    /// A result too large or too small for an `f64`, or a factor whose exponent does not
    /// fit
    PrecisionOverflow { operation: String, value: String },
    /// A quantity divided by a value of zero
    DivisionByZero { operation: String, value: String },
    /// An operand that is infinite or NaN, which is not a real number (UCUM §18.5)
    NonFiniteValue { operation: String, value: String },
    /// A value outside the domain of the function of a special unit (UCUM §21.2), such as
    /// zero or a negative value for a logarithmic unit
    OutOfDomain { unit: String, value: String },
    /// Invalid expression structure
    InvalidExpression { reason: String },
    /// Invalid property for unit validation
    InvalidProperty { property: String },
    /// Special unit handling error
    SpecialUnitError { unit: String, reason: String },
}

/// Enhanced UCUM error with detailed context and suggestions
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct UcumError {
    /// The specific kind of error that occurred
    pub kind: ErrorKind,
    /// Primary error message
    pub message: String,
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
            suggestions: Vec::new(),
            context: Vec::new(),
        }
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

    /// Create a dimension mismatch error
    pub fn dimension_mismatch(expected: Dimension, found: Dimension, operation: &str) -> Self {
        Self::new(
            ErrorKind::DimensionMismatch {
                expected,
                found,
                operation: operation.to_string(),
            },
            format!("Dimension mismatch in {operation}: expected {expected:?}, found {found:?}"),
        )
    }

    /// Create a unit not found error
    pub fn unit_not_found(unit: &str) -> Self {
        Self::new(
            ErrorKind::UnitNotFound {
                unit: unit.to_string(),
            },
            format!("Unknown unit: '{unit}'"),
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
            format!("Cannot convert from '{from}' to '{to}': {reason}"),
        )
    }

    /// Create a precision overflow error
    pub fn precision_overflow(operation: &str, value: &str) -> Self {
        Self::new(
            ErrorKind::PrecisionOverflow {
                operation: operation.to_string(),
                value: value.to_string(),
            },
            format!("Precision overflow during {operation}: {value}"),
        )
    }

    /// Create a division by zero error
    pub fn division_by_zero(operation: &str, value: &str) -> Self {
        Self::new(
            ErrorKind::DivisionByZero {
                operation: operation.to_string(),
                value: value.to_string(),
            },
            format!("Division by zero during {operation}: {value}"),
        )
    }

    /// Create an error for an operand that is infinite or NaN
    pub fn non_finite_value(operation: &str, value: &str) -> Self {
        Self::new(
            ErrorKind::NonFiniteValue {
                operation: operation.to_string(),
                value: value.to_string(),
            },
            format!("Operand of {operation} is not a finite number (UCUM §18.5): {value}"),
        )
    }

    /// Create an error for a value outside the domain of the function of a special unit
    pub fn out_of_domain(unit: &str, value: &str) -> Self {
        Self::new(
            ErrorKind::OutOfDomain {
                unit: unit.to_string(),
                value: value.to_string(),
            },
            format!(
                "{value} is outside the domain of the function of the special unit '{unit}' (UCUM §21.2)"
            ),
        )
    }

    /// Create an invalid expression error
    pub fn invalid_expression(reason: &str) -> Self {
        Self::new(
            ErrorKind::InvalidExpression {
                reason: reason.to_string(),
            },
            format!("Invalid UCUM expression: {reason}"),
        )
    }

    /// Create an invalid property error
    pub fn invalid_property(property: &str) -> Self {
        Self::new(
            ErrorKind::InvalidProperty {
                property: property.to_string(),
            },
            format!("Invalid property: '{property}'"),
        )
    }

    /// Create a special unit error
    pub fn special_unit_error(unit: &str, reason: &str) -> Self {
        Self::new(
            ErrorKind::SpecialUnitError {
                unit: unit.to_string(),
                reason: reason.to_string(),
            },
            format!("Special unit error for '{unit}': {reason}"),
        )
    }
}

impl core::fmt::Display for UcumError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Display the primary error message
        write!(f, "{}", self.message)?;

        // Add suggestions if available
        if !self.suggestions.is_empty() {
            write!(f, "\n\nSuggestions:")?;
            for suggestion in &self.suggestions {
                write!(f, "\n  - {suggestion}")?;
            }
        }

        // Add context information if available
        if !self.context.is_empty() {
            write!(f, "\n\nContext:")?;
            for context in &self.context {
                write!(f, "\n  {context}")?;
            }
        }

        Ok(())
    }
}

impl core::error::Error for UcumError {}

// Maintain backward compatibility with the old error types
impl From<UcumError> for String {
    fn from(error: UcumError) -> Self {
        error.to_string()
    }
}
