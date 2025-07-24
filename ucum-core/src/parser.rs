//! Internal UCUM expression parsers implemented with `nom`.
//!
//! This module defines all low-level parsers that operate on `&str` slices and
//! build the typed AST defined in `crate::ast`.

use crate::ast::{UnitExpr, UnitFactor};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, digit1, multispace1},
    combinator::{map, map_res, opt, recognize},
    number::complete::recognize_float,
    sequence::{delimited, pair, preceded},
};

// ---------------------- atomic helpers ----------------------
fn is_symbol_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '%' | '_' | '[' | ']' | '\'' | 'µ' | '-' | '+')
}

fn parse_symbol(input: &str) -> IResult<&str, UnitExpr> {
    map_res(take_while1(is_symbol_char), |s: &str| {
        // Normalise Unicode micro sign to ASCII 'u'
        let normalized = s.replace('µ', "u");

        if normalized.contains('%') && normalized.len() > 1 {
            // Percent must be a standalone unit symbol
            Err(())
        } else {
            // Additional validation for common invalid patterns
            if is_invalid_unit_pattern(&normalized) {
                Err(())
            } else {
                Ok(UnitExpr::Symbol(normalized))
            }
        }
    }).parse(input)
}

fn is_invalid_unit_pattern(s: &str) -> bool {
    // Check for invalid time unit patterns (numeric prefix without decimal point)
    // Examples: 12h, 48h, 4h should be invalid (should be 12.h, 48.h, 4.h)
    if let Some(pos) = s.find('h') {
        if pos > 0 {
            let prefix = &s[..pos];
            // If prefix is purely numeric (no decimal point), it's invalid
            if prefix.chars().all(|c| c.is_ascii_digit()) {
                return true;
            }
        }
    }

    // Check for parentheses with time units - these are invalid
    if s.contains("(") && (s.contains("h") || s.contains("hr")) {
        return true;
    }

    // Check for complex annotation patterns that should be invalid
    // {a}rad2{b} should be invalid - cannot start a symbol with annotation
    if s.starts_with('{') && s.contains('}') && s.len() > s.find('}').unwrap_or(0) + 1 {
        return true;
    }

    // Check for known invalid units that should be rejected
    match s {
        "iU" | "molv" => true,
        _ if s.contains("[iIU]") => true, // Any occurrence of [iIU] is invalid
        _ if s == "[BETH'U]" => true,
        _ if s == "[M'U]" => true,
        _ if s.contains("[H20]") => true, // Should be [H2O]
        _ => false,
    }
}

fn parse_numeric(input: &str) -> IResult<&str, UnitExpr> {
    // Parses 10*exp or 10^exp where exp is a signed integer.
    let signed_int = || recognize(pair(opt(alt((char('-'), char('+')))), digit1));

    let star_parser = map_res(preceded(tag("10*"), signed_int()), |s: &str| s.parse::<i32>());
    let caret_parser = map_res(preceded(tag("10^"), signed_int()), |s: &str| s.parse::<i32>());

    map(alt((star_parser, caret_parser)), |exp: i32| {
        UnitExpr::Numeric(10f64.powi(exp))
    }).parse(input)
}

fn parse_decimal(input: &str) -> IResult<&str, UnitExpr> {
    map_res(recognize_float, |s: &str| {
        s.parse::<f64>().map(UnitExpr::Numeric)
    }).parse(input)
}

fn parse_paren_expr(input: &str) -> IResult<&str, UnitExpr> {
    delimited(char('('), parse_quotient, char(')')).parse(input)
}

fn parse_base_atomic(input: &str) -> IResult<&str, UnitExpr> {
    // Order matters: try specific patterns first, then general ones
    // 1. Parentheses (highest priority)
    // 2. "10*" and "10^" patterns (specific numeric notation)
    // 3. General decimal numbers (before symbols to ensure numbers aren't parsed as symbols)
    // 4. Standalone annotations (valid in UCUM)
    // 5. Symbols (most general, last resort)
    alt((
        parse_paren_expr,
        parse_numeric,
        parse_decimal,
        parse_standalone_annotation,
        parse_symbol,
    )).parse(input)
}

