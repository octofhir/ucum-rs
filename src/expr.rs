//!
//! High-performance UCUM expression parsing.
//! This module exposes `parse_expression` for public use.

use crate::ast::OwnedUnitExpr;
use crate::parser;

/// Parse a UCUM expression string into a `OwnedUnitExpr` AST (public API).
///
/// This function uses the optimized parser for best performance.
#[allow(clippy::result_large_err)]
pub fn parse_expression(input: &str) -> Result<OwnedUnitExpr, crate::error::UcumError> {
    parser::parse_expression_optimized(input)
}
