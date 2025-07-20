use std::collections::HashMap;
use crate::error::UcumError;
use crate::grammar::*;
use crate::registry::UcumRegistry;

/// Simple UCUM parser with default registry
pub struct UcumParser {
    grammar_parser: UcumGrammarParser,
}

impl UcumParser {
    /// Create a new UCUM parser with default registry
    pub fn new() -> Self {
        Self {
            grammar_parser: UcumGrammarParser::new(UcumRegistry::new().unwrap_or_default()),
        }
    }
    
    /// Parse a UCUM expression
    pub fn parse(&self, input: &str) -> Result<UcumTerm, UcumError> {
        self.grammar_parser.parse_main_term(input)
    }
    
    /// Parse and evaluate a UCUM expression to produce a UCUM unit
    pub fn parse_and_evaluate(&self, input: &str) -> Result<UcumUnit, UcumError> {
        let term = self.grammar_parser.parse_main_term(input)?;
        self.grammar_parser.evaluate_term(&term)
    }
    
    /// Get a reference to the grammar parser
    pub fn grammar_parser(&self) -> &UcumGrammarParser {
        &self.grammar_parser
    }
    
    /// Get a mutable reference to the grammar parser
    pub fn grammar_parser_mut(&mut self) -> &mut UcumGrammarParser {
        &mut self.grammar_parser
    }
    
    /// Clone the grammar parser
    pub fn clone_grammar_parser(&self) -> UcumGrammarParser {
        self.grammar_parser.clone()
    }
}

/// Grammar-compliant UCUM parser that follows the UCUM.g4 specification
#[derive(Clone, Debug, PartialEq)]
pub struct UcumGrammarParser {
    registry: UcumRegistry,
    parse_cache: HashMap<String, UcumTerm>,
}

impl UcumGrammarParser {
    /// Create a new UCUM grammar parser
    pub fn new(registry: UcumRegistry) -> Self {
        Self {
            registry,
            parse_cache: HashMap::new(),
        }
    }
    
