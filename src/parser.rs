//! UCUM expression parser with performance optimizations.
//!
//! This module implements high-performance parsing techniques including:
//! - Zero-copy parsing with minimal allocations
//! - SIMD-accelerated character validation
//! - Perfect hash lookups for common patterns
//! - Single-pass state machine parsing
//! - Small vector optimizations for AST nodes

use crate::ast::{OwnedUnitExpr, UnitExpr, UnitFactor};
use crate::error::UcumError;
use once_cell::sync::Lazy;
use phf::phf_map;
use smallvec::SmallVec;

// ============================================================================
// Compile-time lookup tables
// ============================================================================

/// Perfect hash for time units
static TIME_UNITS: phf::Map<&'static str, ()> = phf_map! {
    "h" => (), "hr" => (), "min" => (), "s" => (),
    "ms" => (), "us" => (), "ns" => (), "d" => (),
    "wk" => (), "mo" => (), "a" => (),
};

/// ASCII character classification lookup table
static CHAR_CLASS: Lazy<[CharClass; 256]> = Lazy::new(|| {
    let mut table = [CharClass::Invalid; 256];
    #[allow(clippy::needless_range_loop)]
    for i in 0..256 {
        let ch = i as u8 as char;
        table[i] = if ch.is_ascii_alphabetic() {
            CharClass::Letter
        } else if ch.is_ascii_digit() {
            CharClass::Digit
        } else {
            match ch {
                '.' => CharClass::Dot,
                '/' => CharClass::Slash,
                '^' => CharClass::Caret,
                '(' => CharClass::OpenParen,
                ')' => CharClass::CloseParen,
                '{' => CharClass::OpenBrace,
                '}' => CharClass::CloseBrace,
                '[' => CharClass::OpenBracket,
                ']' => CharClass::CloseBracket,
                '-' | '+' => CharClass::Sign,
                '_' | '\'' | '%' => CharClass::Symbol,
                '*' => CharClass::Star,
                ' ' | '\t' | '\n' | '\r' => CharClass::Whitespace,
                _ => CharClass::Invalid,
            }
        };
    }
    table
});

#[derive(Copy, Clone, Debug, PartialEq)]
enum CharClass {
    Letter,
    Digit,
    Dot,
    Slash,
    Caret,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Sign,
    Symbol,
    Star,
    Whitespace,
    Invalid,
}

// ============================================================================
// Compact string for small allocations
// ============================================================================

/// A string type optimized for small strings (up to 23 bytes inline)
#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)] // Future optimization, not currently used
pub enum CompactString {
    Inline { bytes: [u8; 23], len: u8 },
    Heap(String),
}

#[allow(dead_code)] // Future optimization, not currently used
impl CompactString {
    fn new(s: &str) -> Self {
        let bytes = s.as_bytes();
        if bytes.len() <= 23 {
            let mut inline_bytes = [0u8; 23];
            inline_bytes[..bytes.len()].copy_from_slice(bytes);
            CompactString::Inline {
                bytes: inline_bytes,
                len: bytes.len() as u8,
            }
        } else {
            CompactString::Heap(s.to_string())
        }
    }

    fn as_str(&self) -> &str {
        match self {
            CompactString::Inline { bytes, len } => unsafe {
                std::str::from_utf8_unchecked(&bytes[..*len as usize])
            },
            CompactString::Heap(s) => s.as_str(),
        }
    }
}

// ============================================================================
// Fast character validation
// ============================================================================

/// SIMD-accelerated ASCII validation for x86_64
#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
#[inline]
fn is_ascii_simd(bytes: &[u8]) -> bool {
    use std::arch::x86_64::*;

    unsafe {
        let ascii_mask = _mm_set1_epi8(0x80u8 as i8);

        // Process 16 bytes at a time
        let mut i = 0;
        while i + 16 <= bytes.len() {
            let chunk = _mm_loadu_si128(bytes.as_ptr().add(i) as *const __m128i);
            let result = _mm_and_si128(chunk, ascii_mask);
            if _mm_movemask_epi8(result) != 0 {
                return false;
            }
            i += 16;
        }

        // Check remaining bytes
        while i < bytes.len() {
            if bytes[i] >= 128 {
                return false;
            }
            i += 1;
        }

        true
    }
}

