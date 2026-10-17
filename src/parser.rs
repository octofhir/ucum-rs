//! Parser of UCUM expressions, following the syntax of UCUM §1-10 and its grammar:
//!
//! ```text
//! mainTerm    := '/' term | term
//! term        := component (('.' | '/') component)*
//! component   := simpleUnit exponent? annotation?
//!              | digits annotation?
//!              | annotation
//!              | '(' term ')'
//! exponent    := ('+' | '-')? digits
//! annotation  := '{' (characters 33-126 but '{' and '}')+ '}'
//! ```
//!
//! A leading solidus inverts the component right after it (UCUM §7.3). Expressions use the
//! characters 33-126 of ASCII only (§3.1): white space is an error, as §3 allows. A simple
//! unit is a run of the characters allowed in unit symbols (§3.2) and of square bracket
//! groups (§5); digits at its end are its exponent (`m2`). A run of digits alone is a
//! positive integer (§8), and is a symbol when another symbol character follows (`10*`).

use crate::ast::UnitExpr;
use crate::error::UcumError;
use crate::number::Number;
use crate::prelude::*;

/// Characters that cannot be part of a unit symbol outside square brackets (UCUM §3.2).
fn is_symbol_char(b: u8) -> bool {
    (33..=126).contains(&b) && !b"\"()+-./=[]{}".contains(&b)
}

struct Parser<'a> {
    input: &'a str,
    bytes: &'a [u8],
    pos: usize,
}

type Result<T> = core::result::Result<T, UcumError>;

