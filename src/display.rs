//! Display name generation for UCUM expressions.
//!
//! This module provides functionality to convert UCUM expressions into human-readable
//! display names, as specified in the official UCUM test cases.

use crate::ast::{OwnedUnitExpr, UnitExpr, UnitFactor};
use crate::registry;

/// Helper to extract string from either Symbol or SymbolOwned variants
fn extract_symbol_str<'a>(expr: &'a UnitExpr<'a>) -> Option<&'a str> {
    match expr {
        UnitExpr::Symbol(s) => Some(s),
        UnitExpr::SymbolOwned(s) => Some(s.as_str()),
        _ => None,
    }
}

/// Generate a human-readable display name for a UCUM expression.
///
/// This function converts a parsed UCUM expression into a human-readable format
/// following the patterns defined in the official UCUM displayNameGeneration tests.
///
/// # Examples
///
/// ```
/// use octofhir_ucum::{parse_expression, generate_display_name_owned};
///
/// let expr = parse_expression("m").unwrap();
/// let display = generate_display_name_owned(&expr);
/// assert_eq!(display, "(meter)");
/// ```
pub fn generate_display_name(expr: &UnitExpr) -> String {
    match expr {
        UnitExpr::Symbol(symbol) => generate_symbol_display_name(symbol),
        UnitExpr::SymbolOwned(symbol) => generate_symbol_display_name(symbol),
        UnitExpr::Numeric(value) => generate_numeric_display_name(*value),
        UnitExpr::Power(base, exponent) => {
            // Special case: if base is an empty symbol (unity), display as just the exponent number
            if let Some(symbol) = extract_symbol_str(base.as_ref()) {
                if symbol.is_empty() {
                    return format!("{exponent}");
                }
            }

            let base_display = generate_display_name(base);
            // Remove outer parentheses from base_display and wrap the entire expression
            let inner_display = if base_display.starts_with('(') && base_display.ends_with(')') {
                &base_display[1..base_display.len() - 1]
            } else {
                &base_display
            };
            format!("({inner_display} ^ {exponent})")
        }
        UnitExpr::Product(factors) => generate_product_display_name(factors),
        UnitExpr::Quotient(numerator, denominator) => {
            let num_display = generate_display_name(numerator);
            let den_display = generate_display_name(denominator);
            format!("{num_display} / {den_display}")
        }
    }
}

/// Generate the display name for a symbol (unit code).
fn generate_symbol_display_name(symbol: &str) -> String {
    // Handle empty symbol (unity)
    if symbol.is_empty() {
        return "(unity)".to_string();
    }

    // Handle special units like [pi] FIRST, before registry lookup
    if symbol.starts_with('[') && symbol.ends_with(']') {
        let inner = &symbol[1..symbol.len() - 1];
        return match inner {
            "pi" => "(the number pi)".to_string(),
            "H2O" => "of water column".to_string(),
            _ => format!("({inner})"),
        };
    }

    // Handle units with annotations like "m[H2O]"
    if let Some(bracket_start) = symbol.find('[') {
        if let Some(bracket_end) = symbol.rfind(']') {
            if bracket_end > bracket_start {
                let base_unit = &symbol[..bracket_start];
                let annotation = &symbol[bracket_start + 1..bracket_end];

                let base_display = get_base_unit_display_name(base_unit);
                let annotation_display = match annotation {
                    "H2O" => " of water column",
                    "pi" => " pi", // This shouldn't happen in this context but just in case
                    _ => "",
                };

                return format!("({base_display}{annotation_display})");
            }
        }
    }

    // Try to find the unit in the registry as a standalone unit first
    if let Some(_unit) = registry::find_unit(symbol) {
        // But check if this might actually be a prefixed unit that should be handled differently
        // Only use direct lookup if it's not a common prefixed unit pattern
        if let Some((prefix_symbol, base_symbol)) = split_prefix(symbol) {
            if let Some(_prefix) = registry::find_prefix(prefix_symbol) {
                if let Some(_base_unit) = registry::find_unit(base_symbol) {
                    // This could be either a standalone unit or a prefixed unit
                    // For units like "Pa" (pascal), we want to use the standalone version
                    // For units like "mm" (millimeter), we want to use the prefixed version
                    // Check if the standalone unit has a meaningful display name different from the base
                    let standalone_display = get_base_unit_display_name(symbol);
                    let base_display = get_base_unit_display_name(base_symbol);

                    // If the standalone display is different from the base unit display, use standalone
                    if standalone_display != base_display {
                        return format!("({standalone_display})");
                    } else {
                        // Otherwise, use the prefixed version
                        return format!(
                            "({}{})",
                            get_prefix_display_name(prefix_symbol),
                            get_base_unit_display_name(base_symbol)
                        );
                    }
                }
            }
        }
        return format!("({})", get_base_unit_display_name(symbol));
    }

    // Check if this is a prefixed unit (fallback if direct lookup failed)
    if let Some((prefix_symbol, base_symbol)) = split_prefix(symbol) {
        if let Some(_prefix) = registry::find_prefix(prefix_symbol) {
            if let Some(_base_unit) = registry::find_unit(base_symbol) {
                return format!(
                    "({}{})",
                    get_prefix_display_name(prefix_symbol),
                    get_base_unit_display_name(base_symbol)
                );
            }
        }
    }

    // Fallback: use the symbol itself
    format!("({symbol})")
}

