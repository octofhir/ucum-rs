use ucum_core::*;

#[test]
fn test_simple_unit_parsing() {
    let registry = UcumRegistry::new().unwrap();
    let parser = UcumGrammarParser::new(registry);
    
    // Test simple units
    assert!(parser.parse_main_term("m").is_ok());
    assert!(parser.parse_main_term("kg").is_ok());
    assert!(parser.parse_main_term("s").is_ok());
    assert!(parser.parse_main_term("cm").is_ok());
    assert!(parser.parse_main_term("mm").is_ok());
}

#[test]
fn test_division_parsing() {
    let registry = UcumRegistry::new().unwrap();
    let parser = UcumGrammarParser::new(registry);
    
    // Test division expressions
    assert!(parser.parse_main_term("m/s").is_ok());
    assert!(parser.parse_main_term("kg/m2").is_ok());
    assert!(parser.parse_main_term("/s").is_ok());  // Valid per grammar
    assert!(parser.parse_main_term("km/h").is_ok());
    assert!(parser.parse_main_term("g/L").is_ok());
}

#[test]
fn test_concatenation_parsing() {
    let registry = UcumRegistry::new().unwrap();
    let parser = UcumGrammarParser::new(registry);
    
    // Test concatenation expressions
    assert!(parser.parse_main_term("kg.m2").is_ok());
    assert!(parser.parse_main_term("m.s").is_ok());
    assert!(parser.parse_main_term("N.m").is_ok());
    assert!(parser.parse_main_term("Pa.s").is_ok());
}

#[test]
fn test_exponent_parsing() {
    let registry = UcumRegistry::new().unwrap();
    let parser = UcumGrammarParser::new(registry);
    
    // Test exponent expressions
    assert!(parser.parse_main_term("m2").is_ok());
    assert!(parser.parse_main_term("m-2").is_ok());
    assert!(parser.parse_main_term("m+2").is_ok());
    assert!(parser.parse_main_term("kg2").is_ok());
    assert!(parser.parse_main_term("s-1").is_ok());
}

#[test]
fn test_annotation_parsing() {
    let registry = UcumRegistry::new().unwrap();
    let parser = UcumGrammarParser::new(registry);
    
    // Test annotation expressions
    assert!(parser.parse_main_term("{count}/min").is_ok());
    assert!(parser.parse_main_term("m{count}").is_ok());
    assert!(parser.parse_main_term("{count}").is_ok());
    assert!(parser.parse_main_term("beats{count}/min").is_ok());
}

#[test]
fn test_square_brackets_parsing() {
    let registry = UcumRegistry::new().unwrap();
    let parser = UcumGrammarParser::new(registry);
    
    // Test square bracket expressions
    assert!(parser.parse_main_term("[iU]").is_ok());
    assert!(parser.parse_main_term("m[iU]").is_ok());
    assert!(parser.parse_main_term("[iU]m").is_ok());
    assert!(parser.parse_main_term("U[IU]").is_ok());
}

#[test]
fn test_complex_expressions() {
    let registry = UcumRegistry::new().unwrap();
    let parser = UcumGrammarParser::new(registry);
    
    // Test complex expressions
    assert!(parser.parse_main_term("kg.m2/s3").is_ok());
    assert!(parser.parse_main_term("(m/s)2").is_ok());
    assert!(parser.parse_main_term("{count}/min.m2").is_ok());
    assert!(parser.parse_main_term("N.m/s").is_ok());
    assert!(parser.parse_main_term("kg/(m.s2)").is_ok());
}

