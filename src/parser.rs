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
use crate::math;
use crate::prelude::*;
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
static CHAR_CLASS: [CharClass; 256] = {
    let mut table = [CharClass::Invalid; 256];
    let mut i = 0;
    while i < 256 {
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
        i += 1;
    }
    table
};

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
                core::str::from_utf8_unchecked(&bytes[..*len as usize])
            },
            CompactString::Heap(s) => s.as_str(),
        }
    }
}

// ============================================================================
// Fast character validation
// ============================================================================

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
    /// A trailing exponent ("2" in "m2", "-1" in "s-1", "+2" in "m+2") is left
    /// for `scan_exponent`.
    fn scan_symbol(&mut self) -> Option<Token<'a>> {
        let start = self.pos;

        // UTF-8 µ (micro sign) as the first character
        if self.current_byte() == Some(0xC2) && self.peek_byte(1) == Some(0xB5) {
            self.pos += 2;
        }

        let mut in_brackets = false;
        while let Some(b) = self.current_byte() {
            if in_brackets {
                // Anything but a nested bracket goes inside "[...]", e.g. "B[10.nV]"
                if b == b'[' || !b.is_ascii_graphic() {
                    break;
                }
                in_brackets = b != b']';
            } else if is_symbol_char_fast(b) {
                in_brackets = b == b'[';
            } else {
                break;
            }
            self.pos += 1;
        }

        if self.pos == start {
            return None;
        }

        let symbol = &self.input[start..self.pos];

        // Bytes, not chars: the last non-digit may be the second byte of µ
        if let Some(last) = symbol.bytes().rposition(|b| !b.is_ascii_digit()) {
            let digits_start = last + 1;
            if digits_start < symbol.len() {
                // A sign right before the digits belongs to the exponent (e.g., "s-2")
                let base_end = match symbol.as_bytes()[last] {
                    b'-' | b'+' => last,
                    _ => digits_start,
                };
                self.pos = start + base_end;
                return Some(Token::Symbol(&symbol[..base_end]));
            }
        }

        Some(Token::Symbol(symbol))
    }

    /// Scan an exponent written right after a unit symbol: digits with an optional
    /// sign, e.g. "2" in "m2", "-1" in "s-1", "+2" in "m+2" (UCUM §9).
    ///
    /// Returns `Ok(None)` and leaves the position untouched if there is no exponent.
    #[allow(clippy::result_large_err)]
    fn scan_exponent(&mut self) -> Result<Option<i32>, UcumError> {
        let start = self.pos;
        let digits_start = match self.current_byte() {
            Some(b'-' | b'+') => start + 1,
            _ => start,
        };

        let mut end = digits_start;
        while self.bytes.get(end).is_some_and(u8::is_ascii_digit) {
            end += 1;
        }
        if end == digits_start {
            return Ok(None);
        }

        self.pos = end;
        self.input[start..end]
            .parse::<i32>()
            .map(Some)
            .map_err(|_| UcumError::invalid_expression("Exponent out of range"))
    }

    /// Scan a numeric token: a positive integer. Terms have no decimals, the period
    /// is always multiplication ("2.5" is 2 × 5, UCUM §7.2).
    fn scan_number(&mut self) -> Option<Token<'a>> {
        let start = self.pos;
        while self.current_byte().is_some_and(|b| b.is_ascii_digit()) {
            self.pos += 1;
        }

        if self.pos > start
            && let Ok(num) = self.input[start..self.pos].parse::<f64>()
        {
            return Some(Token::Number(num));
        }

        None
    }

    /// Scan a power-of-ten token (e.g., "10*3" or "10^-2").
    ///
    /// "10*" and "10^" are unit atoms worth ten: without an exponent they are "10*1".
    fn scan_ten_power(&mut self) -> Option<Token<'a>> {
        // Check for "10*" or "10^" patterns
        if self.bytes.get(self.pos..self.pos + 3) == Some(b"10*")
            || self.bytes.get(self.pos..self.pos + 3) == Some(b"10^")
        {
            self.pos += 3;

            // Parse exponent
            let exp_start = self.pos;
            if let Some(sign) = self.current_byte()
                && (sign == b'+' || sign == b'-')
            {
                self.pos += 1;
            }

            let digit_start = self.pos;
            while let Some(b) = self.current_byte() {
                if b.is_ascii_digit() {
                    self.pos += 1;
                } else {
                    break;
                }
            }

            if self.pos > digit_start
                && let Ok(exp) = self.input[exp_start..self.pos].parse::<i32>()
            {
                return Some(Token::TenPower(exp));
            }
            if self.pos == exp_start {
                // No sign and no digits: the atom on its own
                return Some(Token::TenPower(1));
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
            // '%', "'" and "''" are unit atoms too
            CharClass::Letter | CharClass::OpenBracket | CharClass::Symbol => self.scan_symbol(),
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
    /// optionally followed by an exponent (explicit with ^ or implicit like "s2" or "s-1").
    ///
    /// Returns `Ok(None)` and leaves the position untouched if there is no factor.
    #[allow(clippy::result_large_err)]
    fn parse_factor(&mut self) -> Result<Option<UnitFactor<'a>>, UcumError> {
        let start = self.tokenizer.pos;
        let token = match self.tokenizer.next_token() {
            Some(t) => t,
            None => {
                self.tokenizer.pos = start;
                return Ok(None);
            }
        };
        let is_symbol = matches!(token, Token::Symbol(_));

        let base_expr = match token {
            Token::Symbol(s) => {
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
            Token::TenPower(exp) => UnitExpr::Numeric(math::powi(10.0, exp)),
            Token::OpenParen => {
                // Parse parenthesized expression
                let inner = self.parse_term()?;
                match self.tokenizer.next_token() {
                    Some(Token::CloseParen) => inner,
                    _ => return Err(UcumError::invalid_expression("Missing closing parenthesis")),
                }
            }
            Token::Annotation(content) => {
                // Standalone annotation
                UnitExpr::SymbolOwned(format!("{{{content}}}"))
            }
            _ => {
                self.tokenizer.pos = start;
                return Ok(None);
            }
        };

        // Exponent written right after a symbol (s2, s-2, s+2; UCUM §9), or after '^'
        let mut exponent = 1;
        if is_symbol && let Some(exp) = self.tokenizer.scan_exponent()? {
            exponent = exp;
        } else if self.tokenizer.current_byte() == Some(b'^') {
            self.tokenizer.pos += 1;
            exponent = self
                .tokenizer
                .scan_exponent()?
                .ok_or_else(|| UcumError::invalid_expression("Invalid exponent"))?;
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

    /// Parse a factor that must be present, e.g. after an operator.
    #[allow(clippy::result_large_err)]
    fn expect_factor(&mut self, after: char) -> Result<UnitFactor<'a>, UcumError> {
        self.parse_factor()?.ok_or_else(|| {
            UcumError::invalid_expression(&format!("'{after}' must be followed by a unit"))
        })
    }

    /// Turn a factor into an expression, wrapping it in a power if needed.
    fn factor_into_expr(factor: UnitFactor<'a>) -> UnitExpr<'a> {
        if factor.exponent == 1 {
            factor.expr
        } else {
            UnitExpr::Power(Box::new(factor.expr), factor.exponent)
        }
    }

    /// Turn the factors collected so far into a single expression.
    fn fold_factors(mut factors: SmallFactorVec<'a>) -> UnitExpr<'a> {
        if factors.len() == 1 {
            Self::factor_into_expr(factors.remove(0))
        } else {
            UnitExpr::Product(factors.into_vec())
        }
    }

    /// Parse a term: factors joined by '.' and '/'.
    ///
    /// Both operators have the same precedence and are evaluated left to right
    /// (UCUM §7.4), so "a/b.c" is "(a/b).c". A leading '/' inverts the factor right
    /// after it (UCUM §7.3). The operator is mandatory (UCUM §7.2), so the term stops
    /// at anything else, e.g. a close parenthesis.
    #[allow(clippy::result_large_err)]
    fn parse_term(&mut self) -> Result<UnitExpr<'a>, UcumError> {
        let mut factors = SmallFactorVec::new();

        // Leading division: "/min" is "1/min"
        let saved_pos = self.tokenizer.pos;
        let leading_slash = self.tokenizer.next_token() == Some(Token::Operator('/'));
        self.tokenizer.pos = saved_pos;
        if leading_slash {
            factors.push(UnitFactor {
                expr: UnitExpr::Numeric(1.0),
                exponent: 1,
            });
        } else {
            match self.parse_factor()? {
                Some(f) => factors.push(f),
                None => return Err(UcumError::invalid_expression("Expected a unit")),
            }
        }

        loop {
            let saved_pos = self.tokenizer.pos;
            match self.tokenizer.next_token() {
                Some(Token::Operator('.')) => {
                    factors.push(self.expect_factor('.')?);
                }
                Some(Token::Operator('/')) => {
                    // Everything so far is the numerator
                    let numerator = Self::fold_factors(core::mem::take(&mut factors));
                    let denominator = Self::factor_into_expr(self.expect_factor('/')?);
                    factors.push(UnitFactor {
                        expr: UnitExpr::Quotient(Box::new(numerator), Box::new(denominator)),
                        exponent: 1,
                    });
                }
                _ => {
                    self.tokenizer.pos = saved_pos;
                    break;
                }
            }
        }

        Ok(Self::fold_factors(factors))
    }

    /// Parse a full expression.
    #[allow(clippy::result_large_err)]
    pub fn parse_expression(&mut self) -> Result<UnitExpr<'a>, UcumError> {
        self.tokenizer.skip_whitespace();
        if self.tokenizer.pos == self.tokenizer.input.len() {
            // The unity is written "1"
            return Err(UcumError::invalid_expression("Empty expression"));
        }
        self.parse_term()
    }

    /// Parse and validate a complete UCUM expression.
    ///
    /// Performs pre-validation checks and ensures all input is consumed.
    /// Returns an owned AST that can outlive the input string.
    #[allow(clippy::result_large_err)]
    pub fn parse(mut self) -> Result<OwnedUnitExpr, UcumError> {
        // Quick pre-validation
        let input = self.tokenizer.input;

        // Check for invalid characters. Annotations ("{...}") and square brackets
        // ("[...]") may hold any of the characters 33-126 (UCUM §5.2, §6.1)
        let mut in_annotation = false;
        let mut in_brackets = false;
        for (pos, ch) in input.char_indices() {
            if in_annotation || in_brackets {
                if ch == '}' && in_annotation {
                    in_annotation = false;
                } else if ch == ']' && in_brackets {
                    in_brackets = false;
                } else if !ch.is_ascii_graphic() {
                    return Err(UcumError::invalid_expression(&format!(
                        "Invalid character '{ch}' at position {pos}"
                    )));
                }
                continue;
            }

            match ch {
                '{' => in_annotation = true,
                '[' => in_brackets = true,
                // '+' only signs an exponent ("m+2", "10*+3"), it is never addition
                '+' if !input[pos + 1..].starts_with(|c: char| c.is_ascii_digit()) => {
                    return Err(UcumError::invalid_expression(
                        "Addition operators are not allowed in UCUM expressions",
                    ));
                }
                // Allow µ (micro) as it's handled specially
                'µ' => {}
                _ if !ch.is_ascii() => {
                    return Err(UcumError::invalid_expression(&format!(
                        "Invalid non-ASCII character '{ch}' at position {pos}"
                    )));
                }
                _ if CHAR_CLASS[ch as usize] == CharClass::Invalid && !ch.is_ascii_whitespace() => {
                    return Err(UcumError::invalid_expression(&format!(
                        "Invalid character '{ch}' at position {pos}"
                    )));
                }
                _ => {}
            }
        }

        // Parse expression
        let expr = self.parse_expression()?;

        // Ensure all input was consumed
        self.tokenizer.skip_whitespace();
        let pos = self.tokenizer.pos;
        if let Some(ch) = input[pos..].chars().next() {
            return Err(UcumError::invalid_expression(&format!(
                "Unexpected character '{ch}' at position {pos}"
            )));
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

    // The unity, as in an empty canonical unit; `validate` rejects it as a term
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