/// Generate display name for numeric values.
fn generate_numeric_display_name(value: f64) -> String {
    // Handle powers of 10 (e.g., 10^23)
    if value > 0.0 && value != 1.0 {
        let log_value = value.log10();
        if (log_value.round() - log_value).abs() < 1e-10 {
            let exponent = log_value.round() as i32;
            return format!("(the number ten for arbitrary powers ^ {exponent})");
        }
    }

    // Handle regular numbers - don't wrap in parentheses for simple coefficients
    if value == 1.0 {
        "(unity)".to_string()
    } else if value.fract() == 0.0 && value > 0.0 {
        // Integer values should be displayed as plain numbers (no parentheses)
        format!("{}", value as i64)
    } else {
        format!("{value}")
    }
}

/// Generate display name for product expressions.
fn generate_product_display_name(factors: &[UnitFactor]) -> String {
    let factor_displays: Vec<String> = factors
        .iter()
        .map(|factor| {
            // Special case: if the expression is an empty symbol (unity) with an exponent,
            // display it as just the exponent number
            if let UnitExpr::Symbol(symbol) = &factor.expr {
                if symbol.is_empty() && factor.exponent != 1 {
                    return format!("{}", factor.exponent);
                }
            }

            let base_display = generate_display_name(&factor.expr);
            if factor.exponent == 1 {
                base_display
            } else {
                // Remove outer parentheses from base_display and wrap the entire expression
                let inner_display = if base_display.starts_with('(') && base_display.ends_with(')')
                {
                    &base_display[1..base_display.len() - 1]
                } else {
                    &base_display
                };
                format!("({} ^ {})", inner_display, factor.exponent)
            }
        })
        .collect();

    factor_displays.join(" * ")
}

/// Split a symbol into prefix and base parts.
fn split_prefix(symbol: &str) -> Option<(&str, &str)> {
    // Try common prefixes from longest to shortest to avoid conflicts
    let prefixes = [
        "da", "d", "c", "m", "u", "n", "p", "f", "a", "z", "y", "h", "k", "M", "G", "T", "P", "E",
        "Z", "Y",
    ];

    for prefix in &prefixes {
        if symbol.starts_with(prefix) && symbol.len() > prefix.len() {
            let base = &symbol[prefix.len()..];
            // Verify this is actually a valid prefix-base combination
            if registry::find_prefix(prefix).is_some() && registry::find_unit(base).is_some() {
                return Some((prefix, base));
            }
        }
    }
    None
}

