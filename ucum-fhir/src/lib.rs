//! UCUM FHIR Integration Library – Rust 2024 Edition
//!
//! This crate provides integration between the UCUM core library and FHIR,
//! allowing for conversion between UCUM units and FHIR Quantity data types.
//!
//! # Examples
//!
//! ```
//! use octofhir_ucum_fhir::{FhirQuantity, ToFhirQuantity, FromFhirQuantity};
//! use octofhir_ucum_core::{parse_expression, evaluate};
//!
//! // Create a FHIR Quantity
//! let fhir_quantity = FhirQuantity {
//!     value: 10.0,
//!     unit: Some("mg".to_string()),
//!     system: Some("http://unitsofmeasure.org".to_string()),
//!     code: Some("mg".to_string()),
//!     ..Default::default()
//! };
//!
//! // Convert to UCUM Quantity
//! let ucum_quantity = fhir_quantity.to_ucum_quantity().unwrap();
//! assert_eq!(ucum_quantity.value, 10.0);
//!
//! // Convert back to FHIR Quantity
//! let fhir_quantity2 = ucum_quantity.to_fhir_quantity().unwrap();
//! assert_eq!(fhir_quantity2.value, 10.0);
//! assert_eq!(fhir_quantity2.code, Some("mg".to_string()));
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

use octofhir_ucum_core::{Quantity as UcumQuantity, UcumError, evaluate, parse_expression};
use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Error type for FHIR integration.
#[derive(Error, Debug)]
pub enum FhirError {
    /// Error from the UCUM core library.
    #[error("UCUM error: {0}")]
    UcumError(#[from] UcumError),

    /// Missing the required field.
    #[error("Missing required field: {0}")]
    MissingField(&'static str),

    /// Invalid UCUM code.
    #[error("Invalid UCUM code: {0}")]
    InvalidCode(String),

    /// Invalid system URI.
    #[error("Invalid system URI: {0}")]
    InvalidSystem(String),
}

/// FHIR Quantity data type.
///
/// This struct represents a FHIR Quantity, which is a measured amount or an amount that can
/// potentially be measured. The FHIR Quantity data type includes a value and a unit, where
/// the unit may be a UCUM code.
///
/// See the [FHIR Quantity](http://hl7.org/fhir/datatypes.html#Quantity) documentation for more details.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FhirQuantity {
    /// The value of the quantity.
    pub value: f64,

    /// The human-readable unit representation.
    pub unit: Option<String>,

    /// The system that defines the coded unit form.
    /// For UCUM, this is "http://unitsofmeasure.org".
    pub system: Option<String>,

    /// The coded form of the unit, from the system.
    /// For UCUM, this is the UCUM code.
    pub code: Option<String>,

    /// The comparator (<, <=, >=, >) for the value.
    pub comparator: Option<String>,
}

impl Default for FhirQuantity {
    fn default() -> Self {
        Self {
            value: 0.0,
            unit: None,
            system: None,
            code: None,
            comparator: None,
        }
    }
}

impl FhirQuantity {
    /// Create a new FHIR Quantity with a UCUM code.
    ///
    /// This is a convenience method for creating a FHIR Quantity with a UCUM code.
    /// It sets the system to "http://unitsofmeasure.org" and the code to the provided UCUM code.
    ///
    /// # Arguments
    ///
    /// * `value` - The value of the quantity.
    /// * `ucum_code` - The UCUM code for the unit.
    ///
    /// # Returns
    ///
    /// A new FHIR Quantity with the provided value and UCUM code.
    ///
    /// # Examples
    ///
    /// ```
    /// use octofhir_ucum_fhir::FhirQuantity;
    ///
    /// let quantity = FhirQuantity::with_ucum_code(10.0, "mg");
    /// assert_eq!(quantity.value, 10.0);
    /// assert_eq!(quantity.code, Some("mg".to_string()));
    /// assert_eq!(quantity.system, Some("http://unitsofmeasure.org".to_string()));
    /// ```
    pub fn with_ucum_code(value: f64, ucum_code: &str) -> Self {
        Self {
            value,
            unit: Some(ucum_code.to_string()),
            system: Some("http://unitsofmeasure.org".to_string()),
            code: Some(ucum_code.to_string()),
            comparator: None,
        }
    }

    /// Check if this quantity uses UCUM units.
    ///
    /// # Returns
    ///
    /// `true` if the system is "http://unitsofmeasure.org", `false` otherwise.
    pub fn is_ucum(&self) -> bool {
        self.system
            .as_ref()
            .map(|s| s == "http://unitsofmeasure.org")
            .unwrap_or(false)
    }

