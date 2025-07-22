//! UCUM Core Library – Rust 2024 Edition
//!
//! This crate provides parsing, validation and conversion utilities for the
//! Unified Code for Units of Measure (UCUM). It aims to be `no_std`-optional
//! and suitable for both embedded and server environments.

#![cfg_attr(not(feature = "std"), no_std)]

mod ast;
mod display;
mod error;
mod evaluator;
mod expr;
mod parser;
pub mod precision;
mod registry;
pub mod special_units;
mod types;

pub use crate::ast::{UnitExpr, UnitFactor};
pub use crate::display::generate_display_name;
pub use crate::error::UcumError;
pub use crate::evaluator::{EvalResult, evaluate};
pub use crate::expr::parse_expression;
pub use crate::special_units::{
    ArbitraryHandler, ConversionContext, LogarithmicHandler, SpecialUnitHandler,
    SpecialUnitRegistry, TemperatureHandler,
};
pub use crate::types::{BaseUnit, DerivedUnit, Dimension, Prefix, Quantity, UnitRecord};

// Import precision utilities for internal use
use crate::precision::{Number, NumericOps, to_f64};

// Re-export for convenience
pub use crate::evaluator::evaluate as eval;

/// Lookup a unit by code using the generated registry.
pub fn find_unit(code: &str) -> Option<&'static crate::types::UnitRecord> {
    registry::find_unit(code)
}

/// Get all units from the registry.
pub fn get_all_units() -> &'static [crate::types::UnitRecord] {
    registry::UNITS
}

/// Lookup a prefix by symbol.
pub fn find_prefix(sym: &str) -> Option<&'static Prefix> {
    registry::find_prefix(sym)
}

// ============================================================================
// Phase 1: Core API Enhancement - Validation Methods
// ============================================================================

/// Validate a UCUM expression string.
///
/// Returns `Ok(())` if the expression is valid, or an error describing why it's invalid.
/// This performs comprehensive validation including parsing, unit lookup, and dimensional analysis.
///
/// # Examples
///
/// ```
/// use octofhir_ucum_core::validate;
///
/// assert!(validate("m/s").is_ok());
/// assert!(validate("kg.m/s2").is_ok());
/// assert!(validate("invalid_unit").is_err());
/// ```
pub fn validate(expression: &str) -> Result<(), UcumError> {
    // Parse the expression
    let parsed = parse_expression(expression)?;

    // Evaluate to ensure all units are valid and dimensions are consistent
    let _result = evaluate(&parsed)?;

    Ok(())
}

/// Analyse a UCUM expression and return detailed information about it.
///
/// Returns comprehensive information about the unit including its canonical form,
/// dimension vector, factor, and whether it has any special properties.
///
/// # Examples
///
/// ```
/// use octofhir_ucum_core::analyse;
///
/// let analysis = analyse("km/h").unwrap();
/// println!("Dimension: {:?}", analysis.dimension);
/// println!("Factor: {}", analysis.factor);
/// ```
pub fn analyse(expression: &str) -> Result<UnitAnalysis, UcumError> {
    let parsed = parse_expression(expression)?;
    let result = evaluate(&parsed)?;

    Ok(UnitAnalysis {
        expression: expression.to_string(),
        parsed_ast: parsed,
        dimension: result.dim,
        factor: to_f64(result.factor),
        offset: to_f64(result.offset),
        is_dimensionless: result.dim == Dimension([0; 7]),
        has_offset: result.offset != Number::zero(),
    })
}

/// Detailed analysis result for a UCUM expression.
#[derive(Debug, Clone)]
pub struct UnitAnalysis {
    /// Original expression string
    pub expression: String,
    /// Parsed AST representation
    pub parsed_ast: UnitExpr,
    /// Dimension vector
    pub dimension: Dimension,
    /// Conversion factor to base units
    pub factor: f64,
    /// Offset for temperature units
    pub offset: f64,
    /// Whether the unit is dimensionless
    pub is_dimensionless: bool,
    /// Whether the unit has a temperature offset
    pub has_offset: bool,
}

