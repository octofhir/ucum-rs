//! Precision arithmetic support for UCUM calculations.
//!
//! This module provides high-precision arithmetic using rust_decimal::Decimal
//! for all UCUM calculations to ensure accurate unit conversions.
//! 
//! Enhanced precision features:
//! - Configurable decimal precision
//! - Safe arithmetic with overflow detection
//! - Conversion error bounds tracking
//! - Multiple rounding modes

use rust_decimal::{Decimal, RoundingStrategy};

/// Numeric type used for UCUM calculations.
/// Always uses rust_decimal::Decimal for precision.
pub type Number = Decimal;

/// Trait for numeric operations that work with Decimal
pub trait NumericOps: Copy + Clone + PartialEq + std::fmt::Debug {
    fn zero() -> Self;
    fn one() -> Self;
    fn from_f64(val: f64) -> Self;
    fn to_f64(self) -> f64;
    fn add(self, other: Self) -> Self;
    fn sub(self, other: Self) -> Self;
    fn mul(self, other: Self) -> Self;
    fn div(self, other: Self) -> Self;
    fn pow(self, exp: i32) -> Self;
    fn abs(self) -> Self;
}

impl NumericOps for Decimal {
    fn zero() -> Self {
        Decimal::ZERO
    }
    fn one() -> Self {
        Decimal::ONE
    }
    fn from_f64(val: f64) -> Self {
        Decimal::from_f64_retain(val).unwrap_or(Decimal::ZERO)
    }
    fn to_f64(self) -> f64 {
        self.try_into().unwrap_or(0.0)
    }
    fn add(self, other: Self) -> Self {
        self + other
    }
    fn sub(self, other: Self) -> Self {
        self - other
    }
    fn mul(self, other: Self) -> Self {
        self * other
    }
    fn div(self, other: Self) -> Self {
        self / other
    }
    fn pow(self, exp: i32) -> Self {
        if exp == 0 {
            Decimal::ONE
        } else if exp > 0 {
            let mut result = Decimal::ONE;
            for _ in 0..exp {
                result *= self;
            }
            result
        } else {
            let mut result = Decimal::ONE;
            for _ in 0..(-exp) {
                result *= self;
            }
            Decimal::ONE / result
        }
    }
    fn abs(self) -> Self {
        if self < Decimal::ZERO {
            -self
        } else {
            self
        }
    }
}

/// Helper implementation for f64 compatibility
impl NumericOps for f64 {
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
    fn from_f64(val: f64) -> Self {
        val
    }
    fn to_f64(self) -> f64 {
        self
    }
    fn add(self, other: Self) -> Self {
        self + other
    }
    fn sub(self, other: Self) -> Self {
        self - other
    }
    fn mul(self, other: Self) -> Self {
        self * other
    }
    fn div(self, other: Self) -> Self {
        self / other
    }
    fn pow(self, exp: i32) -> Self {
        self.powi(exp)
    }
    fn abs(self) -> Self {
        self.abs()
    }
}

/// Helper functions for working with the Number type

/// Create a zero Number
pub fn zero() -> Number {
    <Number as NumericOps>::zero()
}

/// Create a one Number
pub fn one() -> Number {
    <Number as NumericOps>::one()
}

/// Convert a Number to f64 for backward compatibility
pub fn to_f64(num: Number) -> f64 {
    <Number as NumericOps>::to_f64(num)
}

/// Convert f64 to Number
pub fn from_f64(val: f64) -> Number {
    <Number as NumericOps>::from_f64(val)
}

// ============================================================================
// Enhanced Precision Configuration and Types
// ============================================================================

/// Precision configuration for decimal arithmetic
#[derive(Debug, Clone, Copy)]
pub struct PrecisionConfig {
    /// Number of decimal places to maintain
    pub decimal_places: u32,
    /// Rounding strategy to use
    pub rounding: RoundingMode,
    /// Tolerance for floating-point comparisons
    pub tolerance: f64,
}

impl Default for PrecisionConfig {
    fn default() -> Self {
        Self {
            decimal_places: 12,
            rounding: RoundingMode::RoundHalfUp,
            tolerance: 1e-10,
        }
    }
}

