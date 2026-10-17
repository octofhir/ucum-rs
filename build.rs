//! Generates the unit registry from `ucum-essence.xml`.
//!
//! Every unit is resolved from its definition down to the seven UCUM base units (m, s, g,
//! rad, K, C, cd) with exact decimal arithmetic, the same `Number` type the crate uses at
//! run time. Special units (UCUM §21-23) keep the factor and dimension of their
//! corresponding proper unit and the kind of their function; arbitrary units (§24-26) keep
//! the arbitrary atoms they reduce to.

extern crate alloc;

#[path = "src/number.rs"]
#[allow(dead_code)]
mod number;

use number::Number;
use quick_xml::Reader;
use quick_xml::events::Event;
use std::collections::HashMap;
use std::fmt::Write as _;
use std::{env, fs, path::PathBuf};

/// Dimension letters of the base units in the XML, in the order of `Dimension`.
const DIMENSIONS: [&str; 7] = ["L", "T", "M", "A", "C", "Q", "F"];

/// A `<prefix>`, `<base-unit>` or `<unit>` element.
#[derive(Default)]
struct Element {
    attributes: HashMap<String, String>,
    names: Vec<String>,
    print_symbol: String,
    property: String,
    value_attributes: HashMap<String, String>,
    function: HashMap<String, String>,
}

impl Element {
    fn attribute(&self, key: &str) -> &str {
        self.attributes.get(key).map_or("", String::as_str)
    }

    fn flag(&self, key: &str) -> bool {
        self.attribute(key) == "yes"
    }
}

fn attributes(e: &quick_xml::events::BytesStart) -> HashMap<String, String> {
    e.attributes()
        .map(|a| {
            let a = a.expect("attribute");
            let value = a
                .normalized_value(quick_xml::XmlVersion::Implicit1_0)
                .expect("attribute value")
                .into_owned();
            (String::from_utf8_lossy(a.key.as_ref()).into_owned(), value)
        })
        .collect()
}

/// Reads the children of an element up to its end tag. Text in nested markup, such as the
/// `<sup>` of a print symbol, belongs to the child it is in; character references
/// ("amp&#232;re") are resolved.
fn read_element(reader: &mut Reader<&[u8]>, start: &quick_xml::events::BytesStart) -> Element {
    let mut element = Element {
        attributes: attributes(start),
        ..Element::default()
    };
    let tag = start.name().as_ref().to_vec();
    let mut child: Option<String> = None;
    let mut depth = 0;
    let mut text = String::new();
    loop {
        match reader.read_event().expect("well-formed XML") {
            Event::Start(e) => {
                if depth == 0 {
                    let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                    if name == "value" {
                        element.value_attributes = attributes(&e);
                    }
                    child = Some(name);
                    text.clear();
                }
                depth += 1;
            }
            Event::Empty(e) => match e.name().as_ref() {
                // Inside <value>
                b"function" => element.function = attributes(&e),
                b"value" if depth == 0 => element.value_attributes = attributes(&e),
                _ => {}
            },
            Event::Text(t) => text.push_str(&t.decode().expect("text")),
            Event::GeneralRef(r) => {
                if let Some(ch) = r.resolve_char_ref().expect("character reference") {
                    text.push(ch);
                } else {
                    let name = String::from_utf8_lossy(&r).into_owned();
                    text.push_str(match name.as_str() {
                        "amp" => "&",
                        "lt" => "<",
                        "gt" => ">",
                        "quot" => "\"",
                        "apos" => "'",
                        other => panic!("unknown entity &{other};"),
                    });
                }
            }
            Event::End(e) => {
                if depth == 0 && e.name().as_ref() == tag.as_slice() {
                    return element;
                }
                depth -= 1;
                if depth == 0 {
                    let value = text.split_whitespace().collect::<Vec<_>>().join(" ");
                    match child.take().as_deref() {
                        Some("name") => element.names.push(value),
                        Some("printSymbol") => element.print_symbol = value,
                        Some("property") => element.property = value,
                        _ => {}
                    }
                }
            }
            Event::Eof => panic!("unterminated element"),
            _ => {}
        }
    }
}