#[test]
fn test_terminal_symbol_validation() {
    // Test valid symbols
    assert!(TerminalUnitSymbol::is_valid('m'));
    assert!(TerminalUnitSymbol::is_valid('2'));
    assert!(TerminalUnitSymbol::is_valid('!'));
    assert!(TerminalUnitSymbol::is_valid('#'));
    assert!(TerminalUnitSymbol::is_valid('$'));
    assert!(TerminalUnitSymbol::is_valid('%'));
    assert!(TerminalUnitSymbol::is_valid('&'));
    assert!(TerminalUnitSymbol::is_valid('\''));
    assert!(TerminalUnitSymbol::is_valid('*'));
    assert!(TerminalUnitSymbol::is_valid(','));
    assert!(TerminalUnitSymbol::is_valid(':'));
    assert!(TerminalUnitSymbol::is_valid(';'));
    assert!(TerminalUnitSymbol::is_valid('<'));
    assert!(TerminalUnitSymbol::is_valid('>'));
    assert!(TerminalUnitSymbol::is_valid('?'));
    assert!(TerminalUnitSymbol::is_valid('@'));
    assert!(TerminalUnitSymbol::is_valid('\\'));
    assert!(TerminalUnitSymbol::is_valid('^'));
    assert!(TerminalUnitSymbol::is_valid('_'));
    assert!(TerminalUnitSymbol::is_valid('`'));
    assert!(TerminalUnitSymbol::is_valid('|'));
    assert!(TerminalUnitSymbol::is_valid('~'));
    
    // Test invalid symbols
    assert!(!TerminalUnitSymbol::is_valid(' '));
    assert!(!TerminalUnitSymbol::is_valid('('));
    assert!(!TerminalUnitSymbol::is_valid(')'));
    assert!(!TerminalUnitSymbol::is_valid('{'));
    assert!(!TerminalUnitSymbol::is_valid('}'));
    assert!(!TerminalUnitSymbol::is_valid('['));
    assert!(!TerminalUnitSymbol::is_valid(']'));
}

#[test]
fn test_terminal_symbol_creation() {
    // Test creating terminal symbols from characters
    assert!(TerminalUnitSymbol::from_char('m').is_ok());
    assert!(TerminalUnitSymbol::from_char('2').is_ok());
    assert!(TerminalUnitSymbol::from_char('!').is_ok());
    assert!(TerminalUnitSymbol::from_char('#').is_ok());
    
    // Test invalid characters
    assert!(TerminalUnitSymbol::from_char(' ').is_err());
    assert!(TerminalUnitSymbol::from_char('(').is_err());
    assert!(TerminalUnitSymbol::from_char(')').is_err());
}

#[test]
fn test_within_cb_symbol_validation() {
    let registry = UcumRegistry::new().unwrap();
    let parser = UcumGrammarParser::new(registry);
    
    // Valid within curly braces symbols
    assert!(parser.validate_within_cb_symbols("count").is_ok());
    assert!(parser.validate_within_cb_symbols("count ").is_ok());
    assert!(parser.validate_within_cb_symbols("count[test]").is_ok());
    assert!(parser.validate_within_cb_symbols("beats per minute").is_ok());
    
    // Invalid symbols
    assert!(parser.validate_within_cb_symbols("count}").is_err());
}

#[test]
fn test_within_sb_symbol_validation() {
    let registry = UcumRegistry::new().unwrap();
    let parser = UcumGrammarParser::new(registry);
    
    // Valid within square brackets symbols
    assert!(parser.validate_within_sb_symbols("iU").is_ok());
    assert!(parser.validate_within_sb_symbols("iU{test}").is_ok());
    assert!(parser.validate_within_sb_symbols("IU").is_ok());
    
    // Invalid symbols
    assert!(parser.validate_within_sb_symbols("iU]").is_err());
}

#[test]
fn test_parser_caching() {
    let registry = UcumRegistry::new().unwrap();
    let mut parser = UcumGrammarParser::new(registry);
    
    // First parse should cache the result
    let result1 = parser.parse_with_cache("m/s").unwrap();
    let result2 = parser.parse_with_cache("m/s").unwrap();
    
    // Results should be identical
    assert_eq!(result1, result2);
}

#[test]
fn test_error_handling() {
    let registry = UcumRegistry::new().unwrap();
    let parser = UcumGrammarParser::new(registry);
    
    // Test empty input
    assert!(parser.parse_main_term("").is_err());
    
    // Test invalid symbols
    assert!(parser.parse_main_term("m(").is_err());
    assert!(parser.parse_main_term("m)").is_err());
    
    // Test malformed expressions
    assert!(parser.parse_main_term("m/").is_err());
    assert!(parser.parse_main_term("/").is_err());
    assert!(parser.parse_main_term("m..s").is_err());
}

#[test]
fn test_grammar_structure_validation() {
    let registry = UcumRegistry::new().unwrap();
    let parser = UcumGrammarParser::new(registry);
    
    // Test that parsed structures match expected grammar
    let term = parser.parse_main_term("kg.m2/s3").unwrap();
    
    match term {
        UcumTerm::Division(left, right) => {
            // Left should be concatenation of kg.m2
            match *left {
                UcumTerm::Concatenation(kg, m2) => {
                    // Verify structure
                    assert!(matches!(*kg, UcumTerm::Component(_)));
                    assert!(matches!(*m2, UcumTerm::Component(_)));
                },
                _ => panic!("Expected concatenation"),
            }
            // Right should be s3
            assert!(matches!(*right, UcumTerm::Component(_)));
        },
        _ => panic!("Expected division"),
    }
}

