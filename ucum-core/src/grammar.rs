use serde::{Deserialize, Serialize};

/// Represents a UCUM term according to the grammar: term → component | '/' term | component '/' term | component '.' term
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UcumTerm {
    Component(UcumComponent),
    Division(Box<UcumTerm>, Box<UcumTerm>),  // left / right
    Concatenation(Box<UcumTerm>, Box<UcumTerm>),  // left . right
}

/// Represents a UCUM component according to the grammar rules
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UcumComponent {
    Parenthesized(Box<UcumTerm>),
    ParenthesizedWithAnnotation(Box<UcumTerm>, UcumAnnotation),
    AnnotatableWithAnnotation(UcumAnnotatable, UcumAnnotation),
    Annotation(UcumAnnotation),
    Annotatable(UcumAnnotatable),
    DigitSymbols(String),
    AsteriskNotation(String, String),  // base, exponent (e.g., "10", "4" for "10*4")
    ComplexExpression(Box<UcumTerm>, Box<UcumTerm>),  // For complex expressions like "U/(10.g){feces}"
}

/// Represents an annotatable unit with optional exponent
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UcumAnnotatable {
    pub simple_unit: UcumSimpleUnit,
    pub exponent: Option<UcumExponent>,
}

/// Represents a simple unit according to the grammar rules
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UcumSimpleUnit {
    SimpleUnitSymbols(String),
    SquareBracketsSymbols(String),
    SquareBracketsWithSimple(String, String),  // [symbols]simple
    SimpleWithSquareBrackets(String, String),  // simple[symbols]
    SimpleSquareBracketsSimple(String, String, String),  // simple[symbols]simple
}

/// Represents an exponent with optional sign
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UcumExponent {
    pub sign: Option<char>,  // '+' or '-'
    pub digits: String,
}

/// Represents an annotation in curly braces
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UcumAnnotation {
    pub symbols: String,  // withinCbSymbol+
}

/// Represents a terminal unit symbol according to the grammar
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TerminalUnitSymbol {
    NonDigit(NonDigitTerminalUnitSymbol),
    Digit(char),
}

/// Represents non-digit terminal unit symbols from the grammar
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NonDigitTerminalUnitSymbol {
    Exclamation,      // '!'
    Hash,            // '#'
    Dollar,          // '$'
    Percent,         // '%'
    Ampersand,       // '&'
    Apostrophe,      // '\''
    Asterisk,        // '*'
    Comma,           // ','
    Colon,           // ':'
    Semicolon,       // ';'
    LessThan,        // '<'
    GreaterThan,     // '>'
    Question,        // '?'
    At,              // '@'
    Letter(char),    // 'A'..'Z' | 'a'..'z'
    Backslash,       // '\\'
    Caret,           // '^'
    Underscore,      // '_'
    Backtick,        // '`'
    Pipe,            // '|'
    Tilde,           // '~'
}

impl TerminalUnitSymbol {
    /// Check if a character is a valid terminal unit symbol
    pub fn is_valid(symbol: char) -> bool {
        match symbol {
            '0'..='9' => true,
            '!' | '#' | '$' | '%' | '&' | '\'' | '*' | ',' | ':' | ';' | '<' | '>' | '?' | '@' | '\\' | '^' | '_' | '`' | '|' | '~' => true,
            'A'..='Z' | 'a'..='z' => true,
            _ => false,
        }
    }
    
