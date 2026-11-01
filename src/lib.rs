//! The Unified Code for Units of Measure (UCUM): parsing, validation, comparison and
//! conversion of unit expressions, following the UCUM specification (version 2.2).
//!
//! The crate is `no_std` + `alloc` compatible (disable default features).

#![no_std]
#![allow(clippy::result_large_err)] // UcumError is necessarily large due to comprehensive error context

extern crate alloc;
#[cfg(any(feature = "std", test))]
#[cfg_attr(test, macro_use)]
extern crate std;

mod ast;
mod display;
mod error;
mod evaluator;
mod math;
mod number;
mod parser;
mod prelude;
mod registry;
mod suggestions;
mod types;

#[cfg(feature = "fhir")]
pub mod fhir;

pub use crate::ast::UnitExpr;
pub use crate::display::generate_display_name;
pub use crate::error::{ErrorKind, UcumError};
pub use crate::evaluator::{
    Case, EvalResult, SpecialTerm, evaluate, evaluate_with_case, lookup_unit,
};
pub use crate::number::Number;
pub use crate::parser::{MAX_NESTING_DEPTH, parse_expression};
pub use crate::registry::{UCUM_REVISION_DATE, UCUM_VERSION};
pub use crate::types::{Dimension, Prefix, Quantity, SpecialKind, UnitRecord};

use crate::prelude::*;
use crate::suggestions::SuggestionEngine;
use alloc::borrow::Cow;
use alloc::collections::BTreeSet;
use hashbrown::HashSet;

/// Shared suggestion engine used to enrich validation errors.
static SUGGESTION_ENGINE: spin::LazyLock<SuggestionEngine> =
    spin::LazyLock::new(SuggestionEngine::new);

// ============================================================================
// Registry
// ============================================================================

/// The unit atom with this case sensitive code (`m`, `[in_i]`), without a prefix. Use
/// `lookup_unit` for a simple unit with a prefix (`km`).
pub fn find_unit(code: &str) -> Option<&'static UnitRecord> {
    evaluator::find_atom(code, Case::Sensitive)
}

/// The prefix with this case sensitive symbol (`k`, `da`).
pub fn find_prefix(symbol: &str) -> Option<&'static Prefix> {
    evaluator::find_prefix(symbol, Case::Sensitive)
}

/// All unit atoms of the UCUM tables, sorted by code.
pub fn get_all_units() -> &'static [UnitRecord] {
    registry::UNITS
}

/// All prefixes of the UCUM tables, sorted by symbol.
pub fn get_all_prefixes() -> &'static [Prefix] {
    registry::PREFIXES
}

// ============================================================================
// Validation and analysis
// ============================================================================

/// Validate a UCUM expression with case sensitive symbols: its syntax (UCUM §1-10) and its
/// semantics (every symbol is a unit, prefixes are on metric units only, special units take
/// part in no algebra).
///
/// ```
/// use octofhir_ucum::validate;
///
/// assert!(validate("m/s").is_ok());
/// assert!(validate("kg.m/s2").is_ok());
/// assert!(validate("invalid_unit").is_err());
/// ```
pub fn validate(expression: &str) -> Result<(), UcumError> {
    validate_with_case(expression, Case::Sensitive)
}

/// Validate a UCUM expression, with case sensitive or case insensitive symbols (UCUM §3.4).
///
/// ```
/// use octofhir_ucum::{Case, validate_with_case};
///
/// assert!(validate_with_case("PAL", Case::Insensitive).is_ok());
/// assert!(validate_with_case("PAL", Case::Sensitive).is_err());
/// ```
pub fn validate_with_case(expression: &str, case: Case) -> Result<(), UcumError> {
    let parsed = parse_expression(expression).map_err(|e| {
        // No unit to suggest for an expression nested too deep, which can be long: the
        // suggestions compare the whole input with every unit
        let too_deep = matches!(e.kind, ErrorKind::NestingTooDeep { .. });
        let suggestions = if too_deep {
            Vec::new()
        } else {
            SUGGESTION_ENGINE.suggest_corrections(expression)
        };
        e.with_suggestions(suggestions).with_context(format!(
            "While parsing UCUM expression: {}",
            error::quoted(expression)
        ))
    })?;
    evaluate_in(&parsed, expression, case)
        .map(|_| ())
        .map_err(|e| e.with_context(format!("In expression: {}", error::quoted(expression))))
}

/// Evaluate `expr`, parsed from `expression`. A unit that is not in the tables gets the
/// expression, where its symbol starts in it, and corrections. Only the unknown symbol is
/// compared with the units, so this is cheap, unlike the suggestions for a syntax error,
/// which `validate` alone computes. The suggestions are case sensitive codes, which mean
/// other units when read case insensitively (§3.4: `Pa` is `PA`, the picoampere), so a
/// case insensitive lookup gets none.
pub(crate) fn evaluate_in(
    expr: &UnitExpr,
    expression: &str,
    case: Case,
) -> Result<EvalResult, UcumError> {
    evaluate_with_case(expr, case).map_err(|mut e| {
        if let ErrorKind::UnitNotFound {
            unit,
            expression: text,
            position,
            ..
        } = &mut e.kind
        {
            *position = parser::symbol_offset(expression, unit);
            *text = expression.to_string();
            if case == Case::Sensitive {
                e.suggestions
                    .extend(SUGGESTION_ENGINE.suggest_corrections(unit));
            }
        }
        e
    })
}

/// Parse and evaluate an expression with case sensitive symbols.
fn eval(expression: &str) -> Result<(UnitExpr, EvalResult), UcumError> {
    let parsed = parse_expression(expression)?;
    let result = evaluate_in(&parsed, expression, Case::Sensitive)?;
    Ok((parsed, result))
}

/// Analyse a UCUM expression: its dimension, magnitude, arbitrary atoms and special unit.
///
/// ```
/// use octofhir_ucum::analyse;
///
/// let analysis = analyse("km/h").unwrap();
/// assert_eq!(analysis.dimension.to_string(), "L.T-1");
/// assert!((analysis.factor.to_f64() - 1000.0 / 3600.0).abs() < 1e-15);
/// // To 28 significant digits, whatever the magnitude
/// assert_eq!(analysis.factor.to_string(), "0.2777777777777777777777777778");
/// assert_eq!(analyse("10*1000").unwrap().factor.to_string(), "1e1000");
/// ```
pub fn analyse(expression: &str) -> Result<UnitAnalysis, UcumError> {
    let (parsed, result) = eval(expression)?;
    let scale = result.special.map_or(Number::ONE, |special| special.scale);
    let offset = temperature_offset(&result).unwrap_or(Number::ZERO);
    Ok(UnitAnalysis {
        expression: expression.to_string(),
        parsed_ast: parsed,
        dimension: result.dim,
        factor: result.factor,
        scale,
        offset,
        is_dimensionless: result.dim.is_dimensionless() && result.arbitrary.is_empty(),
        arbitrary: result.arbitrary,
        special: result.special.map(|s| s.unit.special),
    })
}