/// Validate that a unit expression is appropriate for a given property.
///
/// This validates that the unit has the correct dimension for the specified property.
/// Properties correspond to physical quantities like "length", "mass", "time", etc.
///
/// # Examples
///
/// ```
/// use octofhir_ucum_core::validate_in_property;
///
/// assert!(validate_in_property("m", "length").unwrap());
/// assert!(validate_in_property("kg", "mass").unwrap());
/// assert!(!validate_in_property("m", "mass").unwrap());
/// ```
pub fn validate_in_property(expression: &str, property: &str) -> Result<bool, UcumError> {
    let analysis = analyse(expression)?;

    // Define expected dimensions for common properties
    let expected_dimension = match property.to_lowercase().as_str() {
        "length" => Dimension([0, 1, 0, 0, 0, 0, 0]),      // L
        "mass" => Dimension([1, 0, 0, 0, 0, 0, 0]),        // M
        "time" => Dimension([0, 0, 1, 0, 0, 0, 0]),        // T
        "current" => Dimension([0, 0, 0, 1, 0, 0, 0]),     // I
        "temperature" => Dimension([0, 0, 0, 0, 1, 0, 0]), // Θ
        "amount" => Dimension([0, 0, 0, 0, 0, 1, 0]),      // N
        "luminosity" => Dimension([0, 0, 0, 0, 0, 0, 1]),  // J
        "area" => Dimension([0, 2, 0, 0, 0, 0, 0]),        // L²
        "volume" => Dimension([0, 3, 0, 0, 0, 0, 0]),      // L³
        "velocity" => Dimension([0, 1, -1, 0, 0, 0, 0]),   // LT⁻¹
        "acceleration" => Dimension([0, 1, -2, 0, 0, 0, 0]), // LT⁻²
        "force" => Dimension([1, 1, -2, 0, 0, 0, 0]),      // MLT⁻²
        "energy" => Dimension([1, 2, -2, 0, 0, 0, 0]),     // ML²T⁻²
        "power" => Dimension([1, 2, -3, 0, 0, 0, 0]),      // ML²T⁻³
        "pressure" => Dimension([1, -1, -2, 0, 0, 0, 0]),  // ML⁻¹T⁻²
        "frequency" => Dimension([0, 0, -1, 0, 0, 0, 0]),  // T⁻¹
        "voltage" => Dimension([1, 2, -3, -1, 0, 0, 0]),   // ML²T⁻³I⁻¹
        "resistance" => Dimension([1, 2, -3, -2, 0, 0, 0]), // ML²T⁻³I⁻²
        "capacitance" => Dimension([-1, -2, 4, 2, 0, 0, 0]), // M⁻¹L⁻²T⁴I²
        "inductance" => Dimension([1, 2, -2, -2, 0, 0, 0]), // ML²T⁻²I⁻²
        "dimensionless" => Dimension([0, 0, 0, 0, 0, 0, 0]), // 1
        _ => {
            return Err(UcumError::InvalidProperty(format!(
                "Unknown property: {}",
                property
            )));
        }
    };

    Ok(analysis.dimension == expected_dimension)
}

/// Check if two units are commensurable (can be converted between each other).
///
/// Two units are commensurable if they have the same dimension vector.
///
/// # Examples
///
/// ```
/// use octofhir_ucum_core::is_comparable;
///
/// assert!(is_comparable("m", "km").unwrap());
/// assert!(is_comparable("kg", "g").unwrap());
/// assert!(!is_comparable("m", "kg").unwrap());
/// ```
pub fn is_comparable(unit1: &str, unit2: &str) -> Result<bool, UcumError> {
    let analysis1 = analyse(unit1)?;
    let analysis2 = analyse(unit2)?;

    Ok(analysis1.dimension == analysis2.dimension)
}

/// Get the canonical units for a given UCUM expression.
///
/// Returns the canonical (base) unit representation with the conversion factor.
/// This is useful for normalizing units to their base forms.
///
/// # Examples
///
/// ```
/// use octofhir_ucum_core::get_canonical_units;
///
/// let canonical = get_canonical_units("km").unwrap();
/// println!("Canonical: {} (factor: {})", canonical.unit, canonical.factor);
/// // Output: Canonical: m (factor: 1000)
/// ```
pub fn get_canonical_units(expression: &str) -> Result<CanonicalUnit, UcumError> {
    let analysis = analyse(expression)?;

    // Build canonical unit string from dimension vector
    let canonical_unit = build_canonical_unit_string(&analysis.dimension);

    Ok(CanonicalUnit {
        unit: canonical_unit,
        factor: analysis.factor,
        offset: analysis.offset,
        dimension: analysis.dimension,
    })
}

