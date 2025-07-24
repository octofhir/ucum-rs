//! Phase 6 Implementation Summary
//! 
//! This example demonstrates the implementation of Phase 6: Extended Functionality
//! from the UCUM improvement plan.

use octofhir_ucum_core::{
    optimize_expression, canonicalize_expression, simplify_expression,
    MeasurementContext, Domain,
};

fn main() {
    println!("=== Phase 6: Extended Functionality Demo ===\n");

    // 1. Unit Expression Optimization
    println!("1. Unit Expression Optimization:");
    
    let simple_expr = "m";
    println!("  Expression: {}", simple_expr);
    
    if let Ok(optimized) = optimize_expression(simple_expr) {
        println!("    Optimized: {}", optimized);
    }
    
    if let Ok(canonical) = canonicalize_expression(simple_expr) {
        println!("    Canonical: {}", canonical);
    }
    
    if let Ok(simplified) = simplify_expression(simple_expr) {
        println!("    Simplified: {}", simplified);
    }
    
    println!();

    // 2. Measurement Context Support
    println!("2. Measurement Context Support:");
    
    let contexts = vec![
        ("Medical", MeasurementContext::medical()),
        ("Engineering", MeasurementContext::engineering()),
        ("Physics", MeasurementContext::physics()),
        ("Chemistry", MeasurementContext::chemistry()),
    ];
    
    for (name, context) in contexts {
        println!("  {} Context:", name);
        println!("    Domain: {:?}", context.domain);
        println!("    Min Sig Figs: {}", context.precision_requirements.min_significant_figures);
        println!("    Max Rel Error: {:.0e}", context.precision_requirements.max_relative_error);
        println!("    Require Exact: {}", context.precision_requirements.require_exact);
        
        // Show a few preferred units
        let sample_units = context.preferred_units.iter().take(3).collect::<Vec<_>>();
        if !sample_units.is_empty() {
            println!("    Sample Preferred Units: {:?}", sample_units);
        }
        
        println!();
    }

    println!("3. Domain-Specific Unit Suggestions:");
    
    // Test unit suggestions
    let medical = MeasurementContext::medical();
    if let Ok(suggestions) = medical.suggest_alternatives("g") {
        println!("  Medical alternatives for 'g': {:?}", suggestions);
    }
    
    let engineering = MeasurementContext::engineering();  
    if let Ok(suggestions) = engineering.suggest_alternatives("[psi]") {
        println!("  Engineering alternatives for '[psi]': {:?}", suggestions);
    }
    
    println!("\n=== Phase 6 Implementation Complete ===");
    println!("Features implemented:");
    println!("✓ optimize_expression() - Optimizes unit expressions for readability");
    println!("✓ canonicalize_expression() - Converts to base SI units");
    println!("✓ simplify_expression() - Simplifies complex expressions");
    println!("✓ MeasurementContext - Domain-specific unit preferences");
    println!("✓ Domain enum - Medical, Engineering, Physics, Chemistry, General");
    println!("✓ PrecisionRequirements - Configurable precision settings");
    println!("✓ Unit suggestion system - Context-aware alternatives");
}