/// For a temperature (UCUM §22.4), the proper quantity at 0 in base units: the offset of
/// its function times the factor of its proper unit, 273.15 for `Cel`. `None` for another
/// unit.
fn temperature_offset(result: &EvalResult) -> Option<Number> {
    // The factor is that of the proper unit (1 K, 5/9 K), since a prefix or a number goes
    // into the scale (§22.3), so the product fits
    result
        .special?
        .unit
        .special
        .offset()?
        .checked_mul(result.factor)
}

/// Detailed analysis of a UCUM expression.
#[derive(Debug, Clone)]
pub struct UnitAnalysis {
    /// Original expression string
    pub expression: String,
    /// Parsed expression
    pub parsed_ast: UnitExpr,
    /// Dimension; for a special unit, of its corresponding proper unit
    pub dimension: Dimension,
    /// Magnitude in base units, as computed (28 significant digits, any exponent); for a
    /// special unit, of its corresponding proper unit
    pub factor: Number,
    /// Scale of a special unit (UCUM §22.4): 0.1 for `dB`, 2 for `2.Cel`; 1 otherwise
    pub scale: Number,
    /// For a temperature, the proper quantity at 0 in base units: 273.15 for `Cel`, so that
    /// a value `r` is `factor × scale × r + offset` kelvins; 0 otherwise
    pub offset: Number,
    /// Arbitrary atoms and their exponents (`[IU]/mL` has `[("[iU]", 1)]`)
    pub arbitrary: Vec<(&'static str, i32)>,
    /// The kind of the special unit, if the expression is one
    pub special: Option<SpecialKind>,
    /// Whether the unit has no dimension and no arbitrary atom
    pub is_dimensionless: bool,
}

/// Whether two expressions are commensurable (UCUM §17): the same dimension and the same
/// arbitrary atoms. A special unit is commensurable with its corresponding proper unit
/// (§21.3): `Cel` with `K`, `B[V]` with `V`, `[pH]` with `mol/L`. Commensurable units
/// convert to each other with `convert`.
///
/// ```
/// use octofhir_ucum::is_comparable;
///
/// assert!(is_comparable("m", "km").unwrap());
/// assert!(is_comparable("Cel", "[degF]").unwrap());
/// assert!(!is_comparable("m", "kg").unwrap());
/// assert!(!is_comparable("[IU]", "[arb'U]").unwrap());
/// ```
pub fn is_comparable(unit1: &str, unit2: &str) -> Result<bool, UcumError> {
    let (_, a) = eval(unit1)?;
    let (_, b) = eval(unit2)?;
    Ok(a.is_commensurable(&b))
}

/// Validate that a unit expression measures a kind of quantity: that it is commensurable
/// with a unit of that property in the UCUM tables (`length`, `pressure`, `electric
/// current`, see `get_properties`). The legacy names `current`, `amount`, `luminosity` and
/// `dimensionless` are also accepted.
///
/// ```
/// use octofhir_ucum::validate_in_property;
///
/// assert!(validate_in_property("m", "length").unwrap());
/// assert!(validate_in_property("[psi]", "pressure").unwrap());
/// assert!(!validate_in_property("kg", "length").unwrap());
/// ```
///
/// # Errors
///
/// An expression that is not valid, or a property that no unit has.
pub fn validate_in_property(expression: &str, property: &str) -> Result<bool, UcumError> {
    let (_, result) = eval(expression)?;
    let property = match property.to_lowercase().as_str() {
        "current" => "electric current".to_string(),
        "amount" => "amount of substance".to_string(),
        "luminosity" => "luminous intensity".to_string(),
        "dimensionless" => {
            return Ok(result.dim.is_dimensionless() && result.arbitrary.is_empty());
        }
        other => other.to_string(),
    };
    let mut known = false;
    for unit in get_all_units() {
        if unit.property.to_lowercase() != property {
            continue;
        }
        known = true;
        let unit_result = evaluate(&UnitExpr::Symbol(unit.code.to_string()))?;
        if unit_result.is_commensurable(&result) {
            return Ok(true);
        }
    }
    if known {
        return Ok(false);
    }
    let property_len = property.chars().count();
    let similar: Vec<String> = get_properties()
        .into_iter()
        .filter(|p| {
            SuggestionEngine::similarity_above(&property, property_len, p, p.chars().count(), 0.6)
                .is_some()
        })
        .map(|p| format!("'{p}'"))
        .collect();
    Err(UcumError::invalid_property(&property).with_suggestions(similar))
}

// ============================================================================
// Canonical form
// ============================================================================

/// The canonical unit of an expression: its base units (`g`, `m`, `s`, `rad`, `K`, `C`,
/// `cd`) and arbitrary atoms, with the factor and offset that convert a value to it.
///
/// ```
/// use octofhir_ucum::get_canonical_units;
///
/// let canonical = get_canonical_units("km").unwrap();
/// assert_eq!(canonical.unit, "m");
/// assert_eq!(canonical.factor.to_f64(), 1000.0);
/// ```
///
/// # Errors
///
/// A special unit on a logarithmic or other non-linear scale (`B`, `[pH]`, `[p'diop]`) is a
/// `SpecialUnitError`: no factor and offset turn its function into the canonical value.
/// Temperatures have a canonical unit, `K`, and an offset.
pub fn get_canonical_units(expression: &str) -> Result<CanonicalUnit, UcumError> {
    let (_, result) = eval(expression)?;
    let (factor, offset) = match result.special {
        None => (result.factor, Number::ZERO),
        Some(special) => {
            let offset = temperature_offset(&result).ok_or_else(|| {
                UcumError::special_unit_error(
                    expression,
                    "a unit on a logarithmic or non-linear scale has no linear canonical unit",
                )
            })?;
            let factor = special
                .scale
                .checked_mul(result.factor)
                .ok_or_else(|| UcumError::precision_overflow("canonical form", expression))?;
            (factor, offset)
        }
    };
    Ok(CanonicalUnit {
        unit: canonical_unit_string(&result.dim, &result.arbitrary),
        factor,
        offset,
        dimension: result.dim,
    })
}

/// Canonical unit representation: a value `v` in the unit is `v × factor + offset` in the
/// canonical unit.
#[derive(Debug, Clone)]
pub struct CanonicalUnit {
    /// Canonical unit (`g.m.s-2` for the newton)
    pub unit: String,
    /// Magnitude of the unit in the canonical unit, as computed (28 significant digits, any
    /// exponent)
    pub factor: Number,
    /// Offset of a temperature
    pub offset: Number,
    /// Dimension vector
    pub dimension: Dimension,
}

/// Base units in the order of canonical units.
const CANONICAL_ORDER: [usize; 7] = [2, 0, 1, 3, 4, 5, 6];

/// The canonical unit of a dimension and arbitrary atoms: "g.m.s-2", "m-3.[iU]", "1".
fn canonical_unit_string(dim: &Dimension, arbitrary: &[(&'static str, i32)]) -> String {
    let mut parts: Vec<String> = Vec::new();
    for slot in CANONICAL_ORDER {
        let exp = dim.0[slot];
        let code = Dimension::BASE_UNITS[slot];
        match exp {
            0 => {}
            1 => parts.push(code.to_string()),
            _ => parts.push(format!("{code}{exp}")),
        }
    }
    for &(atom, exp) in arbitrary {
        parts.push(if exp == 1 {
            atom.to_string()
        } else {
            format!("{atom}{exp}")
        });
    }
    if parts.is_empty() {
        "1".to_string()
    } else {
        parts.join(".")
    }
}

/// Validate that `canonical` is the canonical form of `unit`: equal to it, with the same
/// magnitude (`validate_canonical_units("km", "1000.m")`), or the canonical unit when the
/// magnitude is 1 (`("N", "kg.m/s2")`).
///
/// ```
/// use octofhir_ucum::validate_canonical_units;
///
/// assert!(validate_canonical_units("N", "kg.m.s-2").unwrap());
/// assert!(validate_canonical_units("C", "A.s").unwrap());
/// assert!(!validate_canonical_units("km", "m").unwrap());
/// ```
pub fn validate_canonical_units(unit: &str, canonical: &str) -> Result<bool, UcumError> {
    let (_, a) = eval(unit)?;
    let (_, b) = eval(canonical)?;
    Ok(
        a.special.is_none()
            && b.special.is_none()
            && a.is_commensurable(&b)
            && a.factor == b.factor,
    )
}

/// A positive number written in UCUM, which has positive integers only: "1000", "10*-2",
/// "254.10*-4", "602214076.10*15".
fn ucum_number(n: Number) -> String {
    let (digits, exp) = n.to_integer_and_exponent();
    if (0..=3).contains(&exp) {
        format!("{digits}{}", "0".repeat(exp as usize))
    } else if digits == 1 {
        format!("10*{exp}")
    } else {
        format!("{digits}.10*{exp}")
    }
}

// ============================================================================
// Quantity and unit arithmetic
// ============================================================================

/// Reject special units, which take part in no arithmetic (UCUM §22.1).
fn check_arithmetic_operand(
    expression: &str,
    result: &EvalResult,
    operation: &str,
) -> Result<(), UcumError> {
    match result.special {
        Some(special) => Err(UcumError::special_unit_error(
            expression,
            &format!(
                "{} is a special unit, which cannot be used in {operation} (UCUM §22.1)",
                special.unit.code
            ),
        )),
        None => Ok(()),
    }
}

/// A result of quantity arithmetic must fit in an `f64`: an infinite value, or a zero
/// that only comes from a result too small, is a `PrecisionOverflow`.
fn check_number(
    value: f64,
    is_zero_expected: bool,
    overflow: impl Fn() -> UcumError,
) -> Result<f64, UcumError> {
    if value.is_finite() && (value != 0.0 || is_zero_expected) {
        Ok(value)
    } else {
        Err(overflow())
    }
}

/// `value × factor` in base units, exactly.
fn base_value(value: f64, factor: Number) -> Option<Number> {
    Number::from_f64(value)?.checked_mul(factor)
}

/// Multiply two quantities. The result is in canonical units.
///
/// ```
/// use octofhir_ucum::multiply;
///
/// let result = multiply(5.0, "m", 2.0, "s").unwrap();
/// assert_eq!(result.value, 10.0);
/// assert_eq!(result.unit, "m.s");
/// ```
///
/// # Errors
///
/// A special unit (`Cel`, `B`, `[pH]`) takes no part in the product (UCUM §22.1) and is a
/// `SpecialUnitError`. An operand that is infinite or NaN is a `NonFiniteValue` (UCUM §18.5),
/// a result that does not fit in an `f64` is a `PrecisionOverflow`.
pub fn multiply(
    value1: f64,
    unit1: &str,
    value2: f64,
    unit2: &str,
) -> Result<UnitResult, UcumError> {
    arithmetic(value1, unit1, value2, unit2, 1, "multiplication")
}

/// Divide two quantities. The result is in canonical units.
///
/// ```
/// use octofhir_ucum::divide_by;
///
/// let result = divide_by(10.0, "m", 2.0, "s").unwrap();
/// assert_eq!(result.value, 5.0);
/// assert_eq!(result.unit, "m.s-1");
/// ```
///
/// # Errors
///
/// As `multiply`; a divisor value of zero is a `DivisionByZero`.
pub fn divide_by(
    dividend_value: f64,
    dividend_unit: &str,
    divisor_value: f64,
    divisor_unit: &str,
) -> Result<UnitResult, UcumError> {
    arithmetic(
        dividend_value,
        dividend_unit,
        divisor_value,
        divisor_unit,
        -1,
        "division",
    )
}

fn arithmetic(
    value1: f64,
    unit1: &str,
    value2: f64,
    unit2: &str,
    exp: i32,
    operation: &str,
) -> Result<UnitResult, UcumError> {
    let (_, a) = eval(unit1)?;
    let (_, b) = eval(unit2)?;
    check_arithmetic_operand(unit1, &a, operation)?;
    check_arithmetic_operand(unit2, &b, operation)?;
    let operator = if exp == 1 { '*' } else { '/' };
    // The kind keeps the units whole, the message shows each one shortened
    let operands = |show: fn(&str) -> Cow<'_, str>| {
        format!(
            "{value1:?} {} {operator} {value2:?} {}",
            show(unit1),
            show(unit2)
        )
    };
    let (whole, shown) = (error::whole, error::excerpt);
    // A scalar is a real number (UCUM §18.5)
    if !value1.is_finite() || !value2.is_finite() {
        return Err(UcumError::non_finite_value_showing(
            operation,
            &operands(whole),
            &operands(shown),
        ));
    }
    if exp == -1 && value2 == 0.0 {
        return Err(UcumError::division_by_zero_showing(
            operation,
            &operands(whole),
            &operands(shown),
        ));
    }
    let overflow =
        || UcumError::precision_overflow_showing(operation, &operands(whole), &operands(shown));
    let left = base_value(value1, a.factor).ok_or_else(overflow)?;
    let right = base_value(value2, b.factor).ok_or_else(overflow)?;
    let value = if exp == 1 {
        left.checked_mul(right)
    } else {
        left.checked_div(right)
    }
    .ok_or_else(overflow)?;
    let mut dim = a.dim.0;
    evaluator::add_scaled_dim(&mut dim, &b.dim, exp)?;
    let arbitrary = evaluator::combine(&a.arbitrary, &b.arbitrary, exp);
    let dim = Dimension(dim);
    Ok(UnitResult {
        value: check_number(value.to_f64(), value.is_zero(), overflow)?,
        unit: canonical_unit_string(&dim, &arbitrary),
        dimension: dim,
    })
}

/// Result of quantity arithmetic.
#[derive(Debug, Clone)]
pub struct UnitResult {
    /// Value in `unit`
    pub value: f64,
    /// Canonical unit of the result
    pub unit: String,
    /// Dimension of the result
    pub dimension: Dimension,
}

/// Result of unit arithmetic.
#[derive(Debug, Clone)]
pub struct UnitArithmeticResult {
    /// Resulting unit expression
    pub expression: String,
    /// Magnitude of the result in base units, as computed (28 significant digits, any
    /// exponent)
    pub factor: Number,
    /// Dimension of the result
    pub dimension: Dimension,
    /// Whether the result has no dimension and no arbitrary atom
    pub is_dimensionless: bool,
}

/// Multiply two unit expressions.
///
/// ```
/// use octofhir_ucum::unit_multiply;
///
/// let result = unit_multiply("m", "s").unwrap();
/// assert_eq!(result.expression, "m.s");
/// ```
///
/// # Errors
///
/// A special unit is a `SpecialUnitError` (UCUM §22.1). Operands that, joined, nest deeper
/// than [`MAX_NESTING_DEPTH`] levels are a `NestingTooDeep`. A factor whose power of
/// ten does not fit in a `Number` (`10*2000000000` times itself) is a `PrecisionOverflow`.
pub fn unit_multiply(unit1: &str, unit2: &str) -> Result<UnitArithmeticResult, UcumError> {
    let (left, a) = eval(unit1)?;
    let (right, b) = eval(unit2)?;
    check_arithmetic_operand(unit1, &a, "multiplication")?;
    check_arithmetic_operand(unit2, &b, "multiplication")?;
    // Products and quotients read left to right are associative, so the two terms join
    // without parentheses: "kg" times "m/s2" is "kg.m/s2", times "/s.m" is "kg/s.m"
    let expression = match (&left, &right) {
        (UnitExpr::Numeric(n), _) if *n == Number::ONE => right.to_string(),
        (_, UnitExpr::Numeric(n)) if *n == Number::ONE => left.to_string(),
        _ => {
            let right = right.to_string();
            if right.starts_with('/') {
                format!("{left}{right}")
            } else {
                format!("{left}.{right}")
            }
        }
    };
    unit_arithmetic(expression, &a, &b, 1)
}

/// Divide one unit expression by another.
///
/// ```
/// use octofhir_ucum::unit_divide;
///
/// let result = unit_divide("m", "s").unwrap();
/// assert_eq!(result.expression, "m/s");
/// ```
///
/// # Errors
///
/// A special unit is a `SpecialUnitError` (UCUM §22.1). Operands that, joined, nest deeper
/// than [`MAX_NESTING_DEPTH`] levels are a `NestingTooDeep`. A factor whose power of
/// ten does not fit in a `Number` (`10*2000000000` times itself) is a `PrecisionOverflow`.
pub fn unit_divide(numerator: &str, denominator: &str) -> Result<UnitArithmeticResult, UcumError> {
    let (left, a) = eval(numerator)?;
    let (right, b) = eval(denominator)?;
    check_arithmetic_operand(numerator, &a, "division")?;
    check_arithmetic_operand(denominator, &b, "division")?;
    let expression = match (&left, &right) {
        (_, UnitExpr::Numeric(n)) if *n == Number::ONE => left.to_string(),
        (UnitExpr::Numeric(n), _) if *n == Number::ONE => {
            UnitExpr::Reciprocal(Box::new(right)).to_string()
        }
        _ => UnitExpr::Quotient(Box::new(left), Box::new(right)).to_string(),
    };
    unit_arithmetic(expression, &a, &b, -1)
}

fn unit_arithmetic(
    expression: String,
    a: &EvalResult,
    b: &EvalResult,
    exp: i32,
) -> Result<UnitArithmeticResult, UcumError> {
    // Two operands each nested up to the limit can join into an expression nested deeper,
    // which would not parse back
    // An error refers to the joined expression, which it holds
    parse_expression(&expression)?;
    let factor = if exp == 1 {
        a.factor.checked_mul(b.factor)
    } else {
        a.factor.checked_div(b.factor)
    }
    .ok_or_else(|| UcumError::precision_overflow("unit arithmetic", &expression))?;
    let mut dim = a.dim.0;
    evaluator::add_scaled_dim(&mut dim, &b.dim, exp)?;
    let arbitrary = evaluator::combine(&a.arbitrary, &b.arbitrary, exp);
    Ok(UnitArithmeticResult {
        factor,
        is_dimensionless: dim == [0; 7] && arbitrary.is_empty(),
        expression,
        dimension: Dimension(dim),
    })
}

// ============================================================================
// Search
// ============================================================================

#[cfg(feature = "std")]
use fuzzy_matcher::FuzzyMatcher;
#[cfg(feature = "std")]
use fuzzy_matcher::skim::SkimMatcherV2;
#[cfg(feature = "std")]
use regex::Regex;

/// Search unit atoms whose code, name or property contains the query, ignoring case. Exact
/// codes come first, then shorter codes.
///
/// ```
/// use octofhir_ucum::search_units;
///
/// assert!(search_units("meter").iter().any(|u| u.code == "m"));
/// ```
pub fn search_units(query: &str) -> Vec<&'static UnitRecord> {
    let query_lower = query.to_lowercase();
    let mut results: Vec<&'static UnitRecord> = get_all_units()
        .iter()
        .filter(|unit| {
            unit.code.to_lowercase().contains(&query_lower)
                || unit.display_name.to_lowercase().contains(&query_lower)
                || unit.property.to_lowercase().contains(&query_lower)
        })
        .collect();
    results.sort_by(|a, b| {
        let a_exact = a.code.to_lowercase() == query_lower;
        let b_exact = b.code.to_lowercase() == query_lower;
        b_exact.cmp(&a_exact).then(a.code.len().cmp(&b.code.len()))
    });
    results
}

/// All unit atoms of a property (`length`, `pressure`), ignoring case.
///
/// ```
/// use octofhir_ucum::search_units_by_property;
///
/// assert!(search_units_by_property("length").iter().any(|u| u.code == "m"));
/// ```
pub fn search_units_by_property(property: &str) -> Vec<&'static UnitRecord> {
    let property_lower = property.to_lowercase();
    get_all_units()
        .iter()
        .filter(|unit| unit.property.to_lowercase() == property_lower)
        .collect()
}