#[test]
fn test_expression_evaluation() {
    let registry = UcumRegistry::new().unwrap();
    let parser = UcumGrammarParser::new(registry);
    
    // Test simple unit evaluation
    let term = parser.parse_main_term("kg").unwrap();
    let unit = parser.evaluate_term(&term).unwrap();
    assert_eq!(unit.code, "kg");
    assert_eq!(unit.name, "kilogram");
    
    // Test division evaluation
    let term = parser.parse_main_term("kg/m2").unwrap();
    let unit = parser.evaluate_term(&term).unwrap();
    assert_eq!(unit.code, "kg/m2");
    assert_eq!(unit.name, "kilogram per m2");
}

#[test]
fn test_unit_arithmetic() {
    let registry = UcumRegistry::new().unwrap();
    let parser = UcumGrammarParser::new(registry);
    
    // Test unit multiplication
    let kg_unit = parser.parse_main_term("kg").and_then(|t| parser.evaluate_term(&t)).unwrap();
    let m_unit = parser.parse_main_term("m").and_then(|t| parser.evaluate_term(&t)).unwrap();
    
    let result = parser.multiply_units(&kg_unit, &m_unit).unwrap();
    assert_eq!(result.code, "kg.m");
    assert_eq!(result.name, "kilogram times meter");
    
    // Test unit division
    let result = parser.divide_units(&kg_unit, &m_unit).unwrap();
    assert_eq!(result.code, "kg/m");
    assert_eq!(result.name, "kilogram per meter");
}

#[test]
fn test_exponent_handling() {
    let registry = UcumRegistry::new().unwrap();
    let parser = UcumGrammarParser::new(registry);
    
    // Test positive exponent
    let term = parser.parse_main_term("m2").unwrap();
    let unit = parser.evaluate_term(&term).unwrap();
    assert_eq!(unit.code, "m2");
    
    // Test negative exponent
    let term = parser.parse_main_term("m-2").unwrap();
    let unit = parser.evaluate_term(&term).unwrap();
    assert_eq!(unit.code, "m-2");
}

#[test]
fn test_annotation_handling() {
    let registry = UcumRegistry::new().unwrap();
    let parser = UcumGrammarParser::new(registry);
    
    // Test annotation parsing
    let term = parser.parse_main_term("{count}").unwrap();
    let unit = parser.evaluate_term(&term).unwrap();
    assert_eq!(unit.code, "{count}");
    assert_eq!(unit.name, "annotation unit count");
}

#[test]
fn test_square_bracket_handling() {
    let registry = UcumRegistry::new().unwrap();
    let parser = UcumGrammarParser::new(registry);
    
    // Test square bracket parsing
    let term = parser.parse_main_term("[iU]").unwrap();
    let unit = parser.evaluate_term(&term).unwrap();
    assert_eq!(unit.code, "[iU]");
    assert_eq!(unit.name, "arbitrary unit iU");
}

#[test]
fn test_fhirpath_quantity_operations() {
    use crate::fhirpath::FP_Quantity;
    
    let registry = UcumRegistry::new().unwrap();
    let parser = UcumGrammarParser::new(registry);
    
    // Test quantity creation and validation
    let quantity = FP_Quantity::new(100.0, "kg".to_string())
        .with_ucum_parser(parser.clone());
    
    assert!(quantity.validate_unit().is_ok());
    
    // Test quantity arithmetic
    let q1 = FP_Quantity::new(10.0, "kg".to_string())
        .with_ucum_parser(parser.clone());
    let q2 = FP_Quantity::new(5.0, "kg".to_string())
        .with_ucum_parser(parser.clone());
    
    let sum = q1.plus(&q2).unwrap();
    assert_eq!(sum.value, 15.0);
    assert_eq!(sum.unit, "kg");
    
    // Test quantity comparison
    let comparison = q1.compare(&q2);
    assert!(comparison.is_some());
    assert_eq!(comparison.unwrap(), std::cmp::Ordering::Greater);
} 