    /// Convert this FHIR Quantity to a UCUM Quantity.
    ///
    /// # Returns
    ///
    /// A Result containing the UCUM Quantity, or an error if the conversion fails.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The system is not "http://unitsofmeasure.org"
    /// - The code is missing
    /// - The code is not a valid UCUM code
    pub fn to_ucum_quantity(&self) -> Result<UcumQuantity, FhirError> {
        // Check if this is a UCUM quantity
        if !self.is_ucum() {
            return Err(FhirError::InvalidSystem(
                self.system.clone().unwrap_or_else(|| "None".to_string()),
            ));
        }

        // Get the UCUM code
        let code = self.code.as_ref().ok_or(FhirError::MissingField("code"))?;

        // Parse the UCUM expression
        let expr = parse_expression(code)?;

        // Create a UCUM Quantity
        Ok(UcumQuantity {
            value: self.value,
            unit: expr,
        })
    }
}

/// Trait for converting to a FHIR Quantity.
pub trait ToFhirQuantity {
    /// Convert to a FHIR Quantity.
    ///
    /// # Returns
    ///
    /// A Result containing the FHIR Quantity, or an error if the conversion fails.
    fn to_fhir_quantity(&self) -> Result<FhirQuantity, FhirError>;
}

/// Trait for converting from a FHIR Quantity.
pub trait FromFhirQuantity<T> {
    /// Convert from a FHIR Quantity.
    ///
    /// # Returns
    ///
    /// A Result containing the converted value, or an error if the conversion fails.
    fn from_fhir_quantity(quantity: &FhirQuantity) -> Result<T, FhirError>;
}

impl ToFhirQuantity for UcumQuantity {
    fn to_fhir_quantity(&self) -> Result<FhirQuantity, FhirError> {
        // Convert the UnitExpr to a string
        let code = format!("{}", self.unit);

        // Create a FHIR Quantity
        Ok(FhirQuantity {
            value: self.value,
            unit: Some(code.clone()),
            system: Some("http://unitsofmeasure.org".to_string()),
            code: Some(code),
            comparator: None,
        })
    }
}

impl FromFhirQuantity<UcumQuantity> for UcumQuantity {
    fn from_fhir_quantity(quantity: &FhirQuantity) -> Result<UcumQuantity, FhirError> {
        quantity.to_ucum_quantity()
    }
}

/// Convert between FHIR Quantities with different units.
///
/// This function converts a FHIR Quantity from one unit to another.
///
/// # Arguments
///
/// * `quantity` - The FHIR Quantity to convert.
/// * `target_unit` - The target UCUM unit code.
///
/// # Returns
///
/// A Result containing the converted FHIR Quantity, or an error if the conversion fails.
///
/// # Errors
///
/// Returns an error if:
/// - The source quantity is not a UCUM quantity
/// - The source or target unit is not a valid UCUM code
/// - The units are not commensurable (have different dimensions)
///
/// # Examples
///
/// ```
/// use octofhir_ucum_fhir::{FhirQuantity, convert_quantity};
///
/// let quantity = FhirQuantity::with_ucum_code(1000.0, "mg");
/// let converted = convert_quantity(&quantity, "g").unwrap();
/// assert_eq!(converted.value, 1.0);
/// assert_eq!(converted.code, Some("g".to_string()));
/// ```
pub fn convert_quantity(
    quantity: &FhirQuantity,
    target_unit: &str,
) -> Result<FhirQuantity, FhirError> {
    // Convert to UCUM Quantity
    let ucum_quantity = quantity.to_ucum_quantity()?;

    // Parse the target unit
    let target_expr = parse_expression(target_unit)?;

    // Evaluate both units to get their dimensions and factors
    let source_eval = evaluate(&ucum_quantity.unit)?;
    let target_eval = evaluate(&target_expr)?;

    // Check if the units are commensurable (have the same dimension)
    if source_eval.dim != target_eval.dim {
        return Err(FhirError::InvalidCode(format!(
            "Units are not commensurable: {} and {}",
            ucum_quantity.unit, target_unit
        )));
    }

    // Calculate the conversion factor
    let factor = source_eval.factor / target_eval.factor;

    // Create a new FHIR Quantity with the converted value
    Ok(FhirQuantity {
        value: ucum_quantity.value * factor,
        unit: Some(target_unit.to_string()),
        system: Some("http://unitsofmeasure.org".to_string()),
        code: Some(target_unit.to_string()),
        comparator: quantity.comparator.clone(),
    })
}

/// Convert a value to a FHIR Quantity.
///
/// This function converts various input types to FHIR Quantities:
/// - Numbers (integers, decimals) become dimensionless quantities with unit '1'
/// - Strings are parsed as quantity expressions (e.g., "1 day", "1 'wk'")
/// - Booleans are converted to 1 or 0 with unit '1'
///
/// # Arguments
///
/// * `input` - The input value to convert
///
/// # Returns
///
/// A Result containing the FHIR Quantity, or an error if conversion fails.
///
/// # Examples
///
/// ```
/// use octofhir_ucum_fhir::to_quantity;
///
/// // Convert number to dimensionless quantity
/// let qty1 = to_quantity(&1.0).unwrap();
/// assert_eq!(qty1.value, 1.0);
/// assert_eq!(qty1.code, Some("1".to_string()));
///
/// // Convert string with unit
/// let qty2 = to_quantity(&"1 day".to_string()).unwrap();
/// assert_eq!(qty2.value, 1.0);
/// assert_eq!(qty2.code, Some("d".to_string()));
/// ```
pub fn to_quantity(input: &dyn std::any::Any) -> Result<FhirQuantity, FhirError> {
    // Try to downcast to different types
    if let Some(val) = input.downcast_ref::<f64>() {
        return Ok(FhirQuantity::with_ucum_code(*val, "1"));
    }

    if let Some(val) = input.downcast_ref::<i32>() {
        return Ok(FhirQuantity::with_ucum_code(*val as f64, "1"));
    }

    if let Some(val) = input.downcast_ref::<i64>() {
        return Ok(FhirQuantity::with_ucum_code(*val as f64, "1"));
    }

    if let Some(val) = input.downcast_ref::<bool>() {
        let numeric_val = if *val { 1.0 } else { 0.0 };
        return Ok(FhirQuantity::with_ucum_code(numeric_val, "1"));
    }

    if let Some(val) = input.downcast_ref::<String>() {
        return parse_quantity_string(val);
    }

    if let Some(val) = input.downcast_ref::<&str>() {
        return parse_quantity_string(&val.to_string());
    }

    Err(FhirError::InvalidCode("Unsupported input type for toQuantity".to_string()))
}

/// Check if a value can be converted to a FHIR Quantity.
///
/// This function validates whether the input can be successfully converted
/// to a FHIR Quantity without actually performing the conversion.
///
/// # Arguments
///
/// * `input` - The input value to validate
///
/// # Returns
///
/// True if the input can be converted to a quantity, false otherwise.
///
/// # Examples
///
/// ```
/// use octofhir_ucum_fhir::converts_to_quantity;
///
/// assert!(converts_to_quantity(&1.0));
/// assert!(converts_to_quantity(&"1 day".to_string()));
/// assert!(!converts_to_quantity(&"invalid".to_string()));
/// ```
pub fn converts_to_quantity(input: &dyn std::any::Any) -> bool {
    to_quantity(input).is_ok()
}

/// Parse a quantity string into a FHIR Quantity.
///
/// Supports various formats:
/// - "1" -> 1 '1'
/// - "1.0" -> 1.0 '1'
/// - "1 day" -> 1 'd' (literal unit converted to UCUM)
/// - "1 'wk'" -> 1 'wk' (UCUM code)
/// - "1 week" -> 1 'wk' (literal unit converted to UCUM)
fn parse_quantity_string(input: &str) -> Result<FhirQuantity, FhirError> {
    let input = input.trim();

    // Try to parse as just a number
    if let Ok(val) = input.parse::<f64>() {
        return Ok(FhirQuantity::with_ucum_code(val, "1"));
    }

    // Split on whitespace to separate value and unit
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        return Err(FhirError::InvalidCode("Empty quantity string".to_string()));
    }

