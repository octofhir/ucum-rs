//! Semantics of UCUM expressions (UCUM §16-26): the magnitude and dimension of a term in
//! the base units, its arbitrary atoms, and for a special unit its function and scale; and
//! the conversion of a measurement between two commensurable terms.

use crate::ast::UnitExpr;
use crate::error::UcumError;
use crate::number::Number;
use crate::prelude::*;
use crate::registry;
use crate::types::{Dimension, Prefix, SpecialKind, UnitRecord};

/// How symbols are matched against the tables (UCUM §3.4).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Case {
    /// Case sensitive symbols (`Pa`, `mm[Hg]`), the default.
    #[default]
    Sensitive,
    /// Case insensitive symbols (`PAL`, `MM[HG]`), in which case has no meaning.
    Insensitive,
}

/// A special unit with its scale (UCUM §22.4): a prefix or numeric factors, `0.1` for `dB`,
/// `2` for `2.Cel`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpecialTerm {
    /// The special unit
    pub unit: &'static UnitRecord,
    /// The scale `α`: a value `r` in the term is the proper quantity `f⁻¹(α r) × u`
    pub scale: Number,
}

/// The semantics of a term: `factor` times the base units of `dim` times its arbitrary
/// atoms, or for a special unit the factor and dimension of its corresponding proper unit.
#[derive(Clone, Debug, PartialEq)]
pub struct EvalResult {
    /// Magnitude in base units; for a special unit, of its corresponding proper unit
    pub factor: Number,
    /// Exponents of the base units
    pub dim: Dimension,
    /// Arbitrary atoms and their exponents, sorted (`[IU]/mL` has `[("[iU]", 1)]`)
    pub arbitrary: Vec<(&'static str, i32)>,
    /// The special unit of the term, if it is one
    pub special: Option<SpecialTerm>,
}

impl EvalResult {
    fn scalar(factor: Number) -> Self {
        Self {
            factor,
            dim: Dimension::zero(),
            arbitrary: Vec::new(),
            special: None,
        }
    }

    /// Whether the term is a number: a proper unit without dimension or arbitrary atoms.
    pub fn is_scalar(&self) -> bool {
        self.special.is_none() && self.dim.is_dimensionless() && self.arbitrary.is_empty()
    }

    /// Whether two terms are commensurable (UCUM §17, §21.3, §25): the same dimension and
    /// the same arbitrary atoms. A special unit is commensurable with its corresponding
    /// proper unit.
    pub fn is_commensurable(&self, other: &Self) -> bool {
        self.dim == other.dim && self.arbitrary == other.arbitrary
    }
}

fn overflow(operation: &str) -> UcumError {
    UcumError::precision_overflow(operation, "the exponent of a factor does not fit")
}

fn algebra_error(code: &str) -> UcumError {
    UcumError::special_unit_error(
        code,
        "a special unit takes part in no product, quotient or power (UCUM §22.1); it can only be scaled by a number",
    )
}

/// The special unit of a term, for error messages.
fn special_code(term: &EvalResult) -> &'static str {
    term.special.map_or("", |s| s.unit.code)
}

fn multiply(a: EvalResult, b: EvalResult) -> Result<EvalResult, UcumError> {
    match (a.special, b.special) {
        (None, None) => {
            let mut dim = a.dim.0;
            add_scaled_dim(&mut dim, &b.dim, 1)?;
            Ok(EvalResult {
                factor: a
                    .factor
                    .checked_mul(b.factor)
                    .ok_or_else(|| overflow("multiplication"))?,
                dim: Dimension(dim),
                arbitrary: combine(&a.arbitrary, &b.arbitrary, 1),
                special: None,
            })
        }
        // A number scales a special unit (UCUM §22.3)
        (Some(special), None) if b.is_scalar() => scaled(a, special, b.factor),
        (None, Some(special)) if a.is_scalar() => scaled(b, special, a.factor),
        _ => Err(algebra_error(if a.special.is_some() {
            special_code(&a)
        } else {
            special_code(&b)
        })),
    }
}

fn scaled(mut term: EvalResult, special: SpecialTerm, by: Number) -> Result<EvalResult, UcumError> {
    let scale = special
        .scale
        .checked_mul(by)
        .ok_or_else(|| overflow("scale"))?;
    term.special = Some(SpecialTerm { scale, ..special });
    Ok(term)
}

fn divide(a: EvalResult, b: EvalResult) -> Result<EvalResult, UcumError> {
    if let Some(special) = a.special
        && b.is_scalar()
    {
        let inverse = Number::ONE
            .checked_div(b.factor)
            .ok_or_else(|| overflow("division"))?;
        return scaled(a, special, inverse);
    }
    multiply(a, power(b, -1)?)
}

