//! Abstract Syntax Tree (AST) definitions for UCUM expressions.
//!
//! Separated from the parser so that other components (e.g. registry, semantic
//! checks, conversions) can depend on the data model without pulling in the
//! `nom` parsing machinery.

use core::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A zero-copy UCUM expression for parsing (borrows from input string).
#[derive(Clone, Debug, PartialEq)]
pub enum UnitExpr<'a> {
    /// Product of multiple factors (each with an integer exponent).
    Product(Vec<UnitFactor<'a>>),
    /// Quotient of numerator / denominator.
    Quotient(Box<UnitExpr<'a>>, Box<UnitExpr<'a>>),
    /// An expression raised to an integer power.
    Power(Box<UnitExpr<'a>>, i32),
    /// A numeric coefficient (e.g., 1000 from `10*3`).
    Numeric(f64),
    /// A single symbol borrowed from input (zero-copy).
    Symbol(&'a str),
    /// A symbol that requires ownership (normalization, etc).
    SymbolOwned(String),
}

/// Owned version of UnitExpr for final results.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum OwnedUnitExpr {
    /// Product of multiple factors (each with an integer exponent).
    Product(Vec<OwnedUnitFactor>),
    /// Quotient of numerator / denominator.
    Quotient(Box<OwnedUnitExpr>, Box<OwnedUnitExpr>),
    /// An expression raised to an integer power.
    Power(Box<OwnedUnitExpr>, i32),
    /// A numeric coefficient (e.g., 1000 from `10*3`).
    Numeric(f64),
    /// A single symbol (possibly prefix+code) referring to a registry unit.
    Symbol(String),
}

impl<'a> fmt::Display for UnitExpr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnitExpr::Product(factors) => {
                for (idx, fac) in factors.iter().enumerate() {
                    if idx > 0 {
                        write!(f, ".")?;
                    }
                    write!(f, "{}", fac)?;
                }
                Ok(())
            }
            UnitExpr::Quotient(num, den) => write!(f, "{}/{}", num, den),
            UnitExpr::Power(expr, exp) => write!(f, "({})^{}", expr, exp),
            UnitExpr::Numeric(val) => write!(f, "{}", val),
            UnitExpr::Symbol(sym) => write!(f, "{}", sym),
            UnitExpr::SymbolOwned(sym) => write!(f, "{}", sym),
        }
    }
}

impl fmt::Display for OwnedUnitExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OwnedUnitExpr::Product(factors) => {
                for (idx, fac) in factors.iter().enumerate() {
                    if idx > 0 {
                        write!(f, ".")?;
                    }
                    write!(f, "{}", fac)?;
                }
                Ok(())
            }
            OwnedUnitExpr::Quotient(num, den) => write!(f, "{}/{}", num, den),
            OwnedUnitExpr::Power(expr, exp) => write!(f, "({})^{}", expr, exp),
            OwnedUnitExpr::Numeric(val) => write!(f, "{}", val),
            OwnedUnitExpr::Symbol(sym) => write!(f, "{}", sym),
        }
    }
}

/// A base expression accompanied by an integer exponent (default 1) - zero-copy version.
#[derive(Clone, Debug, PartialEq)]
pub struct UnitFactor<'a> {
    pub expr: UnitExpr<'a>,
    pub exponent: i32,
}

/// A base expression accompanied by an integer exponent (default 1) - owned version.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OwnedUnitFactor {
    pub expr: OwnedUnitExpr,
    pub exponent: i32,
}

impl<'a> fmt::Display for UnitFactor<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.exponent == 1 {
            write!(f, "{}", self.expr)
        } else {
            write!(f, "{}^{}", self.expr, self.exponent)
        }
    }
}

impl fmt::Display for OwnedUnitFactor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.exponent == 1 {
            write!(f, "{}", self.expr)
        } else {
            write!(f, "{}^{}", self.expr, self.exponent)
        }
    }
}

/// Conversion from zero-copy to owned AST.
impl<'a> UnitExpr<'a> {
    pub fn to_owned(self) -> OwnedUnitExpr {
        match self {
            UnitExpr::Product(factors) => {
                OwnedUnitExpr::Product(factors.into_iter().map(|f| f.to_owned()).collect())
            }
            UnitExpr::Quotient(num, den) => {
                OwnedUnitExpr::Quotient(Box::new(num.as_ref().clone().to_owned()), Box::new(den.as_ref().clone().to_owned()))
            }
            UnitExpr::Power(expr, exp) => {
                OwnedUnitExpr::Power(Box::new(expr.as_ref().clone().to_owned()), exp)
            }
            UnitExpr::Numeric(val) => OwnedUnitExpr::Numeric(val),
            UnitExpr::Symbol(sym) => OwnedUnitExpr::Symbol(sym.to_string()),
            UnitExpr::SymbolOwned(sym) => OwnedUnitExpr::Symbol(sym),
        }
    }
}

impl<'a> UnitFactor<'a> {
    pub fn to_owned(self) -> OwnedUnitFactor {
        OwnedUnitFactor {
            expr: self.expr.to_owned(),
            exponent: self.exponent,
        }
    }
}