/// The unit atoms commensurable with an expression: the units of the same dimension and
/// arbitrary atoms, special units included.
///
/// ```
/// use octofhir_ucum::get_defined_forms;
///
/// let forms = get_defined_forms("m");
/// assert!(forms.iter().any(|u| u.code == "[in_i]"));
/// assert!(!forms.iter().any(|u| u.code == "g"));
/// ```
pub fn get_defined_forms(expression: &str) -> Vec<&'static UnitRecord> {
    // No suggestions to compute for an error that is not returned
    let Some(result) = parse_expression(expression)
        .ok()
        .and_then(|expr| evaluate(&expr).ok())
    else {
        return Vec::new();
    };
    get_all_units()
        .iter()
        .filter(|unit| {
            evaluate(&UnitExpr::Symbol(unit.code.to_string()))
                .is_ok_and(|r| r.is_commensurable(&result))
        })
        .collect()
}

/// Search unit atoms whose code, name or property matches a regular expression.
///
/// ```
/// use octofhir_ucum::search_units_regex;
///
/// let results = search_units_regex(r"mete?r", false).unwrap();
/// assert!(results.iter().any(|u| u.code == "m"));
/// ```
#[cfg(feature = "std")]
pub fn search_units_regex(
    pattern: &str,
    case_sensitive: bool,
) -> Result<Vec<&'static UnitRecord>, UcumError> {
    let regex_pattern = if case_sensitive {
        pattern.to_string()
    } else {
        format!("(?i){pattern}")
    };
    let regex = Regex::new(&regex_pattern).map_err(|err| {
        // The last line of a syntax error says what is wrong ("unclosed character class");
        // the lines before repeat the pattern, which can be long
        let text = err.to_string();
        let reason = match err {
            regex::Error::CompiledTooBig(_) => "too large to compile",
            _ => text
                .lines()
                .find_map(|line| line.strip_prefix("error: "))
                .unwrap_or("not a valid regular expression"),
        };
        UcumError::invalid_pattern(pattern, reason)
    })?;
    let mut results: Vec<&'static UnitRecord> = get_all_units()
        .iter()
        .filter(|unit| {
            regex.is_match(unit.code)
                || regex.is_match(unit.display_name)
                || regex.is_match(unit.property)
        })
        .collect();
    results.sort_by(|a, b| {
        regex
            .is_match(b.code)
            .cmp(&regex.is_match(a.code))
            .then(a.code.len().cmp(&b.code.len()))
    });
    Ok(results)
}