impl<'a> Parser<'a> {
    fn error(&self, reason: &str) -> UcumError {
        UcumError::invalid_expression(&format!("{reason} at position {}", self.pos))
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    fn main_term(&mut self) -> Result<UnitExpr> {
        if self.peek() == Some(b'/') {
            self.pos += 1;
            let first = UnitExpr::Reciprocal(Box::new(self.component()?));
            self.term_from(first)
        } else {
            let first = self.component()?;
            self.term_from(first)
        }
    }

    fn term(&mut self) -> Result<UnitExpr> {
        let first = self.component()?;
        self.term_from(first)
    }

    /// The rest of a term after its first component, left to right (UCUM §7.4).
    fn term_from(&mut self, first: UnitExpr) -> Result<UnitExpr> {
        let mut terms = vec![first];
        loop {
            match self.peek() {
                Some(b'.') => {
                    self.pos += 1;
                    terms.push(self.component()?);
                }
                Some(b'/') => {
                    self.pos += 1;
                    let denominator = self.component()?;
                    let numerator = fold(core::mem::take(&mut terms));
                    terms.push(UnitExpr::Quotient(
                        Box::new(numerator),
                        Box::new(denominator),
                    ));
                }
                _ => return Ok(fold(terms)),
            }
        }
    }

    fn component(&mut self) -> Result<UnitExpr> {
        match self.peek() {
            None => Err(self.error("Expected a unit")),
            Some(b'(') => {
                self.pos += 1;
                let term = self.term()?;
                if self.peek() != Some(b')') {
                    return Err(self.error("Expected ')'"));
                }
                self.pos += 1;
                Ok(term)
            }
            Some(b'{') => Ok(UnitExpr::Annotation(self.annotation()?)),
            Some(b) if b.is_ascii_digit() && !self.digits_are_symbol() => {
                let start = self.pos;
                while self.peek().is_some_and(|b| b.is_ascii_digit()) {
                    self.pos += 1;
                }
                let value = Number::parse(&self.input[start..self.pos]).expect("digits");
                if value.is_zero() {
                    self.pos = start;
                    return Err(self.error("A number must be a positive integer"));
                }
                self.annotated(UnitExpr::Numeric(value))
            }
            Some(b) if is_symbol_char(b) || b == b'[' => {
                let unit = self.simple_unit()?;
                self.annotated(unit)
            }
            Some(b) => Err(self.error(&format!("Unexpected character '{}'", b as char))),
        }
    }

    /// Whether the digits at the position start a symbol: a symbol character follows them
    /// (`10*`, `12a`; UCUM §8.2).
    fn digits_are_symbol(&self) -> bool {
        let rest = &self.bytes[self.pos..];
        let end = rest
            .iter()
            .position(|b| !b.is_ascii_digit())
            .unwrap_or(rest.len());
        rest.get(end)
            .is_some_and(|&b| is_symbol_char(b) || b == b'[')
    }

    /// A simple unit with its exponent: "m", "m2", "s-1", "mm[Hg]", "10*-3".
    fn simple_unit(&mut self) -> Result<UnitExpr> {
        let start = self.pos;
        // End of the last square bracket group, so that digits inside one ("[H2O]") are
        // never read as an exponent
        let mut bracket_end = start;
        while let Some(b) = self.peek() {
            if b == b'[' {
                self.pos += 1;
                loop {
                    match self.peek() {
                        Some(b']') => break,
                        Some(b'[') => return Err(self.error("Square brackets cannot be nested")),
                        Some(33..=126) => self.pos += 1,
                        _ => return Err(self.error("Expected ']'")),
                    }
                }
                self.pos += 1;
                bracket_end = self.pos;
            } else if is_symbol_char(b) {
                self.pos += 1;
            } else {
                break;
            }
        }
        let text = &self.input[start..self.pos];
        let digits = text.bytes().rev().take_while(u8::is_ascii_digit).count();
        let symbol_end = (self.pos - digits).max(bracket_end);
        let symbol = &self.input[start..symbol_end];
        let unit = UnitExpr::Symbol(symbol.to_string());

        let exponent_start = symbol_end;
        if symbol_end < self.pos {
            // Unsigned exponent: "m2"
            if matches!(self.peek(), Some(b'+' | b'-')) {
                return Err(self.error("Unexpected sign after an exponent"));
            }
        } else if matches!(self.peek(), Some(b'+' | b'-')) {
            // Signed exponent: "m-2", "m+2"
            self.pos += 1;
            if !self.peek().is_some_and(|b| b.is_ascii_digit()) {
                return Err(self.error("Expected the digits of an exponent"));
            }
            while self.peek().is_some_and(|b| b.is_ascii_digit()) {
                self.pos += 1;
            }
        } else {
            return Ok(unit);
        }
        let exponent = self.input[exponent_start..self.pos]
            .parse::<i32>()
            .map_err(|_| self.error("Exponent out of range"))?;
        Ok(UnitExpr::Power(Box::new(unit), exponent))
    }

    /// `expr` followed by an optional annotation.
    fn annotated(&mut self, expr: UnitExpr) -> Result<UnitExpr> {
        if self.peek() == Some(b'{') {
            let text = self.annotation()?;
            Ok(UnitExpr::Annotated(Box::new(expr), text))
        } else {
            Ok(expr)
        }
    }

    /// An annotation, without its braces (UCUM §6).
    fn annotation(&mut self) -> Result<String> {
        self.pos += 1;
        let start = self.pos;
        loop {
            match self.peek() {
                Some(b'}') => break,
                Some(b'{') => return Err(self.error("Curly braces cannot be nested")),
                Some(33..=126) => self.pos += 1,
                _ => return Err(self.error("Expected '}'")),
            }
        }
        if self.pos == start {
            return Err(self.error("An annotation cannot be empty"));
        }
        let text = self.input[start..self.pos].to_string();
        self.pos += 1;
        Ok(text)
    }
}

/// One term from the terms joined by '.'.
fn fold(mut terms: Vec<UnitExpr>) -> UnitExpr {
    if terms.len() == 1 {
        terms.remove(0)
    } else {
        UnitExpr::Product(terms)
    }
}

/// Parse a UCUM expression into its syntax tree.
///
/// The syntax is checked; whether the symbols are units is checked by evaluation (see
/// `validate`).
pub fn parse_expression(input: &str) -> Result<UnitExpr> {
    if input.is_empty() {
        return Err(UcumError::invalid_expression(
            "Empty expression: the unity is written \"1\"",
        ));
    }
    if let Some((pos, ch)) = input
        .char_indices()
        .find(|&(_, ch)| !(ch.is_ascii_graphic()))
    {
        let what = if ch.is_whitespace() {
            "White space"
        } else {
            "A character outside ASCII 33-126"
        };
        return Err(UcumError::invalid_expression(&format!(
            "{what} ({ch:?}) is not allowed in UCUM expressions (UCUM §3.1), at position {pos}"
        )));
    }
    let mut parser = Parser {
        input,
        bytes: input.as_bytes(),
        pos: 0,
    };
    let expr = parser.main_term()?;
    if parser.pos < input.len() {
        let ch = input.as_bytes()[parser.pos] as char;
        return Err(parser.error(&format!("Unexpected character '{ch}'")));
    }
    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &str) -> UnitExpr {
        parse_expression(input).unwrap_or_else(|e| panic!("{input}: {e}"))
    }

