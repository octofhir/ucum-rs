//! Display names of UCUM expressions, in the form of the official UCUM tests:
//! "(millimeter)", "(meter ^ 3) * (kilogram ^ -1)", "4 * (the number pi) / (ampère ^ 2)".

use crate::ast::UnitExpr;
use crate::evaluator::{Case, lookup_unit};
use crate::prelude::*;

/// The display name of a parsed expression: the names of its simple units from the UCUM
/// tables, with their prefixes, joined by " * " and " / ". A symbol that is not a unit is
/// shown as written.
///
/// ```
/// use octofhir_ucum::{generate_display_name, parse_expression};
///
/// let expr = parse_expression("m3.kg-1.s-2").unwrap();
/// assert_eq!(
///     generate_display_name(&expr),
///     "(meter ^ 3) * (kilogram ^ -1) * (second ^ -2)"
/// );
/// ```
pub fn generate_display_name(expr: &UnitExpr) -> String {
    match expr {
        UnitExpr::Symbol(symbol) => format!("({})", unit_name(symbol)),
        UnitExpr::Numeric(n) => format!("{n}"),
        UnitExpr::Power(base, exp) => match base.as_ref() {
            UnitExpr::Symbol(symbol) => format!("({} ^ {exp})", unit_name(symbol)),
            other => format!("({} ^ {exp})", generate_display_name(other)),
        },
        UnitExpr::Annotation(text) => format!("{{{text}}}"),
        UnitExpr::Annotated(inner, text) => {
            format!("{}{{{text}}}", generate_display_name(inner))
        }
        UnitExpr::Product(terms) => terms
            .iter()
            .enumerate()
            .map(|(i, term)| {
                if i > 0 && is_compound(term) {
                    format!("({})", generate_display_name(term))
                } else {
                    generate_display_name(term)
                }
            })
            .collect::<Vec<_>>()
            .join(" * "),
        UnitExpr::Quotient(numerator, denominator) => {
            let denominator_name = if is_compound(denominator) {
                format!("({})", generate_display_name(denominator))
            } else {
                generate_display_name(denominator)
            };
            format!("{} / {denominator_name}", generate_display_name(numerator))
        }
        UnitExpr::Reciprocal(inner) => {
            let name = generate_display_name(inner);
            if is_compound(inner) {
                format!("1 / ({name})")
            } else {
                format!("1 / {name}")
            }
        }
    }
}

fn is_compound(expr: &UnitExpr) -> bool {
    matches!(
        expr,
        UnitExpr::Product(_) | UnitExpr::Quotient(..) | UnitExpr::Reciprocal(_)
    )
}

/// The name of a simple unit with its prefix ("millimeter"), or the symbol itself.
fn unit_name(symbol: &str) -> String {
    match lookup_unit(symbol, Case::Sensitive) {
        Some((prefix, unit)) => {
            format!(
                "{}{}",
                prefix.map_or("", |p| p.display_name),
                unit.display_name
            )
        }
        None => symbol.to_string(),
    }
}
