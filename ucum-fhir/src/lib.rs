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

use octofhir_ucum_core::{Quantity as UcumQuantity, UcumError, evaluate_owned, parse_expression, precision::to_f64};
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

pub fn convert_quantity(
    quantity: &FhirQuantity,
    target_unit: &str,
) -> Result<FhirQuantity, FhirError> {
    let ucum_quantity = quantity.to_ucum_quantity()?;
    let target_expr = parse_expression(target_unit)?;
    let source_eval = evaluate_owned(&ucum_quantity.unit)?;
    let target_eval = evaluate_owned(&target_expr)?;

    if source_eval.dim != target_eval.dim {
        return Err(FhirError::InvalidCode(format!(
            "Units are not commensurable: {} and {}",
            ucum_quantity.unit, target_unit
        )));
    }

    let factor = source_eval.factor / target_eval.factor;

    Ok(FhirQuantity {
        value: ucum_quantity.value * to_f64(factor),
        unit: Some(target_unit.to_string()),
        system: Some("http://unitsofmeasure.org".to_string()),
        code: Some(target_unit.to_string()),
        comparator: quantity.comparator.clone(),
    })
}





pub fn are_equivalent(a: &FhirQuantity, b: &FhirQuantity) -> Result<bool, FhirError> {
    let a_ucum = a.to_ucum_quantity()?;
    let b_ucum = b.to_ucum_quantity()?;
    let a_eval = evaluate_owned(&a_ucum.unit)?;
    let b_eval = evaluate_owned(&b_ucum.unit)?;

    if a_eval.dim != b_eval.dim {
        return Err(FhirError::InvalidCode(format!(
            "Units are not commensurable: {} and {}",
            a_ucum.unit, b_ucum.unit
        )));
    }

    let factor = b_eval.factor / a_eval.factor;
    const EPSILON: f64 = 1e-6;
    let diff = (a_ucum.value - b_ucum.value * to_f64(factor)).abs();
    let max = a_ucum.value.abs().max((b_ucum.value * to_f64(factor)).abs());

    if max < EPSILON {
        Ok(diff < EPSILON)
    } else {
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
        assert!((converted.value - 1.0).abs() < 1e-10, "Expected ~1.0, got {}", converted.value);
        assert_eq!(converted.code, Some("g".to_string()));
    }

    #[test]
    fn test_are_equivalent() {
        let a = FhirQuantity::with_ucum_code(1.0, "g");
        let b = FhirQuantity::with_ucum_code(1000.0, "mg");
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



}
