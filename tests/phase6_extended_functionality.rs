//! Tests for Phase 6: Extended Functionality
//!
//! This module tests the unit expression optimization functions and measurement context support
//! implemented in Phase 6 of the UCUM improvement plan.

use octofhir_ucum::{analyse, canonicalize_expression, optimize_expression, simplify_expression};

#[test]
fn test_optimize_expression() {
    // Test optimization of complex expressions - use valid UCUM syntax
    assert_eq!(optimize_expression("m2/s2").unwrap(), "m2.s-2");

    // Test recognition of common derived units
    assert_eq!(optimize_expression("kg.m/s2").unwrap(), "N");
    assert_eq!(optimize_expression("kg.m2/s2").unwrap(), "J");
    assert_eq!(optimize_expression("kg.m2/s3").unwrap(), "W");
    assert_eq!(optimize_expression("kg/(m.s2)").unwrap(), "Pa");
    assert_eq!(optimize_expression("/s").unwrap(), "Hz");

    // Test that simple units remain unchanged
    assert_eq!(optimize_expression("m").unwrap(), "m");
    assert_eq!(optimize_expression("g").unwrap(), "g");

    // Test error handling for invalid expressions
    assert!(optimize_expression("invalid_unit").is_err());
}

#[test]
fn test_canonicalize_expression() {
    // Test canonicalization of derived units
    // The UCUM base unit of mass is the gram
    assert_eq!(canonicalize_expression("N").unwrap(), "g.m.s-2");
    assert_eq!(canonicalize_expression("J").unwrap(), "g.m2.s-2");
    assert_eq!(canonicalize_expression("W").unwrap(), "g.m2.s-3");
    assert_eq!(canonicalize_expression("Pa").unwrap(), "g.m-1.s-2");
    assert_eq!(canonicalize_expression("Hz").unwrap(), "s-1");

    // Test canonicalization of prefixed units
    assert_eq!(canonicalize_expression("km").unwrap(), "m");
    assert_eq!(canonicalize_expression("mg").unwrap(), "g");
    assert_eq!(canonicalize_expression("kg").unwrap(), "g");

    // Test that base units remain in canonical form
    assert_eq!(canonicalize_expression("m").unwrap(), "m");
    assert_eq!(canonicalize_expression("g").unwrap(), "g");
    assert_eq!(canonicalize_expression("s").unwrap(), "s");

    // Test dimensionless units
    assert_eq!(canonicalize_expression("1").unwrap(), "1");

    // Test error handling
    assert!(canonicalize_expression("invalid_unit").is_err());
}

#[test]
fn test_simplify_expression() {
    // Test simplification of redundant operations - use valid UCUM syntax
    assert_eq!(simplify_expression("m.s/s").unwrap(), "m");
    assert_eq!(simplify_expression("g.m/m").unwrap(), "g");

    // Test that complex expressions are preserved
    let complex_expr = "kg.m2/s3";
    // The simplification should return a valid equivalent
    let simplified = simplify_expression(complex_expr).unwrap();
    assert!(!simplified.is_empty());

    // Test basic expressions remain the same
    assert_eq!(simplify_expression("m").unwrap(), "m");
    assert_eq!(simplify_expression("g").unwrap(), "g");

    // Test error handling
    assert!(simplify_expression("invalid_unit").is_err());
}

#[test]
fn test_expression_optimization_equivalence() {
    // Test that optimized expressions are dimensionally equivalent
    let test_cases = vec!["m2", "kg.m/s2", "kg.m2/s2", "/s", "kg/(m.s2)"];

    for expr in test_cases {
        if let Ok(original_analysis) = analyse(expr)
            && let Ok(optimized) = optimize_expression(expr)
            && let Ok(optimized_analysis) = analyse(&optimized)
        {
            assert_eq!(
                original_analysis.dimension, optimized_analysis.dimension,
                "Optimization changed dimensions for expression: {expr}"
            );
        }
    }
}

#[test]
fn test_simplification_equivalence() {
    // Test that simplified expressions are dimensionally equivalent
    let test_cases = vec!["m.s/s", "kg.m/m", "kg.m2/s2"];

    for expr in test_cases {
        if let Ok(original_analysis) = analyse(expr)
            && let Ok(simplified) = simplify_expression(expr)
            && let Ok(simplified_analysis) = analyse(&simplified)
        {
            assert_eq!(
                original_analysis.dimension, simplified_analysis.dimension,
                "Simplification changed dimensions for expression: {expr}"
            );
        }
    }
}