/// Search unit atoms by fuzzy matching of their code, name or property, best first.
///
/// ```
/// use octofhir_ucum::search_units_fuzzy;
///
/// let results = search_units_fuzzy("meter", 50);
/// assert!(results.iter().any(|(u, _)| u.code == "m"));
/// ```
#[cfg(feature = "std")]
pub fn search_units_fuzzy(query: &str, threshold: i64) -> Vec<(&'static UnitRecord, i64)> {
    let matcher = SkimMatcherV2::default();
    // `SkimMatcherV2` (fuzzy-matcher 0.3, `cheap_matches`) takes one character of the field
    // for each character of the query, in order, so a field with fewer characters cannot
    // match. A field has at least as many bytes as characters, so one with fewer bytes than
    // the query has characters is skipped without converting the query again.
    let query_len = query.chars().count();
    let mut results: Vec<(&'static UnitRecord, i64)> = get_all_units()
        .iter()
        .filter_map(|unit| {
            // A unit whose fields do not match is not a result, whatever the threshold
            let score = [unit.code, unit.display_name, unit.property]
                .iter()
                .filter(|field| field.len() >= query_len)
                .filter_map(|field| matcher.fuzzy_match(field, query))
                .max()?;
            (score >= threshold).then_some((unit, score))
        })
        .collect();
    results.sort_by_key(|a| core::cmp::Reverse(a.1));
    results
}

/// Kinds of unit atoms, for filtering searches.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConceptKind {
    /// The seven base units (meter, second, gram, radian, kelvin, coulomb, candela)
    BaseUnit,
    /// Proper units defined from other units (newton, inch)
    DerivedUnit,
    /// Arbitrary units (`[IU]`, `[arb'U]`)
    ArbitraryUnit,
    /// Special units on non-ratio scales (`Cel`, `B`, `[pH]`)
    SpecialUnit,
}