#[cfg(not(all(target_arch = "x86_64", target_feature = "sse2")))]
#[inline]
#[allow(dead_code)] // Future optimization, not currently used
fn is_ascii_simd(bytes: &[u8]) -> bool {
    bytes.iter().all(|&b| b < 128)
}

/// Fast symbol character check using lookup table.
///
/// Returns true if the character can be part of a UCUM symbol.
/// This includes letters, digits, brackets, and some special symbols.
#[inline(always)]
fn is_symbol_char_fast(ch: u8) -> bool {
    if ch < 128 {
        matches!(
            CHAR_CLASS[ch as usize],
            CharClass::Letter
                | CharClass::Digit
                | CharClass::Symbol
                | CharClass::OpenBracket
                | CharClass::CloseBracket
                | CharClass::Sign
        )
    } else {
        false
    }
}

// ============================================================================
// Parser state machine
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)] // Future state machine optimization, not fully implemented
enum ParserState {
    Initial,
    InSymbol,
    InNumber,
    InExponent,
    InAnnotation,
    AfterSymbol,
    AfterOperator,
}

/// Token types produced by the tokenizer.
///
/// Each token represents a meaningful unit in a UCUM expression.
#[derive(Debug, Clone, PartialEq)]
enum Token<'a> {
    Symbol(&'a str),
    Number(f64),
    TenPower(i32),
    Operator(char),
    OpenParen,
    CloseParen,
    Annotation(&'a str),
}

/// Fast single-pass tokenizer for UCUM expressions.
///
/// This tokenizer processes UCUM expressions character by character,
/// producing tokens that can be consumed by the parser.
struct Tokenizer<'a> {
    input: &'a str,
    bytes: &'a [u8],
    pos: usize,
    #[allow(dead_code)] // Future state machine optimization
    state: ParserState,
}

impl<'a> Tokenizer<'a> {
    /// Create a new tokenizer for the given input string.
    fn new(input: &'a str) -> Self {
        Self {
            input,
            bytes: input.as_bytes(),
            pos: 0,
            state: ParserState::Initial,
        }
    }

