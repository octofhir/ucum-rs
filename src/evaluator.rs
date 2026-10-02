//! Semantic evaluator – computes canonical factor, dimension vector and offset
//! from a parsed `UnitExpr`.
//!
//! The evaluator traverses the expression tree and combines factors using the
//! generated registry data (prefixes & units).
//!
//! Limitations (initial version):
//! • Offsets are supported only for linear temperature units (e.g., °C ↔ K).
//!   Offsets must appear only on standalone symbols, not in products/powers.
//! • Square‐bracket arbitrary units are treated as dimensionless with factor 1.
//! • Percentage symbol (%) is treated as dimensionless factor 0.01 in the parser;
//!   here we simply use the numeric value produced by the parser.
//!
//! Future work:
//! • Full offset algebra, logarithmic units, etc.
//! • Detailed error diagnostics with spans.

use crate::math;
use crate::prelude::*;
use crate::{
    ast::*,
    error::UcumError,
    find_unit,
    performance::find_prefix_optimized,
    precision::{Number, NumericOps, from_f64, to_f64},
    registry::is_metric,
    types::{Dimension, SpecialKind, UnitRecord},
};

/// Helper to extract string from either Symbol or SymbolOwned variants
fn extract_symbol_str<'a>(expr: &'a UnitExpr<'a>) -> Option<&'a str> {
    match expr {
        UnitExpr::Symbol(s) => Some(s),
        UnitExpr::SymbolOwned(s) => Some(s.as_str()),
        _ => None,
    }
}

/// A factor as a `Decimal`, with an error instead of a silent 0 when it is out of
/// the `Decimal` range (about 1e-28 to 7.9e28 in magnitude).
#[allow(clippy::result_large_err)]
fn to_number(value: f64) -> Result<Number, UcumError> {
    let number = from_f64(value);
    if value != 0.0 && number.is_zero() {
        return Err(UcumError::precision_overflow(
            "conversion to Decimal",
            &format!("{value} is out of range"),
        ));
    }
    Ok(number)
}

/// `a * b`, with an error instead of a panic when the result does not fit, or
/// instead of a silent 0 when it is too small.
#[allow(clippy::result_large_err)]
fn checked_mul(a: Number, b: Number) -> Result<Number, UcumError> {
    a.checked_mul(b)
        .filter(|r| !r.is_zero() || a.is_zero() || b.is_zero())
        .ok_or_else(|| UcumError::precision_overflow("multiplication", &format!("{a} * {b}")))
}

/// `a / b`, with an error instead of a panic on overflow or division by zero, or
/// instead of a silent 0 when the result is too small.
#[allow(clippy::result_large_err)]
fn checked_div(a: Number, b: Number) -> Result<Number, UcumError> {
    a.checked_div(b)
        .filter(|r| !r.is_zero() || a.is_zero())
        .ok_or_else(|| UcumError::precision_overflow("division", &format!("{a} / {b}")))
}

/// `base^exp` by squaring: O(log exp) steps, and an error instead of a panic when the
/// result does not fit.
#[allow(clippy::result_large_err)]
fn checked_pow(base: Number, exp: i32) -> Result<Number, UcumError> {
    let mut result = Number::one();
    let mut square = base;
    let mut n = exp.unsigned_abs();
    while n > 0 {
        if n & 1 == 1 {
            result = checked_mul(result, square)?;
        }
        n >>= 1;
        if n > 0 {
            square = checked_mul(square, square)?;
        }
    }
    if exp < 0 {
        checked_div(Number::one(), result)
    } else {
        Ok(result)
    }
}