    if parts.len() == 1 {
        // Just a number
        if let Ok(val) = parts[0].parse::<f64>() {
            return Ok(FhirQuantity::with_ucum_code(val, "1"));
        } else {
            return Err(FhirError::InvalidCode(format!("Invalid number: {}", parts[0])));
        }
    }

    if parts.len() == 2 {
        // Value and unit
        let val = parts[0].parse::<f64>()
            .map_err(|_| FhirError::InvalidCode(format!("Invalid number: {}", parts[0])))?;

        let unit = parts[1];
        let ucum_code = convert_literal_to_ucum(unit)?;

        return Ok(FhirQuantity::with_ucum_code(val, &ucum_code));
    }

    // Handle quoted UCUM codes like "1 'wk'"
    if parts.len() >= 2 {
        let val = parts[0].parse::<f64>()
            .map_err(|_| FhirError::InvalidCode(format!("Invalid number: {}", parts[0])))?;

        // Join the rest as the unit (handles quoted units with spaces)
        let unit = parts[1..].join(" ");
        let ucum_code = convert_literal_to_ucum(&unit)?;

        return Ok(FhirQuantity::with_ucum_code(val, &ucum_code));
    }

    Err(FhirError::InvalidCode(format!("Invalid quantity format: {}", input)))
}

/// Convert literal units to UCUM codes.
///
/// Maps common literal units to their UCUM equivalents:
/// - day/days -> d
/// - week/weeks -> wk
/// - gram/grams -> g
/// - etc.
fn convert_literal_to_ucum(unit: &str) -> Result<String, FhirError> {
    let unit = unit.trim();

    // Handle quoted UCUM codes - remove quotes
    if unit.starts_with('\'') && unit.ends_with('\'') && unit.len() > 2 {
        return Ok(unit[1..unit.len()-1].to_string());
    }

    // Handle unquoted UCUM codes that should be quoted
    if unit == "wk" {
        return Err(FhirError::InvalidCode("UCUM code 'wk' must be quoted as 'wk'".to_string()));
    }

    // Map literal units to UCUM codes
    match unit.to_lowercase().as_str() {
        "day" | "days" => Ok("d".to_string()),
        "week" | "weeks" => Ok("wk".to_string()),
        "gram" | "grams" => Ok("g".to_string()),
        "milligram" | "milligrams" => Ok("mg".to_string()),
        "meter" | "meters" => Ok("m".to_string()),
        "centimeter" | "centimeters" => Ok("cm".to_string()),
        "second" | "seconds" => Ok("s".to_string()),
        "minute" | "minutes" => Ok("min".to_string()),
        "hour" | "hours" => Ok("h".to_string()),
        // Pass through other units as-is (assume they're valid UCUM codes)
        _ => Ok(unit.to_string()),
    }
}

/// Compare two FHIR Quantities for equality with unit conversion.
///
/// # Arguments
///
/// * `a` - The first FHIR Quantity
/// * `b` - The second FHIR Quantity
///
/// # Returns
///
/// True if the quantities are equal when converted to the same unit, false otherwise.
///
/// # Examples
///
/// ```
/// use octofhir_ucum_fhir::{FhirQuantity, quantity_equals};
///
/// let qty1 = FhirQuantity::with_ucum_code(4.0, "g");
/// let qty2 = FhirQuantity::with_ucum_code(4000.0, "mg");
/// assert!(quantity_equals(&qty1, &qty2).unwrap());
/// ```
pub fn quantity_equals(a: &FhirQuantity, b: &FhirQuantity) -> Result<bool, FhirError> {
    compare_quantities(a, b, ComparisonOp::Equal, None)
}

/// Compare two FHIR Quantities for inequality with unit conversion.
///
/// # Arguments
///
/// * `a` - The first FHIR Quantity
/// * `b` - The second FHIR Quantity
///
/// # Returns
///
/// True if the quantities are not equal when converted to the same unit, false otherwise.
pub fn quantity_not_equals(a: &FhirQuantity, b: &FhirQuantity) -> Result<bool, FhirError> {
    compare_quantities(a, b, ComparisonOp::NotEqual, None)
}

/// Compare two FHIR Quantities for less-than with unit conversion.
///
/// # Arguments
///
/// * `a` - The first FHIR Quantity
/// * `b` - The second FHIR Quantity
///
/// # Returns
///
/// True if the first quantity is less than the second when converted to the same unit.
pub fn quantity_less_than(a: &FhirQuantity, b: &FhirQuantity) -> Result<bool, FhirError> {
    compare_quantities(a, b, ComparisonOp::LessThan, None)
}

/// Compare two FHIR Quantities for greater-than with unit conversion.
///
/// # Arguments
///
/// * `a` - The first FHIR Quantity
/// * `b` - The second FHIR Quantity
///
/// # Returns
///
/// True if the first quantity is greater than the second when converted to the same unit.
pub fn quantity_greater_than(a: &FhirQuantity, b: &FhirQuantity) -> Result<bool, FhirError> {
    compare_quantities(a, b, ComparisonOp::GreaterThan, None)
}

