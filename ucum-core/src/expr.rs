//!
//! Low-level parsers are implemented in `parser.rs`. This module exposes
//! `parse_expression` for public use and internal functions for optimization.

use crate::ast::OwnedUnitExpr;
use crate::parser;

/// Parse a UCUM expression string into a `OwnedUnitExpr` AST (public API).
///
/// This function:
/// 1. Trims surrounding whitespace.
/// 2. Performs quick structural validations for clearer error messages.
/// 3. Delegates to the internal `nom` parser.
/// 4. Converts to owned AST for external use.
pub fn parse_expression(input: &str) -> Result<OwnedUnitExpr, crate::error::UcumError> {
    let input = input.trim();

    // Multiple division operators are allowed per UCUM specification §7.4
    // They are evaluated left-to-right with same precedence as multiplication
    // Percent sign must stand alone
    if input.contains('%') && input != "%" {
        return Err(crate::error::UcumError::invalid_percent_placement(input.find('%').unwrap_or(0)));
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
                        return Err(crate::error::UcumError::invalid_expression("Addition operators are not allowed in UCUM expressions"));
                    }
                }
                _ => {}
            }
        }
    }

    // Run full parser
    let (rest, expr) = match parser::parse_quotient(input) {
        Ok(res) => res,
        Err(_) => return Err(crate::error::UcumError::invalid_expression("Failed to parse UCUM expression")),
    };

    if !rest.trim().is_empty() {
        return Err(crate::error::UcumError::invalid_expression("Unexpected characters at end of expression"));
    }
    Ok(expr.to_owned())
}