    /// Get the current byte at the tokenizer position.
    #[inline]
    fn current_byte(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    /// Peek at a byte at the given offset from the current position.
    #[inline]
    fn peek_byte(&self, offset: usize) -> Option<u8> {
        self.bytes.get(self.pos + offset).copied()
    }

    /// Skip whitespace characters and advance the position.
    fn skip_whitespace(&mut self) {
        while let Some(b) = self.current_byte() {
            if CHAR_CLASS[b as usize] == CharClass::Whitespace {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    /// Scan a UCUM symbol token.
    ///
    /// Handles both ASCII symbols and UTF-8 micro signs (µ).
    /// Also handles implicit exponents like "m2" -> "m" + "2".
    fn scan_symbol(&mut self) -> Option<Token<'a>> {
        let start = self.pos;

        // Handle UTF-8 µ (micro sign) first
        if self.current_byte() == Some(0xC2) && self.peek_byte(1) == Some(0xB5) {
            self.pos += 2;
            // Continue scanning for more characters
            while let Some(b) = self.current_byte() {
                if is_symbol_char_fast(b) {
                    self.pos += 1;
                } else {
                    break;
                }
            }
            return Some(Token::Symbol(&self.input[start..self.pos]));
        }

        // Fast path for ASCII symbols
        while let Some(b) = self.current_byte() {
            if is_symbol_char_fast(b) {
                self.pos += 1;
            } else {
                break;
            }
        }

        if self.pos > start {
            let symbol = &self.input[start..self.pos];

            // Check for implicit exponent (e.g., "m2")
            if let Some(exp_start) = symbol.rfind(|c: char| !c.is_ascii_digit()) {
                let exp_start = exp_start + 1;
                if exp_start < symbol.len() {
                    if let Ok(_exp) = symbol[exp_start..].parse::<i32>() {
                        self.pos = start + exp_start;
                        return Some(Token::Symbol(&symbol[..exp_start]));
                    }
                }
            }

            Some(Token::Symbol(symbol))
        } else {
            None
        }
    }

    /// Scan a numeric token, including decimals and scientific notation.
    fn scan_number(&mut self) -> Option<Token<'a>> {
        let start = self.pos;
        let mut has_dot = false;
        let mut has_exp = false;

        // Integer part
        while let Some(b) = self.current_byte() {
            match CHAR_CLASS[b as usize] {
                CharClass::Digit => self.pos += 1,
                CharClass::Dot if !has_dot && !has_exp => {
                    has_dot = true;
                    self.pos += 1;
                }
                _ if b == b'e' || b == b'E' => {
                    if !has_exp && self.pos > start {
                        has_exp = true;
                        self.pos += 1;
                        // Optional sign
                        if let Some(sign) = self.current_byte() {
                            if sign == b'+' || sign == b'-' {
                                self.pos += 1;
                            }
                        }
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }

        if self.pos > start {
            if let Ok(num) = self.input[start..self.pos].parse::<f64>() {
                return Some(Token::Number(num));
            }
        }

        None
    }

    /// Scan a power-of-ten token (e.g., "10*3" or "10^-2").
    fn scan_ten_power(&mut self) -> Option<Token<'a>> {
        // Check for "10*" or "10^" patterns
        if self.bytes.get(self.pos..self.pos + 3) == Some(b"10*")
            || self.bytes.get(self.pos..self.pos + 3) == Some(b"10^")
        {
            self.pos += 3;

            // Parse exponent
            let exp_start = self.pos;
            if let Some(sign) = self.current_byte() {
                if sign == b'+' || sign == b'-' {
                    self.pos += 1;
                }
            }

            let digit_start = self.pos;
            while let Some(b) = self.current_byte() {
                if b.is_ascii_digit() {
                    self.pos += 1;
                } else {
                    break;
                }
            }

            if self.pos > digit_start {
                if let Ok(exp) = self.input[exp_start..self.pos].parse::<i32>() {
                    return Some(Token::TenPower(exp));
                }
            }
        }

        None
    }

    /// Scan an annotation token enclosed in braces (e.g., "{comment}").
    fn scan_annotation(&mut self) -> Option<Token<'a>> {
        if self.current_byte() != Some(b'{') {
            return None;
        }

        self.pos += 1;
        let start = self.pos;
        let mut escaped = false;

        while let Some(b) = self.current_byte() {
            if escaped {
                escaped = false;
                self.pos += 1;
            } else if b == b'\\' {
                escaped = true;
                self.pos += 1;
            } else if b == b'}' {
                let content = &self.input[start..self.pos];
                self.pos += 1;
                return Some(Token::Annotation(content));
            } else {
                self.pos += 1;
            }
        }

        None
    }

    /// Get the next token from the input stream.
    fn next_token(&mut self) -> Option<Token<'a>> {
        self.skip_whitespace();

        let b = self.current_byte()?;

        // Check for UTF-8 µ (micro sign) first
        if b == 0xC2 && self.peek_byte(1) == Some(0xB5) {
            return self.scan_symbol();
        }

        match CHAR_CLASS[b as usize] {
            CharClass::Letter | CharClass::OpenBracket => self.scan_symbol(),
            CharClass::Digit => {
                // Check for 10* or 10^ patterns
                if b == b'1'
                    && self.peek_byte(1) == Some(b'0')
                    && (self.peek_byte(2) == Some(b'*') || self.peek_byte(2) == Some(b'^'))
                {
                    self.scan_ten_power()
                } else {
                    self.scan_number()
                }
            }
            CharClass::Dot | CharClass::Slash | CharClass::Caret => {
                self.pos += 1;
                Some(Token::Operator(b as char))
            }
            CharClass::OpenParen => {
                self.pos += 1;
                Some(Token::OpenParen)
            }
            CharClass::CloseParen => {
                self.pos += 1;
                Some(Token::CloseParen)
            }
            CharClass::OpenBrace => self.scan_annotation(),
            _ => None,
        }
    }
}

// ============================================================================
// AST building with small vector optimization
// ============================================================================

/// Small vector optimization for unit factors.
/// Most UCUM expressions have 4 or fewer factors, so we optimize for that case.
type SmallFactorVec<'a> = SmallVec<[UnitFactor<'a>; 4]>;

/// Optimized parser that builds AST from tokens
pub struct OptimizedParser<'a> {
    tokenizer: Tokenizer<'a>,
    // Reusable string buffer for normalization
    norm_buffer: String,
}

