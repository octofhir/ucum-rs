//! Precision arithmetic support for UCUM calculations.
//!
//! This module provides a unified interface for numeric operations that can use
//! either f64 (fast) or rust_decimal::Decimal (precise) based on feature flags.

#[cfg(feature = "precision")]
use rust_decimal::Decimal;

/// Numeric type used for UCUM calculations.
/// Uses rust_decimal::Decimal when precision feature is enabled, f64 otherwise.
#[cfg(feature = "precision")]
pub type Number = Decimal;

#[cfg(not(feature = "precision"))]
pub type Number = f64;

/// Trait for numeric operations that work with both f64 and Decimal
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

impl NumericOps for f64 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
    fn from_f64(val: f64) -> Self { val }
    fn to_f64(self) -> f64 { self }
    fn add(self, other: Self) -> Self { self + other }
    fn sub(self, other: Self) -> Self { self - other }
    fn mul(self, other: Self) -> Self { self * other }
    fn div(self, other: Self) -> Self { self / other }
    fn pow(self, exp: i32) -> Self { self.powi(exp) }
    fn abs(self) -> Self { self.abs() }
}

#[cfg(feature = "precision")]
impl NumericOps for Decimal {
    fn zero() -> Self { Decimal::ZERO }
    fn one() -> Self { Decimal::ONE }
    fn from_f64(val: f64) -> Self {
        Decimal::from_f64_retain(val).unwrap_or(Decimal::ZERO)
    }
    fn to_f64(self) -> f64 {
        self.to_f64().unwrap_or(0.0)
    }
    fn add(self, other: Self) -> Self { self + other }
    fn sub(self, other: Self) -> Self { self - other }
    fn mul(self, other: Self) -> Self { self * other }
    fn div(self, other: Self) -> Self { self / other }
    fn pow(self, exp: i32) -> Self {
        if exp >= 0 {
            self.powi(exp as i64)
        } else {
            Decimal::ONE / self.powi((-exp) as i64)
        }
    }
    fn abs(self) -> Self { self.abs() }
}

/// Helper functions for working with the Number type
impl Number {
    pub fn zero() -> Self {
        <Self as NumericOps>::zero()
    }

    pub fn one() -> Self {
        <Self as NumericOps>::one()
    }

    pub fn from_f64(val: f64) -> Self {
        <Self as NumericOps>::from_f64(val)
    }

    pub fn to_f64(self) -> f64 {
        <Self as NumericOps>::to_f64(self)
    }
}

/// Convert a Number to f64 for backward compatibility
pub fn to_f64(num: Number) -> f64 {
    num.to_f64()
}

/// Convert f64 to Number
pub fn from_f64(val: f64) -> Number {
    Number::from_f64(val)
}