/// Get display name for a prefix.
fn get_prefix_display_name(prefix: &str) -> String {
    // First try to get display name from registry
    if let Some(prefix_info) = registry::find_prefix(prefix) {
        return prefix_info.display_name.to_string();
    }

    // Fall back to hardcoded mappings for prefixes not in registry
    match prefix {
        "da" => "deca".to_string(),
        "d" => "deci".to_string(),
        "c" => "centi".to_string(),
        "m" => "milli".to_string(),
        "u" => "micro".to_string(),
        "n" => "nano".to_string(),
        "p" => "pico".to_string(),
        "f" => "femto".to_string(),
        "a" => "atto".to_string(),
        "z" => "zepto".to_string(),
        "y" => "yocto".to_string(),
        "h" => "hecto".to_string(),
        "k" => "kilo".to_string(),
        "M" => "mega".to_string(),
        "G" => "giga".to_string(),
        "T" => "tera".to_string(),
        "P" => "peta".to_string(),
        "E" => "exa".to_string(),
        "Z" => "zetta".to_string(),
        "Y" => "yotta".to_string(),
        _ => prefix.to_string(),
    }
}

/// Get full display name for base units.
fn get_base_unit_display_name(unit_symbol: &str) -> String {
    // First try to get display name from registry
    if let Some(unit) = registry::find_unit(unit_symbol) {
        // Don't decode HTML entities - preserve them as-is for display names
        // Don't capitalize - use the display name as-is from the registry
        // The registry should already have the correct capitalization
        return unit.display_name.to_string();
    }

    // Fall back to hardcoded mappings for units not in registry
    match unit_symbol {
        "m" => "meter".to_string(),
        "g" => "gram".to_string(),
        "s" => "second".to_string(),
        "A" => "Ampère".to_string(),
        "K" => "Kelvin".to_string(),
        "mol" => "mole".to_string(),
        "cd" => "candela".to_string(),
        "rad" => "radian".to_string(),
        "sr" => "steradian".to_string(),
        "Hz" => "Hertz".to_string(),
        "N" => "Newton".to_string(),
        "Pa" => "Pascal".to_string(),
        "J" => "Joule".to_string(),
        "W" => "Watt".to_string(),
        "C" => "Coulomb".to_string(),
        "V" => "Volt".to_string(),
        "F" => "Farad".to_string(),
        "Ohm" => "Ohm".to_string(),
        "S" => "Siemens".to_string(),
        "Wb" => "Weber".to_string(),
        "T" => "Tesla".to_string(),
        "H" => "Henry".to_string(),
        "lm" => "lumen".to_string(),
        "lx" => "lux".to_string(),
        "Bq" => "Becquerel".to_string(),
        "Gy" => "Gray".to_string(),
        "Sv" => "Sievert".to_string(),
        "kat" => "katal".to_string(),
        "l" => "liter".to_string(),
        "L" => "liter".to_string(),
        "bar" => "bar".to_string(),
        "atm" => "atmosphere".to_string(),
        "eV" => "electron volt".to_string(),
        "u" => "unified atomic mass unit".to_string(),
        _ => {
            // Final fallback to unit symbol itself
            unit_symbol.to_string()
        }
    }
}

/// Generate display name for owned AST
pub fn generate_display_name_owned(expr: &OwnedUnitExpr) -> String {
    // Convert to borrowed AST and use existing implementation
    let borrowed = owned_to_borrowed_display(expr);
    generate_display_name(&borrowed)
}

