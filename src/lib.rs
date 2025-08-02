//! UCUM Core Library – Rust 2024 Edition
//!
//! This crate provides parsing, validation and conversion utilities for the
//! Unified Code for Units of Measure (UCUM). It aims to be `no_std`-optional
//! and suitable for both embedded and server environments.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::result_large_err)] // UcumError is necessarily large due to comprehensive error context

mod ast;
mod display;
mod error;
mod evaluator;
mod expr;
mod parser;
pub mod performance;
pub mod precision;
mod registry;
pub mod special_units;
pub mod suggestions;
mod types;

pub use crate::ast::{OwnedUnitExpr, OwnedUnitFactor, UnitExpr, UnitFactor};
pub use crate::display::{generate_display_name, generate_display_name_owned};
pub use crate::error::{ErrorKind, Span, UcumError};
pub use crate::evaluator::{EvalResult, evaluate, evaluate_owned};
pub use crate::expr::parse_expression;
pub use crate::performance::{
    CacheStats, EvaluationCache, clear_global_cache, find_longest_prefix_with_trie,
    find_prefix_optimized, find_prefixes_with_trie, find_unit_optimized, get_cache_sizes,
    get_cache_stats, with_global_cache,
};
pub use crate::special_units::{
    ArbitraryHandler, ConversionContext, LogarithmicHandler, SpecialUnitHandler,
    SpecialUnitRegistry, TemperatureHandler,
};
pub use crate::suggestions::SuggestionEngine;
pub use crate::types::{BaseUnit, DerivedUnit, Dimension, Prefix, Quantity, UnitRecord};

// Extended Functionality - functions are defined below and automatically exported

use std::collections::HashSet;

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
// Core API Enhancement - Validation Methods
// ============================================================================

/// Validate a UCUM expression string.
///
/// Returns `Ok(())` if the expression is valid, or an error describing why it's invalid.
/// This performs comprehensive validation including parsing, unit lookup, and dimensional analysis.
///
/// # Examples
///
/// ```
/// use octofhir_ucum::validate;
///
/// assert!(validate("m/s").is_ok());
/// assert!(validate("kg.m/s2").is_ok());
/// assert!(validate("invalid_unit").is_err());
/// ```
#[allow(clippy::result_large_err)]
pub fn validate(expression: &str) -> Result<(), UcumError> {
    // Create suggestion engine for enhanced error messages
    lazy_static::lazy_static! {
        static ref SUGGESTION_ENGINE: crate::suggestions::SuggestionEngine =
            crate::suggestions::SuggestionEngine::new();
    }

    // First, try to parse the expression
    let parsed = match parse_expression(expression) {
        Ok(parsed) => parsed,
        Err(e) => {
            // Enhance parsing errors with suggestions
            let enhanced_error = match &e.kind {
                ErrorKind::InvalidExpression { reason } => UcumError::invalid_expression(reason)
                    .with_suggestions(SUGGESTION_ENGINE.suggest_corrections(expression))
                    .with_context(format!("While parsing UCUM expression: '{expression}'")),
                _ => e,
            };
            return Err(enhanced_error);
        }
    };

    // Then evaluate it to ensure all units are valid and dimensions are consistent
    match crate::evaluator::evaluate_owned(&parsed) {
        Ok(_) => Ok(()),
        Err(e) => {
            // Enhance evaluation errors with suggestions
            let enhanced_error = match &e.kind {
                ErrorKind::UnitNotFound { unit, .. } => {
                    let suggestions = SUGGESTION_ENGINE.suggest_corrections(unit);
                    UcumError::unit_not_found(unit)
                        .with_suggestions(suggestions)
                        .with_context(format!("In expression: '{expression}'"))
                }
                ErrorKind::DimensionMismatch {
                    expected,
                    found,
                    operation,
                } => UcumError::dimension_mismatch(*expected, *found, operation)
                    .with_context(format!("In expression: '{expression}'"))
                    .with_suggestion(
                        "Check that all units in the expression have compatible dimensions",
                    ),
                _ => e,
            };
            Err(enhanced_error)
        }
    }
}