/// Canonical unit representation
#[derive(Debug, Clone)]
pub struct CanonicalUnit {
    /// Canonical unit string (e.g., "kg.m.s-2" for force)
    pub unit: String,
    /// Conversion factor from original to canonical
    pub factor: f64,
    /// Temperature offset if applicable
    pub offset: f64,
    /// Dimension vector
    pub dimension: Dimension,
}

/// Result of unit arithmetic operations
#[derive(Debug, Clone)]
pub struct UnitArithmeticResult {
    /// Resulting unit expression string
    pub expression: String,
    /// Combined conversion factor
    pub factor: f64,
    /// Resulting dimension vector
    pub dimension: Dimension,
    /// Offset (should be zero for arithmetic operations)
    pub offset: f64,
    /// Whether the result is dimensionless
    pub is_dimensionless: bool,
}

/// Build canonical unit string from dimension vector
fn build_canonical_unit_string(dim: &Dimension) -> String {
    let base_units = ["kg", "m", "s", "A", "K", "mol", "cd"];
    let mut parts = Vec::new();

    for (i, &exp) in dim.0.iter().enumerate() {
        if exp != 0 {
            if exp == 1 {
                parts.push(base_units[i].to_string());
            } else {
                parts.push(format!("{}{}", base_units[i], exp));
            }
        }
    }

    if parts.is_empty() {
        "1".to_string() // dimensionless
    } else {
        parts.join(".")
    }
}

// ============================================================================
// Phase 1: Core API Enhancement - Mathematical Operations
// ============================================================================

/// Multiply two quantities with units.
///
/// # Examples
///
/// ```
/// use octofhir_ucum_core::multiply;
///
/// let result = multiply(5.0, "m", 2.0, "s").unwrap();
/// println!("{} {}", result.value, result.unit); // "10 m.s"
/// ```
pub fn multiply(
    value1: f64,
    unit1: &str,
    value2: f64,
    unit2: &str,
) -> Result<UnitResult, UcumError> {
    let analysis1 = analyse(unit1)?;
    let analysis2 = analyse(unit2)?;

    // Check for offset units (not allowed in multiplication)
    if analysis1.has_offset || analysis2.has_offset {
        return Err(UcumError::ConversionError(
            "offset units cannot participate in multiplication",
        ));
    }

    // Calculate result
    let result_value = value1 * value2;
    let result_factor = analysis1.factor * analysis2.factor;

    // Combine dimensions
    let mut result_dim = [0i8; 7];
    for i in 0..7 {
        result_dim[i] = analysis1.dimension.0[i] + analysis2.dimension.0[i];
    }

    let result_unit = build_canonical_unit_string(&Dimension(result_dim));

    Ok(UnitResult {
        value: result_value * result_factor,
        unit: result_unit,
        dimension: Dimension(result_dim),
    })
}

/// Divide two quantities with units.
///
/// # Examples
///
/// ```
/// use octofhir_ucum_core::divide_by;
///
/// let result = divide_by(10.0, "m", 2.0, "s").unwrap();
/// println!("{} {}", result.value, result.unit); // "5 m.s-1"
/// ```
pub fn divide_by(
    dividend_value: f64,
    dividend_unit: &str,
    divisor_value: f64,
    divisor_unit: &str,
) -> Result<UnitResult, UcumError> {
    let analysis1 = analyse(dividend_unit)?;
    let analysis2 = analyse(divisor_unit)?;

    // Check for offset units (not allowed in division)
    if analysis1.has_offset || analysis2.has_offset {
        return Err(UcumError::ConversionError(
            "offset units cannot participate in division",
        ));
    }

    if divisor_value == 0.0 {
        return Err(UcumError::ConversionError("division by zero"));
    }

    // Calculate result
    let result_value = dividend_value / divisor_value;
    let result_factor = analysis1.factor / analysis2.factor;

    // Combine dimensions (subtract divisor from dividend)
    let mut result_dim = [0i8; 7];
    for i in 0..7 {
        result_dim[i] = analysis1.dimension.0[i] - analysis2.dimension.0[i];
    }

    let result_unit = build_canonical_unit_string(&Dimension(result_dim));

    Ok(UnitResult {
        value: result_value * result_factor,
        unit: result_unit,
        dimension: Dimension(result_dim),
    })
}