fn power(term: EvalResult, exp: i32) -> Result<EvalResult, UcumError> {
    if term.special.is_some() {
        return if exp == 1 {
            Ok(term)
        } else {
            Err(algebra_error(special_code(&term)))
        };
    }
    let mut dim = [0; 7];
    add_scaled_dim(&mut dim, &term.dim, exp)?;
    // A unit is never zero: a number is a positive integer (UCUM §8.1)
    let factor = term
        .factor
        .checked_powi(exp)
        .ok_or_else(|| overflow("exponentiation"))?;
    Ok(EvalResult {
        factor,
        dim: Dimension(dim),
        arbitrary: combine(&[], &term.arbitrary, exp),
        special: None,
    })
}

/// `a + b × exp` for sorted lists of atoms with exponents.
pub(crate) fn combine(
    a: &[(&'static str, i32)],
    b: &[(&'static str, i32)],
    exp: i32,
) -> Vec<(&'static str, i32)> {
    let mut atoms: Vec<(&'static str, i32)> = a.to_vec();
    for &(atom, e) in b {
        let e = e.saturating_mul(exp);
        match atoms.binary_search_by(|(x, _)| x.cmp(&atom)) {
            Ok(i) => atoms[i].1 = atoms[i].1.saturating_add(e),
            Err(i) => atoms.insert(i, (atom, e)),
        }
    }
    atoms.retain(|&(_, e)| e != 0);
    atoms
}

/// `acc += dim * exp` per component, with an error instead of wrapping or saturating
/// when a component does not fit in `i8`.
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

pub(crate) fn find_atom(code: &str, case: Case) -> Option<&'static UnitRecord> {
    match case {
        Case::Sensitive => registry::UNITS
            .binary_search_by(|u| u.code.cmp(code))
            .ok()
            .map(|i| &registry::UNITS[i]),
        Case::Insensitive => {
            let upper = code.to_ascii_uppercase();
            registry::UNITS_CI
                .binary_search_by(|(c, _)| (*c).cmp(upper.as_str()))
                .ok()
                .map(|i| &registry::UNITS[registry::UNITS_CI[i].1])
        }
    }
}

pub(crate) fn find_prefix(code: &str, case: Case) -> Option<&'static Prefix> {
    match case {
        Case::Sensitive => registry::PREFIXES
            .binary_search_by(|p| p.symbol.cmp(code))
            .ok()
            .map(|i| &registry::PREFIXES[i]),
        Case::Insensitive => {
            let upper = code.to_ascii_uppercase();
            registry::PREFIXES_CI
                .binary_search_by(|(c, _)| (*c).cmp(upper.as_str()))
                .ok()
                .map(|i| &registry::PREFIXES[registry::PREFIXES_CI[i].1])
        }
    }
}

/// A simple unit: its prefix, if any, and its unit atom. The prefix is the longest leading
/// substring that is a prefix and leaves a metric unit atom; otherwise the whole symbol is
/// the atom (UCUM §4.4).
pub fn lookup_unit(
    code: &str,
    case: Case,
) -> Option<(Option<&'static Prefix>, &'static UnitRecord)> {
    for len in (1..code.len().min(3)).rev() {
        let (Some(prefix), Some(atom)) = (code.get(..len), code.get(len..)) else {
            continue;
        };
        if let Some(prefix) = find_prefix(prefix, case)
            && let Some(unit) = find_atom(atom, case).filter(|u| u.is_metric)
        {
            return Some((Some(prefix), unit));
        }
    }
    find_atom(code, case).map(|unit| (None, unit))
}

fn simple_unit(code: &str, case: Case) -> Result<EvalResult, UcumError> {
    let (prefix, unit) = lookup_unit(code, case).ok_or_else(|| UcumError::unit_not_found(code))?;
    let prefix_factor = prefix.map_or(Number::ONE, |p| p.factor);
    if unit.special.is_special() {
        // The prefix scales the value of a special unit (UCUM §22.3)
        return Ok(EvalResult {
            factor: unit.factor,
            dim: unit.dim,
            arbitrary: Vec::new(),
            special: Some(SpecialTerm {
                unit,
                scale: prefix_factor,
            }),
        });
    }
    let mut arbitrary: Vec<(&'static str, i32)> = unit.arbitrary.to_vec();
    arbitrary.sort();
    Ok(EvalResult {
        factor: prefix_factor
            .checked_mul(unit.factor)
            .ok_or_else(|| overflow("prefix"))?,
        dim: unit.dim,
        arbitrary,
        special: None,
    })
}

/// Evaluate a parsed expression with case sensitive symbols.
pub fn evaluate(expr: &UnitExpr) -> Result<EvalResult, UcumError> {
    evaluate_with_case(expr, Case::Sensitive)
}

/// The deepest syntax tree the evaluator walks, in nodes. `parse_expression` never returns
/// one deeper: its levels (`MAX_NESTING_DEPTH`) are the quotients and products of the tree,
/// and a component adds at most two nodes (`m2{a}`) to the symbol. A deeper tree can only be
/// built by hand, and would overflow the stack.
const MAX_TREE_HEIGHT: usize = crate::parser::MAX_NESTING_DEPTH + 3;

/// Evaluate a parsed expression, matching symbols as `case` says.
///
/// # Errors
///
/// Besides the errors of the units themselves, a syntax tree built by hand deeper than
/// `parse_expression` allows is a `NestingTooDeep`.
pub fn evaluate_with_case(expr: &UnitExpr, case: Case) -> Result<EvalResult, UcumError> {
    evaluate_at(expr, case, 1)
}

/// `evaluate_with_case` for a node at `depth` in the tree.
fn evaluate_at(expr: &UnitExpr, case: Case, depth: usize) -> Result<EvalResult, UcumError> {
    if depth > MAX_TREE_HEIGHT {
        return Err(UcumError::tree_too_deep(MAX_TREE_HEIGHT));
    }
    let child = |expr| evaluate_at(expr, case, depth + 1);
    match expr {
        UnitExpr::Symbol(code) => simple_unit(code, case),
        UnitExpr::Numeric(n) => Ok(EvalResult::scalar(*n)),
        UnitExpr::Annotation(_) => Ok(EvalResult::scalar(Number::ONE)),
        UnitExpr::Annotated(expr, _) => child(expr),
        UnitExpr::Power(base, exp) => power(child(base)?, *exp),
        UnitExpr::Product(terms) => {
            let mut result = EvalResult::scalar(Number::ONE);
            for term in terms {
                result = multiply(result, child(term)?)?;
            }
            Ok(result)
        }
        UnitExpr::Quotient(numerator, denominator) => {
            divide(child(numerator)?, child(denominator)?)
        }
        UnitExpr::Reciprocal(expr) => divide(EvalResult::scalar(Number::ONE), child(expr)?),
    }
}

/// A measurement mapping `value × factor + offset` from one term to another.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Linear {
    pub(crate) factor: Number,
    pub(crate) offset: Number,
}

/// The magnitude a term is measured against: the factor of a proper term, the corresponding
/// unit of a special one.
fn magnitude(term: &EvalResult) -> Number {
    term.factor
}

/// `(α, o, u)` such that a value `r` in the term is the proper quantity `(α r + o) × u`: a
/// proper term (`α = 1`, `o = 0`, `u` its factor), or a special unit on an interval scale (a
/// temperature).
fn affine(term: &EvalResult) -> Option<(Number, Number, Number)> {
    match term.special {
        None => Some((Number::ONE, Number::ZERO, term.factor)),
        Some(special) => Some((special.scale, special.unit.special.offset()?, term.factor)),
    }
}

/// `(α r + o) × u` of `from` as a value of `to`, step by step, so that rounding errors of
/// the factors do not add up: 32 [degF] is 0 Cel, not 2e-26.
fn affine_value(
    value: Number,
    (scale, offset, magnitude): (Number, Number, Number),
    (to_scale, to_offset, to_magnitude): (Number, Number, Number),
) -> Option<Number> {
    let base = scale
        .checked_mul(value)?
        .checked_add(offset)?
        .checked_mul(magnitude)?;
    base.checked_div(to_magnitude)?
        .checked_sub(to_offset)?
        .checked_div(to_scale)
}

/// `log10(n)` for a positive number, exact for powers of ten.
fn log10(n: Number) -> f64 {
    let text = format!("{n:e}");
    let (mantissa, exp) = text.split_once('e').expect("scientific notation");
    let mantissa: f64 = mantissa.parse().expect("mantissa");
    let exp: f64 = exp.parse().expect("exponent");
    if mantissa == 1.0 {
        exp
    } else {
        crate::math::log10(mantissa) + exp
    }
}

/// The linear mapping between two commensurable terms, when there is one: between proper
/// units and temperatures, between logarithmic units (`dB` to `Np`, `B[V]` to `B[mV]`),
/// between tangent units or between square roots of the same unit.
pub(crate) fn linear_conversion(from: &EvalResult, to: &EvalResult) -> Option<Linear> {
    if let (Some(a), Some(b)) = (affine(from), affine(to)) {
        let offset = affine_value(Number::ZERO, a, b)?;
        return Some(Linear {
            factor: affine_value(Number::ONE, a, b)?.checked_sub(offset)?,
            offset,
        });
    }
    let (from_special, to_special) = (from.special?, to.special?);
    let ratio = magnitude(from).checked_div(magnitude(to))?;
    let (alpha_from, alpha_to) = (from_special.scale, to_special.scale);
    let (kind_from, kind_to) = (from_special.unit.special, to_special.unit.special);
    if let (Some(((n_from, d_from), base_from)), Some(((n_to, d_to), base_to))) =
        (kind_from.logarithm(), kind_to.logarithm())
    {
        // r_to = k_to (ln ratio + α_from r ln b_from / k_from) / (α_to ln b_to)
        let k_ratio =
            Number::from_i64(n_to * d_from).checked_div(Number::from_i64(d_to * n_from))?;
        let mut factor = k_ratio.checked_mul(alpha_from)?.checked_div(alpha_to)?;
        if base_from != base_to {
            let bases = crate::math::ln(base_from) / crate::math::ln(base_to);
            factor = factor.checked_mul(Number::from_f64(bases)?)?;
        }
        let k_to = Number::from_i64(n_to).checked_div(Number::from_i64(d_to))?;
        let log_ratio = if ratio == Number::ONE {
            Number::ZERO
        } else if base_to == 10.0 {
            Number::from_f64(log10(ratio))?
        } else {
            Number::from_f64(crate::math::ln(ratio.to_f64()) / crate::math::ln(base_to))?
        };
        let offset = k_to.checked_mul(log_ratio)?.checked_div(alpha_to)?;
        return Some(Linear { factor, offset });
    }
    if kind_from == kind_to && ratio == Number::ONE {
        // 100 tan and √ of the same unit: r_to = α_from r / α_to
        if matches!(kind_from, SpecialKind::TanTimes100 | SpecialKind::Sqrt) {
            return Some(Linear {
                factor: alpha_from.checked_div(alpha_to)?,
                offset: Number::ZERO,
            });
        }
    }
    None
}

/// `value` if it fits in an `f64`: finite, and zero only where the exact value is zero.
fn fits(value: f64, is_zero_expected: bool) -> Option<f64> {
    (value.is_finite() && (value != 0.0 || is_zero_expected)).then_some(value)
}

/// Convert a measurement `value` from one commensurable term to another (UCUM §21.2,
/// §22.4): exactly through decimal arithmetic when the mapping is linear, through the
/// functions of the special units otherwise.
pub(crate) fn convert_value(
    value: f64,
    from: &str,
    from_eval: &EvalResult,
    to: &str,
    to_eval: &EvalResult,
) -> Result<f64, UcumError> {
    let operands = || format!("{value:?} {from} to {to}");
    // A measurement value is a real number (UCUM §18.5); a function would also map an
    // infinite value to a finite one: 10^-∞ is 0
    if !value.is_finite() {
        return Err(UcumError::non_finite_value("conversion", &operands()));
    }
    let overflow = || UcumError::precision_overflow("conversion", &operands());
    let exact = |result: Option<Number>| {
        let result = result.ok_or_else(overflow)?;
        fits(result.to_f64(), result.is_zero()).ok_or_else(overflow)
    };
    let out_of_domain = |unit: &str| UcumError::out_of_domain(unit, &format!("{value:?} {from}"));
    if let (Some(a), Some(b)) = (affine(from_eval), affine(to_eval)) {
        return exact(Number::from_f64(value).and_then(|v| affine_value(v, a, b)));
    }
    if let Some(linear) = linear_conversion(from_eval, to_eval) {
        return exact(
            Number::from_f64(value)
                .and_then(|v| v.checked_mul(linear.factor))
                .and_then(|v| v.checked_add(linear.offset)),
        );
    }
    // In multiples of the magnitude of `from`, then of `to`. A function result of zero is
    // a value only where the function has a zero
    let x = match from_eval.special {
        Some(special) => {
            let kind = special.unit.special;
            let r = exact(Number::from_f64(value).and_then(|v| v.checked_mul(special.scale)))?;
            let x = kind.inverse(r).ok_or_else(|| out_of_domain(from))?;
            fits(x, kind.forward(0.0) == Some(r)).ok_or_else(overflow)?
        }
        None => value,
    };
    let ratio = magnitude(from_eval).checked_div(magnitude(to_eval));
    let y = exact(
        Number::from_f64(x)
            .zip(ratio)
            .and_then(|(x, ratio)| x.checked_mul(ratio)),
    )?;
    match to_eval.special {
        Some(special) => {
            let kind = special.unit.special;
            let r = kind.forward(y).ok_or_else(|| out_of_domain(to))?;
            let r = fits(r, kind.inverse(0.0) == Some(y)).ok_or_else(overflow)?;
            exact(Number::from_f64(r).and_then(|r| r.checked_div(special.scale)))
        }
        None => Ok(y),
    }
}