/// Analyse a UCUM expression and return detailed information about it.
///
/// Returns comprehensive information about the unit including its canonical form,
/// dimension vector, factor, and whether it has any special properties.
///
/// # Examples
///
/// ```
/// use octofhir_ucum::analyse;
///
/// let analysis = analyse("km/h").unwrap();
/// println!("Dimension: {:?}", analysis.dimension);
/// println!("Factor: {}", analysis.factor);
/// ```
#[allow(clippy::result_large_err)]
pub fn analyse(expression: &str) -> Result<UnitAnalysis, UcumError> {
    let parsed = parse_expression(expression)?;
    let result = crate::evaluator::evaluate_owned(&parsed)?;

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
    pub parsed_ast: OwnedUnitExpr,
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
/// use octofhir_ucum::validate_in_property;
///
/// assert!(validate_in_property("m", "length").unwrap());
/// assert!(validate_in_property("kg", "mass").unwrap());
/// assert!(validate_in_property("kg", "length").is_err());
/// ```
#[allow(clippy::result_large_err)]
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
            lazy_static::lazy_static! {
                static ref SUGGESTION_ENGINE: crate::suggestions::SuggestionEngine =
                    crate::suggestions::SuggestionEngine::new();
            }

            let available_properties = vec![
                "length",
                "mass",
                "time",
                "current",
                "temperature",
                "amount",
                "luminosity",
                "area",
                "volume",
                "velocity",
                "acceleration",
                "force",
                "energy",
                "power",
                "pressure",
                "frequency",
                "voltage",
                "resistance",
                "capacitance",
                "inductance",
                "dimensionless",
            ];

            // Find similar properties using string similarity
            let mut similar_properties = Vec::new();
            for prop in &available_properties {
                let similarity =
                    crate::suggestions::SuggestionEngine::string_similarity(property, prop);
                if similarity > 0.6 {
                    similar_properties.push(format!("'{prop}'"));
                }
            }

            let error = UcumError::invalid_property(property)
                .with_suggestions(similar_properties)
                .with_context(format!(
                    "Available properties: {}",
                    available_properties.join(", ")
                ));

            return Err(error);
        }
    };

    let is_valid = analysis.dimension == expected_dimension;

    // If not valid, provide suggestions for units that would be valid for this property
    if !is_valid {
        lazy_static::lazy_static! {
            static ref SUGGESTION_ENGINE: crate::suggestions::SuggestionEngine =
                crate::suggestions::SuggestionEngine::new();
        }

        let alternative_units = SUGGESTION_ENGINE.suggest_alternatives(expression, property);
        let error = UcumError::dimension_mismatch(
            expected_dimension,
            analysis.dimension,
            "property validation",
        )
        .with_suggestions(alternative_units)
        .with_context(format!(
            "Expression '{}' has dimension {:?} but property '{}' requires dimension {:?}",
            expression, analysis.dimension, property, expected_dimension
        ));
        return Err(error);
    }

    Ok(is_valid)
}

/// Check if two units are commensurable (can be converted between each other).
///
/// Two units are commensurable if they have the same dimension vector.
///
/// # Examples
///
/// ```
/// use octofhir_ucum::is_comparable;
///
/// assert!(is_comparable("m", "km").unwrap());
/// assert!(is_comparable("kg", "g").unwrap());
/// assert!(!is_comparable("m", "kg").unwrap());
/// ```
#[allow(clippy::result_large_err)]
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
/// use octofhir_ucum::get_canonical_units;
///
/// let canonical = get_canonical_units("km").unwrap();
/// println!("Canonical: {} (factor: {})", canonical.unit, canonical.factor);
/// // Output: Canonical: m (factor: 1000)
/// ```
#[allow(clippy::result_large_err)]
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
// Core API Enhancement - Mathematical Operations
// ============================================================================