/// The kind of a unit atom.
pub fn classify_unit(unit: &UnitRecord) -> ConceptKind {
    if unit.is_base {
        ConceptKind::BaseUnit
    } else if unit.special.is_arbitrary() {
        ConceptKind::ArbitraryUnit
    } else if unit.special.is_special() {
        ConceptKind::SpecialUnit
    } else {
        ConceptKind::DerivedUnit
    }
}

/// Search unit atoms of some kinds, by substring or, with the `std` feature, by fuzzy
/// matching.
///
/// ```
/// use octofhir_ucum::{search_units_filtered, ConceptKind};
///
/// let results = search_units_filtered("meter", &[ConceptKind::BaseUnit], false);
/// assert!(results.iter().all(|u| u.is_base));
/// ```
pub fn search_units_filtered(
    query: &str,
    kinds: &[ConceptKind],
    use_fuzzy: bool,
) -> Vec<&'static UnitRecord> {
    #[cfg(feature = "std")]
    let all_results = if use_fuzzy {
        search_units_fuzzy(query, 30)
            .into_iter()
            .map(|(unit, _)| unit)
            .collect()
    } else {
        search_units(query)
    };
    #[cfg(not(feature = "std"))]
    let all_results = {
        let _ = use_fuzzy;
        search_units(query)
    };
    all_results
        .into_iter()
        .filter(|unit| kinds.contains(&classify_unit(unit)))
        .collect()
}