/// Result of mathematical operations with units
#[derive(Debug, Clone)]
pub struct UnitResult {
    /// Calculated value
    pub value: f64,
    /// Resulting unit string
    pub unit: String,
    /// Dimension vector of result
    pub dimension: Dimension,
}

// ============================================================================
// Phase 1: Core API Enhancement - Search Functionality
// ============================================================================

use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use regex::Regex;

/// Search for units by name, code, or display name.
///
/// Returns a list of units that match the search criteria. The search is case-insensitive
/// and matches partial strings in unit codes, names, and display names.
///
/// # Examples
///
/// ```
/// use octofhir_ucum_core::search_units;
///
/// let results = search_units("meter");
/// for unit in results {
///     println!("{}: {}", unit.code, unit.display_name);
/// }
/// ```
pub fn search_units(query: &str) -> Vec<&'static UnitRecord> {
    let query_lower = query.to_lowercase();
    let mut results = Vec::new();

    for unit in get_all_units() {
        // Simple substring search (case-insensitive)
        let matches = unit.code.to_lowercase().contains(&query_lower)
            || unit.display_name.to_lowercase().contains(&query_lower)
            || unit.property.to_lowercase().contains(&query_lower);

        if matches {
            results.push(unit);
        }
    }

    // Sort results by relevance (exact matches first, then by code length)
    results.sort_by(|a, b| {
        let a_exact = a.code.to_lowercase() == query_lower;
        let b_exact = b.code.to_lowercase() == query_lower;

        match (a_exact, b_exact) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.code.len().cmp(&b.code.len()),
        }
    });

    results
}

/// Search for units by property (e.g., "length", "mass", "time").
///
/// Returns all units that have the specified property.
///
/// # Examples
///
/// ```
/// use octofhir_ucum_core::search_units_by_property;
///
/// let length_units = search_units_by_property("length");
/// for unit in length_units {
///     println!("{}: {}", unit.code, unit.display_name);
/// }
/// ```
pub fn search_units_by_property(property: &str) -> Vec<&'static UnitRecord> {
    let property_lower = property.to_lowercase();
    let mut results = Vec::new();

    for unit in get_all_units() {
        if unit.property.to_lowercase() == property_lower {
            results.push(unit);
        }
    }

    results
}

/// Get all defined forms of a unit code.
///
/// This includes the base unit and any prefixed variants that might exist.
/// Note: This is a simplified implementation that doesn't generate all possible
/// prefixed forms, but returns units that contain the base code.
///
/// # Examples
///
/// ```
/// use octofhir_ucum_core::get_defined_forms;
///
/// let forms = get_defined_forms("m");
/// for unit in forms {
///     println!("{}: {}", unit.code, unit.display_name);
/// }
/// ```
pub fn get_defined_forms(base_code: &str) -> Vec<&'static UnitRecord> {
    let mut results = Vec::new();

    // Add exact match if it exists
    if let Some(unit) = find_unit(base_code) {
        results.push(unit);
    }

    // Find units that might be prefixed versions
    for unit in get_all_units() {
        if unit.code != base_code && unit.code.ends_with(base_code) {
            // Check if the prefix part is a valid prefix
            let prefix_part = &unit.code[..unit.code.len() - base_code.len()];
            if find_prefix(prefix_part).is_some() {
                results.push(unit);
            }
        }
    }

    results
}

/// Search for units using regular expressions.
///
/// Returns a list of units that match the regex pattern in unit codes, names, or display names.
/// The search is case-insensitive by default.
///
/// # Arguments
/// * `pattern` - Regular expression pattern to match against
/// * `case_sensitive` - Whether the search should be case-sensitive
///
/// # Examples
///
/// ```
/// use octofhir_ucum_core::search_units_regex;
///
/// // Find all units containing "meter" or "metre"
/// let results = search_units_regex(r"mete?r", false).unwrap();
/// for unit in results {
///     println!("{}: {}", unit.code, unit.display_name);
/// }
/// ```
pub fn search_units_regex(
    pattern: &str,
    case_sensitive: bool,
) -> Result<Vec<&'static UnitRecord>, UcumError> {
    let regex_pattern = if case_sensitive {
        pattern.to_string()
    } else {
        format!("(?i){}", pattern)
    };

    let regex = Regex::new(&regex_pattern).map_err(|_| UcumError::InvalidExpression)?;

    let mut results = Vec::new();

    for unit in get_all_units() {
        let matches = regex.is_match(&unit.code)
            || regex.is_match(&unit.display_name)
            || regex.is_match(&unit.property);

        if matches {
            results.push(unit);
        }
    }

    // Sort by relevance (code matches first, then display name matches)
    results.sort_by(|a, b| {
        let a_code_match = regex.is_match(&a.code);
        let b_code_match = regex.is_match(&b.code);

        match (a_code_match, b_code_match) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.code.len().cmp(&b.code.len()),
        }
    });

    Ok(results)
}