/// A resolved unit: factor and dimension in base units, and arbitrary atoms.
#[derive(Clone)]
struct Resolved {
    factor: Number,
    dim: [i8; 7],
    arbitrary: Vec<(String, i32)>,
}

impl Resolved {
    fn one() -> Self {
        Self {
            factor: Number::ONE,
            dim: [0; 7],
            arbitrary: Vec::new(),
        }
    }

    fn mul(mut self, other: &Self, exp: i32) -> Self {
        let power = other.factor.checked_powi(exp).expect("factor in range");
        self.factor = self.factor.checked_mul(power).expect("factor in range");
        for (acc, d) in self.dim.iter_mut().zip(other.dim) {
            *acc = i8::try_from(i32::from(*acc) + i32::from(d) * exp).expect("dimension");
        }
        for (atom, e) in &other.arbitrary {
            match self.arbitrary.iter_mut().find(|(a, _)| a == atom) {
                Some(entry) => entry.1 += e * exp,
                None => self.arbitrary.push((atom.clone(), e * exp)),
            }
        }
        self.arbitrary.retain(|(_, e)| *e != 0);
        self.arbitrary.sort();
        self
    }
}

/// Evaluates unit definitions, which are UCUM terms: components joined by '.' and '/',
/// left to right, with an optional leading '/', exponents, prefixes and integers.
struct Resolver<'a> {
    base: &'a HashMap<String, usize>,
    units: &'a HashMap<String, &'a Element>,
    prefixes: &'a HashMap<String, Number>,
    cache: HashMap<String, Option<Resolved>>,
}