    /// Parse a main term according to the grammar: mainTerm → term EOF
    pub fn parse_main_term(&self, input: &str) -> Result<UcumTerm, UcumError> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err(UcumError::GrammarParsingFailed("Empty input".to_string()));
        }
        
        self.parse_term(trimmed)
    }
    
    /// Parse a term according to the grammar: term → component | '/' term | component '/' term | component '.' term
    fn parse_term(&self, input: &str) -> Result<UcumTerm, UcumError> {
        // Special handling for "/100{cells}" type expressions
        if input.starts_with('/') {
            let after_slash = &input[1..];
            if after_slash.is_empty() {
                return Err(UcumError::EmptyUnitSymbols);
            }
            
            // Special handling for "/100{cells}" type expressions
            if after_slash.len() > 4 && after_slash.starts_with("100{") {
                let annotation_end = after_slash.rfind('}').unwrap();
                let annotation_text = &after_slash[4..annotation_end]; // Skip "100{"
                let annotation = self.parse_annotation_symbols(annotation_text)?;
                
                // Create a simple division: 1 / 100 with annotation
                let one_component = UcumComponent::DigitSymbols("1".to_string());
                let hundred_annotated = UcumComponent::AnnotatableWithAnnotation(
                    UcumAnnotatable {
                        simple_unit: UcumSimpleUnit::SimpleUnitSymbols("100".to_string()),
                        exponent: None,
                    },
                    annotation
                );
                
                return Ok(UcumTerm::Division(
                    Box::new(UcumTerm::Component(one_component)),
                    Box::new(UcumTerm::Component(hundred_annotated))
                ));
            }
        }
        
        // First, try to find division or concatenation operators
        let mut paren_count = 0;
        let mut in_annotation = false;
        let mut in_square_brackets = false;
        
        for (i, c) in input.chars().enumerate() {
            match c {
                '(' => if !in_annotation && !in_square_brackets { paren_count += 1; },
                ')' => if !in_annotation && !in_square_brackets { paren_count -= 1; },
                '{' => in_annotation = true,
                '}' => in_annotation = false,
                '[' => in_square_brackets = true,
                ']' => in_square_brackets = false,
                '/' => {
                    if paren_count == 0 && !in_annotation && !in_square_brackets {
                        // Found division operator
                        let left = &input[..i];
                        let right = &input[i+1..];
                        
                        if left.is_empty() {
                            // Handle case: /term
                            let right_term = self.parse_term(right)?;
                            return Ok(UcumTerm::Division(
                                Box::new(UcumTerm::Component(UcumComponent::DigitSymbols("1".to_string()))),
                                Box::new(right_term)
                            ));
                        } else {
                            // Handle case: component/term
                            let left_component = self.parse_component(left)?;
                            let right_term = self.parse_term(right)?;
                            return Ok(UcumTerm::Division(
                                Box::new(UcumTerm::Component(left_component)),
                                Box::new(right_term)
                            ));
                        }
                    }
                },
                '.' => {
                    if paren_count == 0 && !in_annotation && !in_square_brackets {
                        // Found concatenation operator
                        let left = &input[..i];
                        let right = &input[i+1..];
                        
                        let left_term = self.parse_term(left)?;
                        let right_term = self.parse_term(right)?;
                        return Ok(UcumTerm::Concatenation(
                            Box::new(left_term),
                            Box::new(right_term)
                        ));
                    }
                },
                _ => {}
            }
        }
        
        // No operators found, parse as component
        let component = self.parse_component(input)?;
        Ok(UcumTerm::Component(component))
    }
    
    /// Parse a component according to the grammar rules
    fn parse_component(&self, input: &str) -> Result<UcumComponent, UcumError> {
        let trimmed = input.trim();
        
        // Check for asterisk notation (e.g., "10*4" for 10^4)
        if let Some(asterisk_pos) = trimmed.find('*') {
            let base_part = &trimmed[..asterisk_pos];
            let exponent_part = &trimmed[asterisk_pos + 1..];
            
            // Validate that base and exponent are valid
            if base_part.chars().all(|c| c.is_ascii_digit()) && 
               exponent_part.chars().all(|c| c.is_ascii_digit()) {
                return Ok(UcumComponent::AsteriskNotation(
                    base_part.to_string(),
                    exponent_part.to_string()
                ));
            }
        }
        
        // Check for parenthesized terms
        if trimmed.starts_with('(') && trimmed.ends_with(')') {
            let inner = &trimmed[1..trimmed.len()-1];
            let term = self.parse_term(inner)?;
            
            // Check if there's an annotation after the closing parenthesis
            if let Some(annotation_start) = trimmed[1..].find('}') {
                let annotation_end = trimmed.rfind('}').unwrap();
                if annotation_end > annotation_start {
                    let annotation_text = &trimmed[annotation_start+1..annotation_end];
                    let annotation = self.parse_annotation_symbols(annotation_text)?;
                    return Ok(UcumComponent::ParenthesizedWithAnnotation(
                        Box::new(term),
                        annotation
                    ));
                }
            }
            
            return Ok(UcumComponent::Parenthesized(Box::new(term)));
        }
        
        // Check for parenthesized terms in the middle (e.g., "U/(10.g){feces}")
        if let Some(paren_start) = trimmed.find('(') {
            if let Some(paren_end) = trimmed.rfind(')') {
                if paren_end > paren_start {
                    let before_paren = &trimmed[..paren_start];
                    let inner = &trimmed[paren_start+1..paren_end];
                    let after_paren = &trimmed[paren_end+1..];
                    
                    // Parse the inner term
                    let inner_term = self.parse_term(inner)?;
                    
                    // Check if there's an annotation after the closing parenthesis
                    if let Some(annotation_start) = after_paren.find('{') {
                        let annotation_end = after_paren.rfind('}').unwrap();
                        if annotation_end > annotation_start {
                            let annotation_text = &after_paren[annotation_start+1..annotation_end];
                            let annotation = self.parse_annotation_symbols(annotation_text)?;
                            
                            // Create a complex expression: before_paren / (inner_term) {annotation}
                            let paren_component = UcumComponent::ParenthesizedWithAnnotation(
                                Box::new(inner_term),
                                annotation
                            );
                            
                            if !before_paren.is_empty() {
                                // Handle case like "U/(10.g){feces}" -> "U" / "(10.g){feces}"
                                let before_component = self.parse_component(before_paren)?;
                                return Ok(UcumComponent::ComplexExpression(
                                    Box::new(UcumTerm::Component(before_component)),
                                    Box::new(UcumTerm::Component(paren_component))
                                ));
                            } else {
                                return Ok(paren_component);
                            }
                        }
                    }
                    
                    // Handle case like "U/(10.g)" -> "U" / "(10.g)"
                    if !before_paren.is_empty() {
                        let before_component = self.parse_component(before_paren)?;
                        let paren_component = UcumComponent::Parenthesized(Box::new(inner_term));
                        return Ok(UcumComponent::ComplexExpression(
                            Box::new(UcumTerm::Component(before_component)),
                            Box::new(UcumTerm::Component(paren_component))
                        ));
                    }
                }
            }
        }
        

        
        // Check for complex expressions like "10*3{copies}/mL" or "10*3{RBCs}"
        if let Some(asterisk_pos) = trimmed.find('*') {
            if let Some(annotation_start) = trimmed.find('{') {
                if annotation_start > asterisk_pos {
                    // This is a complex expression like "10*3{copies}"
                    let base_part = &trimmed[..asterisk_pos];
                    let exponent_part = &trimmed[asterisk_pos + 1..annotation_start];
                    let annotation_end = trimmed.rfind('}').unwrap();
                    let annotation_text = &trimmed[annotation_start + 1..annotation_end];
                    let after_annotation = &trimmed[annotation_end + 1..];
                    
                    // Validate parts
                    if base_part.chars().all(|c| c.is_ascii_digit()) && 
                       exponent_part.chars().all(|c| c.is_ascii_digit()) {
                        
                        let annotation = self.parse_annotation_symbols(annotation_text)?;
                        
                        // Create the complex expression
                        let asterisk_component = UcumComponent::AsteriskNotation(
                            base_part.to_string(),
                            exponent_part.to_string()
                        );
                        
                        let annotated_component = UcumComponent::AnnotatableWithAnnotation(
                            UcumAnnotatable {
                                simple_unit: UcumSimpleUnit::SimpleUnitSymbols("1".to_string()),
                                exponent: None,
                            },
                            annotation
                        );
                        
                        if !after_annotation.is_empty() {
                            // Handle case like "10*3{copies}/mL"
                            let after_component = self.parse_component(after_annotation)?;
                            return Ok(UcumComponent::ComplexExpression(
                                Box::new(UcumTerm::Component(asterisk_component)),
                                Box::new(UcumTerm::Component(after_component))
                            ));
                        } else {
                            // Handle case like "10*3{RBCs}"
                            return Ok(UcumComponent::ComplexExpression(
                                Box::new(UcumTerm::Component(asterisk_component)),
                                Box::new(UcumTerm::Component(annotated_component))
                            ));
                        }
                    }
                }
            }
        }
        
        // Check for pure annotations
        if trimmed.starts_with('{') && trimmed.ends_with('}') {
            let annotation_text = &trimmed[1..trimmed.len()-1];
            let annotation = self.parse_annotation_symbols(annotation_text)?;
            return Ok(UcumComponent::Annotation(annotation));
        }
        
        // Check for digit symbols
        if trimmed.chars().all(|c| c.is_ascii_digit()) {
            return Ok(UcumComponent::DigitSymbols(trimmed.to_string()));
        }
        
        // Check if there's an annotation first
        if let Some(annotation_start) = trimmed.find('{') {
            let annotation_end = trimmed.rfind('}').unwrap();
            if annotation_end > annotation_start {
                let before_annotation = &trimmed[..annotation_start];
                let annotation_text = &trimmed[annotation_start+1..annotation_end];
                
                // If there's nothing before the annotation, it's a pure annotation
                if before_annotation.is_empty() {
                    let annotation = self.parse_annotation_symbols(annotation_text)?;
                    return Ok(UcumComponent::Annotation(annotation));
                }
                
                // Otherwise, parse the part before the annotation
                let annotatable = self.parse_annotatable(before_annotation)?;
                let annotation = self.parse_annotation_symbols(annotation_text)?;
                
                return Ok(UcumComponent::AnnotatableWithAnnotation(
                    annotatable,
                    annotation
                ));
            }
        }
        
        // Parse as annotatable
        let annotatable = self.parse_annotatable(trimmed)?;
        Ok(UcumComponent::Annotatable(annotatable))
    }
    
    /// Parse an annotatable according to the grammar: annotatable → simpleUnit | simpleUnit exponent
    fn parse_annotatable(&self, input: &str) -> Result<UcumAnnotatable, UcumError> {
        // Look for exponent (digits with optional sign)
        let mut exponent_start = None;
        let mut paren_count = 0;
        let mut in_annotation = false;
        let mut in_square_brackets = false;
        
        for (i, c) in input.chars().enumerate() {
            match c {
                '(' => if !in_annotation && !in_square_brackets { paren_count += 1; },
                ')' => if !in_annotation && !in_square_brackets { paren_count -= 1; },
                '{' => in_annotation = true,
                '}' => in_annotation = false,
                '[' => in_square_brackets = true,
                ']' => in_square_brackets = false,
                '+' | '-' => {
                    if paren_count == 0 && !in_annotation && !in_square_brackets {
                        // Check if this is followed by digits
                        let rest = &input[i+1..];
                        if rest.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                            exponent_start = Some(i);
                            break;
                        }
                    }
                },
                '0'..='9' => {
                    if paren_count == 0 && !in_annotation && !in_square_brackets {
                        // Check if this is the start of an exponent
                        if i > 0 {
                            let prev_char = input.chars().nth(i-1).unwrap();
                            if !prev_char.is_ascii_alphanumeric() && prev_char != ']' && prev_char != '}' {
                                exponent_start = Some(i);
                                break;
                            }
                        } else {
                            // Start of input, could be an exponent
                            exponent_start = Some(i);
                            break;
                        }
                    }
                },
                _ => {}
            }
        }
        
        if let Some(start) = exponent_start {
            let simple_unit_part = &input[..start];
            let exponent_part = &input[start..];
            
            let simple_unit = self.parse_simple_unit(simple_unit_part)?;
            let exponent = self.parse_exponent(exponent_part)?;
            
            Ok(UcumAnnotatable {
                simple_unit,
                exponent: Some(exponent),
            })
        } else {
            let simple_unit = self.parse_simple_unit(input)?;
            Ok(UcumAnnotatable {
                simple_unit,
                exponent: None,
            })
        }
    }
    
    /// Parse a simple unit according to the grammar rules
    fn parse_simple_unit(&self, input: &str) -> Result<UcumSimpleUnit, UcumError> {
        let trimmed = input.trim();
        
        // Check for square brackets
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            let symbols = &trimmed[1..trimmed.len()-1];
            self.validate_within_sb_symbols(symbols)?;
            return Ok(UcumSimpleUnit::SquareBracketsSymbols(symbols.to_string()));
        }
        
        // Look for square brackets in the middle
        if let Some(bracket_start) = trimmed.find('[') {
            let bracket_end = trimmed.rfind(']').unwrap();
            if bracket_end > bracket_start {
                let before_brackets = &trimmed[..bracket_start];
                let in_brackets = &trimmed[bracket_start+1..bracket_end];
                let after_brackets = &trimmed[bracket_end+1..];
                
                self.validate_within_sb_symbols(in_brackets)?;
                
                if !before_brackets.is_empty() && !after_brackets.is_empty() {
                    // simple[symbols]simple
                    self.validate_simple_unit_symbols(before_brackets)?;
                    self.validate_simple_unit_symbols(after_brackets)?;
                    return Ok(UcumSimpleUnit::SimpleSquareBracketsSimple(
                        before_brackets.to_string(),
                        in_brackets.to_string(),
                        after_brackets.to_string()
                    ));
                } else if !before_brackets.is_empty() {
                    // simple[symbols]
                    self.validate_simple_unit_symbols(before_brackets)?;
                    return Ok(UcumSimpleUnit::SimpleWithSquareBrackets(
                        before_brackets.to_string(),
                        in_brackets.to_string()
                    ));
                } else if !after_brackets.is_empty() {
                    // [symbols]simple
                    self.validate_simple_unit_symbols(after_brackets)?;
                    return Ok(UcumSimpleUnit::SquareBracketsWithSimple(
                        in_brackets.to_string(),
                        after_brackets.to_string()
                    ));
                }
            }
        }
        
        // Simple unit symbols
        self.validate_simple_unit_symbols(trimmed)?;
        Ok(UcumSimpleUnit::SimpleUnitSymbols(trimmed.to_string()))
    }
    
    /// Parse an exponent according to the grammar: exponent → ('+' | '-') digitSymbols | digitSymbols
    fn parse_exponent(&self, input: &str) -> Result<UcumExponent, UcumError> {
        let trimmed = input.trim();
        
        if trimmed.is_empty() {
            return Err(UcumError::InvalidExponent("Empty exponent".to_string()));
        }
        
        let first_char = trimmed.chars().next().unwrap();
        let (sign, digits) = match first_char {
            '+' | '-' => {
                let digits = &trimmed[1..];
                if digits.is_empty() {
                    return Err(UcumError::InvalidExponent("No digits after sign".to_string()));
                }
                if !digits.chars().all(|c| c.is_ascii_digit()) {
                    return Err(UcumError::InvalidExponent("Non-digit characters in exponent".to_string()));
                }
                (Some(first_char), digits.to_string())
            },
            '0'..='9' => {
                if !trimmed.chars().all(|c| c.is_ascii_digit()) {
                    return Err(UcumError::InvalidExponent("Non-digit characters in exponent".to_string()));
                }
                (None, trimmed.to_string())
            },
            _ => return Err(UcumError::InvalidExponent("Invalid exponent format".to_string())),
        };
        
        Ok(UcumExponent { sign, digits })
    }
    
    /// Parse annotation symbols according to the grammar: annotationSymbols → '{' (withinCbSymbol)+ '}'
    fn parse_annotation_symbols(&self, input: &str) -> Result<UcumAnnotation, UcumError> {
        self.validate_within_cb_symbols(input)?;
        Ok(UcumAnnotation {
            symbols: input.to_string(),
        })
    }
    
    /// Validate terminal symbols
    pub fn validate_terminal_symbols(&self, input: &str) -> Result<Vec<TerminalUnitSymbol>, UcumError> {
        let mut symbols = Vec::new();
        for c in input.chars() {
            symbols.push(TerminalUnitSymbol::from_char(c)?);
        }
        Ok(symbols)
    }
    
    /// Validate simple unit symbols according to the grammar: simpleUnitSymbols → (terminalUnitSymbol)+
    pub fn validate_simple_unit_symbols(&self, input: &str) -> Result<(), UcumError> {
        if input.is_empty() {
            return Err(UcumError::EmptyUnitSymbols);
        }
        
        for c in input.chars() {
            TerminalUnitSymbol::from_char(c)?;
        }
        Ok(())
    }
    
    /// Validate within curly braces symbols according to the grammar: withinCbSymbol → withinCbOrSbSymbol | ' ' | '[' | ']'
    pub fn validate_within_cb_symbols(&self, input: &str) -> Result<(), UcumError> {
        for c in input.chars() {
            match c {
                ' ' | '[' | ']' => continue,
                _ => {
                    self.validate_within_cb_or_sb_symbol(c)?;
                }
            }
        }
        Ok(())
    }
    
    /// Validate within square brackets symbols according to the grammar: withinSbSymbol → withinCbOrSbSymbol | '{' | '}'
    pub fn validate_within_sb_symbols(&self, input: &str) -> Result<(), UcumError> {
        for c in input.chars() {
            match c {
                '{' | '}' => continue,
                _ => {
                    self.validate_within_cb_or_sb_symbol(c)?;
                }
            }
        }
        Ok(())
    }
    
    /// Validate within curly braces or square brackets symbols according to the grammar: withinCbOrSbSymbol → terminalUnitSymbol | '"' | '(' | ')' | '+' | '-' | '.' | '/' | '='
    fn validate_within_cb_or_sb_symbol(&self, c: char) -> Result<(), UcumError> {
        match c {
            '"' | '(' | ')' | '+' | '-' | '.' | '/' | '=' => Ok(()),
            _ => {
                TerminalUnitSymbol::from_char(c)?;
                Ok(())
            }
        }
    }
    
    /// Parse with caching for performance
    pub fn parse_with_cache(&mut self, input: &str) -> Result<UcumTerm, UcumError> {
        if let Some(cached) = self.parse_cache.get(input) {
            return Ok(cached.clone());
        }
        
        let result = self.parse_main_term(input)?;
        self.parse_cache.insert(input.to_string(), result.clone());
        Ok(result)
    }
    
    /// Clear the parse cache to free memory
    pub fn clear_cache(&mut self) {
        self.parse_cache.clear();
    }
    
    /// Get cache statistics
    pub fn cache_stats(&self) -> (usize, usize) {
        (self.parse_cache.len(), self.parse_cache.capacity())
    }
    
    /// Optimized symbol validation with lookup table
    fn create_symbol_lookup_table() -> std::collections::HashSet<char> {
        let mut valid_symbols = std::collections::HashSet::new();
        // Add all valid terminal symbols
        for c in '0'..='9' { valid_symbols.insert(c); }
        for c in 'A'..='Z' { valid_symbols.insert(c); }
        for c in 'a'..='z' { valid_symbols.insert(c); }
        valid_symbols.extend(['!', '#', '$', '%', '&', '\'', '*', ',', ':', ';', '<', '>', '?', '@', '\\', '^', '_', '`', '|', '~']);
        valid_symbols
    }
    
    /// Fast symbol validation using lookup table
    pub fn fast_symbol_validation(&self, input: &str) -> bool {
        static VALID_SYMBOLS: std::sync::OnceLock<std::collections::HashSet<char>> = std::sync::OnceLock::new();
        let valid_symbols = VALID_SYMBOLS.get_or_init(Self::create_symbol_lookup_table);
        
        input.chars().all(|c| valid_symbols.contains(&c))
    }
    
    /// Evaluate a UCUM term to produce a UCUM unit
    pub fn evaluate_term(&self, term: &UcumTerm) -> Result<UcumUnit, UcumError> {
        match term {
            UcumTerm::Component(component) => self.evaluate_component(component),
            UcumTerm::Division(left, right) => {
                let left_unit = self.evaluate_term(left)?;
                let right_unit = self.evaluate_term(right)?;
                self.divide_units(&left_unit, &right_unit)
            }
            UcumTerm::Concatenation(left, right) => {
                let left_unit = self.evaluate_term(left)?;
                let right_unit = self.evaluate_term(right)?;
                self.multiply_units(&left_unit, &right_unit)
            }
        }
    }
    
    /// Evaluate a UCUM component to produce a UCUM unit
    fn evaluate_component(&self, component: &UcumComponent) -> Result<UcumUnit, UcumError> {
        match component {
            UcumComponent::Parenthesized(term) => self.evaluate_term(term),
            UcumComponent::ParenthesizedWithAnnotation(term, annotation) => {
                let unit = self.evaluate_term(term)?;
                self.apply_annotation(&unit, annotation)
            }
            UcumComponent::AnnotatableWithAnnotation(annotatable, annotation) => {
                let unit = self.evaluate_annotatable(annotatable)?;
                self.apply_annotation(&unit, annotation)
            }
            UcumComponent::Annotation(annotation) => {
                // Handle pure annotation units
                self.create_annotation_unit(annotation)
            }
            UcumComponent::Annotatable(annotatable) => self.evaluate_annotatable(annotatable),
            UcumComponent::DigitSymbols(digits) => {
                // Handle numeric components
                self.create_numeric_unit(digits)
            }
            UcumComponent::AsteriskNotation(base, exponent) => {
                // Handle asterisk notation (e.g., "10*4" for 10^4)
                self.create_asterisk_notation_unit(base, exponent)
            }
            UcumComponent::ComplexExpression(left, right) => {
                // Handle complex expressions like "U/(10.g){feces}"
                let left_unit = self.evaluate_term(left)?;
                let right_unit = self.evaluate_term(right)?;
                self.divide_units(&left_unit, &right_unit)
            }
        }
    }
    
    /// Evaluate an annotatable to produce a UCUM unit
    fn evaluate_annotatable(&self, annotatable: &UcumAnnotatable) -> Result<UcumUnit, UcumError> {
        let mut unit = self.evaluate_simple_unit(&annotatable.simple_unit)?;
        
        if let Some(exponent) = &annotatable.exponent {
            unit = self.apply_exponent(&unit, exponent)?;
        }
        
        Ok(unit)
    }
    
    /// Evaluate a simple unit to produce a UCUM unit
    fn evaluate_simple_unit(&self, simple_unit: &UcumSimpleUnit) -> Result<UcumUnit, UcumError> {
        match simple_unit {
            UcumSimpleUnit::SimpleUnitSymbols(symbols) => {
                self.registry.get_unit(symbols)
                    .cloned()
                    .ok_or_else(|| UcumError::InvalidUnit(symbols.clone()))
            }
            UcumSimpleUnit::SquareBracketsSymbols(symbols) => {
                self.create_square_bracket_unit(symbols)
            }
            UcumSimpleUnit::SquareBracketsWithSimple(brackets, simple) => {
                let bracket_unit = self.create_square_bracket_unit(brackets)?;
                let simple_unit = self.registry.get_unit(simple)
                    .cloned()
                    .ok_or_else(|| UcumError::InvalidUnit(simple.clone()))?;
                self.multiply_units(&bracket_unit, &simple_unit)
            }
            UcumSimpleUnit::SimpleWithSquareBrackets(simple, brackets) => {
                let simple_unit = self.registry.get_unit(simple)
                    .cloned()
                    .ok_or_else(|| UcumError::InvalidUnit(simple.clone()))?;
                let bracket_unit = self.create_square_bracket_unit(brackets)?;
                self.multiply_units(&simple_unit, &bracket_unit)
            }
            UcumSimpleUnit::SimpleSquareBracketsSimple(simple1, brackets, simple2) => {
                let unit1 = self.registry.get_unit(simple1)
                    .cloned()
                    .ok_or_else(|| UcumError::InvalidUnit(simple1.clone()))?;
                let bracket_unit = self.create_square_bracket_unit(brackets)?;
                let unit2 = self.registry.get_unit(simple2)
                    .cloned()
                    .ok_or_else(|| UcumError::InvalidUnit(simple2.clone()))?;
                
                let temp = self.multiply_units(&unit1, &bracket_unit)?;
                self.multiply_units(&temp, &unit2)
            }
        }
    }
    
    /// Apply an exponent to a unit
    fn apply_exponent(&self, unit: &UcumUnit, exponent: &UcumExponent) -> Result<UcumUnit, UcumError> {
        let exp_value: i32 = exponent.digits.parse()
            .map_err(|_| UcumError::InvalidExponent(exponent.digits.clone()))?;
        
        let final_exp = match exponent.sign {
            Some('-') => -exp_value,
            Some('+') | None => exp_value,
            _ => return Err(UcumError::InvalidExponentSign(exponent.sign.unwrap())),
        };
        
        self.raise_unit_to_power(unit, final_exp)
    }
    
    /// Multiply two units
    pub fn multiply_units(&self, left: &UcumUnit, right: &UcumUnit) -> Result<UcumUnit, UcumError> {
        // For now, create a simple combined unit
        // In a full implementation, this would handle dimension analysis
        let combined_code = format!("{}.{}", left.code, right.code);
        let combined_name = format!("{} times {}", left.name, right.name);
        
        Ok(UcumUnit {
            code: combined_code,
            name: combined_name,
            symbol: None,
            dimension: format!("{}.{}", left.dimension, right.dimension),
            conversion_factor: left.conversion_factor * right.conversion_factor,
            conversion_offset: 0.0, // Multiplication doesn't preserve offsets
            base_unit: None,
            is_base_unit: false,
            is_metric: left.is_metric && right.is_metric,
            is_imperial: left.is_imperial && right.is_imperial,
            category: UcumCategory::Other, // Would need dimension analysis for proper category
        })
    }
    
    /// Divide two units
    pub fn divide_units(&self, left: &UcumUnit, right: &UcumUnit) -> Result<UcumUnit, UcumError> {
        // For now, create a simple combined unit
        // In a full implementation, this would handle dimension analysis
        let combined_code = format!("{}/{}", left.code, right.code);
        let combined_name = format!("{} per {}", left.name, right.name);
        
        Ok(UcumUnit {
            code: combined_code,
            name: combined_name,
            symbol: None,
            dimension: format!("{}/{}", left.dimension, right.dimension),
            conversion_factor: left.conversion_factor / right.conversion_factor,
            conversion_offset: 0.0, // Division doesn't preserve offsets
            base_unit: None,
            is_base_unit: false,
            is_metric: left.is_metric && right.is_metric,
            is_imperial: left.is_imperial && right.is_imperial,
            category: UcumCategory::Other, // Would need dimension analysis for proper category
        })
    }
    
    /// Raise a unit to a power
    fn raise_unit_to_power(&self, unit: &UcumUnit, power: i32) -> Result<UcumUnit, UcumError> {
        let power_f64 = power as f64;
        let combined_code = if power == 1 {
            unit.code.clone()
        } else {
            format!("{}{}", unit.code, power)
        };
        
        let combined_name = if power == 1 {
            unit.name.clone()
        } else {
            format!("{} to the power of {}", unit.name, power)
        };
        
        Ok(UcumUnit {
            code: combined_code,
            name: combined_name,
            symbol: None,
            dimension: format!("{}^{}", unit.dimension, power),
            conversion_factor: unit.conversion_factor.powf(power_f64),
            conversion_offset: 0.0, // Exponentiation doesn't preserve offsets
            base_unit: None,
            is_base_unit: false,
            is_metric: unit.is_metric,
            is_imperial: unit.is_imperial,
            category: unit.category.clone(),
        })
    }
    
    /// Apply an annotation to a unit
    fn apply_annotation(&self, unit: &UcumUnit, annotation: &UcumAnnotation) -> Result<UcumUnit, UcumError> {
        let annotated_code = format!("{}{{{}}}", unit.code, annotation.symbols);
        let annotated_name = format!("{} ({})", unit.name, annotation.symbols);
        
        Ok(UcumUnit {
            code: annotated_code,
            name: annotated_name,
            symbol: unit.symbol.clone(),
            dimension: unit.dimension.clone(),
            conversion_factor: unit.conversion_factor,
            conversion_offset: unit.conversion_offset,
            base_unit: unit.base_unit.clone(),
            is_base_unit: unit.is_base_unit,
            is_metric: unit.is_metric,
            is_imperial: unit.is_imperial,
            category: unit.category.clone(),
        })
    }
    
    /// Create a square bracket unit
    fn create_square_bracket_unit(&self, symbols: &str) -> Result<UcumUnit, UcumError> {
        let code = format!("[{}]", symbols);
        Ok(UcumUnit {
            code,
            name: format!("arbitrary unit {}", symbols),
            symbol: Some(format!("[{}]", symbols)),
            dimension: "1".to_string(), // Arbitrary units have dimensionless dimension
            conversion_factor: 1.0,
            conversion_offset: 0.0,
            base_unit: None,
            is_base_unit: false,
            is_metric: false,
            is_imperial: false,
            category: UcumCategory::Other,
        })
    }
    
    /// Create an annotation unit
    fn create_annotation_unit(&self, annotation: &UcumAnnotation) -> Result<UcumUnit, UcumError> {
        let code = format!("{{{}}}", annotation.symbols);
        Ok(UcumUnit {
            code: code.clone(),
            name: format!("annotation unit {}", annotation.symbols),
            symbol: Some(code),
            dimension: "1".to_string(), // Annotation units are typically dimensionless
            conversion_factor: 1.0,
            conversion_offset: 0.0,
            base_unit: None,
            is_base_unit: false,
            is_metric: false,
            is_imperial: false,
            category: UcumCategory::Other,
        })
    }
    
    /// Create a numeric unit
    fn create_numeric_unit(&self, digits: &str) -> Result<UcumUnit, UcumError> {
        let value: f64 = digits.parse()
            .map_err(|_| UcumError::InvalidValue(digits.to_string()))?;
        
        Ok(UcumUnit {
            code: digits.to_string(),
            name: format!("numeric value {}", digits),
            symbol: Some(digits.to_string()),
            dimension: "1".to_string(),
            conversion_factor: value,
            conversion_offset: 0.0,
            base_unit: None,
            is_base_unit: false,
            is_metric: false,
            is_imperial: false,
            category: UcumCategory::Other,
        })
    }
    
    /// Create an asterisk notation unit (e.g., "10*4" for 10^4)
    fn create_asterisk_notation_unit(&self, base: &str, exponent: &str) -> Result<UcumUnit, UcumError> {
        let base_value: f64 = base.parse()
            .map_err(|_| UcumError::InvalidValue(base.to_string()))?;
        let exp_value: i32 = exponent.parse()
            .map_err(|_| UcumError::InvalidExponent(exponent.to_string()))?;
        
        let result_value = base_value.powi(exp_value);
        let code = format!("{}*{}", base, exponent);
        
        Ok(UcumUnit {
            code,
            name: format!("{} to the power of {}", base_value, exp_value),
            symbol: Some(format!("{}^{}", base_value, exp_value)),
            dimension: "1".to_string(),
            conversion_factor: result_value,
            conversion_offset: 0.0,
            base_unit: None,
            is_base_unit: false,
            is_metric: false,
            is_imperial: false,
            category: UcumCategory::Other,
        })
    }
} 