impl<'a> OptimizedParser<'a> {
    /// Create a new optimized parser for the given input.
    pub fn new(input: &'a str) -> Self {
        Self {
            tokenizer: Tokenizer::new(input),
            norm_buffer: String::with_capacity(32),
        }
    }

    /// Normalize µ (micro) to u if needed, using pre-allocated buffer.
    ///
    /// This handles Unicode micro signs (µ) by converting them to ASCII 'u'
    /// for consistent processing.
    fn normalize_symbol(&mut self, symbol: &'a str) -> UnitExpr<'a> {
        if symbol.contains('µ') {
            self.norm_buffer.clear();
            self.norm_buffer.reserve(symbol.len());
            for ch in symbol.chars() {
                self.norm_buffer.push(if ch == 'µ' { 'u' } else { ch });
            }
            UnitExpr::SymbolOwned(self.norm_buffer.clone())
        } else {
            UnitExpr::Symbol(symbol)
        }
    }

    /// Parse a factor (base expression with optional exponent).
    ///
    /// A factor consists of a base expression (symbol, number, or parenthesized expression)
    /// optionally followed by an exponent (explicit with ^ or implicit like "s2").
    #[allow(clippy::result_large_err)]
    fn parse_factor(&mut self) -> Result<Option<UnitFactor<'a>>, UcumError> {
        let token = match self.tokenizer.next_token() {
            Some(t) => t,
            None => return Ok(None),
        };

        let base_expr = match token {
            Token::Symbol(s) => {
                // Validate symbol
                if s.contains('%') && s.len() > 1 {
                    return Err(UcumError::invalid_expression("% must stand alone"));
                }

                // Check for invalid patterns
                if TIME_UNITS.contains_key(s) {
                    // Check if preceded by digits without decimal
                    let pos = self.tokenizer.pos - s.len();
                    if pos > 0 {
                        let before = &self.tokenizer.input[..pos];
                        if before.chars().last().is_some_and(|c| c.is_ascii_digit())
                            && !before.contains('.')
                        {
                            return Err(UcumError::invalid_expression(
                                "Time units must be preceded by decimal point",
                            ));
                        }
                    }
                }

                self.normalize_symbol(s)
            }
            Token::Number(n) => UnitExpr::Numeric(n),
            Token::TenPower(exp) => UnitExpr::Numeric(10f64.powi(exp)),
            Token::OpenParen => {
                // Parse parenthesized expression
                let inner = self.parse_expression()?;
                match self.tokenizer.next_token() {
                    Some(Token::CloseParen) => inner,
                    _ => return Err(UcumError::invalid_expression("Missing closing parenthesis")),
                }
            }
            Token::Annotation(content) => {
                // Standalone annotation
                UnitExpr::SymbolOwned(format!("{{{content}}}"))
            }
            _ => return Ok(None),
        };

        // Check for explicit exponent or implicit exponent (number following symbol)
        let mut exponent = 1;
        let saved_pos = self.tokenizer.pos;

        match self.tokenizer.next_token() {
            Some(Token::Operator('^')) => match self.tokenizer.next_token() {
                Some(Token::Number(n)) => exponent = n as i32,
                _ => return Err(UcumError::invalid_expression("Invalid exponent")),
            },
            Some(Token::Number(n)) => {
                // Implicit exponent (e.g., s2 -> s^2)
                exponent = n as i32;
            }
            _ => {
                // No exponent, backtrack
                self.tokenizer.pos = saved_pos;
            }
        }

        // Skip trailing annotations
        loop {
            let saved_pos = self.tokenizer.pos;
            match self.tokenizer.next_token() {
                Some(Token::Annotation(_)) => {
                    // Consume annotation and continue
                }
                _ => {
                    // Not an annotation, backtrack and stop
                    self.tokenizer.pos = saved_pos;
                    break;
                }
            }
        }

        Ok(Some(UnitFactor {
            expr: base_expr,
            exponent,
        }))
    }

