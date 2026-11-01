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

/// How many levels an expression can nest, along any path of its syntax tree: each pair of
/// parentheses is a level, each solidus, which nests the term before it (`a/b/c` is
/// `(a/b)/c`, UCUM §7.4), and each product of several terms. `m/m/m` has 2 levels, `((m))` 2,
/// `m/(m/m)` 3, `m.m/s` 2. The parser and the evaluator recurse once per level, so a deeper
/// expression would overflow the stack (1 MiB in WebAssembly) and abort the process; it is
/// a `NestingTooDeep` instead.
pub const MAX_NESTING_DEPTH: usize = 128;

struct Parser<'a> {
    input: &'a str,
    bytes: &'a [u8],
    pos: usize,
    /// Levels above the component being read: its parentheses, and the quotients and
    /// products it is a term of. The parser recurses through them, so it never goes deeper
    /// than `MAX_NESTING_DEPTH` either.
    depth: usize,
    /// A symbol to look for, and the offset where it is first read, once found
    wanted: Option<&'a str>,
    found: Option<usize>,
}

type Result<T> = core::result::Result<T, UcumError>;

impl<'a> Parser<'a> {
    fn error(&self, reason: &str) -> UcumError {
        UcumError::invalid_expression_in(self.input, reason, Some(self.pos))
    }

    /// `expr` and its levels of nesting, once checked against `MAX_NESTING_DEPTH` with the
    /// levels above it.
    fn checked(&self, expr: UnitExpr, height: usize) -> Result<(UnitExpr, usize)> {
        if self.depth + height > MAX_NESTING_DEPTH {
            return Err(self.too_deep());
        }
        Ok((expr, height))
    }

    fn too_deep(&self) -> UcumError {
        UcumError::nesting_too_deep(MAX_NESTING_DEPTH)
    }

    /// A component one level below the current one: a term of a quotient or a product.
    fn component_below(&mut self) -> Result<(UnitExpr, usize)> {
        self.depth += 1;
        let component = self.component();
        self.depth -= 1;
        component
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    fn main_term(&mut self) -> Result<UnitExpr> {
        let first = if self.peek() == Some(b'/') {
            self.pos += 1;
            let (component, height) = self.component_below()?;
            self.checked(UnitExpr::Reciprocal(Box::new(component)), height + 1)?
        } else {
            self.component()?
        };
        Ok(self.term_from(first)?.0)
    }

    fn term(&mut self) -> Result<(UnitExpr, usize)> {
        let first = self.component()?;
        self.term_from(first)
    }

    /// The rest of a term after its first component, left to right (UCUM §7.4), with its
    /// levels of nesting.
    fn term_from(&mut self, (first, first_height): (UnitExpr, usize)) -> Result<(UnitExpr, usize)> {
        let mut terms = vec![first];
        let mut height = first_height;
        loop {
            match self.peek() {
                Some(b'.') => {
                    self.pos += 1;
                    let (component, component_height) = self.component_below()?;
                    terms.push(component);
                    height = height.max(component_height);
                }
                Some(b'/') => {
                    self.pos += 1;
                    let (denominator, denominator_height) = self.component_below()?;
                    let numerator_height = height + usize::from(terms.len() > 1);
                    let numerator = fold(core::mem::take(&mut terms));
                    let (quotient, quotient_height) = self.checked(
                        UnitExpr::Quotient(Box::new(numerator), Box::new(denominator)),
                        1 + numerator_height.max(denominator_height),
                    )?;
                    terms.push(quotient);
                    height = quotient_height;
                }
                _ => {
                    let height = height + usize::from(terms.len() > 1);
                    return self.checked(fold(terms), height);
                }
            }
        }
    }

    /// A component, with its levels of nesting.
    fn component(&mut self) -> Result<(UnitExpr, usize)> {
        match self.peek() {
            None => Err(self.error("Expected a unit")),
            Some(b'(') => {
                if self.depth >= MAX_NESTING_DEPTH {
                    return Err(self.too_deep());
                }
                self.pos += 1;
                self.depth += 1;
                let term = self.term();
                self.depth -= 1;
                let (term, height) = term?;
                if self.peek() != Some(b')') {
                    return Err(self.error("Expected ')'"));
                }
                self.pos += 1;
                self.checked(term, height + 1)
            }
            Some(b'{') => Ok((UnitExpr::Annotation(self.annotation()?), 0)),
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
                Ok((self.annotated(UnitExpr::Numeric(value))?, 0))
            }
            Some(b) if is_symbol_char(b) || b == b'[' => {
                let unit = self.simple_unit()?;
                Ok((self.annotated(unit)?, 0))
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
        if self.found.is_none() && self.wanted == Some(symbol) {
            self.found = Some(start);
        }
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
/// `validate`). An expression nested deeper than [`MAX_NESTING_DEPTH`] levels (parentheses,
/// solidi and products) is a `NestingTooDeep`: the parser and the evaluator recurse once per
/// level, and would otherwise overflow the stack. Other syntax errors are an
/// `InvalidExpression` with the byte offset where the parser stopped.
pub fn parse_expression(input: &str) -> Result<UnitExpr> {
    parse(input, None).map(|(expr, _)| expr)
}

/// The byte offset in `input` where the symbol `symbol` is first written, if `input`
/// parses: a whole symbol, never part of an annotation or of a longer symbol.
pub(crate) fn symbol_offset(input: &str, symbol: &str) -> Option<usize> {
    parse(input, Some(symbol)).ok()?.1
}

/// `parse_expression`, with the offset where `wanted`, if given, is first read as a symbol
fn parse<'a>(input: &'a str, wanted: Option<&'a str>) -> Result<(UnitExpr, Option<usize>)> {
    if input.is_empty() {
        return Err(UcumError::invalid_expression_in(
            input,
            "Empty expression: the unity is written \"1\"",
            None,
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
        return Err(UcumError::invalid_expression_in(
            input,
            &format!("{what} ({ch:?}) is not allowed in UCUM expressions (UCUM §3.1)"),
            Some(pos),
        ));
    }
    let mut parser = Parser {
        input,
        bytes: input.as_bytes(),
        pos: 0,
        depth: 0,
        wanted,
        found: None,
    };
    let expr = parser.main_term()?;
    if parser.pos < input.len() {
        let ch = input.as_bytes()[parser.pos] as char;
        return Err(parser.error(&format!("Unexpected character '{ch}'")));
    }
    Ok((expr, parser.found))
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