// ============================================================================
// Model
// ============================================================================

/// The UCUM tables.
#[derive(Debug, Clone)]
pub struct UcumModel {
    /// Version of the tables (`2.2`)
    pub version: &'static str,
    /// Revision date of the tables
    pub revision_date: &'static str,
    /// Prefixes
    pub prefixes: &'static [Prefix],
    /// Unit atoms
    pub units: &'static [UnitRecord],
}

/// The UCUM tables this crate implements.
///
/// ```
/// use octofhir_ucum::get_model;
///
/// let model = get_model();
/// assert_eq!(model.version, "2.2");
/// assert_eq!(model.prefixes.len(), 24);
/// ```
pub fn get_model() -> UcumModel {
    UcumModel {
        version: UCUM_VERSION,
        revision_date: UCUM_REVISION_DATE,
        prefixes: registry::PREFIXES,
        units: registry::UNITS,
    }
}

/// Check the tables for consistency: unique codes, the seven UCUM base units, every unit
/// atom valid. Returns the issues found, none for consistent tables.
///
/// ```
/// use octofhir_ucum::validate_ucum;
///
/// assert!(validate_ucum().is_empty());
/// ```
pub fn validate_ucum() -> Vec<String> {
    let mut issues = Vec::new();
    let mut seen_codes = HashSet::new();
    for unit in get_all_units() {
        if !seen_codes.insert(unit.code) {
            issues.push(format!("Duplicate unit code: {}", unit.code));
        }
        if let Err(e) = validate(unit.code) {
            issues.push(format!("Unit {} failed validation: {e}", unit.code));
        }
        if unit.factor.is_zero() || unit.factor.is_negative() {
            issues.push(format!(
                "Unit {} has a factor of {}",
                unit.code, unit.factor
            ));
        }
    }
    let mut seen_prefixes = HashSet::new();
    for prefix in get_all_prefixes() {
        if !seen_prefixes.insert(prefix.symbol) {
            issues.push(format!("Duplicate prefix symbol: {}", prefix.symbol));
        }
    }
    let base: BTreeSet<&str> = get_all_units()
        .iter()
        .filter(|u| u.is_base)
        .map(|u| u.code)
        .collect();
    let expected: BTreeSet<&str> = Dimension::BASE_UNITS.into_iter().collect();
    if base != expected {
        issues.push(format!("Base units are {base:?}, UCUM has {expected:?}"));
    }
    issues
}

/// The distinct properties of the unit atoms (`length`, `mass`, `electric current`).
pub fn get_properties() -> BTreeSet<String> {
    get_all_units()
        .iter()
        .map(|unit| unit.property.to_string())
        .collect()
}

// ============================================================================
// Display names
// ============================================================================

/// The name of a unit: "kilogram" for `kg`, "meter" for `m`, the display name of
/// `generate_display_name` for a compound expression, and the code itself when it is not
/// valid.
///
/// ```
/// use octofhir_ucum::get_common_display;
///
/// assert_eq!(get_common_display("m"), "meter");
/// assert_eq!(get_common_display("kg"), "kilogram");
/// assert_eq!(get_common_display("k[in_i]"), "k[in_i]");
/// ```
pub fn get_common_display(code: &str) -> String {
    if let Some((prefix, unit)) = lookup_unit(code, Case::Sensitive) {
        let prefix = prefix.map_or("", |p| p.display_name);
        return format!("{prefix}{}", unit.display_name);
    }
    match parse_expression(code) {
        Ok(expr) if evaluate(&expr).is_ok() => generate_display_name(&expr),
        _ => code.to_string(),
    }
}

/// The display name of the official UCUM tests for an expression: "(meter)",
/// "(meter ^ 3) * (kilogram ^ -1)", and "(unity)" for an empty code.
///
/// ```
/// use octofhir_ucum::display_name;
///
/// assert_eq!(display_name("mm").unwrap(), "(millimeter)");
/// assert_eq!(display_name("").unwrap(), "(unity)");
/// ```
pub fn display_name(code: &str) -> Result<String, UcumError> {
    if code.is_empty() {
        return Ok("(unity)".to_string());
    }
    let expr = parse_expression(code)?;
    evaluate_in(&expr, code, Case::Sensitive)?;
    Ok(generate_display_name(&expr))
}

// ============================================================================
// Conversion
// ============================================================================

/// Options of `convert_with_context`.
#[derive(Debug, Clone)]
pub struct AdvancedConversionContext {
    /// Rounding of the result
    pub precision: DecimalPrecision,
    /// How the last kept digit is rounded when `precision` is not `Default`
    pub rounding: RoundingMode,
    /// Case sensitive or case insensitive symbols (UCUM §3.4)
    pub case: Case,
}

/// Rounding of a converted value.
#[derive(Debug, Clone)]
pub enum DecimalPrecision {
    /// No rounding
    Default,
    /// A fixed number of decimal places
    Fixed(u32),
    /// A number of significant figures
    Significant(u32),
}

/// Rounding mode of a converted value.
#[derive(Debug, Clone)]
pub enum RoundingMode {
    /// Round to nearest, halves away from zero (default)
    Nearest,
    /// Round up (ceiling)
    Up,
    /// Round down (floor)
    Down,
    /// Truncate (toward zero)
    Truncate,
}

impl Default for AdvancedConversionContext {
    fn default() -> Self {
        Self {
            precision: DecimalPrecision::Default,
            rounding: RoundingMode::Nearest,
            case: Case::Sensitive,
        }
    }
}

/// Result of `convert_with_context`.
#[derive(Debug, Clone)]
pub struct AdvancedConversionResult {
    /// Converted value
    pub value: f64,
    /// Target unit
    pub unit: String,
    /// When the conversion is linear, `value = input × factor + offset`, as computed (28
    /// significant digits, any exponent); `None` for a conversion through the function of a
    /// special unit (`B` to `1`, `[pH]` to `mol/L`)
    pub factor: Option<Number>,
    /// Offset of a linear conversion (temperatures, levels with different references)
    pub offset: Option<Number>,
    /// Description of the rounding
    pub precision_info: String,
    /// Whether a special unit took part in the conversion
    pub used_special_units: bool,
}

