//! Abstract Syntax Tree (AST) definitions for UCUM expressions.
//!
//! Separated from the parser so that other components (e.g. registry, semantic
//! checks, conversions) can depend on the data model without pulling in the
//! `nom` parsing machinery.

use core::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A fully parsed UCUM expression.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum UnitExpr {
    /// Product of multiple factors (each with an integer exponent).
    Product(Vec<UnitFactor>),
    /// Quotient of numerator / denominator.
    Quotient(Box<UnitExpr>, Box<UnitExpr>),
    /// An expression raised to an integer power.
    Power(Box<UnitExpr>, i32),
    /// A numeric coefficient (e.g., 1000 from `10*3`).
    Numeric(f64),
    /// A single symbol (possibly prefix+code) referring to a registry unit.
    Symbol(String),
}

impl fmt::Display for UnitExpr {
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
        }
    }
}

/// A base expression accompanied by an integer exponent (default 1).
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UnitFactor {
    pub expr: UnitExpr,
    pub exponent: i32,
}

impl fmt::Display for UnitFactor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.exponent == 1 {
            write!(f, "{}", self.expr)
        } else {
            write!(f, "{}^{}", self.expr, self.exponent)
        }
    }
}