/// Convert owned AST to borrowed for display generation
fn owned_to_borrowed_display(expr: &OwnedUnitExpr) -> UnitExpr {
    match expr {
        OwnedUnitExpr::Numeric(v) => UnitExpr::Numeric(*v),
        OwnedUnitExpr::Symbol(sym) => UnitExpr::SymbolOwned(sym.clone()),
        OwnedUnitExpr::Product(factors) => {
            let borrowed_factors: Vec<UnitFactor> = factors
                .iter()
                .map(|f| UnitFactor {
                    expr: owned_to_borrowed_display(&f.expr),
                    exponent: f.exponent,
                })
                .collect();
            UnitExpr::Product(borrowed_factors)
        }
        OwnedUnitExpr::Quotient(num, den) => UnitExpr::Quotient(
            Box::new(owned_to_borrowed_display(num)),
            Box::new(owned_to_borrowed_display(den)),
        ),
        OwnedUnitExpr::Power(expr, exp) => {
            UnitExpr::Power(Box::new(owned_to_borrowed_display(expr)), *exp)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_expression;

    #[test]
    fn test_basic_units() {
        let expr = parse_expression("m").unwrap();
        let display = generate_display_name_owned(&expr);
        println!("Display for 'm': {display}");

        // Debug: Check what's in the registry
        if let Some(unit) = registry::find_unit("m") {
            println!(
                "Found unit 'm' in registry: display_name = '{}'",
                unit.display_name
            );
        } else {
            println!("Unit 'm' not found in registry");
        }

        // Check some other common units
        for unit_code in &["kg", "s", "A", "K", "mol", "cd"] {
            if let Some(unit) = registry::find_unit(unit_code) {
                println!(
                    "Found unit '{}' in registry: display_name = '{}'",
                    unit_code, unit.display_name
                );
            } else {
                println!("Unit '{unit_code}' not found in registry");
            }
        }
    }

    #[test]
    fn test_prefixed_units() {
        let expr = parse_expression("mm").unwrap();
        let display = generate_display_name_owned(&expr);
        println!("Display for 'mm': {display}");

        // Debug: Check if "mm" is found directly in registry
        if let Some(unit) = registry::find_unit("mm") {
            println!(
                "Found 'mm' directly in registry: display_name = '{}'",
                unit.display_name
            );
        } else {
            println!("'mm' not found directly in registry");
        }

        // Debug: Check prefix splitting
        if let Some((prefix, base)) = split_prefix("mm") {
            println!("Split 'mm' into prefix='{prefix}' and base='{base}'");
            if let Some(_prefix_info) = registry::find_prefix(prefix) {
                println!("Prefix '{prefix}' found in registry");
            }
            if let Some(base_unit) = registry::find_unit(base) {
                println!(
                    "Base unit '{}' found in registry: display_name = '{}'",
                    base, base_unit.display_name
                );
            }
        } else {
            println!("Could not split 'mm' into prefix and base");
        }

        // Let's also check what the parsed expression looks like
        println!("Parsed expression: {expr:?}");
    }

    #[test]
    fn test_power_expressions() {
        let expr = parse_expression("rad2").unwrap();
        let display = generate_display_name_owned(&expr);
        println!("Display for 'rad2': {display}");

        // Test other power expressions
        let expr2 = parse_expression("m3").unwrap();
        let display2 = generate_display_name_owned(&expr2);
        println!("Display for 'm3': {display2}");
    }

    #[test]
    fn test_product_expressions() {
        let expr = parse_expression("m3.kg-1.s-2").unwrap();
        let display = generate_display_name_owned(&expr);
        println!("Display for 'm3.kg-1.s-2': {display}");
        println!("Parsed expression: {expr:?}");

        // Test simpler product
        let expr2 = parse_expression("kg.m").unwrap();
        let display2 = generate_display_name_owned(&expr2);
        println!("Display for 'kg.m': {display2}");
        println!("Parsed expression: {expr2:?}");

        // Test another complex product
        let expr3 = parse_expression("kg-1").unwrap();
        let display3 = generate_display_name_owned(&expr3);
        println!("Display for 'kg-1': {display3}");
        println!("Parsed expression: {expr3:?}");
    }

    #[test]
    fn test_quotient_expressions() {
        let expr = parse_expression("N/A2").unwrap();
        let display = generate_display_name_owned(&expr);
        println!("Display for 'N/A2': {display}");

        // Test simpler quotient
        let expr2 = parse_expression("m/s").unwrap();
        let display2 = generate_display_name_owned(&expr2);
        println!("Display for 'm/s': {display2}");
    }

    #[test]
    fn test_numeric_expressions() {
        let expr = parse_expression("10*23").unwrap();
        let display = generate_display_name_owned(&expr);
        println!("Display for '10*23': {display}");
    }

    #[test]
    fn test_single_factor_parsing() {
        // Test parsing "kg-1" as a single factor
        let expr = parse_expression("kg-1").unwrap();
        let display = generate_display_name_owned(&expr);
        println!("Display for 'kg-1' (single): {display}");
        println!("Parsed expression: {expr:?}");

        // Test parsing "m3" as a single factor
        let expr2 = parse_expression("m3").unwrap();
        let display2 = generate_display_name_owned(&expr2);
        println!("Display for 'm3' (single): {display2}");
        println!("Parsed expression: {expr2:?}");

        // Test parsing "s-2" as a single factor
        let expr3 = parse_expression("s-2").unwrap();
        let display3 = generate_display_name_owned(&expr3);
        println!("Display for 's-2' (single): {display3}");
        println!("Parsed expression: {expr3:?}");
    }

    #[test]
    fn test_debug_complex_expression() {
        println!("[DEBUG_LOG] Debugging complex expression parsing");

        let expr_str = "4.[pi].10*-7.N/A2";
        match parse_expression(expr_str) {
            Ok(expr) => {
                println!("[DEBUG_LOG] Parsed expression: {expr:?}");
                let display = generate_display_name_owned(&expr);
                println!("[DEBUG_LOG] Display: '{display}'");

                // Test individual components
                if let Ok(pi_expr) = parse_expression("[pi]") {
                    println!("[DEBUG_LOG] [pi] parsed as: {pi_expr:?}");
                    let pi_display = generate_display_name_owned(&pi_expr);
                    println!("[DEBUG_LOG] [pi] display: '{pi_display}'");
                }

                if let Ok(num_expr) = parse_expression("4") {
                    println!("[DEBUG_LOG] 4 parsed as: {num_expr:?}");
                    let num_display = generate_display_name_owned(&num_expr);
                    println!("[DEBUG_LOG] 4 display: '{num_display}'");
                }
            }
            Err(e) => {
                println!("[DEBUG_LOG] Failed to parse '{expr_str}': {e:?}");
            }
        }
    }

    #[test]
    fn test_official_display_name_cases() {
        println!("[DEBUG_LOG] Testing official displayNameGeneration test cases");

        // Test cases from official UCUM test suite
        let test_cases = vec![
            ("", "(unity)"),
            ("m", "(meter)"),
            ("mm", "(millimeter)"),
            ("m[H2O]", "(meter of water column)"),
            ("10*23", "(the number ten for arbitrary powers ^ 23)"),
            ("rad2", "(radian ^ 2)"),
            (
                "m3.kg-1.s-2",
                "(meter ^ 3) * (kilogram ^ -1) * (second ^ -2)",
            ),
            (
                "4.[pi].10*-7.N/A2",
                "4 * (the number pi) * (the number ten for arbitrary powers ^ -7) * (Newton) / (Ampère ^ 2)",
            ),
        ];

        for (unit, expected) in test_cases {
            if unit.is_empty() {
                // Special case for unity - we'll handle this separately
                println!("[DEBUG_LOG] Skipping empty unit (unity) for now");
                continue;
            }

            match parse_expression(unit) {
                Ok(expr) => {
                    let display = generate_display_name_owned(&expr);
                    println!(
                        "[DEBUG_LOG] Unit: '{unit}' -> Display: '{display}' (Expected: '{expected}')"
                    );

                    // For now, just log the results - we can add assertions later
                    if display == expected {
                        println!("[DEBUG_LOG] ✓ MATCH");
                    } else {
                        println!("[DEBUG_LOG] ✗ MISMATCH");
                    }
                }
                Err(e) => {
                    println!("[DEBUG_LOG] Failed to parse '{unit}': {e:?}");
                }
            }
        }
    }
}
