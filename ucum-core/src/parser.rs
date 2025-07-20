//! Internal UCUM expression parsers implemented with `nom`.
//!
//! This module defines all low-level parsers that operate on `&str` slices and
//! build the typed AST defined in `crate::ast`.

use crate::ast::{UnitExpr, UnitFactor};
use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, digit1, multispace1},
    combinator::{map, map_res, opt, recognize},
    number::complete::recognize_float,
    sequence::{delimited, pair, preceded, tuple},
};

// ---------------------- atomic helpers ----------------------
fn is_symbol_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '%' | '_' | '[' | ']' | '\'' | 'µ')
}

fn parse_symbol(input: &str) -> IResult<&str, UnitExpr> {
    map_res(take_while1(is_symbol_char), |s: &str| {
        // Normalise Unicode micro sign to ASCII 'u'
        let normalized = s.replace('µ', "u");
        if normalized.contains('%') && normalized.len() > 1 {
            // Percent must be a standalone unit symbol
            Err(())
        } else {
            Ok(UnitExpr::Symbol(normalized))
        }
    })(input)
}

fn parse_numeric(input: &str) -> IResult<&str, UnitExpr> {
    // Parses 10*exp or 10^exp where exp is a signed integer.
    let signed_int = |input| recognize(pair(opt(char('-')), digit1))(input);

    let star_parser = map_res(preceded(tag("10*"), signed_int), |s: &str| s.parse::<i32>());
    let caret_parser = map_res(preceded(tag("10^"), signed_int), |s: &str| s.parse::<i32>());

    map(alt((star_parser, caret_parser)), |exp: i32| {
        UnitExpr::Numeric(10f64.powi(exp))
    })(input)
}

fn parse_decimal(input: &str) -> IResult<&str, UnitExpr> {
    map_res(recognize_float, |s: &str| {
        s.parse::<f64>().map(UnitExpr::Numeric)
    })(input)
}

fn parse_paren_expr(input: &str) -> IResult<&str, UnitExpr> {
    delimited(char('('), parse_quotient, char(')'))(input)
}

fn parse_base_atomic(input: &str) -> IResult<&str, UnitExpr> {
    alt((parse_paren_expr, parse_numeric, parse_decimal, parse_symbol))(input)
}

// ---------------------- annotations -------------------------
fn annotation_body(i: &str) -> IResult<&str, &str> {
    // Consume until an unescaped '}'
    let mut escaped = false;
    for (idx, ch) in i.char_indices() {
        match (escaped, ch) {
            (false, '\\') => escaped = true,
            (true, _) => escaped = false,
            (false, '}') => return Ok((&i[idx..], &i[..idx])),
            _ => {}
        }
    }
    Err(nom::Err::Error(nom::error::Error::new(
        i,
        nom::error::ErrorKind::Eof,
    )))
}

fn parse_annotation(input: &str) -> IResult<&str, ()> {
    map(delimited(char('{'), annotation_body, char('}')), |_| ())(input)
}

// ---------------------- exponents & factors -----------------
fn parse_exponent(input: &str) -> IResult<&str, i32> {
    preceded(
        char('^'),
        map_res(recognize(pair(opt(char('-')), digit1)), |s: &str| {
            s.parse::<i32>()
        }),
    )(input)
}

fn parse_base_expr(input: &str) -> IResult<&str, UnitExpr> {
    let (rest, expr) = parse_base_atomic(input)?;
    let (rest, _) = opt(parse_annotation)(rest)?;
    Ok((rest, expr))
}

pub fn parse_factor(input: &str) -> IResult<&str, UnitFactor> {
    // Parse base expression and possible exponent/annotation that follow.
    let (rest, base_expr) = parse_base_expr(input)?;
    let (rest, explicit_exp) = opt(parse_exponent)(rest)?;
    let (rest, _) = opt(parse_annotation)(rest)?;

    // Determine exponent:
    // 1. If explicit ^n provided, use it.
    // 2. Otherwise, attempt implicit integer suffix on a symbol (e.g., "m2").
    // 3. Default is 1.
    let (expr, exponent) = match (&base_expr, explicit_exp) {
        (_, Some(exp)) => (base_expr.clone(), exp),
        (UnitExpr::Symbol(sym), None) => {
            // Scan for an implicit exponent suffix which may include an optional sign, e.g. "m2", "s-1", "mol+3".
            // Algorithm:
            // 1. Walk backwards gathering trailing digits.
            // 2. Optionally capture a preceding '+' or '-' sign.
            // 3. If we found at least one digit, treat the collected substring as the exponent.
            // 4. Everything before the exponent substring is considered the base symbol.
            let mut chars = sym.char_indices().collect::<Vec<_>>();
            let mut idx = sym.len();

            // Step backwards over digits
            while let Some(&(pos, ch)) = chars.last() {
                if ch.is_ascii_digit() {
                    idx = pos;
                    chars.pop();
                } else {
                    break;
                }
            }

            // Capture optional sign character immediately preceding the digits
            if idx < sym.len() {
                if let Some(&(pos, ch)) = chars.last() {
                    if ch == '+' || ch == '-' {
                        idx = pos;
                    }
                }
            }

            // If we consumed at least one digit, interpret suffix as exponent (allowing optional sign)
            if idx < sym.len() && !sym.contains('[') {
                let exp_str = &sym[idx..];
                if let Ok(exp) = exp_str.parse::<i32>() {
                    let base = &sym[..idx];
                    return Ok((
                        rest,
                        UnitFactor {
                            expr: UnitExpr::Symbol(base.to_string()),
                            exponent: exp,
                        },
                    ));
                }
            }
            (base_expr.clone(), 1)
        }
        _ => (base_expr.clone(), 1),
    };

    Ok((rest, UnitFactor { expr, exponent }))
}

fn factors_to_expr(factors: Vec<UnitFactor>) -> UnitExpr {
    if factors.len() == 1 {
        let f = &factors[0];
        if f.exponent == 1 {
            return f.expr.clone();
        } else {
            return UnitExpr::Power(Box::new(f.expr.clone()), f.exponent);
        }
    }
    UnitExpr::Product(factors)
}

fn product_separator(input: &str) -> IResult<&str, ()> {
    map(
        alt((map(char('.'), |_| ()), map(multispace1, |_| ()))),
        |_| (),
    )(input)
}

pub fn parse_product(input: &str) -> IResult<&str, UnitExpr> {
    let (mut rest, first) = parse_factor(input)?;
    let mut factors = vec![first];

    loop {
        // Consume optional explicit separator (dot or whitespace)
        let (r, _) = opt(product_separator)(rest)?;
        rest = r;
        match parse_factor(rest) {
            Ok((next, fac)) => {
                if next.len() == rest.len() {
                    break; // ensure progress
                }
                factors.push(fac);
                rest = next;
            }
            Err(_) => break,
        }
    }

    Ok((rest, factors_to_expr(factors)))
}

pub fn parse_quotient(input: &str) -> IResult<&str, UnitExpr> {
    map(
        tuple((
            parse_product,
            opt(preceded(
                tuple((
                    nom::character::complete::multispace0,
                    char('/'),
                    nom::character::complete::multispace0,
                )),
                parse_product,
            )),
        )),
        |(num, maybe_den)| match maybe_den {
            Some(den) => UnitExpr::Quotient(Box::new(num), Box::new(den)),
            None => num,
        },
    )(input)
}