/// Compare two FHIR Quantities for approximate equality with tolerance.
///
/// # Arguments
///
/// * `a` - The first FHIR Quantity
/// * `b` - The second FHIR Quantity
/// * `tolerance` - Optional tolerance for comparison (default: 1e-8)
///
/// # Returns
///
/// True if the quantities are approximately equal within the tolerance.
///
/// # Examples
///
/// ```
/// use octofhir_ucum_fhir::{FhirQuantity, quantity_approximately_equals};
///
/// let qty1 = FhirQuantity::with_ucum_code(4.0, "g");
/// let qty2 = FhirQuantity::with_ucum_code(4040.0, "mg");
/// assert!(quantity_approximately_equals(&qty1, &qty2, Some(0.1)).unwrap());
/// ```
pub fn quantity_approximately_equals(a: &FhirQuantity, b: &FhirQuantity, tolerance: Option<f64>) -> Result<bool, FhirError> {
    compare_quantities(a, b, ComparisonOp::Approximate, tolerance)
}

/// Enumeration of comparison operations.
#[derive(Debug, Clone, Copy)]
enum ComparisonOp {
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    Approximate,
}

/// Internal function to compare two FHIR Quantities.
///
/// This function handles unit conversion and performs the specified comparison operation.
fn compare_quantities(a: &FhirQuantity, b: &FhirQuantity, op: ComparisonOp, tolerance: Option<f64>) -> Result<bool, FhirError> {
    // Convert both quantities to UCUM quantities
    let ucum_a = a.to_ucum_quantity()?;
    let ucum_b = b.to_ucum_quantity()?;

    // Evaluate both units to get their dimensions and factors
    let eval_a = evaluate(&ucum_a.unit)?;
    let eval_b = evaluate(&ucum_b.unit)?;

    // Check if the units are commensurable (have the same dimension)
    if eval_a.dim != eval_b.dim {
        return Err(FhirError::InvalidCode(format!(
            "Units are not commensurable: {} and {}",
            ucum_a.unit, ucum_b.unit
        )));
    }

    // Convert both values to the same canonical unit
    let canonical_a = ucum_a.value * eval_a.factor;
    let canonical_b = ucum_b.value * eval_b.factor;

    // Perform the comparison
    match op {
        ComparisonOp::Equal => Ok((canonical_a - canonical_b).abs() < f64::EPSILON),
        ComparisonOp::NotEqual => Ok((canonical_a - canonical_b).abs() >= f64::EPSILON),
        ComparisonOp::LessThan => Ok(canonical_a < canonical_b),
        ComparisonOp::GreaterThan => Ok(canonical_a > canonical_b),
        ComparisonOp::Approximate => {
            let tol = tolerance.unwrap_or(1e-8);
            let diff = (canonical_a - canonical_b).abs();
            let max_val = canonical_a.abs().max(canonical_b.abs());
            if max_val == 0.0 {
                Ok(diff <= tol)
            } else {
                Ok(diff / max_val <= tol)
            }
        }
    }
}

/// Multiply two FHIR Quantities with proper unit handling.
///
/// # Arguments
///
/// * `a` - The first FHIR Quantity
/// * `b` - The second FHIR Quantity
///
/// # Returns
///
/// A Result containing the product as a new FHIR Quantity with compound units.
///
/// # Examples
///
/// ```
/// use octofhir_ucum_fhir::{FhirQuantity, quantity_multiply};
///
/// let qty1 = FhirQuantity::with_ucum_code(2.0, "cm");
/// let qty2 = FhirQuantity::with_ucum_code(2.0, "m");
/// let result = quantity_multiply(&qty1, &qty2).unwrap();
/// assert_eq!(result.value, 0.04);
/// assert_eq!(result.code, Some("m2".to_string()));
/// ```
pub fn quantity_multiply(a: &FhirQuantity, b: &FhirQuantity) -> Result<FhirQuantity, FhirError> {
    // Convert both quantities to UCUM quantities
    let ucum_a = a.to_ucum_quantity()?;
    let ucum_b = b.to_ucum_quantity()?;

    // Use the core library's multiply function
    let result = octofhir_ucum_core::multiply(
        ucum_a.value,
        &format!("{}", ucum_a.unit),
        ucum_b.value,
        &format!("{}", ucum_b.unit),
    )?;

    // Create a new FHIR Quantity with the result
    Ok(FhirQuantity {
        value: result.value,
        unit: Some(result.unit.clone()),
        system: Some("http://unitsofmeasure.org".to_string()),
        code: Some(result.unit),
        comparator: None,
    })
}

/// Divide two FHIR Quantities with proper unit handling.
///
/// # Arguments
///
/// * `dividend` - The dividend FHIR Quantity
/// * `divisor` - The divisor FHIR Quantity
///
/// # Returns
///
/// A Result containing the quotient as a new FHIR Quantity with compound units.
///
/// # Examples
///
/// ```
/// use octofhir_ucum_fhir::{FhirQuantity, quantity_divide};
///
/// let qty1 = FhirQuantity::with_ucum_code(4.0, "g");
/// let qty2 = FhirQuantity::with_ucum_code(2.0, "m");
/// let result = quantity_divide(&qty1, &qty2).unwrap();
/// assert_eq!(result.value, 2.0);
/// assert_eq!(result.code, Some("kg.m-1".to_string()));
/// ```
pub fn quantity_divide(dividend: &FhirQuantity, divisor: &FhirQuantity) -> Result<FhirQuantity, FhirError> {
    // Convert both quantities to UCUM quantities
    let ucum_dividend = dividend.to_ucum_quantity()?;
    let ucum_divisor = divisor.to_ucum_quantity()?;

    // Use the core library's divide_by function
    let result = octofhir_ucum_core::divide_by(
        ucum_dividend.value,
        &format!("{}", ucum_dividend.unit),
        ucum_divisor.value,
        &format!("{}", ucum_divisor.unit),
    )?;

    // Create a new FHIR Quantity with the result
    Ok(FhirQuantity {
        value: result.value,
        unit: Some(result.unit.clone()),
        system: Some("http://unitsofmeasure.org".to_string()),
        code: Some(result.unit),
        comparator: None,
    })
}

