//! Enhanced error types for UCUM operations with detailed diagnostics.

use crate::prelude::*;
use crate::types::Dimension;

/// Specific error kinds with detailed context.
///
/// The enum and each variant are `#[non_exhaustive]`: a `match` needs a `_` arm, a pattern
/// needs `..`, and new kinds and fields are not breaking. [`ErrorKind::code`] names each
/// kind with a stable string.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ErrorKind {
    /// Units of different dimensions (UCUM §17), which do not convert to each other: the
    /// dimension of the target and of the source, and for a conversion the two units
    #[non_exhaustive]
    DimensionMismatch {
        expected: Dimension,
        found: Dimension,
        operation: String,
        from: Option<String>,
        to: Option<String>,
    },
    /// An atom that is not a unit of the tables, with a prefix or not
    #[non_exhaustive]
    UnitNotFound { unit: String },
    /// Units of the same dimension that still do not convert: different arbitrary units
    /// (UCUM §25)
    #[non_exhaustive]
    ConversionError {
        from: String,
        to: String,
        reason: String,
    },
    /// A result too large or too small for an `f64`, or a factor whose exponent does not
    /// fit
    #[non_exhaustive]
    PrecisionOverflow { operation: String, value: String },
    /// A quantity divided by a value of zero
    #[non_exhaustive]
    DivisionByZero { operation: String, value: String },
    /// An operand that is infinite or NaN, which is not a real number (UCUM §18.5)
    #[non_exhaustive]
    NonFiniteValue { operation: String, value: String },
    /// A value outside the domain of the function of a special unit (UCUM §21.2), such as
    /// zero or a negative value for a logarithmic unit
    #[non_exhaustive]
    OutOfDomain { unit: String, value: String },
    /// An expression that is not UCUM syntax (UCUM §1-10): the expression that was parsed
    /// (for `unit_multiply` and `unit_divide`, the joined one), and the byte offset in it
    /// where the parser stopped, when there is one
    #[non_exhaustive]
    InvalidExpression {
        reason: String,
        expression: String,
        position: Option<usize>,
    },
    /// An expression nested deeper than the parser allows, a limit of this library rather
    /// than of UCUM: `limit` is [`crate::MAX_NESTING_DEPTH`] levels for a parsed
    /// expression, and `MAX_NESTING_DEPTH + 3` nodes for a syntax tree built by hand and
    /// evaluated, which the message tells apart
    #[non_exhaustive]
    NestingTooDeep { limit: usize },
    /// A search pattern that is not a valid regular expression
    #[non_exhaustive]
    InvalidPattern { pattern: String, reason: String },
    /// A property that no unit of the tables has
    #[non_exhaustive]
    InvalidProperty { property: String },
    /// A special unit where it cannot be used: in algebra (UCUM §22.1), or without a
    /// linear canonical form
    #[non_exhaustive]
    SpecialUnitError { unit: String, reason: String },
}

impl ErrorKind {
    /// A stable name of the kind, in snake case: `"invalid_expression"`,
    /// `"unit_not_found"`, `"dimension_mismatch"`. It does not change between versions, so
    /// it can be returned to clients, for instance in the body of an HTTP error.
    ///
    /// ```
    /// use octofhir_ucum::{convert, validate};
    ///
    /// assert_eq!(validate("m//s").unwrap_err().kind.code(), "invalid_expression");
    /// assert_eq!(validate("xyz").unwrap_err().kind.code(), "unit_not_found");
    /// assert_eq!(convert(1.0, "m", "kg").unwrap_err().kind.code(), "dimension_mismatch");
    /// ```
    pub fn code(&self) -> &'static str {
        match self {
            ErrorKind::DimensionMismatch { .. } => "dimension_mismatch",
            ErrorKind::UnitNotFound { .. } => "unit_not_found",
            ErrorKind::ConversionError { .. } => "conversion_error",
            ErrorKind::PrecisionOverflow { .. } => "precision_overflow",
            ErrorKind::DivisionByZero { .. } => "division_by_zero",
            ErrorKind::NonFiniteValue { .. } => "non_finite_value",
            ErrorKind::OutOfDomain { .. } => "out_of_domain",
            ErrorKind::InvalidExpression { .. } => "invalid_expression",
            ErrorKind::NestingTooDeep { .. } => "nesting_too_deep",
            ErrorKind::InvalidPattern { .. } => "invalid_pattern",
            ErrorKind::InvalidProperty { .. } => "invalid_property",
            ErrorKind::SpecialUnitError { .. } => "special_unit_error",
        }
    }
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

    /// Create a dimension mismatch error: `found` where `expected` was needed
    pub fn dimension_mismatch(expected: Dimension, found: Dimension, operation: &str) -> Self {
        Self::new(
            ErrorKind::DimensionMismatch {
                expected,
                found,
                operation: operation.to_string(),
                from: None,
                to: None,
            },
            format!("Dimension mismatch in {operation}: expected {expected}, found {found}"),
        )
    }

    /// Create a dimension mismatch error for a conversion between units of different
    /// dimensions: `to` has the dimension `expected`, `from` has `found`
    pub fn incommensurable_dimensions(
        from: &str,
        to: &str,
        expected: Dimension,
        found: Dimension,
    ) -> Self {
        let operation = format!("conversion from '{from}' to '{to}'");
        let mut error = Self::dimension_mismatch(expected, found, &operation);
        if let ErrorKind::DimensionMismatch {
            from: from_unit,
            to: to_unit,
            ..
        } = &mut error.kind
        {
            *from_unit = Some(from.to_string());
            *to_unit = Some(to.to_string());
        }
        error
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

    /// Create an invalid expression error, for no expression in particular
    pub fn invalid_expression(reason: &str) -> Self {
        Self::invalid_expression_in("", reason, None)
    }

    /// Create an invalid expression error for `expression`, found at a byte offset of it
    pub fn invalid_expression_in(expression: &str, reason: &str, position: Option<usize>) -> Self {
        let message = match position {
            Some(position) => format!("Invalid UCUM expression: {reason} at position {position}"),
            None => format!("Invalid UCUM expression: {reason}"),
        };
        Self::new(
            ErrorKind::InvalidExpression {
                reason: reason.to_string(),
                expression: expression.to_string(),
                position,
            },
            message,
        )
    }

    /// Create an error for an expression nested deeper than `limit` levels
    pub fn nesting_too_deep(limit: usize) -> Self {
        Self::new(
            ErrorKind::NestingTooDeep { limit },
            format!("Invalid UCUM expression: nested deeper than {limit} levels"),
        )
    }

    /// Create an error for a syntax tree, built by hand, deeper than `limit` nodes
    pub(crate) fn tree_too_deep(limit: usize) -> Self {
        Self::new(
            ErrorKind::NestingTooDeep { limit },
            format!("Invalid syntax tree: deeper than {limit} nodes"),
        )
    }

    /// Create an error for a search pattern that is not a valid regular expression
    pub fn invalid_pattern(pattern: &str, reason: &str) -> Self {
        Self::new(
            ErrorKind::InvalidPattern {
                pattern: pattern.to_string(),
                reason: reason.to_string(),
            },
            format!("Invalid search pattern: {reason}"),
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
