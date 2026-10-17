//! Abstract syntax tree of UCUM expressions (UCUM §1-10).
//!
//! The tree keeps annotations so that an expression displays as it was written
//! (`kg{dry}/m3`), although they have no meaning (UCUM §6.2) and evaluation ignores them.

use crate::number::Number;
use crate::prelude::*;
use core::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A parsed UCUM expression.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum UnitExpr {
    /// A simple unit: a unit atom, with a prefix if the atom is metric (`m`, `kg`,
    /// `mm[Hg]`, `10*`).
    Symbol(String),
    /// A positive integer factor (UCUM §8): `4` in `4.[pi]`.
    Numeric(Number),
    /// A simple unit raised to an integer power (UCUM §9): `m2`, `s-1`, `10*3`.
    Power(Box<UnitExpr>, i32),
    /// An annotation on its own, the unity (UCUM §6.4): `{rbc}`, without the braces.
    Annotation(String),
    /// A simple unit, a power or an integer followed by an annotation (`kg{dry}`), without
    /// the braces.
    Annotated(Box<UnitExpr>, String),
    /// Terms joined by '.': `kg.m`.
    Product(Vec<UnitExpr>),
    /// A term divided by a component: `m/s`. Terms are read left to right (UCUM §7.4), so
    /// `a/b.c` is `Product([Quotient(a, b), c])`.
    Quotient(Box<UnitExpr>, Box<UnitExpr>),
    /// A leading solidus, which inverts the component right after it (UCUM §7.3): `/min`.
    Reciprocal(Box<UnitExpr>),
}

impl UnitExpr {
    /// Whether the expression needs parentheses to be one component of a term.
    fn is_compound(&self) -> bool {
        matches!(
            self,
            UnitExpr::Product(_) | UnitExpr::Quotient(..) | UnitExpr::Reciprocal(_)
        )
    }
}

/// `expr` as a component: in parentheses when it is a term of its own.
struct Component<'a>(&'a UnitExpr);

impl fmt::Display for Component<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_compound() {
            f.write_str("(")?;
            write_term(f, self.0, false)?;
            f.write_str(")")
        } else {
            write_term(f, self.0, false)
        }
    }
}

/// Write a term. A leading solidus is only valid at the start of an expression (UCUM §7.3),
/// so elsewhere, inside parentheses, a reciprocal is written `1/x`.
fn write_term(f: &mut fmt::Formatter<'_>, expr: &UnitExpr, top: bool) -> fmt::Result {
    match expr {
        UnitExpr::Symbol(symbol) => f.write_str(symbol),
        UnitExpr::Numeric(n) => write!(f, "{n}"),
        UnitExpr::Power(base, exp) => write!(f, "{}{exp}", Component(base)),
        UnitExpr::Annotation(text) => write!(f, "{{{text}}}"),
        UnitExpr::Annotated(expr, text) => write!(f, "{}{{{text}}}", Component(expr)),
        UnitExpr::Product(terms) => {
            for (i, term) in terms.iter().enumerate() {
                if i == 0 {
                    // Terms are read left to right, so the first one needs no parentheses
                    write_term(f, term, top)?;
                } else {
                    write!(f, ".{}", Component(term))?;
                }
            }
            Ok(())
        }
        UnitExpr::Quotient(numerator, denominator) => {
            write_term(f, numerator, top)?;
            write!(f, "/{}", Component(denominator))
        }
        UnitExpr::Reciprocal(expr) if top => write!(f, "/{}", Component(expr)),
        UnitExpr::Reciprocal(expr) => write!(f, "1/{}", Component(expr)),
    }
}

// Display writes UCUM that parses back to the same expression: "kg/(m.s)", "s-2", "/min".
impl fmt::Display for UnitExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_term(f, self, true)
    }
}