/// `acc += dim * exp` per component, with an error instead of wrapping or saturating
/// when a component does not fit in `i8`.
#[allow(clippy::result_large_err)]
pub(crate) fn add_scaled_dim(
    acc: &mut [i8; 7],
    dim: &Dimension,
    exp: i32,
) -> Result<(), UcumError> {
    for (a, &d) in acc.iter_mut().zip(dim.0.iter()) {
        let current = *a;
        *a = i32::from(d)
            .checked_mul(exp)
            .and_then(|v| v.checked_add(i32::from(current)))
            .and_then(|v| i8::try_from(v).ok())
            .ok_or_else(|| {
                UcumError::precision_overflow(
                    "dimension exponent",
                    &format!("{current} + {d} * {exp} does not fit in i8"),
                )
            })?;
    }
    Ok(())
}

/// Result returned by `evaluate()` – canonical factor, dimension vector, offset.
#[derive(Debug, Clone, PartialEq)]
pub struct EvalResult {
    pub factor: Number,
    pub dim: Dimension,
    pub offset: Number,
}

impl EvalResult {
    const ZERO_DIM: Dimension = Dimension([0; 7]);

    #[allow(clippy::result_large_err)]
    fn numeric(val: f64) -> Result<Self, UcumError> {
        Ok(Self {
            factor: to_number(val)?,
            dim: Self::ZERO_DIM,
            offset: Number::zero(),
        })
    }

    #[allow(clippy::result_large_err)]
    fn from_unit(code: &str) -> Result<Self, UcumError> {
        // Handle empty string and a standalone annotation ("{rbc}") as the unity.
        // Annotations hold the characters 33-126 only (UCUM §6.1)
        let is_annotation = code.len() >= 2
            && code.starts_with('{')
            && code.ends_with('}')
            && code.bytes().all(|b| b.is_ascii_graphic());
        if code.is_empty() || is_annotation {
            return Ok(Self {
                factor: Number::one(),
                dim: Self::ZERO_DIM,
                offset: Number::zero(),
            });
        }

        let (pref_factor, unit) =
            lookup_unit(code).ok_or_else(|| UcumError::unit_not_found(code))?;
        match unit.special {
            // The prefix of a logarithmic unit scales its argument, see
            // `special_function_value`
            SpecialKind::Log10 | SpecialKind::Ln | SpecialKind::TanTimes100 => Ok(Self {
                factor: to_number(unit.factor)?,
                dim: unit.dim,
                offset: Number::zero(),
            }),
            SpecialKind::None | SpecialKind::LinearOffset | SpecialKind::Arbitrary => Ok(Self {
                factor: checked_mul(to_number(pref_factor)?, to_number(unit.factor)?)?,
                dim: unit.dim,
                offset: from_f64(unit.offset),
            }),
        }
    }
}

/// Evaluate a parsed `UnitExpr` into canonical factor, dimension and offset.
#[allow(clippy::result_large_err)]
pub fn evaluate(expr: &UnitExpr) -> Result<EvalResult, UcumError> {
    evaluate_impl(expr)
}

/// Evaluate an owned `UnitExpr` into canonical factor, dimension and offset.
#[allow(clippy::result_large_err)]
pub fn evaluate_owned(expr: &crate::ast::OwnedUnitExpr) -> Result<EvalResult, UcumError> {
    evaluate_owned_impl(expr)
}