impl Resolver<'_> {
    /// A unit atom, or `None` for a special unit (which has no proper value).
    fn unit(&mut self, code: &str) -> Option<Resolved> {
        if let Some(resolved) = self.cache.get(code) {
            return resolved.clone();
        }
        self.cache.insert(code.to_string(), None);
        let resolved = self.compute(code);
        self.cache.insert(code.to_string(), resolved.clone());
        resolved
    }

    fn compute(&mut self, code: &str) -> Option<Resolved> {
        if let Some(&slot) = self.base.get(code) {
            let mut resolved = Resolved::one();
            resolved.dim[slot] = 1;
            return Some(resolved);
        }
        let unit = self
            .units
            .get(code)
            .unwrap_or_else(|| panic!("unit {code}"));
        if unit.flag("isSpecial") {
            return None;
        }
        if unit.flag("isArbitrary") {
            // An arbitrary unit is an atom of its own, unless it is defined as another
            // arbitrary unit: `[IU]` is 1 `[iU]`
            let term = &unit.value_attributes["Unit"];
            if term != "1" {
                return self.term(term);
            }
            let mut resolved = Resolved::one();
            resolved.arbitrary.push((code.to_string(), 1));
            return Some(resolved);
        }
        let value = Number::parse(&unit.value_attributes["value"])
            .unwrap_or_else(|| panic!("value of {code}"));
        let mut resolved = self.term(&unit.value_attributes["Unit"])?;
        resolved.factor = resolved.factor.checked_mul(value).expect("factor in range");
        Some(resolved)
    }

    /// A unit atom with an optional prefix ("kg", "mm[Hg]"); prefixes take metric atoms only.
    fn simple_unit(&mut self, symbol: &str) -> Option<Resolved> {
        if self.base.contains_key(symbol) || self.units.contains_key(symbol) {
            return self.unit(symbol);
        }
        for len in [1, 2] {
            let (Some(prefix), Some(atom)) = (symbol.get(..len), symbol.get(len..)) else {
                continue;
            };
            let metric = self.base.contains_key(atom)
                || self.units.get(atom).is_some_and(|u| u.flag("isMetric"));
            if let Some(&factor) = self.prefixes.get(prefix)
                && metric
            {
                let mut resolved = self.unit(atom)?;
                resolved.factor = resolved.factor.checked_mul(factor).expect("factor");
                return Some(resolved);
            }
        }
        panic!("unknown unit {symbol:?} in a definition")
    }

    /// An integer, or a simple unit with an exponent.
    fn component(&mut self, text: &str) -> Option<Resolved> {
        if text.bytes().all(|b| b.is_ascii_digit()) {
            let mut resolved = Resolved::one();
            resolved.factor = Number::parse(text).expect("integer");
            return Some(resolved);
        }
        let last = text
            .bytes()
            .rposition(|b| !b.is_ascii_digit())
            .expect("symbol");
        let (symbol, exp) = if last + 1 < text.len() {
            let end = if matches!(text.as_bytes()[last], b'-' | b'+') {
                last
            } else {
                last + 1
            };
            (&text[..end], text[end..].parse::<i32>().expect("exponent"))
        } else {
            (text, 1)
        };
        let unit = self.simple_unit(symbol)?;
        Some(Resolved::one().mul(&unit, exp))
    }

    fn term(&mut self, text: &str) -> Option<Resolved> {
        let mut resolved = Resolved::one();
        let (mut divide, mut rest) = match text.strip_prefix('/') {
            Some(rest) => (true, rest),
            None => (false, text),
        };
        loop {
            let mut depth = 0;
            let end = rest
                .bytes()
                .position(|b| {
                    match b {
                        b'[' => depth += 1,
                        b']' => depth -= 1,
                        _ => {}
                    }
                    depth == 0 && (b == b'.' || b == b'/')
                })
                .unwrap_or(rest.len());
            let component = self.component(&rest[..end])?;
            resolved = resolved.mul(&component, if divide { -1 } else { 1 });
            if end == rest.len() {
                return Some(resolved);
            }
            divide = rest.as_bytes()[end] == b'/';
            rest = &rest[end + 1..];
        }
    }
}

/// The `SpecialKind` of a special unit's function (UCUM §21, the `<function>` element).
fn special_kind(function: &str) -> &'static str {
    match function {
        "Cel" => "Celsius",
        "degF" => "Fahrenheit",
        "degRe" => "Reaumur",
        "lg" => "Lg",
        "lgTimes2" => "LgTimes2",
        "ln" => "Ln",
        "ld" => "Ld",
        "pH" => "PH",
        "hpX" => "HpX",
        "hpC" => "HpC",
        "hpM" => "HpM",
        "hpQ" => "HpQ",
        // `[p'diop]` and `%[slope]`: 100 times the tangent of an angle
        "tanTimes100" | "100tan" => "TanTimes100",
        "sqrt" => "Sqrt",
        other => panic!("unknown special function {other:?}"),
    }
}

/// The corresponding proper unit of a special unit, from its definition
/// `<value Unit="function(number term)">` (UCUM §23.1): "cel(1 K)", "2lg(2 10*-5.Pa)".
///
/// The definition is read rather than the `<function>` element, which disagrees with it and
/// with the specification for `%[slope]` (`deg` instead of `rad`). Any other disagreement
/// fails the build.
fn corresponding_unit(code: &str, unit: &Element) -> (String, String) {
    let definition = &unit.value_attributes["Unit"];
    let inner = definition
        .split_once('(')
        .and_then(|(_, rest)| rest.strip_suffix(')'))
        .unwrap_or_else(|| panic!("special definition {definition:?} of {code}"));
    let (value, term) = inner
        .split_once(' ')
        .unwrap_or_else(|| panic!("special definition {definition:?} of {code}"));
    let function_term = unit.function.get("Unit").map_or("", String::as_str);
    let function_value = unit.function.get("value").map_or("", String::as_str);
    if (function_value, function_term) != (value, term) && code != "%[slope]" {
        panic!("{code}: <function> {function_value} {function_term} disagrees with {definition}");
    }
    (value.to_string(), term.to_string())
}