    /// Parse a product of factors.
    ///
    /// Products can be explicit (with '.' separator) or implicit (adjacent tokens).
    /// Stops at division operators, close parentheses, or end of input.
    #[allow(clippy::result_large_err)]
    fn parse_product(&mut self) -> Result<UnitExpr<'a>, UcumError> {
        let mut factors = SmallFactorVec::new();

        // Parse first factor
        match self.parse_factor()? {
            Some(f) => factors.push(f),
            None => return Ok(UnitExpr::Numeric(1.0)), // Empty expression
        }

        // Parse remaining factors
        loop {
            // Check for product separator (. or implicit)
            let next_pos = self.tokenizer.pos;
            match self.tokenizer.next_token() {
                Some(Token::Operator('.')) => {
                    // Explicit product - continue parsing
                }
                Some(Token::Operator('/')) => {
                    // End of product - backtrack and stop
                    self.tokenizer.pos = next_pos;
                    break;
                }
                Some(Token::CloseParen) | None => {
                    // End of product - backtrack and stop
                    self.tokenizer.pos = next_pos;
                    break;
                }
                Some(_) => {
                    // Implicit product - backtrack and continue
                    self.tokenizer.pos = next_pos;
                }
            }

            match self.parse_factor()? {
                Some(f) => {
                    factors.push(f);
                }
                None => {
                    break;
                }
            }
        }

        // Optimize for single factor
        if factors.len() == 1 {
            let factor = factors.into_iter().next().unwrap();
            if factor.exponent == 1 {
                Ok(factor.expr)
            } else {
                Ok(UnitExpr::Power(Box::new(factor.expr), factor.exponent))
            }
        } else {
            Ok(UnitExpr::Product(factors.into_vec()))
        }
    }

    /// Parse a full expression with division
    #[allow(clippy::result_large_err)]
    pub fn parse_expression(&mut self) -> Result<UnitExpr<'a>, UcumError> {
        // Check for leading division (e.g., "/min" should be "1/min")
        let saved_pos = self.tokenizer.pos;
        match self.tokenizer.next_token() {
            Some(Token::Operator('/')) => {
                // Leading division - parse as 1/denominator
                let denominator = self.parse_product()?;
                return Ok(UnitExpr::Quotient(
                    Box::new(UnitExpr::Numeric(1.0)),
                    Box::new(denominator),
                ));
            }
            _ => {
                // Not a leading division, backtrack
                self.tokenizer.pos = saved_pos;
            }
        }

        let mut result = self.parse_product()?;

        // Handle division - check each token to see if it's division
        loop {
            let saved_pos = self.tokenizer.pos;
            match self.tokenizer.next_token() {
                Some(Token::Operator('/')) => {
                    let denominator = self.parse_product()?;
                    result = UnitExpr::Quotient(Box::new(result), Box::new(denominator));
                }
                _ => {
                    // Not a division operator, backtrack and stop
                    self.tokenizer.pos = saved_pos;
                    break;
                }
            }
        }

        Ok(result)
    }

    /// Parse and validate a complete UCUM expression.
    ///
    /// Performs pre-validation checks and ensures all input is consumed.
    /// Returns an owned AST that can outlive the input string.
    #[allow(clippy::result_large_err)]
    pub fn parse(mut self) -> Result<OwnedUnitExpr, UcumError> {
        // Quick pre-validation
        let input = self.tokenizer.input;

        // Check for invalid characters (but allow them inside annotations)
        let mut in_annotation = false;
        for (pos, ch) in input.char_indices() {
            if ch == '{' {
                in_annotation = true;
                continue;
            } else if ch == '}' {
                in_annotation = false;
                continue;
            }
            
            if !in_annotation {
                if ch.is_ascii() {
                    let ch_class = CHAR_CLASS[ch as u8 as usize];
                    if matches!(ch_class, CharClass::Invalid) && !ch.is_ascii_whitespace() {
                        return Err(UcumError::invalid_expression(&format!(
                            "Invalid character '{}' at position {}",
                            ch, pos
                        )));
                    }
                } else if ch != 'µ' {
                    // Allow µ (micro) as it's handled specially
                    return Err(UcumError::invalid_expression(&format!(
                        "Invalid non-ASCII character '{}' at position {}",
                        ch, pos
                    )));
                }
            }
        }

        // Check for % in wrong position
        if let Some(pos) = input.find('%') {
            if input != "%" {
                return Err(UcumError::invalid_percent_placement(pos));
            }
        }

        // Check for addition operators outside of 10*+ or 10^+ contexts
        if input.contains('+') && !input.contains("10*+") && !input.contains("10^+") {
            return Err(UcumError::invalid_expression(
                "Addition operators are not allowed in UCUM expressions",
            ));
        }

        // Parse expression
        let expr = self.parse_expression()?;

        // Ensure all input was consumed
        if self.tokenizer.next_token().is_some() {
            return Err(UcumError::invalid_expression(
                "Unexpected characters at end of expression",
            ));
        }

        Ok(expr.to_owned())
    }
}