    /// Create a TerminalUnitSymbol from a character
    pub fn from_char(c: char) -> Result<Self, crate::error::UcumError> {
        match c {
            '0'..='9' => Ok(TerminalUnitSymbol::Digit(c)),
            '!' => Ok(TerminalUnitSymbol::NonDigit(NonDigitTerminalUnitSymbol::Exclamation)),
            '#' => Ok(TerminalUnitSymbol::NonDigit(NonDigitTerminalUnitSymbol::Hash)),
            '$' => Ok(TerminalUnitSymbol::NonDigit(NonDigitTerminalUnitSymbol::Dollar)),
            '%' => Ok(TerminalUnitSymbol::NonDigit(NonDigitTerminalUnitSymbol::Percent)),
            '&' => Ok(TerminalUnitSymbol::NonDigit(NonDigitTerminalUnitSymbol::Ampersand)),
            '\'' => Ok(TerminalUnitSymbol::NonDigit(NonDigitTerminalUnitSymbol::Apostrophe)),
            '*' => Ok(TerminalUnitSymbol::NonDigit(NonDigitTerminalUnitSymbol::Asterisk)),
            ',' => Ok(TerminalUnitSymbol::NonDigit(NonDigitTerminalUnitSymbol::Comma)),
            ':' => Ok(TerminalUnitSymbol::NonDigit(NonDigitTerminalUnitSymbol::Colon)),
            ';' => Ok(TerminalUnitSymbol::NonDigit(NonDigitTerminalUnitSymbol::Semicolon)),
            '<' => Ok(TerminalUnitSymbol::NonDigit(NonDigitTerminalUnitSymbol::LessThan)),
            '>' => Ok(TerminalUnitSymbol::NonDigit(NonDigitTerminalUnitSymbol::GreaterThan)),
            '?' => Ok(TerminalUnitSymbol::NonDigit(NonDigitTerminalUnitSymbol::Question)),
            '@' => Ok(TerminalUnitSymbol::NonDigit(NonDigitTerminalUnitSymbol::At)),
            '\\' => Ok(TerminalUnitSymbol::NonDigit(NonDigitTerminalUnitSymbol::Backslash)),
            '^' => Ok(TerminalUnitSymbol::NonDigit(NonDigitTerminalUnitSymbol::Caret)),
            '_' => Ok(TerminalUnitSymbol::NonDigit(NonDigitTerminalUnitSymbol::Underscore)),
            '`' => Ok(TerminalUnitSymbol::NonDigit(NonDigitTerminalUnitSymbol::Backtick)),
            '|' => Ok(TerminalUnitSymbol::NonDigit(NonDigitTerminalUnitSymbol::Pipe)),
            '~' => Ok(TerminalUnitSymbol::NonDigit(NonDigitTerminalUnitSymbol::Tilde)),
            'A'..='Z' | 'a'..='z' => Ok(TerminalUnitSymbol::NonDigit(NonDigitTerminalUnitSymbol::Letter(c))),
            _ => Err(crate::error::UcumError::InvalidSymbol(c.to_string())),
        }
    }
}

/// Represents a UCUM unit with all its properties
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UcumUnit {
    pub code: String,
    pub name: String,
    pub symbol: Option<String>,
    pub dimension: String,
    pub conversion_factor: f64,
    pub conversion_offset: f64,
    pub base_unit: Option<String>,
    pub is_base_unit: bool,
    pub is_metric: bool,
    pub is_imperial: bool,
    pub category: UcumCategory,
}

/// Represents UCUM unit categories
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UcumCategory {
    Length,
    Mass,
    Time,
    Temperature,
    Volume,
    Pressure,
    Energy,
    Power,
    Frequency,
    Angle,
    Information,
    Other,
}

/// Represents a UCUM quantity with value and unit
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UcumQuantity {
    pub value: f64,
    pub unit: UcumUnit,
    pub precision: Option<u32>,
}

impl UcumQuantity {
    pub fn new(value: f64, unit: UcumUnit) -> Self {
        Self {
            value,
            unit,
            precision: None,
        }
    }
}

/// Represents a UCUM conversion between units
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UcumConversion {
    pub from_unit: String,
    pub to_unit: String,
    pub factor: f64,
    pub offset: f64,
    pub formula: Option<String>,
}

/// Represents a UCUM prefix
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UcumPrefix {
    pub code: String,
    pub name: String,
    pub symbol: String,
    pub factor: f64,
} 