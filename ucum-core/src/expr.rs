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

    // Multiple division operators are allowed per UCUM specification §7.4
    // They are evaluated left-to-right with same precedence as multiplication
    // Percent sign must stand alone
    if input.contains('%') && input != "%" {
        return Err(crate::error::UcumError::InvalidPercentPlacement);
    }
    // Addition operators are not allowed in UCUM expressions
    if input.contains('+') && !input.starts_with("10*+") && !input.starts_with("10^+") {
        // Allow + in 10*+n and 10^+n contexts, but reject standalone + operations
        let mut in_annotation = false;
        let mut chars = input.chars().peekable();
        while let Some(ch) = chars.next() {
            match ch {
                '{' => in_annotation = true,
                '}' => in_annotation = false,
                '+' if !in_annotation => {
                    // Check if this + is part of a valid 10*+ or 10^+ pattern
                    let before = input.split('+').next().unwrap_or("");
                    if !before.ends_with("10*") && !before.ends_with("10^") {
                        return Err(crate::error::UcumError::InvalidExpression);
                    }
                }
                _ => {}
            }
        }
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