/// Convert a value from one unit to another.
///
/// Special units convert to and from their corresponding proper units through their
/// functions (UCUM §21.2): 0 `Cel` is 273.15 `K`, 20 `dB[SPL]` is 0.0002 `Pa`, 7 `[pH]` is
/// 1e-7 `mol/L`. Arbitrary units convert only to the same arbitrary atoms (UCUM §25):
/// `[IU]/mL` to `[IU]/L`, but not to `[arb'U]/mL` or `/mL`.
///
/// ```
/// use octofhir_ucum::convert;
///
/// assert_eq!(convert(1.0, "km", "m").unwrap(), 1000.0);
/// assert_eq!(convert(100.0, "Cel", "[degF]").unwrap(), 212.0);
/// assert!((convert(7.0, "[pH]", "mol/L").unwrap() - 1e-7).abs() < 1e-20);
/// ```
///
/// # Errors
///
/// Proper units of different dimensions are a `DimensionMismatch` (UCUM §17, §19.1), with
/// both dimensions; a conversion involving an arbitrary unit to anything but the same
/// arbitrary unit is a `ConversionError` (UCUM §24.3, §25). A value that is infinite or NaN
/// is a `NonFiniteValue` (UCUM §18.5). A value outside the domain of the function of a
/// special unit is an `OutOfDomain` (UCUM §21.2): 0 `mol/L` has no `[pH]`, -1 `W` has no
/// `B[W]`. A result too large or too small for an `f64` is a `PrecisionOverflow`.
pub fn convert(value: f64, from: &str, to: &str) -> Result<f64, UcumError> {
    convert_with_context(value, from, to, &AdvancedConversionContext::default())
        .map(|result| result.value)
}

/// Convert a value from one unit to another, with rounding and case options.
///
/// ```
/// use octofhir_ucum::{convert_with_context, AdvancedConversionContext, DecimalPrecision};
///
/// let context = AdvancedConversionContext {
///     precision: DecimalPrecision::Fixed(2),
///     ..Default::default()
/// };
/// let result = convert_with_context(1.0, "[in_i]", "cm", &context).unwrap();
/// assert_eq!(result.value, 2.54);
/// ```
///
/// # Errors
///
/// As `convert`.
pub fn convert_with_context(
    value: f64,
    from: &str,
    to: &str,
    context: &AdvancedConversionContext,
) -> Result<AdvancedConversionResult, UcumError> {
    let from_eval = evaluate_in(&parse_expression(from)?, from, context.case)?;
    let to_eval = evaluate_in(&parse_expression(to)?, to, context.case)?;
    let (converted, linear) = convert_evaluated(value, from, &from_eval, to, &to_eval)?;

    let round = |x: f64| match context.rounding {
        RoundingMode::Nearest => math::round(x),
        RoundingMode::Up => math::ceil(x),
        RoundingMode::Down => math::floor(x),
        RoundingMode::Truncate => math::trunc(x),
    };
    let final_value = match context.precision {
        DecimalPrecision::Default => converted,
        DecimalPrecision::Fixed(places) => {
            let multiplier = math::powi(10.0, places as i32);
            round(converted * multiplier) / multiplier
        }
        DecimalPrecision::Significant(sig_figs) if converted != 0.0 => {
            let magnitude = math::floor(math::log10(converted.abs()));
            let factor = math::powi(10.0, (sig_figs as i32 - 1) - magnitude as i32);
            round(converted * factor) / factor
        }
        DecimalPrecision::Significant(_) => converted,
    };
    let precision_info = match context.precision {
        DecimalPrecision::Default => "default".to_string(),
        DecimalPrecision::Fixed(places) => format!("{places} decimal places"),
        DecimalPrecision::Significant(sig_figs) => format!("{sig_figs} significant figures"),
    };
    Ok(AdvancedConversionResult {
        value: final_value,
        unit: to.to_string(),
        factor: linear.map(|l| l.factor),
        offset: linear.map(|l| l.offset),
        precision_info,
        used_special_units: from_eval.special.is_some() || to_eval.special.is_some(),
    })
}

/// Convert a value between two evaluated expressions, after checking that they are
/// commensurable. Returns the value and the linear mapping, if the conversion is linear.
pub(crate) fn convert_evaluated(
    value: f64,
    from: &str,
    from_eval: &EvalResult,
    to: &str,
    to_eval: &EvalResult,
) -> Result<(f64, Option<evaluator::Linear>), UcumError> {
    if !from_eval.is_commensurable(to_eval) {
        // An arbitrary unit has no dimension (§24.3), and nor has a term with one (§25.1)
        return Err(
            if from_eval.arbitrary.is_empty() && to_eval.arbitrary.is_empty() {
                UcumError::incommensurable_dimensions(from, to, to_eval.dim, from_eval.dim)
            } else {
                UcumError::conversion_error(
                    from,
                    to,
                    "an arbitrary unit has no dimension and converts only to the same arbitrary unit (UCUM §24.3, §25)",
                )
            },
        );
    }
    let converted = evaluator::convert_value(value, from, from_eval, to, to_eval)?;
    Ok((converted, evaluator::linear_conversion(from_eval, to_eval)))
}

// ============================================================================
// Expression rewriting
// ============================================================================

/// An equal expression that is easier to read: a base or derived SI unit with a prefix when
/// one is equal (`g.m2.s-2` is `mJ`, `kg.m/s2` is `N`, `ft/ft.km` is `km`), otherwise the
/// canonical unit with its magnitude (`[in_i]` is `254.10*-4.m`). Special units are returned
/// unchanged.
///
/// ```
/// use octofhir_ucum::optimize_expression;
///
/// assert_eq!(optimize_expression("kg.m/s2").unwrap(), "N");
/// assert_eq!(optimize_expression("g.m2.s-2").unwrap(), "mJ");
/// assert_eq!(optimize_expression("m2/s2").unwrap(), "m2.s-2");
/// ```
pub fn optimize_expression(expr: &str) -> Result<String, UcumError> {
    // Base units first, then SI derived units
    const UNITS: [&str; 21] = [
        "m", "s", "g", "rad", "K", "C", "cd", "N", "J", "W", "Pa", "Hz", "V", "Ohm", "F", "S",
        "Wb", "T", "H", "A", "L",
    ];
    let (parsed, result) = eval(expr)?;
    if result.special.is_some() {
        return Ok(parsed.to_string());
    }
    if result.arbitrary.is_empty() {
        for code in UNITS {
            let unit = find_unit(code).expect("SI unit");
            if unit.dim != result.dim {
                continue;
            }
            let ratio = result
                .factor
                .checked_div(unit.factor)
                .ok_or_else(|| UcumError::precision_overflow("optimization", expr))?;
            if ratio == Number::ONE {
                return Ok(code.to_string());
            }
            if let Some(prefix) = get_all_prefixes()
                .iter()
                .filter(|p| !p.symbol.ends_with('i') || p.symbol.len() == 1)
                .find(|p| p.factor == ratio)
            {
                return Ok(format!("{}{code}", prefix.symbol));
            }
        }
    }
    let unit = canonical_unit_string(&result.dim, &result.arbitrary);
    Ok(if result.factor == Number::ONE {
        unit
    } else if unit == "1" {
        ucum_number(result.factor)
    } else {
        format!("{}.{unit}", ucum_number(result.factor))
    })
}