/// Internal implementation of evaluate without caching.
#[allow(clippy::result_large_err)]
fn evaluate_impl(expr: &UnitExpr) -> Result<EvalResult, UcumError> {
    match expr {
        UnitExpr::Numeric(v) => EvalResult::numeric(*v),
        UnitExpr::Symbol(sym) => EvalResult::from_unit(sym),
        UnitExpr::SymbolOwned(sym) => EvalResult::from_unit(sym),
        UnitExpr::Product(factors) => {
            // A number times a logarithmic or prism diopter unit ("20.dB", "2.Np",
            // "100.[p'diop]"): the number is the argument of the unit's function
            if let [a, b] = factors.as_slice()
                && let Some((v, code)) = numeric_and_symbol(a, b).or(numeric_and_symbol(b, a))
                && let Some((pref_factor, unit)) = lookup_unit(code)
                && matches!(
                    unit.special,
                    SpecialKind::Log10 | SpecialKind::Ln | SpecialKind::TanTimes100
                )
            {
                return special_function_value(v, code, to_number(pref_factor)?, unit.special);
            }

            let mut factor_acc = Number::one();
            let mut dim_acc = [0i8; 7];
            for fac in factors {
                let res = evaluate(&fac.expr)?;
                if res.offset != Number::zero() {
                    return Err(UcumError::conversion_error(
                        "offset units",
                        "products",
                        "offset units cannot participate in products",
                    ));
                }
                factor_acc = checked_mul(factor_acc, checked_pow(res.factor, fac.exponent)?)?;
                add_scaled_dim(&mut dim_acc, &res.dim, fac.exponent)?;
            }

            Ok(EvalResult {
                factor: factor_acc,
                dim: Dimension(dim_acc),
                offset: Number::zero(),
            })
        }
        UnitExpr::Quotient(num, den) => {
            let n = evaluate(num)?;
            let d = evaluate(den)?;

            if n.offset != Number::zero() || d.offset != Number::zero() {
                return Err(UcumError::conversion_error(
                    "offset units",
                    "quotient expressions",
                    "offset units not allowed in quotient expressions",
                ));
            }

            // Check if numerator is an arbitrary unit (dimensionless)
            // For arbitrary units in the numerator, we need to adopt the inverse dimension of the denominator
            // This is a special case for arbitrary units like [IU] that are dimensionless by definition
            // but need to adopt the inverse dimension of what they're divided by (e.g., [IU]/mL should have
            // dimension L^-3, the inverse of volume). This ensures proper dimensional analysis and
            // commensurability checks when working with arbitrary units in complex expressions.
            let is_arbitrary_numerator = extract_symbol_str(num.as_ref())
                .and_then(find_unit)
                .is_some_and(|unit| unit.special == crate::types::SpecialKind::Arbitrary);

            let mut dim_vec = if is_arbitrary_numerator {
                // For arbitrary units in numerator, use negated dimension of denominator
                // This ensures arbitrary units correctly adopt the inverse dimensions of what they're divided by
                [0i8; 7]
            } else {
                // Normal case: subtract denominator dimension from numerator dimension
                n.dim.0
            };
            add_scaled_dim(&mut dim_vec, &d.dim, -1)?;

            Ok(EvalResult {
                factor: checked_div(n.factor, d.factor)?,
                dim: Dimension(dim_vec),
                offset: Number::zero(),
            })
        }
        UnitExpr::Power(expr, exp) => {
            let base = evaluate(expr)?;
            if base.offset != Number::zero() {
                return Err(UcumError::conversion_error(
                    "offset units",
                    "exponentiation",
                    "offset units not allowed with exponentiation",
                ));
            }
            let mut dim_vec = [0i8; 7];
            add_scaled_dim(&mut dim_vec, &base.dim, *exp)?;
            Ok(EvalResult {
                factor: checked_pow(base.factor, *exp)?,
                dim: Dimension(dim_vec),
                offset: Number::zero(),
            })
        }
    }
}

/// Look up a unit code: an exact match first, then a prefix on a metric unit
/// ("km", "dam", "mm[Hg]"). Returns the prefix factor and the unit.
fn lookup_unit(code: &str) -> Option<(f64, &'static UnitRecord)> {
    if let Some(unit) = find_unit(code)
        && unit.code == code
    {
        return Some((1.0, unit));
    }
    // Prefixes have one ("k") or two ("da", "Ki") characters; the longest one that
    // leaves a metric unit wins (UCUM §4.4)
    (1..=2).rev().find_map(|len| {
        let prefix = code.get(..len).and_then(find_prefix_optimized)?;
        let rest = &code[len..];
        let unit = find_unit(rest).filter(|u| u.code == rest && is_metric(rest))?;
        Some((prefix.factor, unit))
    })
}