    fn symbol(s: &str) -> UnitExpr {
        UnitExpr::Symbol(s.to_string())
    }

    fn power(s: &str, exp: i32) -> UnitExpr {
        UnitExpr::Power(Box::new(symbol(s)), exp)
    }

    #[test]
    fn reads_terms_left_to_right() {
        assert_eq!(
            parse("kg.m/s2"),
            UnitExpr::Quotient(
                Box::new(UnitExpr::Product(vec![symbol("kg"), symbol("m")])),
                Box::new(power("s", 2))
            )
        );
        assert_eq!(
            parse("m/s.kg"),
            UnitExpr::Product(vec![
                UnitExpr::Quotient(Box::new(symbol("m")), Box::new(symbol("s"))),
                symbol("kg")
            ])
        );
    }

    #[test]
    fn reads_exponents_and_ten_powers() {
        assert_eq!(parse("m2"), power("m", 2));
        assert_eq!(parse("s-1"), power("s", -1));
        assert_eq!(parse("m+2"), power("m", 2));
        assert_eq!(parse("10*3"), power("10*", 3));
        assert_eq!(parse("10*-3"), power("10*", -3));
        assert_eq!(parse("10^"), symbol("10^"));
        assert_eq!(parse("[H2O]"), symbol("[H2O]"));
        assert_eq!(parse("m[H2O]2"), power("m[H2O]", 2));
    }

    #[test]
    fn reads_numbers_and_annotations() {
        assert_eq!(
            parse("4.[pi]"),
            UnitExpr::Product(vec![UnitExpr::Numeric(Number::from_i64(4)), symbol("[pi]")])
        );
        assert_eq!(parse("{rbc}"), UnitExpr::Annotation("rbc".into()));
        assert_eq!(
            parse("rad2{a}"),
            UnitExpr::Annotated(Box::new(power("rad", 2)), "a".into())
        );
        assert_eq!(parse("/min"), UnitExpr::Reciprocal(Box::new(symbol("min"))));
    }

    #[test]
    fn rejects_invalid_syntax() {
        for input in [
            "", " m", "m s", "m/", "/", "m..s", "(m", "m)", "(/m)", "m.(/s)", "µg", "0", "0.m",
            "10+3", "m{a}{b}", "(m){a}", "{a{b}", "{}", "{a", "m2-1", "[[a]]", "[a", "m]", "m=s",
            "kg\"", "(m.s)2",
        ] {
            assert!(
                parse_expression(input).is_err(),
                "{input:?} should be invalid"
            );
        }
    }

    #[test]
    fn displays_as_written() {
        for input in [
            "kg.m/s2",
            "/min",
            "m/s.kg",
            "kg/(m.s)",
            "10*-3",
            "4.[pi]",
            "mg{total}",
            "{rbc}",
            "1{c}",
            "m[H2O]2",
        ] {
            assert_eq!(parse(input).to_string(), input);
        }
    }
}