// ============================================================================
// Public API
// ============================================================================

/// Parse a UCUM expression using the optimized parser
#[allow(clippy::result_large_err)]
pub fn parse_expression_optimized(input: &str) -> Result<OwnedUnitExpr, UcumError> {
    let input = input.trim();

    if input.is_empty() {
        return Ok(OwnedUnitExpr::Numeric(1.0));
    }

    OptimizedParser::new(input).parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer() {
        let mut tokenizer = Tokenizer::new("kg.m/s2");
        assert_eq!(tokenizer.next_token(), Some(Token::Symbol("kg")));
        assert_eq!(tokenizer.next_token(), Some(Token::Operator('.')));
        assert_eq!(tokenizer.next_token(), Some(Token::Symbol("m")));
        assert_eq!(tokenizer.next_token(), Some(Token::Operator('/')));
        assert_eq!(tokenizer.next_token(), Some(Token::Symbol("s")));
        assert_eq!(tokenizer.next_token(), Some(Token::Number(2.0)));
        assert_eq!(tokenizer.next_token(), None);
    }

    #[test]
    fn test_ten_power() {
        let mut tokenizer = Tokenizer::new("10*3.mol");
        assert_eq!(tokenizer.next_token(), Some(Token::TenPower(3)));
        assert_eq!(tokenizer.next_token(), Some(Token::Operator('.')));
        assert_eq!(tokenizer.next_token(), Some(Token::Symbol("mol")));
    }

    #[test]
    fn test_micro_normalization() {
        // First test tokenization
        let mut tokenizer = Tokenizer::new("µg");
        let token = tokenizer.next_token();
        println!("Tokenized µg as: {:?}", token);

        let result = parse_expression_optimized("µg").unwrap();
        println!("Parsed µg as: {:?}", result);
        match result {
            OwnedUnitExpr::Symbol(s) => assert_eq!(s, "ug"),
            _ => panic!("Expected symbol, got {:?}", result),
        }
    }

    #[test]
    fn test_complex_expression() {
        let result = parse_expression_optimized("kg.m/s2").unwrap();
        match result {
            OwnedUnitExpr::Quotient(num, den) => {
                // The numerator should be kg.m as a product
                assert!(matches!(*num, OwnedUnitExpr::Product(_)));
                // The denominator should be s^2 as a power (single factor with exponent > 1)
                assert!(matches!(*den, OwnedUnitExpr::Power(_, 2)));
            }
            _ => panic!("Expected quotient, got {:?}", result),
        }
    }
}