/// `(value, code)` when `a` is a plain number and `b` a plain unit symbol.
fn numeric_and_symbol<'e>(a: &'e UnitFactor, b: &'e UnitFactor) -> Option<(f64, &'e str)> {
    match (&a.expr, a.exponent, b.exponent) {
        (UnitExpr::Numeric(v), 1, 1) => Some((*v, extract_symbol_str(&b.expr)?)),
        _ => None,
    }
}

/// The ratio a logarithmic or prism diopter unit stands for, e.g. 20 dB -> 10^2.
/// The prefix scales the argument: 1 dB is 0.1 B.
#[allow(clippy::result_large_err)]
fn special_function_value(
    value: f64,
    code: &str,
    pref_factor: Number,
    special: SpecialKind,
) -> Result<EvalResult, UcumError> {
    let arg = value * to_f64(pref_factor);
    let ratio = match special {
        SpecialKind::Log10 => math::powf(10.0, arg),
        SpecialKind::Ln => math::exp(arg),
        // 100 [p'diop] is a deflection of tan(1 rad)
        SpecialKind::TanTimes100 => math::tan(arg / 100.0),
        _ => return Err(UcumError::special_unit_error(code, "not a function unit")),
    };
    Ok(EvalResult {
        factor: to_number(ratio)?,
        dim: EvalResult::ZERO_DIM,
        offset: Number::zero(),
    })
}

/// Internal implementation of evaluate for owned AST
#[allow(clippy::result_large_err)]
fn evaluate_owned_impl(expr: &crate::ast::OwnedUnitExpr) -> Result<EvalResult, UcumError> {
    match expr {
        crate::ast::OwnedUnitExpr::Numeric(v) => EvalResult::numeric(*v),
        crate::ast::OwnedUnitExpr::Symbol(sym) => EvalResult::from_unit(sym),
        crate::ast::OwnedUnitExpr::Product(factors) => {
            // Convert owned factors to borrowed for evaluation
            let borrowed_factors: Vec<UnitFactor> = factors
                .iter()
                .map(|f| UnitFactor {
                    expr: owned_to_borrowed(&f.expr),
                    exponent: f.exponent,
                })
                .collect();

            let borrowed_expr = UnitExpr::Product(borrowed_factors);
            evaluate_impl(&borrowed_expr)
        }
        crate::ast::OwnedUnitExpr::Quotient(num, den) => {
            let borrowed_num = owned_to_borrowed(num);
            let borrowed_den = owned_to_borrowed(den);
            let borrowed_expr = UnitExpr::Quotient(Box::new(borrowed_num), Box::new(borrowed_den));
            evaluate_impl(&borrowed_expr)
        }
        crate::ast::OwnedUnitExpr::Power(expr, exp) => {
            let borrowed_expr_inner = owned_to_borrowed(expr);
            let borrowed_expr = UnitExpr::Power(Box::new(borrowed_expr_inner), *exp);
            evaluate_impl(&borrowed_expr)
        }
    }
}

/// Convert owned AST to borrowed AST for evaluation
fn owned_to_borrowed(expr: &crate::ast::OwnedUnitExpr) -> UnitExpr<'_> {
    match expr {
        crate::ast::OwnedUnitExpr::Numeric(v) => UnitExpr::Numeric(*v),
        crate::ast::OwnedUnitExpr::Symbol(sym) => UnitExpr::SymbolOwned(sym.clone()),
        crate::ast::OwnedUnitExpr::Product(factors) => {
            let borrowed_factors: Vec<UnitFactor> = factors
                .iter()
                .map(|f| UnitFactor {
                    expr: owned_to_borrowed(&f.expr),
                    exponent: f.exponent,
                })
                .collect();
            UnitExpr::Product(borrowed_factors)
        }
        crate::ast::OwnedUnitExpr::Quotient(num, den) => UnitExpr::Quotient(
            Box::new(owned_to_borrowed(num)),
            Box::new(owned_to_borrowed(den)),
        ),
        crate::ast::OwnedUnitExpr::Power(expr, exp) => {
            UnitExpr::Power(Box::new(owned_to_borrowed(expr)), *exp)
        }
    }
}