/// The canonical unit of an expression, without its magnitude (see `get_canonical_units`).
///
/// ```
/// use octofhir_ucum::canonicalize_expression;
///
/// assert_eq!(canonicalize_expression("N").unwrap(), "g.m.s-2");
/// ```
pub fn canonicalize_expression(expr: &str) -> Result<String, UcumError> {
    get_canonical_units(expr).map(|canonical| canonical.unit)
}

/// An equal expression with the exponents of each simple unit combined, numbers
/// multiplied and annotations dropped: `m.s/s` is `m`, `km.km` is `km2`. Special units are
/// returned unchanged.
///
/// ```
/// use octofhir_ucum::simplify_expression;
///
/// assert_eq!(simplify_expression("m.s/s").unwrap(), "m");
/// assert_eq!(simplify_expression("km.s/s").unwrap(), "km");
/// assert_eq!(simplify_expression("2.m/4").unwrap(), "5.10*-1.m");
/// ```
pub fn simplify_expression(expr: &str) -> Result<String, UcumError> {
    let (parsed, result) = eval(expr)?;
    if result.special.is_some() {
        return Ok(parsed.to_string());
    }
    let mut number = Number::ONE;
    let mut symbols: Vec<(String, i32)> = Vec::new();
    collect_terms(&parsed, 1, &mut number, &mut symbols)
        .ok_or_else(|| UcumError::precision_overflow("simplification", expr))?;
    let mut parts: Vec<String> = Vec::new();
    if number != Number::ONE {
        parts.push(ucum_number(number));
    }
    for (symbol, exp) in symbols.into_iter().filter(|(_, e)| *e != 0) {
        parts.push(if exp == 1 {
            symbol
        } else {
            format!("{symbol}{exp}")
        });
    }
    Ok(if parts.is_empty() {
        "1".to_string()
    } else {
        parts.join(".")
    })
}

/// Collect the simple units with their exponents, in order of appearance, and the product
/// of the numbers of an expression raised to `exp`.
fn collect_terms(
    expr: &UnitExpr,
    exp: i32,
    number: &mut Number,
    symbols: &mut Vec<(String, i32)>,
) -> Option<()> {
    match expr {
        UnitExpr::Symbol(symbol) => {
            match symbols.iter_mut().find(|(s, _)| s == symbol) {
                Some(entry) => entry.1 = entry.1.checked_add(exp)?,
                None => symbols.push((symbol.clone(), exp)),
            }
            Some(())
        }
        UnitExpr::Numeric(n) => {
            *number = number.checked_mul(n.checked_powi(exp)?)?;
            Some(())
        }
        UnitExpr::Annotation(_) => Some(()),
        UnitExpr::Annotated(inner, _) => collect_terms(inner, exp, number, symbols),
        UnitExpr::Power(base, e) => collect_terms(base, exp.checked_mul(*e)?, number, symbols),
        UnitExpr::Product(terms) => terms
            .iter()
            .try_for_each(|term| collect_terms(term, exp, number, symbols)),
        UnitExpr::Quotient(numerator, denominator) => {
            collect_terms(numerator, exp, number, symbols)?;
            collect_terms(denominator, exp.checked_neg()?, number, symbols)
        }
        UnitExpr::Reciprocal(inner) => collect_terms(inner, exp.checked_neg()?, number, symbols),
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;

    /// The matches of `search_units_fuzzy` as 0.8.0 computed them, with every field
    /// matched: the score of each unit with a field that matches.
    fn unfiltered_matches(query: &str) -> Vec<(&'static str, i64)> {
        let matcher = SkimMatcherV2::default();
        let mut results: Vec<(&'static str, i64)> = get_all_units()
            .iter()
            .filter_map(|unit| {
                let score = [unit.code, unit.display_name, unit.property]
                    .iter()
                    .filter_map(|field| matcher.fuzzy_match(field, query))
                    .max()?;
                Some((unit.code, score))
            })
            .collect();
        results.sort_by_key(|a| core::cmp::Reverse(a.1));
        results
    }

    #[test]
    fn skipping_short_fields_does_not_change_fuzzy_matches() {
        let mut queries: Vec<String> = [
            "",
            "m",
            "M",
            "metr",
            // "[BAU]" scores -1
            "ay",
            "Pascal",
            "inch",
            "[in_i]",
            "é",
            "µ",
            "°",
            "kilogram per",
            "international unit",
            "10*",
            "'",
            "{",
        ]
        .iter()
        .map(|q| q.to_string())
        .collect();
        // Fields themselves, and one character longer or shorter than them
        let fields = || {
            get_all_units()
                .iter()
                .flat_map(|unit| [unit.code, unit.display_name, unit.property])
        };
        for field in fields().step_by(11) {
            queries.push(field.to_string());
            queries.push(format!("{field}s"));
            queries.push(field.chars().skip(1).collect());
            queries.push(field.to_uppercase());
        }
        // The longest field, and queries of about its length
        let longest = fields().max_by_key(|field| field.chars().count()).unwrap();
        let length = longest.chars().count();
        queries.push(longest.to_string());
        queries.push(format!("{longest}s"));
        queries.push("e".repeat(length));
        queries.push("e".repeat(length + 1));
        for query in &queries {
            let expected = unfiltered_matches(query);
            for threshold in [i64::MIN, 0, 30] {
                let actual: Vec<(&str, i64)> = search_units_fuzzy(query, threshold)
                    .into_iter()
                    .map(|(unit, score)| (unit.code, score))
                    .collect();
                let expected: Vec<(&str, i64)> = expected
                    .iter()
                    .copied()
                    .filter(|(_, score)| *score >= threshold)
                    .collect();
                assert_eq!(actual, expected, "{query:?} {threshold}");
            }
        }
    }
}
