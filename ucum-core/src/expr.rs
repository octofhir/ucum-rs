//!
//! Low-level parsers are implemented in `parser.rs`. This module only exposes
//! `parse_expression`, which performs early validation checks and then
//! delegates to `parser::parse_quotient` to build the final `UnitExpr` AST.

use crate::ast::UnitExpr;
use crate::parser;

/// Parse a UCUM expression string into a `UnitExpr` AST.
///
/// This function:
/// 1. Trims surrounding whitespace.
/// 2. Performs quick structural validations for clearer error messages.
/// 3. Delegates to the internal `nom` parser.
pub fn parse_expression(input: &str) -> Result<UnitExpr, crate::error::UcumError> {
    let input = input.trim();

    // Only one top-level slash allowed
    if input.matches('/').count() > 1 {
        return Err(crate::error::UcumError::MultipleSlash);
    }
    // Percent sign must stand alone
    if input.contains('%') && input != "%" {
        return Err(crate::error::UcumError::InvalidPercentPlacement);
    }

    // Run full parser
    let (rest, expr) = match parser::parse_quotient(input) {
        Ok(res) => res,
        Err(_) => return Err(crate::error::UcumError::InvalidExpression),
    };

    if !rest.trim().is_empty() {
        return Err(crate::error::UcumError::InvalidExpression);
    }
    Ok(expr)
}
