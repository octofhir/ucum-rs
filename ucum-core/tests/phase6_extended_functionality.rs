//! Tests for Phase 6: Extended Functionality
//! 
//! This module tests the unit expression optimization functions and measurement context support
//! implemented in Phase 6 of the UCUM improvement plan.

use octofhir_ucum_core::{
    optimize_expression, canonicalize_expression, simplify_expression,
    MeasurementContext, Domain, analyse,
};

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
    assert_eq!(optimize_expression("kg").unwrap(), "kg");
    
    // Test error handling for invalid expressions
    assert!(optimize_expression("invalid_unit").is_err());
}

#[test]
fn test_canonicalize_expression() {
    // Test canonicalization of derived units
    assert_eq!(canonicalize_expression("N").unwrap(), "kg.m.s-2");
    assert_eq!(canonicalize_expression("J").unwrap(), "kg.m2.s-2");
    assert_eq!(canonicalize_expression("W").unwrap(), "kg.m2.s-3");
    assert_eq!(canonicalize_expression("Pa").unwrap(), "kg.m-1.s-2");
    assert_eq!(canonicalize_expression("Hz").unwrap(), "s-1");
    
    // Test canonicalization of prefixed units
    assert_eq!(canonicalize_expression("km").unwrap(), "m");
    assert_eq!(canonicalize_expression("mg").unwrap(), "kg");
    
    // Test that base units remain in canonical form
    assert_eq!(canonicalize_expression("m").unwrap(), "m");
    assert_eq!(canonicalize_expression("kg").unwrap(), "kg");
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
    assert_eq!(simplify_expression("kg.m/m").unwrap(), "kg");
    
    // Test that complex expressions are preserved
    let complex_expr = "kg.m2/s3";
    // The simplification should return a valid equivalent
    let simplified = simplify_expression(complex_expr).unwrap();
    assert!(!simplified.is_empty());
    
    // Test basic expressions remain the same
    assert_eq!(simplify_expression("m").unwrap(), "m");
    assert_eq!(simplify_expression("kg").unwrap(), "kg");
    
    // Test error handling
    assert!(simplify_expression("invalid_unit").is_err());
}

#[test]
fn test_expression_optimization_equivalence() {
    // Test that optimized expressions are dimensionally equivalent
    let test_cases = vec![
        "m2",
        "kg.m/s2", 
        "kg.m2/s2",
        "/s",
        "kg/(m.s2)",
    ];
    
    for expr in test_cases {
        if let Ok(original_analysis) = analyse(expr) {
            if let Ok(optimized) = optimize_expression(expr) {
                if let Ok(optimized_analysis) = analyse(&optimized) {
                    assert_eq!(
                        original_analysis.dimension, 
                        optimized_analysis.dimension,
                        "Optimization changed dimensions for expression: {}", expr
                    );
                }
            }
        }
    }
}

#[test]
fn test_simplification_equivalence() {
    // Test that simplified expressions are dimensionally equivalent
    let test_cases = vec![
        "m.s/s",
        "kg.m/m", 
        "kg.m2/s2",
    ];
    
    for expr in test_cases {
        if let Ok(original_analysis) = analyse(expr) {
            if let Ok(simplified) = simplify_expression(expr) {
                if let Ok(simplified_analysis) = analyse(&simplified) {
                    assert_eq!(
                        original_analysis.dimension, 
                        simplified_analysis.dimension,
                        "Simplification changed dimensions for expression: {}", expr
                    );
                }
            }
        }
    }
}

#[test]
fn test_measurement_context_default() {
    let context = MeasurementContext::default();
    
    assert_eq!(context.domain, Domain::General);
    assert_eq!(context.precision_requirements.min_significant_figures, 3);
    assert_eq!(context.precision_requirements.max_relative_error, 1e-6);
    assert!(!context.precision_requirements.require_exact);
    assert!(context.preferred_units.is_empty());
    assert!(context.avoided_units.is_empty());
}

#[test]
fn test_measurement_context_medical() {
    let context = MeasurementContext::medical();
    
    assert_eq!(context.domain, Domain::Medical);
    assert_eq!(context.precision_requirements.min_significant_figures, 4);
    assert_eq!(context.precision_requirements.max_relative_error, 1e-8);
    assert!(context.precision_requirements.require_exact);
    
    // Check that medical-specific units are preferred
    assert!(context.is_preferred_unit("mg"));
    assert!(context.is_preferred_unit("mL"));
    assert!(context.is_preferred_unit("Cel"));
    
    // Check that ambiguous units are avoided
    assert!(context.is_avoided_unit("[IU]"));
}

