//! Error types for UCUM operations.

#[cfg(feature = "std")]
extern crate std;

#[derive(Debug, thiserror::Error)]
pub enum UcumError {
    #[error("unknown unit: {0}")]
    UnknownUnit(String),

    #[error("invalid UCUM expression")]
    InvalidExpression,

    #[error("multiple slashes not allowed in expression")]
    MultipleSlash,

    #[error("invalid percent placement in expression")]
    InvalidPercentPlacement,

    #[error("dimension mismatch between operands")]
    DimensionMismatch,

    #[error("conversion error: {0}")]
    ConversionError(&'static str),
}