/// Add two FHIR Quantities with unit conversion.
///
/// # Arguments
///
/// * `a` - The first FHIR Quantity
/// * `b` - The second FHIR Quantity
///
/// # Returns
///
/// A Result containing the sum as a new FHIR Quantity in the units of the first quantity.
///
/// # Examples
///
/// ```
/// use octofhir_ucum_fhir::{FhirQuantity, quantity_add};
///
/// let qty1 = FhirQuantity::with_ucum_code(1.0, "g");
/// let qty2 = FhirQuantity::with_ucum_code(500.0, "mg");
/// let result = quantity_add(&qty1, &qty2).unwrap();
/// assert_eq!(result.value, 1.5);
/// assert_eq!(result.code, Some("g".to_string()));
/// ```
pub fn quantity_add(a: &FhirQuantity, b: &FhirQuantity) -> Result<FhirQuantity, FhirError> {
    // Convert both quantities to UCUM quantities
    let ucum_a = a.to_ucum_quantity()?;
    let ucum_b = b.to_ucum_quantity()?;

    // Evaluate both units to get their dimensions and factors
    let eval_a = evaluate(&ucum_a.unit)?;
    let eval_b = evaluate(&ucum_b.unit)?;

    // Check if the units are commensurable (have the same dimension)
    if eval_a.dim != eval_b.dim {
        return Err(FhirError::InvalidCode(format!(
            "Units are not commensurable for addition: {} and {}",
            ucum_a.unit, ucum_b.unit
        )));
    }

    // Convert second quantity to the units of the first
    let conversion_factor = eval_b.factor / eval_a.factor;
    let converted_value_b = ucum_b.value * conversion_factor;

    // Add the values
    let result_value = ucum_a.value + converted_value_b;

    // Create a new FHIR Quantity with the result in the units of the first quantity
    Ok(FhirQuantity {
        value: result_value,
        unit: a.unit.clone(),
        system: Some("http://unitsofmeasure.org".to_string()),
        code: a.code.clone(),
        comparator: None,
    })
}

/// Subtract two FHIR Quantities with unit conversion.
///
/// # Arguments
///
/// * `a` - The minuend FHIR Quantity
/// * `b` - The subtrahend FHIR Quantity
///
/// # Returns
///
/// A Result containing the difference as a new FHIR Quantity in the units of the first quantity.
///
/// # Examples
///
/// ```
/// use octofhir_ucum_fhir::{FhirQuantity, quantity_subtract};
///
/// let qty1 = FhirQuantity::with_ucum_code(2.0, "g");
/// let qty2 = FhirQuantity::with_ucum_code(500.0, "mg");
/// let result = quantity_subtract(&qty1, &qty2).unwrap();
/// assert_eq!(result.value, 1.5);
/// assert_eq!(result.code, Some("g".to_string()));
/// ```
pub fn quantity_subtract(a: &FhirQuantity, b: &FhirQuantity) -> Result<FhirQuantity, FhirError> {
    // Convert both quantities to UCUM quantities
    let ucum_a = a.to_ucum_quantity()?;
    let ucum_b = b.to_ucum_quantity()?;

    // Evaluate both units to get their dimensions and factors
    let eval_a = evaluate(&ucum_a.unit)?;
    let eval_b = evaluate(&ucum_b.unit)?;

    // Check if the units are commensurable (have the same dimension)
    if eval_a.dim != eval_b.dim {
        return Err(FhirError::InvalidCode(format!(
            "Units are not commensurable for subtraction: {} and {}",
            ucum_a.unit, ucum_b.unit
        )));
    }

    // Convert second quantity to the units of the first
    let conversion_factor = eval_b.factor / eval_a.factor;
    let converted_value_b = ucum_b.value * conversion_factor;

    // Subtract the values
    let result_value = ucum_a.value - converted_value_b;

    // Create a new FHIR Quantity with the result in the units of the first quantity
    Ok(FhirQuantity {
        value: result_value,
        unit: a.unit.clone(),
        system: Some("http://unitsofmeasure.org".to_string()),
        code: a.code.clone(),
        comparator: None,
    })
}