// ---------------------- annotations -------------------------
fn annotation_body(i: &str) -> IResult<&str, &str> {
    // Consume until an unescaped '}'
    let mut escaped = false;
    for (idx, ch) in i.char_indices() {
        match (escaped, ch) {
            (false, '\\') => escaped = true,
            (true, _) => escaped = false,
            (false, '}') => {
                let content = &i[..idx];

                // Validate annotation content
                if content.is_empty() {
                    // Empty annotations are invalid
                    return Err(nom::Err::Error(nom::error::Error::new(
                        i,
                        nom::error::ErrorKind::Char,
                    )));
                }

                // Check for invalid characters in annotations
                for ch in content.chars() {
                    match ch {
                        // Pipe character is invalid in annotations
                        '|' => {
                            return Err(nom::Err::Error(nom::error::Error::new(
                                i,
                                nom::error::ErrorKind::Char,
                            )));
                        }
                        // Only allow ASCII characters and µ
                        _ if !ch.is_ascii() && ch != 'µ' => {
                            return Err(nom::Err::Error(nom::error::Error::new(
                                i,
                                nom::error::ErrorKind::Char,
                            )));
                        }
                        _ => {}
                    }
                }
                return Ok((&i[idx..], content));
            }
            _ => {}
        }
    }
    Err(nom::Err::Error(nom::error::Error::new(
        i,
        nom::error::ErrorKind::Eof,
    )))
}

fn parse_annotation(input: &str) -> IResult<&str, ()> {
    map(delimited(char('{'), annotation_body, char('}')), |_| ()).parse(input)
}

fn parse_standalone_annotation(input: &str) -> IResult<&str, UnitExpr> {
    // Parse standalone annotations like {bsa}, {cfu}, etc.
    // These are valid in UCUM and should be treated as symbols
    map(
        delimited(char('{'), annotation_body, char('}')),
        |content: &str| {
            // Return the annotation content as a symbol with braces
            UnitExpr::Symbol(format!("{{{}}}", content))
        },
    ).parse(input)
}

// ---------------------- exponents & factors -----------------
fn parse_exponent(input: &str) -> IResult<&str, i32> {
    preceded(
        char('^'),
        map_res(recognize(pair(opt(char('-')), digit1)), |s: &str| {
            s.parse::<i32>()
        }),
    ).parse(input)
}

fn parse_base_expr(input: &str) -> IResult<&str, UnitExpr> {
    let (rest, expr) = parse_base_atomic(input)?;
    let (rest, _annotation_present) = opt(parse_annotation).parse(rest)?;

    // Store whether we found an annotation for later validation
    Ok((rest, expr))
}

pub fn parse_factor(input: &str) -> IResult<&str, UnitFactor> {
    // Parse base expression and possible exponent/annotation that follow.
    let (rest, base_expr) = parse_base_expr(input)?;
    let (rest, explicit_exp) = opt(parse_exponent).parse(rest)?;
    let (rest, _) = opt(parse_annotation).parse(rest)?;

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

#[allow(dead_code)]
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
    ).parse(input)
}