/// Rounding modes for decimal operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundingMode {
    RoundHalfUp,
    RoundHalfDown,
    RoundHalfEven,
    RoundUp,
    RoundDown,
}

impl From<RoundingMode> for RoundingStrategy {
    fn from(mode: RoundingMode) -> Self {
        match mode {
            RoundingMode::RoundHalfUp => RoundingStrategy::MidpointAwayFromZero,
            RoundingMode::RoundHalfDown => RoundingStrategy::MidpointTowardZero,
            RoundingMode::RoundHalfEven => RoundingStrategy::MidpointNearestEven,
            RoundingMode::RoundUp => RoundingStrategy::AwayFromZero,
            RoundingMode::RoundDown => RoundingStrategy::ToZero,
        }
    }
}

/// Result of a high-precision calculation with error bounds
#[derive(Debug, Clone)]
pub struct DecimalResult {
    /// The calculated value
    pub value: Number,
    /// The original unit expression
    pub unit: String,
    /// Estimated error bound
    pub error_bound: f64,
    /// Precision configuration used
    pub config: PrecisionConfig,
}

impl DecimalResult {
    /// Create a new decimal result
    pub fn new(value: Number, unit: String, config: PrecisionConfig) -> Self {
        Self {
            value,
            unit,
            error_bound: 0.0,
            config,
        }
    }

    /// Create with error bound
    pub fn with_error_bound(value: Number, unit: String, error_bound: f64, config: PrecisionConfig) -> Self {
        Self {
            value,
            unit,
            error_bound,
            config,
        }
    }

    /// Convert to f64 for compatibility
    pub fn to_f64(&self) -> f64 {
        to_f64(self.value)
    }

    /// Check if the result is within tolerance of expected value
    pub fn is_close_to(&self, expected: f64, tolerance: Option<f64>) -> bool {
        let tolerance = tolerance.unwrap_or(self.config.tolerance);
        let diff = (self.to_f64() - expected).abs();
        diff <= tolerance
    }

    /// Get the relative error compared to expected value
    pub fn relative_error(&self, expected: f64) -> f64 {
        if expected == 0.0 {
            self.to_f64().abs()
        } else {
            ((self.to_f64() - expected) / expected).abs()
        }
    }
}

/// Safe factor representation for numerical stability
#[derive(Debug, Clone, Copy)]
pub struct SafeFactor {
    /// Mantissa part
    pub mantissa: i64,
    /// Exponent part (base 10)
    pub exponent: i16,
    /// Number of significant digits
    pub precision_bits: u8,
}

impl SafeFactor {
    /// Create a new SafeFactor from f64
    pub fn from_f64(value: f64) -> Result<Self, crate::error::UcumError> {
        if !value.is_finite() {
            return Err(crate::error::UcumError::conversion_error(
                "non-finite number", "SafeFactor",
                "Cannot convert non-finite number to SafeFactor"
            ));
        }

        if value == 0.0 {
            return Ok(Self {
                mantissa: 0,
                exponent: 0,
                precision_bits: 1,
            });
        }

        // Extract mantissa and exponent using scientific notation
        let abs_value = value.abs();
        let exponent = abs_value.log10().floor() as i16;
        let mantissa_f64 = abs_value / 10.0_f64.powi(exponent as i32);
        
        // Scale to integer mantissa with maximum precision
        let scale = 10_i64.pow(15); // ~15 digits precision
        let mantissa = (mantissa_f64 * scale as f64).round() as i64;
        let mantissa = if value < 0.0 { -mantissa } else { mantissa };

        Ok(Self {
            mantissa,
            exponent: exponent - 15, // Adjust for scaling
            precision_bits: 60, // ~15 decimal digits
        })
    }

    /// Safely multiply two SafeFactors
    pub fn multiply_safe(&self, other: &SafeFactor) -> Result<SafeFactor, crate::error::UcumError> {
        // Check for overflow
        if self.mantissa.checked_mul(other.mantissa).is_none() {
            return Err(crate::error::UcumError::conversion_error(
                "SafeFactor", "multiplication",
                "Multiplication overflow in SafeFactor"
            ));
        }

        let mantissa = self.mantissa * other.mantissa;
        let exponent = self.exponent + other.exponent;

        // Normalize if needed
        let mut result_mantissa = mantissa;
        let mut result_exponent = exponent;

        if result_mantissa != 0 {
            while result_mantissa % 10 == 0 {
                result_mantissa /= 10;
                result_exponent += 1;
            }
        }

        Ok(SafeFactor {
            mantissa: result_mantissa,
            exponent: result_exponent,
            precision_bits: std::cmp::min(self.precision_bits, other.precision_bits),
        })
    }