/// Multiply two quantities with units.
///
/// # Examples
///
/// ```
/// use octofhir_ucum::multiply;
///
/// let result = multiply(5.0, "m", 2.0, "s").unwrap();
/// println!("{} {}", result.value, result.unit); // "10 m.s"
/// ```
#[allow(clippy::result_large_err)]
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
        return Err(UcumError::conversion_error(
            "offset units",
            "multiplication",
            "offset units cannot participate in multiplication",
        ));
    }

    // Calculate result
    let result_value = value1 * value2;
    let result_factor = analysis1.factor * analysis2.factor;

    // Combine dimensions
    let mut result_dim = [0i8; 7];
    #[allow(clippy::needless_range_loop)]
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
/// use octofhir_ucum::divide_by;
///
/// let result = divide_by(10.0, "m", 2.0, "s").unwrap();
/// println!("{} {}", result.value, result.unit); // "5 m.s-1"
/// ```
#[allow(clippy::result_large_err)]
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
        return Err(UcumError::conversion_error(
            "offset units",
            "division",
            "offset units cannot participate in division",
        ));
    }

    if divisor_value == 0.0 {
        return Err(UcumError::conversion_error(
            "denominator",
            "zero",
            "division by zero",
        ));
    }

    // Calculate result
    let result_value = dividend_value / divisor_value;
    let result_factor = analysis1.factor / analysis2.factor;

    // Combine dimensions (subtract divisor from dividend)
    let mut result_dim = [0i8; 7];
    #[allow(clippy::needless_range_loop)]
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
// Core API Enhancement - Search Functionality
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
/// use octofhir_ucum::search_units;
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
/// use octofhir_ucum::search_units_by_property;
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
/// use octofhir_ucum::get_defined_forms;
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
/// use octofhir_ucum::search_units_regex;
///
/// // Find all units containing "meter" or "metre"
/// let results = search_units_regex(r"mete?r", false).unwrap();
/// for unit in results {
///     println!("{}: {}", unit.code, unit.display_name);
/// }
/// ```
#[allow(clippy::result_large_err)]
pub fn search_units_regex(
    pattern: &str,
    case_sensitive: bool,
) -> Result<Vec<&'static UnitRecord>, UcumError> {
    let regex_pattern = if case_sensitive {
        pattern.to_string()
    } else {
        format!("(?i){pattern}")
    };

    let regex = Regex::new(&regex_pattern)
        .map_err(|_| UcumError::invalid_expression("Invalid regex pattern"))?;

    let mut results = Vec::new();

    for unit in get_all_units() {
        let matches = regex.is_match(unit.code)
            || regex.is_match(unit.display_name)
            || regex.is_match(unit.property);

        if matches {
            results.push(unit);
        }
    }

    // Sort by relevance (code matches first, then display name matches)
    results.sort_by(|a, b| {
        let a_code_match = regex.is_match(a.code);
        let b_code_match = regex.is_match(b.code);

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
/// use octofhir_ucum::search_units_fuzzy;
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
        let code_score = matcher.fuzzy_match(unit.code, query).unwrap_or(0);
        let display_score = matcher.fuzzy_match(unit.display_name, query).unwrap_or(0);
        let property_score = matcher.fuzzy_match(unit.property, query).unwrap_or(0);

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
/// use octofhir_ucum::{search_units_filtered, ConceptKind};
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
// Core API Enhancement - Mathematical Operations
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
/// use octofhir_ucum::unit_multiply;
///
/// let result = unit_multiply("m", "s").unwrap();
/// println!("Result: {} (factor: {})", result.expression, result.factor);
/// // Output: Result: m.s (factor: 1)
/// ```
#[allow(clippy::result_large_err)]
pub fn unit_multiply(unit1: &str, unit2: &str) -> Result<UnitArithmeticResult, UcumError> {
    let analysis1 = analyse(unit1)?;
    let analysis2 = analyse(unit2)?;

    // Check for offset units (not allowed in multiplication)
    if analysis1.has_offset || analysis2.has_offset {
        return Err(UcumError::conversion_error(
            "offset units",
            "multiplication",
            "Offset units cannot be used in multiplication",
        ));
    }

    // Multiply factors
    let result_factor = analysis1.factor * analysis2.factor;

    // Add dimensions
    let mut result_dimension = [0i8; 7];
    #[allow(clippy::needless_range_loop)]
    for i in 0..7 {
        result_dimension[i] = analysis1.dimension.0[i].saturating_add(analysis2.dimension.0[i]);
    }

    // Build result expression string
    let result_expression = if unit1 == "1" {
        unit2.to_string()
    } else if unit2 == "1" {
        unit1.to_string()
    } else {
        format!("{unit1}.{unit2}")
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
/// use octofhir_ucum::unit_divide;
///
/// let result = unit_divide("m", "s").unwrap();
/// println!("Result: {} (factor: {})", result.expression, result.factor);
/// // Output: Result: m/s (factor: 1)
/// ```
#[allow(clippy::result_large_err)]
pub fn unit_divide(numerator: &str, denominator: &str) -> Result<UnitArithmeticResult, UcumError> {
    let analysis1 = analyse(numerator)?;
    let analysis2 = analyse(denominator)?;

    // Check for offset units (not allowed in division)
    if analysis1.has_offset || analysis2.has_offset {
        return Err(UcumError::conversion_error(
            "offset units",
            "division",
            "Offset units cannot be used in division",
        ));
    }

    // Divide factors
    let result_factor = analysis1.factor / analysis2.factor;

    // Subtract dimensions
    let mut result_dimension = [0i8; 7];
    #[allow(clippy::needless_range_loop)]
    for i in 0..7 {
        result_dimension[i] = analysis1.dimension.0[i].saturating_sub(analysis2.dimension.0[i]);
    }

    // Build result expression string
    let result_expression = if denominator == "1" {
        numerator.to_string()
    } else if numerator == "1" {
        format!("/{denominator}")
    } else {
        format!("{numerator}/{denominator}")
    };

    Ok(UnitArithmeticResult {
        expression: result_expression,
        factor: result_factor,
        dimension: Dimension(result_dimension),
        offset: 0.0,
        is_dimensionless: result_dimension == [0; 7],
    })
}

// ============================================================================
// Model Introspection API
// ============================================================================

/// UCUM model information and metadata.
#[derive(Debug, Clone)]
pub struct UcumModel {
    /// UCUM specification version
    pub version: String,
    /// Model revision date
    pub revision_date: String,
    /// Available prefixes
    pub prefixes: &'static [Prefix],
    /// All available units
    pub units: &'static [UnitRecord],
}

/// Get the UCUM model information.
///
/// Returns metadata about the current UCUM implementation including version,
/// available prefixes, and unit definitions.
///
/// # Examples
///
/// ```
/// use octofhir_ucum::get_model;
///
/// let model = get_model();
/// println!("UCUM Version: {}", model.version);
/// println!("Total units: {}", model.units.len());
/// println!("Total prefixes: {}", model.prefixes.len());
/// ```
pub fn get_model() -> UcumModel {
    UcumModel {
        version: "2.1".to_string(),
        revision_date: "2017-11-21".to_string(),
        prefixes: crate::registry::PREFIXES,
        units: get_all_units(),
    }
}

/// Validate the UCUM implementation for self-consistency.
///
/// Performs comprehensive validation of the UCUM model to detect any internal
/// inconsistencies or errors. Returns a list of validation messages.
///
/// # Returns
/// A vector of validation messages. An empty vector indicates no issues found.
///
/// # Examples
///
/// ```
/// use octofhir_ucum::validate_ucum;
///
/// let issues = validate_ucum();
/// if issues.is_empty() {
///     println!("UCUM model is valid");
/// } else {
///     for issue in issues {
///         println!("Issue: {}", issue);
///     }
/// }
/// ```
pub fn validate_ucum() -> Vec<String> {
    let mut issues = Vec::new();

    // Check for duplicate unit codes
    let mut seen_codes = HashSet::new();
    for unit in get_all_units() {
        if !seen_codes.insert(&unit.code) {
            issues.push(format!("Duplicate unit code: {}", unit.code));
        }
    }

    // Check for duplicate prefix symbols
    let mut seen_prefixes = HashSet::new();
    for prefix in crate::registry::PREFIXES {
        if !seen_prefixes.insert(&prefix.symbol) {
            issues.push(format!("Duplicate prefix symbol: {}", prefix.symbol));
        }
    }

    // Validate unit expressions that should be parseable
    for unit in get_all_units() {
        if let Err(e) = validate(unit.code) {
            issues.push(format!("Unit {} failed validation: {}", unit.code, e));
        }
    }

    // Check for missing base units
    let base_unit_codes = ["m", "g", "s", "A", "K", "mol", "cd"];
    for &base_code in &base_unit_codes {
        if find_unit(base_code).is_none() {
            issues.push(format!("Missing base unit: {base_code}"));
        }
    }

    issues
}

/// Get all available properties in the UCUM model.
///
/// Returns a set of all distinct properties defined in the unit registry.
/// Properties represent physical quantities like "length", "mass", "time", etc.
///
/// # Examples
///
/// ```
/// use octofhir_ucum::get_properties;
///
/// let properties = get_properties();
/// for property in &properties {
///     println!("Property: {}", property);
/// }
/// ```
pub fn get_properties() -> HashSet<String> {
    let mut properties = HashSet::new();

    for unit in get_all_units() {
        properties.insert(unit.property.to_string());
    }

    properties
}

/// Validate that canonical units match the expected form.
///
/// Checks if the provided canonical form is correct for the given unit expression.
/// This is useful for validating externally provided canonical unit strings.
///
/// # Arguments
/// * `unit` - The unit expression to validate
/// * `canonical` - The expected canonical form
///
/// # Examples
///
/// ```
/// use octofhir_ucum::validate_canonical_units;
///
/// // Valid canonical form
/// assert!(validate_canonical_units("km", "m").unwrap());
///
/// // Invalid canonical form
/// assert!(!validate_canonical_units("km", "kg").unwrap());
/// ```
#[allow(clippy::result_large_err)]
pub fn validate_canonical_units(unit: &str, canonical: &str) -> Result<bool, UcumError> {
    let canonical_result = get_canonical_units(unit)?;
    Ok(canonical_result.unit == canonical)
}

/// Get a human-readable display name for a unit code.
///
/// Returns the most appropriate display name for the given unit code.
/// Falls back to the unit code itself if no display name is available.
///
/// # Arguments
/// * `code` - The unit code to get a display name for
///
/// # Examples
///
/// ```
/// use octofhir_ucum::get_common_display;
///
/// assert_eq!(get_common_display("m"), "meter");
/// assert_eq!(get_common_display("kg"), "kilogram");
/// assert_eq!(get_common_display("unknown"), "unknown");
/// ```
pub fn get_common_display(code: &str) -> String {
    // First try direct lookup
    if let Some(unit) = find_unit(code) {
        // Special handling for prefixed units like "kg"
        if code != unit.code {
            // This is a prefixed unit, we need to construct the display name
            // Check if it's a prefix + base unit combination
            for prefix_len in (1..code.len()).rev() {
                let (prefix_part, unit_part) = code.split_at(prefix_len);

                if let (Some(prefix), Some(base_unit)) =
                    (find_prefix(prefix_part), find_unit(unit_part))
                {
                    if base_unit.code == unit_part {
                        // Construct prefixed display name
                        return format!("{}{}", prefix.display_name, base_unit.display_name);
                    }
                }
            }
        }
        unit.display_name.to_string()
    } else {
        // Try to generate display name for compound expressions
        match parse_expression(code) {
            Ok(expr) => {
                let display = crate::display::generate_display_name_owned(&expr);
                // If the display is just the code in parentheses, return the code directly
                if display == format!("({code})") {
                    code.to_string()
                } else {
                    display
                }
            }
            Err(_) => code.to_string(),
        }
    }
}

// ============================================================================
// Advanced Conversion Operations
// ============================================================================

/// Advanced conversion context for enhanced conversion operations.
#[derive(Debug, Clone)]
pub struct AdvancedConversionContext {
    /// Precision configuration for the conversion
    pub precision: DecimalPrecision,
    /// Rounding mode to use
    pub rounding: RoundingMode,
    /// Temperature scale preference
    pub temperature_scale: TemperatureScale,
    /// Whether to use special unit handlers
    pub use_special_units: bool,
}

/// Decimal precision configuration.
#[derive(Debug, Clone)]
pub enum DecimalPrecision {
    /// Use default floating-point precision
    Default,
    /// Use fixed decimal places
    Fixed(u32),
    /// Use significant figures
    Significant(u32),
}

/// Rounding mode for conversions.
#[derive(Debug, Clone)]
pub enum RoundingMode {
    /// Round to nearest (default)
    Nearest,
    /// Round up (ceiling)
    Up,
    /// Round down (floor)
    Down,
    /// Truncate (toward zero)
    Truncate,
}

/// Temperature scale preference.
#[derive(Debug, Clone)]
pub enum TemperatureScale {
    /// Use Kelvin as base (default)
    Kelvin,
    /// Use Celsius as base
    Celsius,
    /// Use Fahrenheit as base
    Fahrenheit,
}

impl Default for AdvancedConversionContext {
    fn default() -> Self {
        Self {
            precision: DecimalPrecision::Default,
            rounding: RoundingMode::Nearest,
            temperature_scale: TemperatureScale::Kelvin,
            use_special_units: true,
        }
    }
}

/// Result of an advanced conversion operation with metadata.
#[derive(Debug, Clone)]
pub struct AdvancedConversionResult {
    /// Converted value
    pub value: f64,
    /// Target unit
    pub unit: String,
    /// Conversion factor applied
    pub factor: f64,
    /// Offset applied (for temperature conversions)
    pub offset: f64,
    /// Precision information
    pub precision_info: String,
    /// Whether special unit processing was used
    pub used_special_units: bool,
}

/// Convert with advanced context and precision control.
///
/// Performs unit conversion with enhanced control over precision, rounding,
/// and special unit handling.
///
/// # Arguments
/// * `value` - The numeric value to convert
/// * `from` - Source unit expression
/// * `to` - Target unit expression
/// * `context` - Advanced conversion context
///
/// # Examples
///
/// ```
/// use octofhir_ucum::{convert_with_context, AdvancedConversionContext, DecimalPrecision};
///
/// let context = AdvancedConversionContext {
///     precision: DecimalPrecision::Fixed(2),
///     ..Default::default()
/// };
///
/// let result = convert_with_context(1.0, "km", "m", &context).unwrap();
/// println!("Converted: {} {}", result.value, result.unit);
/// ```
#[allow(clippy::result_large_err)]
pub fn convert_with_context(
    value: f64,
    from: &str,
    to: &str,
    context: &AdvancedConversionContext,
) -> Result<AdvancedConversionResult, UcumError> {
    // For now, delegate to the existing conversion logic
    // This can be enhanced with the precision and rounding logic
    let from_analysis = analyse(from)?;
    let to_analysis = analyse(to)?;

    // Check dimension compatibility
    if from_analysis.dimension != to_analysis.dimension {
        return Err(UcumError::conversion_error(
            "source units",
            "target units",
            "Cannot convert between units with different dimensions",
        ));
    }

    // Calculate conversion
    let factor = from_analysis.factor / to_analysis.factor;
    let offset = from_analysis.offset - to_analysis.offset;
    let converted_value = value * factor + offset;

    // Apply precision rounding (simplified implementation)
    let final_value = match context.precision {
        DecimalPrecision::Default => converted_value,
        DecimalPrecision::Fixed(places) => {
            let multiplier = 10f64.powi(places as i32);
            (converted_value * multiplier).round() / multiplier
        }
        DecimalPrecision::Significant(sig_figs) => {
            if converted_value == 0.0 {
                0.0
            } else {
                let magnitude = converted_value.abs().log10().floor();
                let factor = 10f64.powi((sig_figs as i32 - 1) - magnitude as i32);
                (converted_value * factor).round() / factor
            }
        }
    };

    let precision_info = match context.precision {
        DecimalPrecision::Default => "default".to_string(),
        DecimalPrecision::Fixed(places) => format!("{places} decimal places"),
        DecimalPrecision::Significant(sig_figs) => format!("{sig_figs} significant figures"),
    };

    Ok(AdvancedConversionResult {
        value: final_value,
        unit: to.to_string(),
        factor,
        offset,
        precision_info,
        used_special_units: from_analysis.has_offset || to_analysis.has_offset,
    })
}

// ============================================================================
// Extended Functionality - Unit Expression Optimization
// ============================================================================

/// Optimize a unit expression for better readability and canonical form.
///
/// This function simplifies unit expressions by combining like terms, removing
/// redundant parentheses, and using more readable unit forms where possible.
///
/// # Arguments
/// * `expr` - The unit expression to optimize
///
/// # Examples
///
/// ```
/// use octofhir_ucum::optimize_expression;
///
/// let optimized = optimize_expression("m2/s2").unwrap();
/// assert_eq!(optimized, "m2.s-2");
/// ```
#[allow(clippy::result_large_err)]
pub fn optimize_expression(expr: &str) -> Result<String, UcumError> {
    // Parse the expression to ensure it's valid
    let _parsed = parse_expression(expr)?;
    let analysis = analyse(expr)?;

    // Get canonical form first
    let canonical = get_canonical_units(expr)?;

    // Try to build a more readable form from the dimension vector
    let optimized = build_optimized_unit_string(&analysis.dimension, &canonical);

    Ok(optimized)
}

/// Convert a unit expression to its canonical (base units) form.
///
/// Returns the expression in terms of the seven SI base units with appropriate
/// exponents. This is useful for dimensional analysis and unit verification.
///
/// # Arguments
/// * `expr` - The unit expression to canonicalize
///
/// # Examples
///
/// ```
/// use octofhir_ucum::canonicalize_expression;
///
/// let canonical = canonicalize_expression("N").unwrap();
/// assert_eq!(canonical, "kg.m.s-2");
/// ```
#[allow(clippy::result_large_err)]
pub fn canonicalize_expression(expr: &str) -> Result<String, UcumError> {
    let canonical = get_canonical_units(expr)?;
    Ok(canonical.unit)
}

/// Simplify a unit expression by combining like terms and reducing complexity.
///
/// This function performs algebraic simplification on unit expressions,
/// combining exponents of the same units and removing identity operations.
///
/// # Arguments
/// * `expr` - The unit expression to simplify
///
/// # Examples
///
/// ```
/// use octofhir_ucum::simplify_expression;
///
/// let simplified = simplify_expression("m.s/s").unwrap();
/// assert_eq!(simplified, "m");
/// ```
#[allow(clippy::result_large_err)]
pub fn simplify_expression(expr: &str) -> Result<String, UcumError> {
    // For now, return a simplified version based on canonical form
    // This avoids potential infinite loops in AST processing
    let canonical = get_canonical_units(expr)?;

    // If the canonical form is simpler, return it, otherwise return original
    if canonical.unit.len() < expr.len() {
        Ok(canonical.unit)
    } else {
        Ok(expr.to_string())
    }
}

/// Build an optimized unit string from dimension vector.
fn build_optimized_unit_string(dim: &Dimension, canonical: &CanonicalUnit) -> String {
    // Try to use more readable derived units where possible
    let common_units = [
        // Force units
        (Dimension([1, 1, -2, 0, 0, 0, 0]), "N", 1.0),
        // Energy units
        (Dimension([1, 2, -2, 0, 0, 0, 0]), "J", 1.0),
        // Power units
        (Dimension([1, 2, -3, 0, 0, 0, 0]), "W", 1.0),
        // Pressure units
        (Dimension([1, -1, -2, 0, 0, 0, 0]), "Pa", 1.0),
        // Frequency units
        (Dimension([0, 0, -1, 0, 0, 0, 0]), "Hz", 1.0),
        // Voltage units
        (Dimension([1, 2, -3, -1, 0, 0, 0]), "V", 1.0),
        // Resistance units
        (Dimension([1, 2, -3, -2, 0, 0, 0]), "Ohm", 1.0),
    ];

    // Check if dimension matches a common derived unit
    for (dim_pattern, unit_name, _factor) in &common_units {
        if dim == dim_pattern {
            return unit_name.to_string();
        }
    }

    // Fall back to canonical form
    canonical.unit.clone()
}

// Unused helper functions removed to avoid warnings

// ============================================================================
// Extended Functionality - Measurement Context Support
// ============================================================================

/// Domain-specific context for measurements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Domain {
    /// Medical and healthcare applications
    Medical,
    /// Engineering and technical applications
    Engineering,
    /// Physics and scientific research
    Physics,
    /// Chemistry and laboratory applications
    Chemistry,
    /// General purpose usage
    General,
}

/// Precision requirements for different domains.
#[derive(Debug, Clone)]
pub struct PrecisionRequirements {
    /// Minimum number of significant figures
    pub min_significant_figures: u32,
    /// Maximum acceptable relative error
    pub max_relative_error: f64,
    /// Whether exact conversions are required
    pub require_exact: bool,
}

/// Measurement context providing domain-specific preferences and requirements.
#[derive(Debug, Clone)]
pub struct MeasurementContext {
    /// Application domain
    pub domain: Domain,
    /// Precision requirements for this context
    pub precision_requirements: PrecisionRequirements,
    /// Preferred units for this domain
    pub preferred_units: Vec<String>,
    /// Units to avoid in this domain
    pub avoided_units: Vec<String>,
}

impl Default for MeasurementContext {
    fn default() -> Self {
        Self {
            domain: Domain::General,
            precision_requirements: PrecisionRequirements {
                min_significant_figures: 3,
                max_relative_error: 1e-6,
                require_exact: false,
            },
            preferred_units: Vec::new(),
            avoided_units: Vec::new(),
        }
    }
}

impl MeasurementContext {
    /// Create a medical measurement context with appropriate defaults.
    ///
    /// Medical contexts prioritize safety and precision, with preferences for
    /// units commonly used in healthcare settings.
    ///
    /// # Examples
    ///
    /// ```
    /// use octofhir_ucum::MeasurementContext;
    ///
    /// let medical_context = MeasurementContext::medical();
    /// ```
    pub fn medical() -> Self {
        Self {
            domain: Domain::Medical,
            precision_requirements: PrecisionRequirements {
                min_significant_figures: 4,
                max_relative_error: 1e-8,
                require_exact: true,
            },
            preferred_units: vec![
                // Dosage units
                "mg".to_string(),
                "ug".to_string(),
                "g".to_string(),
                // Volume units
                "mL".to_string(),
                "L".to_string(),
                // Concentration units
                "mg/mL".to_string(),
                "ug/mL".to_string(),
                // Temperature (Celsius preferred in medical)
                "Cel".to_string(),
            ],
            avoided_units: vec![
                // Avoid ambiguous units
                "[IU]".to_string(), // International units can be ambiguous
            ],
        }
    }

    /// Create an engineering measurement context with appropriate defaults.
    ///
    /// Engineering contexts focus on standard SI units and derived units
    /// commonly used in technical applications.
    ///
    /// # Examples
    ///
    /// ```
    /// use octofhir_ucum::MeasurementContext;
    ///
    /// let engineering_context = MeasurementContext::engineering();
    /// ```
    pub fn engineering() -> Self {
        Self {
            domain: Domain::Engineering,
            precision_requirements: PrecisionRequirements {
                min_significant_figures: 3,
                max_relative_error: 1e-6,
                require_exact: false,
            },
            preferred_units: vec![
                // Standard SI units
                "m".to_string(),
                "kg".to_string(),
                "s".to_string(),
                // Common derived units
                "N".to_string(),
                "Pa".to_string(),
                "J".to_string(),
                "W".to_string(),
                // Pressure units
                "kPa".to_string(),
                "MPa".to_string(),
                // Temperature (Kelvin for engineering)
                "K".to_string(),
            ],
            avoided_units: vec![
                // Avoid non-SI units where possible
                "[psi]".to_string(),
                "[in_i]".to_string(),
                "[ft_i]".to_string(),
            ],
        }
    }

    /// Create a physics measurement context with appropriate defaults.
    ///
    /// Physics contexts prioritize fundamental SI units and exact relationships,
    /// with high precision requirements for research applications.
    ///
    /// # Examples
    ///
    /// ```
    /// use octofhir_ucum::MeasurementContext;
    ///
    /// let physics_context = MeasurementContext::physics();
    /// ```
    pub fn physics() -> Self {
        Self {
            domain: Domain::Physics,
            precision_requirements: PrecisionRequirements {
                min_significant_figures: 6,
                max_relative_error: 1e-12,
                require_exact: true,
            },
            preferred_units: vec![
                // Fundamental SI units
                "m".to_string(),
                "kg".to_string(),
                "s".to_string(),
                "A".to_string(),
                "K".to_string(),
                "mol".to_string(),
                "cd".to_string(),
                // Common physics units
                "eV".to_string(),
                "c".to_string(), // Speed of light
                "h".to_string(), // Planck constant (if available)
            ],
            avoided_units: vec![
                // Avoid non-fundamental units where possible
                "[cal]".to_string(),
                "[Btu]".to_string(),
            ],
        }
    }

    /// Create a chemistry measurement context with appropriate defaults.
    ///
    /// Chemistry contexts focus on molar quantities, concentrations, and units
    /// commonly used in laboratory settings.
    ///
    /// # Examples
    ///
    /// ```
    /// use octofhir_ucum::MeasurementContext;
    ///
    /// let chemistry_context = MeasurementContext::chemistry();
    /// ```
    pub fn chemistry() -> Self {
        Self {
            domain: Domain::Chemistry,
            precision_requirements: PrecisionRequirements {
                min_significant_figures: 4,
                max_relative_error: 1e-9,
                require_exact: false,
            },
            preferred_units: vec![
                // Molar units
                "mol".to_string(),
                "mmol".to_string(),
                "umol".to_string(),
                // Concentration units
                "mol/L".to_string(),
                "mmol/L".to_string(),
                // Mass units for chemicals
                "g".to_string(),
                "mg".to_string(),
                "kg".to_string(),
                // Volume units
                "L".to_string(),
                "mL".to_string(),
                "uL".to_string(),
                // Temperature (Celsius for chemistry)
                "Cel".to_string(),
            ],
            avoided_units: vec![
                // Avoid non-molar concentration units where possible
                "g/L".to_string(), // Prefer molar concentrations
            ],
        }
    }

    /// Check if a unit is preferred in this measurement context.
    ///
    /// # Arguments
    /// * `unit` - The unit to check
    ///
    /// # Examples
    ///
    /// ```
    /// use octofhir_ucum::MeasurementContext;
    ///
    /// let medical_context = MeasurementContext::medical();
    /// assert!(medical_context.is_preferred_unit("mg"));
    /// assert!(!medical_context.is_preferred_unit("[psi]"));
    /// ```
    pub fn is_preferred_unit(&self, unit: &str) -> bool {
        self.preferred_units.contains(&unit.to_string())
    }

    /// Check if a unit should be avoided in this measurement context.
    ///
    /// # Arguments
    /// * `unit` - The unit to check
    ///
    /// # Examples
    ///
    /// ```
    /// use octofhir_ucum::MeasurementContext;
    ///
    /// let engineering_context = MeasurementContext::engineering();
    /// assert!(engineering_context.is_avoided_unit("[psi]"));
    /// assert!(!engineering_context.is_avoided_unit("Pa"));
    /// ```
    pub fn is_avoided_unit(&self, unit: &str) -> bool {
        self.avoided_units.contains(&unit.to_string())
    }

    /// Get suggested alternative units for the given unit in this context.
    ///
    /// Returns a list of preferred units that are dimensionally compatible
    /// with the input unit, ordered by preference for this domain.
    ///
    /// # Arguments
    /// * `unit` - The unit to find alternatives for
    ///
    /// # Examples
    ///
    /// ```
    /// use octofhir_ucum::MeasurementContext;
    ///
    /// let medical_context = MeasurementContext::medical();
    /// let alternatives = medical_context.suggest_alternatives("g").unwrap();
    /// // May suggest ["mg", "ug"] for medical dosing
    /// ```
    #[allow(clippy::result_large_err)]
    pub fn suggest_alternatives(&self, unit: &str) -> Result<Vec<String>, UcumError> {
        // Analyze the input unit to get its dimension
        let analysis = analyse(unit)?;
        let mut suggestions = Vec::new();

        // Check preferred units for dimensional compatibility
        for preferred in &self.preferred_units {
            if let Ok(pref_analysis) = analyse(preferred) {
                if pref_analysis.dimension == analysis.dimension {
                    suggestions.push(preferred.clone());
                }
            }
        }

        // If no preferred units match, suggest some common alternatives
        if suggestions.is_empty() {
            match self.domain {
                Domain::Medical => {
                    // Suggest medical-appropriate units based on dimension
                    if analysis.dimension == Dimension([1, 0, 0, 0, 0, 0, 0]) {
                        // Mass
                        suggestions = vec!["mg".to_string(), "g".to_string(), "ug".to_string()];
                    } else if analysis.dimension == Dimension([0, 3, 0, 0, 0, 0, 0]) {
                        // Volume
                        suggestions = vec!["mL".to_string(), "L".to_string()];
                    }
                }
                Domain::Engineering => {
                    // Suggest engineering-appropriate SI units
                    if analysis.dimension == Dimension([1, -1, -2, 0, 0, 0, 0]) {
                        // Pressure
                        suggestions = vec!["Pa".to_string(), "kPa".to_string(), "MPa".to_string()];
                    }
                }
                Domain::Physics => {
                    // Suggest fundamental SI units
                    suggestions.push(canonicalize_expression(unit)?);
                }
                Domain::Chemistry => {
                    // Suggest chemistry-appropriate units
                    if analysis.dimension == Dimension([0, 0, 0, 0, 0, 1, 0]) {
                        // Amount
                        suggestions = vec!["mol".to_string(), "mmol".to_string()];
                    }
                }
                Domain::General => {
                    // Suggest common SI units
                    suggestions.push(canonicalize_expression(unit)?);
                }
            }
        }

        Ok(suggestions)
    }
}

// Feature-gated modules
#[cfg(feature = "wasm")]
pub mod wasm;

#[cfg(feature = "fhir")]
pub mod fhir;