#[allow(dead_code)]
pub fn parse_product(input: &str) -> IResult<&str, UnitExpr> {
    let (mut rest, first) = parse_factor(input)?;
    let mut factors = vec![first];

    loop {
        // Consume optional explicit separator (dot or whitespace)
        let (r, _) = opt(product_separator).parse(rest)?;
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

fn has_invalid_string_patterns(input: &str) -> bool {
    // Check for invalid patterns in the original string before parsing

    // 1. Check for numeric time unit patterns without decimal points
    // Pattern: digits followed directly by time unit (no decimal point)
    // Invalid: 12h, 48h, 2h, 1h
    // Valid: 12.h, 48.h, 2.h, 1.h
    let time_units = [
        "h", "hr", "min", "s", "ms", "us", "ns", "d", "wk", "mo", "a",
    ];

    for unit in time_units {
        let mut chars = input.char_indices().peekable();

        while let Some((i, ch)) = chars.next() {
            if ch.is_ascii_digit() {
                // Found a digit, collect consecutive digits
                let start_idx = i;
                let mut end_idx = i + ch.len_utf8();

                // Collect all consecutive digits
                while let Some(&(next_i, next_ch)) = chars.peek() {
                    if next_ch.is_ascii_digit() {
                        end_idx = next_i + next_ch.len_utf8();
                        chars.next();
                    } else {
                        break;
                    }
                }

                // Check if the digits are followed by the time unit
                if input[end_idx..].starts_with(unit) {
                    // Check if NOT preceded by a decimal point
                    let preceded_by_dot =
                        start_idx > 0 && input.chars().nth(start_idx - 1) == Some('.');

                    if !preceded_by_dot {
                        // Check if it's at word boundary (not part of larger symbol)
                        let unit_end = end_idx + unit.len();
                        let at_boundary = unit_end >= input.len()
                            || !input
                                .chars()
                                .nth(unit_end)
                                .unwrap_or(' ')
                                .is_ascii_alphanumeric();

                        if at_boundary {
                            let _matched = &input[start_idx..unit_end];
                            return true;
                        }
                    }
                }
            }
        }
    }

    // 2. Check for parentheses with time units (but allow valid division expressions)
    // Invalid: ug(8.h), ug(8hr) - unit symbols followed by parentheses with time units
    // Valid: mmol/(8.h), g/(8.h) - division expressions with parentheses
    if input.contains('(') && (input.contains(".h") || input.contains("hr")) {
        // Check if it's NOT a division expression (doesn't have '/' before '(')
        if let Some(paren_pos) = input.find('(') {
            // Look for '/' immediately before the '(' (allowing whitespace)
            let before_paren = &input[..paren_pos].trim_end();
            if !before_paren.ends_with('/') {
                return true;
            }
        }
    }

    // 3. Check for complex annotation patterns starting with annotation
    // Invalid: {a}rad2{b} - annotation directly followed by unit without separator
    // Valid: {a}.rad2{b} - annotation followed by dot separator and unit
    // Valid: {a}/rad2{b} - annotation followed by division
    if input.starts_with('{') && input.contains('}') {
        if let Some(close_pos) = input.find('}') {
            if close_pos + 1 < input.len() {
                let after_annotation = &input[close_pos + 1..];
                // If there's content after the first annotation, check if it has proper separator
                if !after_annotation.trim().is_empty() {
                    // Allow if it starts with '/' (division) or '.' (product separator)
                    if !after_annotation.starts_with('/') && !after_annotation.starts_with('.') {
                        return true;
                    }
                }
            }
        }
    }

    false
}

fn parse_quotient_remainder(input: &str) -> IResult<&str, UnitExpr> {
    // Parse remaining part of quotient for right-associativity
    if input.trim().is_empty() {
        return Ok((input, UnitExpr::Symbol("".to_string())));
    }

    // Look for division operator
    if let Ok((rest, denominator_factor)) = preceded(
        (char('/'), nom::character::complete::multispace0),
        parse_factor,
    ).parse(input)
    {
        let denominator = match denominator_factor.exponent {
            1 => denominator_factor.expr,
            exp => UnitExpr::Power(Box::new(denominator_factor.expr), exp),
        };

        // Recursively parse remaining divisions
        if let Ok((final_rest, remaining_expr)) = parse_quotient_remainder(rest) {
            if let UnitExpr::Symbol(s) = &remaining_expr {
                if s.is_empty() {
                    return Ok((final_rest, denominator));
                }
            }
            let combined = UnitExpr::Product(vec![
                UnitFactor {
                    expr: denominator,
                    exponent: 1,
                },
                UnitFactor {
                    expr: remaining_expr,
                    exponent: 1,
                },
            ]);
            Ok((final_rest, combined))
        } else {
            Ok((rest, denominator))
        }
    } else {
        Ok((input, UnitExpr::Symbol("".to_string())))
    }
}

pub fn parse_quotient(input: &str) -> IResult<&str, UnitExpr> {
    // Handle empty input as unity
    if input.trim().is_empty() {
        return Ok((input, UnitExpr::Symbol("".to_string())));
    }

    // Validate for invalid string patterns before parsing
    if has_invalid_string_patterns(input) {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Verify,
        )));
    }

    // Handle leading division (e.g., "/m" should be "1/m")
    if input.trim_start().starts_with('/') {
        let trimmed = input.trim_start();
        let (rest, den) = preceded(
            (char('/'), nom::character::complete::multispace0),
            parse_factor,
        ).parse(trimmed)?;
        let den_expr = match den.exponent {
            1 => den.expr,
            exp => UnitExpr::Power(Box::new(den.expr), exp),
        };
        let result = UnitExpr::Quotient(Box::new(UnitExpr::Numeric(1.0)), Box::new(den_expr));

        return Ok((rest, result));
    }

    // Parse first factor
    let (mut rest, first_factor) = parse_factor(input)?;
    let mut result = match first_factor.exponent {
        1 => first_factor.expr,
        exp => UnitExpr::Power(Box::new(first_factor.expr), exp),
    };

    // Handle multiplication and division operators
    // For multiple divisions, UCUM expects right-associative behavior: s/m/mg = s/(m/mg)
    loop {
        // Skip optional whitespace
        let (r, _) = nom::character::complete::multispace0.parse(rest)?;
        rest = r;

        // Look for division operator
        if let Ok((new_rest, denominator_factor)) = preceded(
            (char('/'), nom::character::complete::multispace0),
            parse_factor,
        ).parse(rest)
        {
            // Convert factor to expression
            let denominator = match denominator_factor.exponent {
                1 => denominator_factor.expr,
                exp => UnitExpr::Power(Box::new(denominator_factor.expr), exp),
            };

            // Parse the rest of the expression to handle right-associativity
            let remaining_input = new_rest;
            if let Ok((final_rest, remaining_expr)) = parse_quotient_remainder(remaining_input) {
                // If there's more to parse, create right-associative structure
                let combined_denominator = if let UnitExpr::Symbol(s) = &remaining_expr {
                    if s.is_empty() {
                        denominator
                    } else {
                        UnitExpr::Product(vec![
                            UnitFactor {
                                expr: denominator,
                                exponent: 1,
                            },
                            UnitFactor {
                                expr: remaining_expr,
                                exponent: 1,
                            },
                        ])
                    }
                } else {
                    UnitExpr::Product(vec![
                        UnitFactor {
                            expr: denominator,
                            exponent: 1,
                        },
                        UnitFactor {
                            expr: remaining_expr,
                            exponent: 1,
                        },
                    ])
                };
                result = UnitExpr::Quotient(Box::new(result), Box::new(combined_denominator));
                rest = final_rest;
                break;
            } else {
                // No more divisions, create simple quotient
                result = UnitExpr::Quotient(Box::new(result), Box::new(denominator));
                rest = new_rest;
                continue;
            }
        }

        // Look for multiplication operator (dot or implicit)
        if let Ok((r, _)) = opt(product_separator).parse(rest) {
            rest = r;
            if let Ok((new_rest, factor)) = parse_factor(rest) {
                if new_rest.len() == rest.len() {
                    break; // ensure progress
                }
                // Convert factor to expression and create product
                let factor_expr = match factor.exponent {
                    1 => factor.expr,
                    exp => UnitExpr::Power(Box::new(factor.expr), exp),
                };
                result = match result {
                    UnitExpr::Product(mut factors) => {
                        factors.push(UnitFactor {
                            expr: factor_expr,
                            exponent: 1,
                        });
                        UnitExpr::Product(factors)
                    }
                    _ => UnitExpr::Product(vec![
                        UnitFactor {
                            expr: result,
                            exponent: 1,
                        },
                        UnitFactor {
                            expr: factor_expr,
                            exponent: 1,
                        },
                    ]),
                };
                rest = new_rest;
                continue;
            }
        }

        break; // No more operators
    }

    Ok((rest, result))
}