/// Search for units using fuzzy matching.
///
/// Returns a list of units that fuzzy match the query string, sorted by match score.
/// Higher scores indicate better matches.
///
/// # Arguments
/// * `query` - Query string to fuzzy match against
/// * `threshold` - Minimum match score threshold (0-100, higher is more strict)
///
/// # Examples
///
/// ```
/// use octofhir_ucum_core::search_units_fuzzy;
///
/// // Find units similar to "meter" (will match "metre", "meter", etc.)
/// let results = search_units_fuzzy("meter", 50);
/// for (unit, score) in results {
///     println!("{}: {} (score: {})", unit.code, unit.display_name, score);
/// }
/// ```
pub fn search_units_fuzzy(query: &str, threshold: i64) -> Vec<(&'static UnitRecord, i64)> {
    let matcher = SkimMatcherV2::default();
    let mut results = Vec::new();

    for unit in get_all_units() {
        // Try matching against code, display name, and property
        let code_score = matcher.fuzzy_match(&unit.code, query).unwrap_or(0);
        let display_score = matcher.fuzzy_match(&unit.display_name, query).unwrap_or(0);
        let property_score = matcher.fuzzy_match(&unit.property, query).unwrap_or(0);

        // Use the best score among all fields
        let best_score = code_score.max(display_score).max(property_score);

        if best_score >= threshold {
            results.push((unit, best_score));
        }
    }

    // Sort by score (descending - best matches first)
    results.sort_by(|a, b| b.1.cmp(&a.1));

    results
}

/// Concept kinds for filtering search results.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConceptKind {
    /// Base units (meter, gram, second, etc.)
    BaseUnit,
    /// Derived units (newton, pascal, joule, etc.)
    DerivedUnit,
    /// Prefixed units (kilometer, milligram, etc.)
    PrefixedUnit,
    /// Arbitrary units (square-bracketed units)
    ArbitraryUnit,
    /// Special units (temperature, logarithmic, etc.)
    SpecialUnit,
}

/// Search for units with concept kind filtering.
///
/// Returns a list of units that match the query and belong to the specified concept kinds.
///
/// # Arguments
/// * `query` - Query string to search for
/// * `kinds` - List of concept kinds to include in results
/// * `use_fuzzy` - Whether to use fuzzy matching (if false, uses substring matching)
///
/// # Examples
///
/// ```
/// use octofhir_ucum_core::{search_units_filtered, ConceptKind};
///
/// // Find only base units containing "meter"
/// let results = search_units_filtered("meter", &[ConceptKind::BaseUnit], false);
/// for unit in results {
///     println!("{}: {}", unit.code, unit.display_name);
/// }
/// ```
pub fn search_units_filtered(
    query: &str,
    kinds: &[ConceptKind],
    use_fuzzy: bool,
) -> Vec<&'static UnitRecord> {
    let all_results = if use_fuzzy {
        search_units_fuzzy(query, 30)
            .into_iter()
            .map(|(unit, _score)| unit)
            .collect()
    } else {
        search_units(query)
    };

    all_results
        .into_iter()
        .filter(|unit| {
            let unit_kind = classify_unit(unit);
            kinds.contains(&unit_kind)
        })
        .collect()
}

/// Classify a unit into its concept kind.
fn classify_unit(unit: &UnitRecord) -> ConceptKind {
    // Check for arbitrary units (square brackets)
    if unit.code.starts_with('[') && unit.code.ends_with(']') {
        return ConceptKind::ArbitraryUnit;
    }

    // Check for special units
    if unit.special != crate::types::SpecialKind::None {
        return ConceptKind::SpecialUnit;
    }

    // Check for prefixed units by looking for known prefixes
    for prefix in crate::registry::PREFIXES {
        if unit.code.starts_with(prefix.symbol) && unit.code.len() > prefix.symbol.len() {
            let remainder = &unit.code[prefix.symbol.len()..];
            if find_unit(remainder).is_some() {
                return ConceptKind::PrefixedUnit;
            }
        }
    }

    // Check if it's a base unit (has simple dimension vector with single 1 or -1)
    let non_zero_dims = unit.dim.0.iter().filter(|&&x| x != 0).count();
    if non_zero_dims <= 1 && unit.dim.0.iter().any(|&x| x.abs() == 1) {
        ConceptKind::BaseUnit
    } else {
        ConceptKind::DerivedUnit
    }
}