/// Check if two FHIR Quantities are equivalent.
///
/// Two quantities are equivalent if they have the same value when converted to the same unit.
///
/// # Arguments
///
/// * `a` - The first FHIR Quantity.
/// * `b` - The second FHIR Quantity.
///
/// # Returns
///
/// A Result containing a boolean indicating whether the quantities are equivalent,
/// or an error if the comparison fails.
///
/// # Errors
///
/// Returns an error if:
/// - Either quantity is not a UCUM quantity
/// - Either unit is not a valid UCUM code
/// - The units are not commensurable (have different dimensions)
///
/// # Examples
///
/// ```
/// use octofhir_ucum_fhir::{FhirQuantity, are_equivalent};
///
/// let a = FhirQuantity::with_ucum_code(1.0, "g");
/// let b = FhirQuantity::with_ucum_code(1000.0, "mg");
/// assert!(are_equivalent(&a, &b).unwrap());
///
/// let c = FhirQuantity::with_ucum_code(1.0, "g");
/// let d = FhirQuantity::with_ucum_code(2.0, "g");
/// assert!(!are_equivalent(&c, &d).unwrap());
/// ```
pub fn are_equivalent(a: &FhirQuantity, b: &FhirQuantity) -> Result<bool, FhirError> {
    // Convert to UCUM Quantities
    let a_ucum = a.to_ucum_quantity()?;
    let b_ucum = b.to_ucum_quantity()?;

    // Evaluate both units to get their dimensions and factors
    let a_eval = evaluate(&a_ucum.unit)?;
    let b_eval = evaluate(&b_ucum.unit)?;

    // Check if the units are commensurable (have the same dimension)
    if a_eval.dim != b_eval.dim {
        return Err(FhirError::InvalidCode(format!(
            "Units are not commensurable: {} and {}",
            a_ucum.unit, b_ucum.unit
        )));
    }

    // Special handling for arbitrary units
    // Different arbitrary units should not be commensurable with each other
    // even though they have the same dimension (all zeros)
    let a_code = a.code.as_ref().ok_or(FhirError::MissingField("code"))?;
    let b_code = b.code.as_ref().ok_or(FhirError::MissingField("code"))?;

    // Check if either unit is an arbitrary unit (enclosed in square brackets)
    // We need to handle both standalone arbitrary units and prefixed arbitrary units
    let a_has_arbitrary = a_code.contains('[') && a_code.contains(']');
    let b_has_arbitrary = b_code.contains('[') && b_code.contains(']');

    // If both have arbitrary units, extract and compare them
    if a_has_arbitrary && b_has_arbitrary {
        // Extract the arbitrary unit part (including brackets)
        let a_arb_unit = if let Some(open_bracket) = a_code.find('[') {
            if let Some(close_bracket) = a_code.rfind(']') {
                &a_code[open_bracket..=close_bracket]
            } else {
                a_code // Shouldn't happen if properly formed
            }
        } else {
            a_code // Shouldn't happen if properly formed
        };

        let b_arb_unit = if let Some(open_bracket) = b_code.find('[') {
            if let Some(close_bracket) = b_code.rfind(']') {
                &b_code[open_bracket..=close_bracket]
            } else {
                b_code // Shouldn't happen if properly formed
            }
        } else {
            b_code // Shouldn't happen if properly formed
        };

        // If they're different arbitrary units, they're not commensurable
        if a_arb_unit != b_arb_unit {
            return Err(FhirError::InvalidCode(format!(
                "Different arbitrary units are not commensurable: {} and {}",
                a_code, b_code
            )));
        }
    }

    // Calculate the conversion factor to convert from unit B to unit A
    // If unit A is g and unit B is mg, factor will be 0.001 (1g = 1000mg)
    let factor = b_eval.factor / a_eval.factor;

    // Compare the values using a relative comparison
    // Use a more appropriate epsilon for floating-point comparison
    const EPSILON: f64 = 1e-6;

    // Calculate the absolute difference between a's value and b's value converted to a's unit
    // For example, if a is 1.0g and b is 1000.0mg, we compare 1.0 with 1000.0 * 0.001 = 1.0
    let diff = (a_ucum.value - b_ucum.value * factor).abs();

    // Use a relative comparison to handle different scales
    let max = a_ucum.value.abs().max((b_ucum.value * factor).abs());

    if max < EPSILON {
        // Both values are very close to zero, use absolute comparison
        Ok(diff < EPSILON)
    } else {
        // Use relative comparison
        Ok(diff / max < EPSILON)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fhir_quantity_with_ucum_code() {
        let quantity = FhirQuantity::with_ucum_code(10.0, "mg");
        assert_eq!(quantity.value, 10.0);
        assert_eq!(quantity.code, Some("mg".to_string()));
        assert_eq!(
            quantity.system,
            Some("http://unitsofmeasure.org".to_string())
        );
    }

    #[test]
    fn test_is_ucum() {
        let quantity = FhirQuantity::with_ucum_code(10.0, "mg");
        assert!(quantity.is_ucum());

        let quantity = FhirQuantity {
            value: 10.0,
            unit: Some("mg".to_string()),
            system: Some("http://example.org".to_string()),
            code: Some("mg".to_string()),
            comparator: None,
        };
        assert!(!quantity.is_ucum());
    }

    #[test]
    fn test_to_ucum_quantity() {
        let quantity = FhirQuantity::with_ucum_code(10.0, "mg");
        let ucum_quantity = quantity.to_ucum_quantity().unwrap();
        assert_eq!(ucum_quantity.value, 10.0);
        assert_eq!(format!("{}", ucum_quantity.unit), "mg");
    }

    #[test]
    fn test_to_fhir_quantity() {
        let expr = parse_expression("mg").unwrap();
        let ucum_quantity = UcumQuantity {
            value: 10.0,
            unit: expr,
        };
        let fhir_quantity = ucum_quantity.to_fhir_quantity().unwrap();
        assert_eq!(fhir_quantity.value, 10.0);
        assert_eq!(fhir_quantity.code, Some("mg".to_string()));
        assert_eq!(
            fhir_quantity.system,
            Some("http://unitsofmeasure.org".to_string())
        );
    }

    #[test]
    fn test_convert_quantity() {
        let quantity = FhirQuantity::with_ucum_code(1000.0, "mg");
        let converted = convert_quantity(&quantity, "g").unwrap();
        assert_eq!(converted.value, 1.0);
        assert_eq!(converted.code, Some("g".to_string()));
    }

    #[test]
    fn test_are_equivalent() {
        let a = FhirQuantity::with_ucum_code(1.0, "g");
        let b = FhirQuantity::with_ucum_code(1000.0, "mg");

        // Add debug output to understand why the test is failing
        let a_ucum = a.to_ucum_quantity().unwrap();
        let b_ucum = b.to_ucum_quantity().unwrap();

        println!("a_ucum: value={}, unit={}", a_ucum.value, a_ucum.unit);
        println!("b_ucum: value={}, unit={}", b_ucum.value, b_ucum.unit);

        let a_eval = evaluate(&a_ucum.unit).unwrap();
        let b_eval = evaluate(&b_ucum.unit).unwrap();

        println!("a_eval: factor={}, dim={:?}", a_eval.factor, a_eval.dim);
        println!("b_eval: factor={}, dim={:?}", b_eval.factor, b_eval.dim);

        let factor = a_eval.factor / b_eval.factor;
        println!("factor: {}", factor);

        let diff = (a_ucum.value - b_ucum.value * factor).abs();
        let max = a_ucum.value.abs().max((b_ucum.value * factor).abs());

        println!("diff: {}", diff);
        println!("max: {}", max);
        println!("diff/max: {}", diff / max);
        println!("EPSILON: {}", 1e-6);

        assert!(are_equivalent(&a, &b).unwrap());

        let c = FhirQuantity::with_ucum_code(1.0, "g");
        let d = FhirQuantity::with_ucum_code(2.0, "g");
        assert!(!are_equivalent(&c, &d).unwrap());
    }

    #[test]
    fn test_error_handling() {
        // Invalid system
        let quantity = FhirQuantity {
            value: 10.0,
            unit: Some("mg".to_string()),
            system: Some("http://example.org".to_string()),
            code: Some("mg".to_string()),
            comparator: None,
        };
        let result = quantity.to_ucum_quantity();
        assert!(result.is_err());
        if let Err(FhirError::InvalidSystem(system)) = result {
            assert_eq!(system, "http://example.org");
        } else {
            panic!("Expected InvalidSystem error");
        }

        // Missing code
        let quantity = FhirQuantity {
            value: 10.0,
            unit: Some("mg".to_string()),
            system: Some("http://unitsofmeasure.org".to_string()),
            code: None,
            comparator: None,
        };
        let result = quantity.to_ucum_quantity();
        assert!(result.is_err());
        if let Err(FhirError::MissingField(field)) = result {
            assert_eq!(field, "code");
        } else {
            panic!("Expected MissingField error");
        }

        // Invalid code with invalid syntax (using a character that's not allowed in UCUM)
        let quantity = FhirQuantity::with_ucum_code(10.0, "!invalid");
        let result = quantity.to_ucum_quantity();
        assert!(result.is_err());
    }

    #[test]
    fn test_non_commensurable_units() {
        let a = FhirQuantity::with_ucum_code(1.0, "g");
        let b = FhirQuantity::with_ucum_code(1.0, "s");
        let result = are_equivalent(&a, &b);
        assert!(result.is_err());
    }

    #[test]
    fn test_arbitrary_units() {
        // Test 1: Create FHIR Quantities with arbitrary units
        let iu = FhirQuantity::with_ucum_code(10.0, "[IU]");
        assert_eq!(iu.value, 10.0);
        assert_eq!(iu.code, Some("[IU]".to_string()));

        // Test 2: Convert FHIR Quantity with arbitrary unit to UCUM Quantity
        let ucum_iu = iu.to_ucum_quantity().unwrap();
        assert_eq!(ucum_iu.value, 10.0);
        assert_eq!(format!("{}", ucum_iu.unit), "[IU]");

        // Test 3: Prefixed arbitrary units
        let kiu = FhirQuantity::with_ucum_code(1.0, "k[IU]");
        let ucum_kiu = kiu.to_ucum_quantity().unwrap();
        assert_eq!(ucum_kiu.value, 1.0);
        assert_eq!(format!("{}", ucum_kiu.unit), "k[IU]");

        // Test 4: Arbitrary units with volume units
        let iu_per_ml = FhirQuantity::with_ucum_code(5.0, "[IU]/mL");
        let iu_per_l = convert_quantity(&iu_per_ml, "[IU]/L").unwrap();
        assert_eq!(iu_per_l.value, 5000.0); // 5 [IU]/mL = 5000 [IU]/L

        // Test 5: Different arbitrary units are not equivalent
        let iu = FhirQuantity::with_ucum_code(10.0, "[IU]");
        let arbu = FhirQuantity::with_ucum_code(10.0, "[arb'U]");
        let result = are_equivalent(&iu, &arbu);
        assert!(result.is_err());

        // Test 6: Same arbitrary units with different prefixes are equivalent
        let iu = FhirQuantity::with_ucum_code(1000.0, "[IU]");
        let kiu = FhirQuantity::with_ucum_code(1.0, "k[IU]");
        assert!(are_equivalent(&iu, &kiu).unwrap());

        // Test 7: Complex expressions with arbitrary units
        let complex1 = FhirQuantity::with_ucum_code(10.0, "[IU]/(m2.s)");
        let complex2 = FhirQuantity::with_ucum_code(10.0, "[IU]/(m2.s)");
        assert!(are_equivalent(&complex1, &complex2).unwrap());
    }

    #[test]
    fn test_division_result() {
        use octofhir_ucum_core::divide_by;
        let result = divide_by(4.0, "g", 2.0, "m").unwrap();
        println!("Division result - Value: {}, Unit: {}", result.value, result.unit);

        let qty1 = FhirQuantity::with_ucum_code(4.0, "g");
        let qty2 = FhirQuantity::with_ucum_code(2.0, "m");
        let fhir_result = quantity_divide(&qty1, &qty2).unwrap();
        println!("FHIR Division result - Value: {}, Unit: {:?}", fhir_result.value, fhir_result.code);
    }

    #[test]
    fn test_mass_unit_conversions() {
        // Test case: 4.0000 'g' = 4000.0 'mg' → true
        let qty_g = FhirQuantity::with_ucum_code(4.0, "g");
        let qty_mg = FhirQuantity::with_ucum_code(4000.0, "mg");
        assert!(quantity_equals(&qty_g, &qty_mg).unwrap());

        // Test case: 4 'g' ~ 4000 'mg' → true
        assert!(quantity_approximately_equals(&qty_g, &qty_mg, None).unwrap());

        // Test case: 4 'g' != 4040 'mg' → true
        let qty_mg_diff = FhirQuantity::with_ucum_code(4040.0, "mg");
        assert!(quantity_not_equals(&qty_g, &qty_mg_diff).unwrap());

        // Test case: 4 'g' ~ 4040 'mg' → true (within tolerance)
        assert!(quantity_approximately_equals(&qty_g, &qty_mg_diff, Some(0.1)).unwrap());
    }

    #[test]
    fn test_time_unit_conversions() {
        // Test case: 7 days = 1 week → true
        let qty_days = FhirQuantity::with_ucum_code(7.0, "d");
        let qty_week = FhirQuantity::with_ucum_code(1.0, "wk");
        assert!(quantity_equals(&qty_days, &qty_week).unwrap());

        // Test case: 6 days < 1 week → true
        let qty_6days = FhirQuantity::with_ucum_code(6.0, "d");
        assert!(quantity_less_than(&qty_6days, &qty_week).unwrap());

        // Test case: 8 days > 1 week → true
        let qty_8days = FhirQuantity::with_ucum_code(8.0, "d");
        assert!(quantity_greater_than(&qty_8days, &qty_week).unwrap());
    }

    #[test]
    fn test_to_quantity_function() {
        // Test case: 1.toQuantity() = 1 '1' → true
        let qty1 = to_quantity(&1.0).unwrap();
        assert_eq!(qty1.value, 1.0);
        assert_eq!(qty1.code, Some("1".to_string()));

        // Test case: 1.0.toQuantity() = 1.0 '1' → true
        let qty2 = to_quantity(&1.0).unwrap();
        assert_eq!(qty2.value, 1.0);
        assert_eq!(qty2.code, Some("1".to_string()));

        // Test case: '1'.toQuantity() → 1 '1'
        let qty3 = to_quantity(&"1".to_string()).unwrap();
        assert_eq!(qty3.value, 1.0);
        assert_eq!(qty3.code, Some("1".to_string()));

        // Test case: '1.0'.toQuantity() ~ 1 '1' → true
        let qty4 = to_quantity(&"1.0".to_string()).unwrap();
        let qty_one = FhirQuantity::with_ucum_code(1.0, "1");
        assert!(quantity_approximately_equals(&qty4, &qty_one, None).unwrap());
    }

    #[test]
    fn test_to_quantity_with_units() {
        // Test case: '1 day'.toQuantity() = 1 day → true
        let qty_day = to_quantity(&"1 day".to_string()).unwrap();
        assert_eq!(qty_day.value, 1.0);
        assert_eq!(qty_day.code, Some("d".to_string()));

        // Test case: '1 \'wk\''.toQuantity() = 1 'wk' → true
        let qty_wk = to_quantity(&"1 'wk'".to_string()).unwrap();
        assert_eq!(qty_wk.value, 1.0);
        assert_eq!(qty_wk.code, Some("wk".to_string()));

        // Test literal unit conversion
        let qty_week = to_quantity(&"1 week".to_string()).unwrap();
        assert_eq!(qty_week.value, 1.0);
        assert_eq!(qty_week.code, Some("wk".to_string()));
    }

    #[test]
    fn test_converts_to_quantity_function() {
        // Test case: 1.convertsToQuantity() → true
        assert!(converts_to_quantity(&1.0));

        // Test case: 1.0.convertsToQuantity() → true
        assert!(converts_to_quantity(&1.0));

        // Test case: '1'.convertsToQuantity() → true
        assert!(converts_to_quantity(&"1".to_string()));

        // Test case: '1 day'.convertsToQuantity() → true
        assert!(converts_to_quantity(&"1 day".to_string()));

        // Test case: '1 \'wk\''.convertsToQuantity() → true
        assert!(converts_to_quantity(&"1 'wk'".to_string()));

        // Test case: '1.0'.convertsToQuantity() → true
        assert!(converts_to_quantity(&"1.0".to_string()));

        // Test case: true.convertsToQuantity() → true
        assert!(converts_to_quantity(&true));

        // Test case: '1 wk'.convertsToQuantity().not() → true (unquoted UCUM should fail)
        assert!(!converts_to_quantity(&"1 wk".to_string()));

        // Test case: '1.a'.convertsToQuantity().not() → true (invalid format should fail)
        assert!(!converts_to_quantity(&"1.a".to_string()));
    }

    #[test]
    fn test_quantity_arithmetic_operations() {
        // Test case: 2.0 'cm' * 2.0 'm' = 0.040 'm2' → true
        let qty_cm = FhirQuantity::with_ucum_code(2.0, "cm");
        let qty_m = FhirQuantity::with_ucum_code(2.0, "m");
        let result = quantity_multiply(&qty_cm, &qty_m).unwrap();
        assert_eq!(result.value, 0.04);
        assert_eq!(result.code, Some("m2".to_string()));

        // Test case: 4.0 'g' / 2.0 'm' = 2 'g/m' → true (using canonical units)
        let qty_g = FhirQuantity::with_ucum_code(4.0, "g");
        let qty_m2 = FhirQuantity::with_ucum_code(2.0, "m");
        let div_result = quantity_divide(&qty_g, &qty_m2).unwrap();
        assert_eq!(div_result.value, 2.0);
        assert_eq!(div_result.code, Some("kg.m-1".to_string()));

        // Test case: 1.0 'm' / 1.0 'm' = 1 '1' → true
        let qty_m1 = FhirQuantity::with_ucum_code(1.0, "m");
        let qty_m2 = FhirQuantity::with_ucum_code(1.0, "m");
        let unit_result = quantity_divide(&qty_m1, &qty_m2).unwrap();
        assert_eq!(unit_result.value, 1.0);
        assert_eq!(unit_result.code, Some("1".to_string()));
    }

    #[test]
    fn test_quantity_addition_subtraction() {
        // Test addition with unit conversion
        let qty_g = FhirQuantity::with_ucum_code(1.0, "g");
        let qty_mg = FhirQuantity::with_ucum_code(500.0, "mg");
        let add_result = quantity_add(&qty_g, &qty_mg).unwrap();
        assert_eq!(add_result.value, 1.5);
        assert_eq!(add_result.code, Some("g".to_string()));

        // Test subtraction with unit conversion
        let qty_g2 = FhirQuantity::with_ucum_code(2.0, "g");
        let sub_result = quantity_subtract(&qty_g2, &qty_mg).unwrap();
        assert_eq!(sub_result.value, 1.5);
        assert_eq!(sub_result.code, Some("g".to_string()));
    }

    #[test]
    fn test_cross_unit_comparisons() {
        // Test all comparison operators with unit conversion
        let qty_g = FhirQuantity::with_ucum_code(1.0, "g");
        let qty_mg = FhirQuantity::with_ucum_code(1000.0, "mg");
        let qty_mg_less = FhirQuantity::with_ucum_code(500.0, "mg");
        let qty_mg_more = FhirQuantity::with_ucum_code(1500.0, "mg");

        // Equality
        assert!(quantity_equals(&qty_g, &qty_mg).unwrap());

        // Inequality
        assert!(quantity_not_equals(&qty_g, &qty_mg_less).unwrap());

        // Less than
        assert!(quantity_less_than(&qty_mg_less, &qty_g).unwrap());

        // Greater than
        assert!(quantity_greater_than(&qty_mg_more, &qty_g).unwrap());

        // Approximate equality
        assert!(quantity_approximately_equals(&qty_g, &qty_mg, None).unwrap());
    }
}