fn number(n: Number) -> String {
    let (lo, mid, hi, negative, scale, exp) = n.to_parts();
    format!("Number::from_parts({lo}, {mid}, {hi}, {negative}, {scale}, {exp})")
}

fn escape(text: &str) -> String {
    text.escape_default().to_string()
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let xml_path = manifest_dir.join("ucum-essence.xml");
    println!("cargo:rerun-if-changed={}", xml_path.display());
    println!("cargo:rerun-if-changed=src/number.rs");
    let xml = fs::read_to_string(&xml_path).expect("read ucum-essence.xml");

    let mut root = HashMap::new();
    let mut prefixes = Vec::new();
    let mut base_units = Vec::new();
    let mut units = Vec::new();
    let mut reader = Reader::from_str(&xml);
    loop {
        match reader.read_event().expect("well-formed XML") {
            Event::Start(e) if e.name().as_ref() == b"root" => root = attributes(&e),
            Event::Start(e) => match e.name().as_ref() {
                b"prefix" => prefixes.push(read_element(&mut reader, &e)),
                b"base-unit" => base_units.push(read_element(&mut reader, &e)),
                b"unit" => units.push(read_element(&mut reader, &e)),
                _ => {}
            },
            Event::Eof => break,
            _ => {}
        }
    }

    let prefix_factors: HashMap<String, Number> = prefixes
        .iter()
        .map(|p| {
            let value = Number::parse(&p.value_attributes["value"]).expect("prefix value");
            (p.attribute("Code").to_string(), value)
        })
        .collect();
    let base: HashMap<String, usize> = base_units
        .iter()
        .map(|b| {
            let slot = DIMENSIONS
                .iter()
                .position(|d| *d == b.attribute("dim"))
                .unwrap_or_else(|| panic!("dimension {:?}", b.attribute("dim")));
            (b.attribute("Code").to_string(), slot)
        })
        .collect();
    let unit_map: HashMap<String, &Element> = units
        .iter()
        .map(|u| (u.attribute("Code").to_string(), u))
        .collect();
    let mut resolver = Resolver {
        base: &base,
        units: &unit_map,
        prefixes: &prefix_factors,
        cache: HashMap::new(),
    };

    // (code, record source)
    let mut records: Vec<(String, String, String)> = Vec::new();
    for b in &base_units {
        let code = b.attribute("Code");
        let resolved = resolver.unit(code).expect("base unit");
        records.push((
            code.to_string(),
            b.attribute("CODE").to_string(),
            record(b, &resolved, "None", true, true),
        ));
    }
    for u in &units {
        let code = u.attribute("Code");
        let (resolved, kind) = if u.flag("isSpecial") {
            let (value, term) = corresponding_unit(code, u);
            let mut resolved = resolver.term(&term).expect("proper corresponding unit");
            let value = Number::parse(&value).expect("special value");
            resolved.factor = resolved.factor.checked_mul(value).expect("factor");
            (resolved, special_kind(&u.function["name"]))
        } else {
            let resolved = resolver
                .unit(code)
                .unwrap_or_else(|| panic!("{code} is defined through a special unit"));
            let kind = if u.flag("isArbitrary") {
                "Arbitrary"
            } else {
                "None"
            };
            (resolved, kind)
        };
        records.push((
            code.to_string(),
            u.attribute("CODE").to_string(),
            record(u, &resolved, kind, false, u.flag("isMetric")),
        ));
    }
    records.sort_by(|a, b| a.0.cmp(&b.0));
    for pair in records.windows(2) {
        assert_ne!(pair[0].0, pair[1].0, "duplicate unit code");
    }

    let mut out = String::new();
    out.push_str("use crate::number::Number;\n");
    out.push_str("use crate::types::{Dimension, Prefix, SpecialKind, UnitRecord};\n\n");
    writeln!(
        out,
        "/// The version of the UCUM tables.\npub const UCUM_VERSION: &str = \"{}\";",
        root["version"]
    )
    .unwrap();
    writeln!(
        out,
        "/// The revision date of the UCUM tables.\npub const UCUM_REVISION_DATE: &str = \"{}\";\n",
        root["revision-date"]
    )
    .unwrap();

    let mut sorted_prefixes: Vec<&Element> = prefixes.iter().collect();
    sorted_prefixes.sort_by_key(|p| p.attribute("Code"));
    out.push_str("pub static PREFIXES: &[Prefix] = &[\n");
    for p in &sorted_prefixes {
        let code = p.attribute("Code");
        writeln!(
            out,
            "    Prefix {{ symbol: \"{}\", symbol_ci: \"{}\", factor: {}, display_name: \"{}\", print_symbol: \"{}\" }},",
            escape(code),
            escape(p.attribute("CODE")),
            number(prefix_factors[code]),
            escape(&p.names[0]),
            escape(&p.print_symbol),
        )
        .unwrap();
    }
    out.push_str("];\n\n");

    out.push_str("pub static UNITS: &[UnitRecord] = &[\n");
    for (_, _, record) in &records {
        writeln!(out, "    {record},").unwrap();
    }
    out.push_str("];\n\n");

    // Case insensitive codes (UCUM §3.4), upper case, with the index of their unit
    let mut ci: Vec<(String, usize)> = records
        .iter()
        .enumerate()
        .map(|(i, (_, ci, _))| (ci.to_ascii_uppercase(), i))
        .collect();
    ci.sort();
    out.push_str("pub static UNITS_CI: &[(&str, usize)] = &[\n");
    for (code, index) in &ci {
        writeln!(out, "    (\"{}\", {index}),", escape(code)).unwrap();
    }
    out.push_str("];\n\n");
    let mut prefixes_ci: Vec<(String, usize)> = sorted_prefixes
        .iter()
        .enumerate()
        .map(|(i, p)| (p.attribute("CODE").to_ascii_uppercase(), i))
        .collect();
    prefixes_ci.sort();
    out.push_str("pub static PREFIXES_CI: &[(&str, usize)] = &[\n");
    for (code, index) in &prefixes_ci {
        writeln!(out, "    (\"{}\", {index}),", escape(code)).unwrap();
    }
    out.push_str("];\n");

    let dest = PathBuf::from(env::var("OUT_DIR").unwrap()).join("registry.rs");
    fs::write(&dest, out).expect("write registry.rs");
    println!("cargo:rustc-env=UCUM_REGISTRY={}", dest.display());
}

fn record(unit: &Element, resolved: &Resolved, kind: &str, is_base: bool, metric: bool) -> String {
    let arbitrary: Vec<String> = resolved
        .arbitrary
        .iter()
        .map(|(atom, exp)| format!("(\"{}\", {exp})", escape(atom)))
        .collect();
    let d = resolved.dim;
    format!(
        "UnitRecord {{ code: \"{}\", code_ci: \"{}\", dim: Dimension([{}, {}, {}, {}, {}, {}, {}]), factor: {}, special: SpecialKind::{kind}, arbitrary: &[{}], is_base: {is_base}, is_metric: {metric}, class: \"{}\", property: \"{}\", display_name: \"{}\", print_symbol: \"{}\" }}",
        escape(unit.attribute("Code")),
        escape(unit.attribute("CODE")),
        d[0],
        d[1],
        d[2],
        d[3],
        d[4],
        d[5],
        d[6],
        number(resolved.factor),
        arbitrary.join(", "),
        escape(unit.attribute("class")),
        escape(&unit.property),
        escape(&unit.names[0]),
        escape(&unit.print_symbol),
    )
}