    /// Safely divide two SafeFactors
    pub fn divide_safe(&self, other: &SafeFactor) -> Result<SafeFactor, crate::error::UcumError> {
        if other.mantissa == 0 {
            return Err(crate::error::UcumError::conversion_error(
                "SafeFactor", "division",
                "Division by zero in SafeFactor"
            ));
        }

        // For division, we need to maintain precision
        let dividend = self.mantissa * 1_000_000; // Scale up for precision
        let mantissa = dividend / other.mantissa;
        let exponent = self.exponent - other.exponent - 6; // Adjust for scaling

        Ok(SafeFactor {
            mantissa,
            exponent,
            precision_bits: std::cmp::min(self.precision_bits, other.precision_bits),
        })
    }

    /// Convert to decimal with specified precision
    pub fn to_decimal(&self) -> Decimal {
        let mantissa_decimal = Decimal::from(self.mantissa);
        let power_of_ten = if self.exponent >= 0 {
            Decimal::from(10_i64.pow(self.exponent as u32))
        } else {
            Decimal::ONE / Decimal::from(10_i64.pow((-self.exponent) as u32))
        };
        mantissa_decimal * power_of_ten
    }

    /// Convert to f64
    pub fn to_f64(&self) -> f64 {
        if self.mantissa == 0 {
            0.0
        } else {
            (self.mantissa as f64) * 10.0_f64.powi(self.exponent as i32)
        }
    }
}

/// Precision-aware conversion functions
pub mod conversion {
    use super::*;

    /// Convert between units with specified precision
    pub fn convert_precise(
        value: Number,
        from_factor: Number,
        to_factor: Number,
        config: PrecisionConfig,
    ) -> Result<DecimalResult, crate::error::UcumError> {
        // Perform conversion: result = value * (from_factor / to_factor)
        if to_factor == zero() {
            return Err(crate::error::UcumError::conversion_error(
                "unit", "conversion",
                "Cannot convert to unit with zero factor"
            ));
        }

        let ratio = from_factor.div(to_factor);
        let result_value = value.mul(ratio);

        // Estimate error bound based on precision
        let error_bound = if config.decimal_places > 0 {
            10.0_f64.powi(-(config.decimal_places as i32))
        } else {
            f64::EPSILON
        };

        Ok(DecimalResult::with_error_bound(
            result_value,
            "".to_string(), // Unit will be set by caller
            error_bound,
            config,
        ))
    }

    /// Convert with safe factor arithmetic
    pub fn convert_with_safe_factors(
        value: f64,
        from_factor: SafeFactor,
        to_factor: SafeFactor,
    ) -> Result<f64, crate::error::UcumError> {
        let value_factor = SafeFactor::from_f64(value)?;
        let intermediate = value_factor.multiply_safe(&from_factor)?;
        let result = intermediate.divide_safe(&to_factor)?;
        Ok(result.to_f64())
    }
}

/// Enhanced comparison functions for floating-point numbers
pub mod comparison {
    /// Check if two f64 values are approximately equal within tolerance
    pub fn approx_equal(a: f64, b: f64, tolerance: f64) -> bool {
        (a - b).abs() <= tolerance
    }

    /// Check if two f64 values are approximately equal with relative tolerance
    pub fn approx_equal_relative(a: f64, b: f64, relative_tolerance: f64) -> bool {
        if a == b {
            return true;
        }
        
        let diff = (a - b).abs();
        let largest = a.abs().max(b.abs());
        
        if largest == 0.0 {
            diff <= relative_tolerance
        } else {
            diff / largest <= relative_tolerance
        }
    }

    /// Get the relative error between two values
    pub fn relative_error(actual: f64, expected: f64) -> f64 {
        if expected == 0.0 {
            actual.abs()
        } else {
            ((actual - expected) / expected).abs()
        }
    }
}