#[test]
fn test_measurement_context_engineering() {
    let context = MeasurementContext::engineering();
    
    assert_eq!(context.domain, Domain::Engineering);
    assert_eq!(context.precision_requirements.min_significant_figures, 3);
    assert_eq!(context.precision_requirements.max_relative_error, 1e-6);
    assert!(!context.precision_requirements.require_exact);
    
    // Check that SI units are preferred
    assert!(context.is_preferred_unit("m"));
    assert!(context.is_preferred_unit("kg"));
    assert!(context.is_preferred_unit("N"));
    assert!(context.is_preferred_unit("Pa"));
    assert!(context.is_preferred_unit("K"));
    
    // Check that non-SI units are avoided
    assert!(context.is_avoided_unit("[psi]"));
    assert!(context.is_avoided_unit("[in_i]"));
    assert!(context.is_avoided_unit("[ft_i]"));
}

#[test]
fn test_measurement_context_physics() {
    let context = MeasurementContext::physics();
    
    assert_eq!(context.domain, Domain::Physics);
    assert_eq!(context.precision_requirements.min_significant_figures, 6);
    assert_eq!(context.precision_requirements.max_relative_error, 1e-12);
    assert!(context.precision_requirements.require_exact);
    
    // Check that fundamental SI units are preferred
    assert!(context.is_preferred_unit("m"));
    assert!(context.is_preferred_unit("kg"));
    assert!(context.is_preferred_unit("s"));
    assert!(context.is_preferred_unit("A"));
    assert!(context.is_preferred_unit("K"));
    assert!(context.is_preferred_unit("mol"));
    assert!(context.is_preferred_unit("cd"));
    
    // Check that non-fundamental units are avoided
    assert!(context.is_avoided_unit("[cal]"));
    assert!(context.is_avoided_unit("[Btu]"));
}

#[test]
fn test_measurement_context_chemistry() {
    let context = MeasurementContext::chemistry();
    
    assert_eq!(context.domain, Domain::Chemistry);
    assert_eq!(context.precision_requirements.min_significant_figures, 4);
    assert_eq!(context.precision_requirements.max_relative_error, 1e-9);
    assert!(!context.precision_requirements.require_exact);
    
    // Check that molar units are preferred
    assert!(context.is_preferred_unit("mol"));
    assert!(context.is_preferred_unit("mmol"));
    assert!(context.is_preferred_unit("mol/L"));
    assert!(context.is_preferred_unit("mL"));
    assert!(context.is_preferred_unit("Cel"));
    
    // Check that non-molar concentration units are avoided where appropriate
    assert!(context.is_avoided_unit("g/L"));
}

#[test]
fn test_measurement_context_suggest_alternatives() {
    let medical_context = MeasurementContext::medical(); 
    let engineering_context = MeasurementContext::engineering();
    
    // Test medical alternatives for mass units
    let mass_alternatives = medical_context.suggest_alternatives("g").unwrap();
    assert!(mass_alternatives.contains(&"mg".to_string()));
    assert!(mass_alternatives.contains(&"g".to_string()));
    
    // Test engineering alternatives for pressure units
    let pressure_alternatives = engineering_context.suggest_alternatives("[psi]").unwrap();
    assert!(pressure_alternatives.contains(&"Pa".to_string()) || 
            pressure_alternatives.contains(&"kPa".to_string()) ||
            pressure_alternatives.contains(&"MPa".to_string()));
    
    // Test error handling for invalid units
    assert!(medical_context.suggest_alternatives("invalid_unit").is_err());
}

#[test]
fn test_measurement_context_domain_specific_suggestions() {
    let contexts = vec![
        (MeasurementContext::medical(), Domain::Medical),
        (MeasurementContext::engineering(), Domain::Engineering), 
        (MeasurementContext::physics(), Domain::Physics),
        (MeasurementContext::chemistry(), Domain::Chemistry),
    ];
    
    for (context, expected_domain) in contexts {
        assert_eq!(context.domain, expected_domain);
        
        // Test that each context provides some suggestions for common units
        if let Ok(mass_suggestions) = context.suggest_alternatives("g") {
            assert!(!mass_suggestions.is_empty(), 
                   "Context {:?} should provide mass unit suggestions", expected_domain);
        }
    }
}

#[test]
fn test_measurement_context_precision_requirements() {
    let contexts = vec![
        MeasurementContext::medical(),
        MeasurementContext::engineering(),
        MeasurementContext::physics(), 
        MeasurementContext::chemistry(),
    ];
    
    for context in contexts {
        // All contexts should have reasonable precision requirements
        assert!(context.precision_requirements.min_significant_figures >= 3,
               "Context {:?} should require at least 3 significant figures", context.domain);
        assert!(context.precision_requirements.max_relative_error > 0.0,
               "Context {:?} should have positive max relative error", context.domain);
        assert!(context.precision_requirements.max_relative_error < 1.0,
               "Context {:?} should have max relative error < 1.0", context.domain);
    }
}