// ============================================================================
// Phase 1: Core API Enhancement - Mathematical Operations
// ============================================================================

/// Multiply two unit expressions together.
///
/// This performs unit arithmetic by multiplying the factors and adding the dimensions.
/// The result represents the product of the two input units.
///
/// # Arguments
/// * `unit1` - First unit expression (e.g., "m")
/// * `unit2` - Second unit expression (e.g., "s")
///
/// # Returns
/// A `UnitArithmeticResult` containing the resulting unit information.
///
/// # Examples
///
/// ```
/// use octofhir_ucum_core::unit_multiply;
///
/// let result = unit_multiply("m", "s").unwrap();
/// println!("Result: {} (factor: {})", result.expression, result.factor);
/// // Output: Result: m.s (factor: 1)
/// ```
pub fn unit_multiply(unit1: &str, unit2: &str) -> Result<UnitArithmeticResult, UcumError> {
    let analysis1 = analyse(unit1)?;
    let analysis2 = analyse(unit2)?;

    // Check for offset units (not allowed in multiplication)
    if analysis1.has_offset || analysis2.has_offset {
        return Err(UcumError::ConversionError(
            "Offset units cannot be used in multiplication",
        ));
    }

    // Multiply factors
    let result_factor = analysis1.factor * analysis2.factor;

    // Add dimensions
    let mut result_dimension = [0i8; 7];
    for i in 0..7 {
        result_dimension[i] = analysis1.dimension.0[i].saturating_add(analysis2.dimension.0[i]);
    }

    // Build result expression string
    let result_expression = if unit1 == "1" {
        unit2.to_string()
    } else if unit2 == "1" {
        unit1.to_string()
    } else {
        format!("{}.{}", unit1, unit2)
    };

    Ok(UnitArithmeticResult {
        expression: result_expression,
        factor: result_factor,
        dimension: Dimension(result_dimension),
        offset: 0.0,
        is_dimensionless: result_dimension == [0; 7],
    })
}

/// Divide one unit expression by another.
///
/// This performs unit arithmetic by dividing the factors and subtracting the dimensions.
/// The result represents the quotient of the two input units.
///
/// # Arguments
/// * `numerator` - Numerator unit expression (e.g., "m")
/// * `denominator` - Denominator unit expression (e.g., "s")
///
/// # Returns
/// A `UnitArithmeticResult` containing the resulting unit information.
///
/// # Examples
///
/// ```
/// use octofhir_ucum_core::unit_divide;
///
/// let result = unit_divide("m", "s").unwrap();
/// println!("Result: {} (factor: {})", result.expression, result.factor);
/// // Output: Result: m/s (factor: 1)
/// ```
pub fn unit_divide(numerator: &str, denominator: &str) -> Result<UnitArithmeticResult, UcumError> {
    let analysis1 = analyse(numerator)?;
    let analysis2 = analyse(denominator)?;

    // Check for offset units (not allowed in division)
    if analysis1.has_offset || analysis2.has_offset {
        return Err(UcumError::ConversionError(
            "Offset units cannot be used in division",
        ));
    }

    // Divide factors
    let result_factor = analysis1.factor / analysis2.factor;

    // Subtract dimensions
    let mut result_dimension = [0i8; 7];
    for i in 0..7 {
        result_dimension[i] = analysis1.dimension.0[i].saturating_sub(analysis2.dimension.0[i]);
    }

    // Build result expression string
    let result_expression = if denominator == "1" {
        numerator.to_string()
    } else if numerator == "1" {
        format!("/{}", denominator)
    } else {
        format!("{}/{}", numerator, denominator)
    };

    Ok(UnitArithmeticResult {
        expression: result_expression,
        factor: result_factor,
        dimension: Dimension(result_dimension),
        offset: 0.0,
        is_dimensionless: result_dimension == [0; 7],
    })
}
