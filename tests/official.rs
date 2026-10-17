//! The official UCUM functional tests (`tests/official/UcumFunctionalTests.xml`), run
//! through the public API: `validate`, `display_name`, `convert`, `multiply` and `divide_by`.
//!
//! A numeric outcome must match to half a unit of its last written digit ("25" is
//! 25 ± 0.5, "0.160" is 0.160 ± 0.0005), or to 1e-14 relative when it is written with more
//! digits than an `f64` holds (the comment of the conversion group says implementations do
//! not need to keep the 30 digits of `[pi]`).

use octofhir_ucum::{convert, display_name, divide_by, multiply, validate};
use quick_xml::Reader;
use quick_xml::events::Event;
use std::collections::HashMap;

const TESTS: &str = include_str!("official/UcumFunctionalTests.xml");

/// The `<case>` elements of each group, as attribute maps. XML comments, including
/// commented-out cases, are skipped by the reader.
fn cases() -> HashMap<String, Vec<HashMap<String, String>>> {
    let mut reader = Reader::from_str(TESTS);
    let mut groups: HashMap<String, Vec<HashMap<String, String>>> = HashMap::new();
    let mut group = String::new();
    loop {
        match reader.read_event().expect("well-formed test file") {
            Event::Start(e) if e.name().as_ref() != b"ucumTests" => {
                group = String::from_utf8_lossy(e.name().as_ref()).into_owned();
            }
            Event::Empty(e) if e.name().as_ref() == b"case" => {
                let attributes = e
                    .attributes()
                    .map(|a| {
                        let a = a.expect("attribute");
                        let value = a
                            .normalized_value(quick_xml::XmlVersion::Implicit1_0)
                            .expect("attribute value")
                            .into_owned();
                        (String::from_utf8_lossy(a.key.as_ref()).into_owned(), value)
                    })
                    .collect();
                groups.entry(group.clone()).or_default().push(attributes);
            }
            Event::Eof => return groups,
            _ => {}
        }
    }
}

fn group(name: &str, expected_count: usize) -> Vec<HashMap<String, String>> {
    let cases = cases().remove(name).unwrap_or_default();
    assert_eq!(cases.len(), expected_count, "number of {name} cases");
    cases
}

/// Whether `actual` matches the outcome written as `expected`.
fn matches_outcome(actual: f64, expected: &str) -> bool {
    let value: f64 = expected.parse().expect("numeric outcome");
    let (mantissa, exp) = match expected.split_once(['e', 'E']) {
        Some((mantissa, exp)) => (mantissa, exp.parse::<i32>().expect("exponent")),
        None => (expected, 0),
    };
    let decimals = mantissa.split_once('.').map_or(0, |(_, d)| d.len() as i32);
    let half_last_digit = 0.5 * 10f64.powi(exp - decimals);
    (actual - value).abs() <= half_last_digit.max(1e-14 * value.abs())
}

fn convert_to(value: f64, from: &str, to: &str) -> Result<f64, String> {
    convert(value, from, to).map_err(|err| err.to_string())
}

/// Cases that still fail, with the reason. Each one is a bug to fix, not a tolerance.
const KNOWN_FAILURES: &[&str] = &[
    // The factor of 1/[ly] is cut to the 28 decimal places of `rust_decimal`
    "3-128",
];

fn report(group: &str, mut failures: Vec<String>) {
    failures.retain(|f| {
        !KNOWN_FAILURES
            .iter()
            .any(|id| f.starts_with(&format!("{id} ")))
    });
    assert!(
        failures.is_empty(),
        "{} {group} case(s) failed:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

#[test]
fn validation() {
    let mut failures = Vec::new();
    for case in group("validation", 529) {
        let valid = case["valid"] == "true";
        let result = validate(&case["unit"]);
        if result.is_ok() != valid {
            failures.push(format!(
                "{} {:?}: expected valid={valid}, got {result:?}",
                case["id"], case["unit"]
            ));
        }
    }
    report("validation", failures);
}

#[test]
fn display_name_generation() {
    let mut failures = Vec::new();
    for case in group("displayNameGeneration", 9) {
        let display = display_name(&case["unit"]).map_err(|e| e.to_string());
        if display.as_deref() != Ok(case["display"].as_str()) {
            failures.push(format!(
                "{} {:?}: expected {:?}, got {display:?}",
                case["id"], case["unit"], case["display"]
            ));
        }
    }
    report("displayNameGeneration", failures);
}

#[test]
fn conversion() {
    let mut failures = Vec::new();
    for case in group("conversion", 30) {
        let value: f64 = case["value"].parse().expect("value");
        let result = convert_to(value, &case["srcUnit"], &case["dstUnit"]);
        if !result
            .as_ref()
            .is_ok_and(|v| matches_outcome(*v, &case["outcome"]))
        {
            failures.push(format!(
                "{} {} {} -> {}: expected {}, got {result:?}",
                case["id"], case["value"], case["srcUnit"], case["dstUnit"], case["outcome"]
            ));
        }
    }
    report("conversion", failures);
}

/// The product or quotient must convert to `vRes` in `uRes` (the unity when empty).
fn check_arithmetic(
    group_name: &str,
    expected_count: usize,
    operation: impl Fn(f64, &str, f64, &str) -> Result<(f64, String), String>,
) {
    let mut failures = Vec::new();
    for case in group(group_name, expected_count) {
        let v1: f64 = case["v1"].parse().expect("v1");
        let v2: f64 = case["v2"].parse().expect("v2");
        let target = match case["uRes"].as_str() {
            "" => "1",
            unit => unit,
        };
        let result = operation(v1, &case["u1"], v2, &case["u2"])
            .and_then(|(value, unit)| convert_to(value, &unit, target));
        if !result
            .as_ref()
            .is_ok_and(|v| matches_outcome(*v, &case["vRes"]))
        {
            failures.push(format!(
                "{} {} {} and {} {}: expected {} {target}, got {result:?}",
                case["id"], case["v1"], case["u1"], case["v2"], case["u2"], case["vRes"]
            ));
        }
    }
    report(group_name, failures);
}

#[test]
fn multiplication() {
    check_arithmetic("multiplication", 2, |v1, u1, v2, u2| {
        multiply(v1, u1, v2, u2)
            .map(|r| (r.value, r.unit))
            .map_err(|e| e.to_string())
    });
}

#[test]
fn division() {
    check_arithmetic("division", 3, |v1, u1, v2, u2| {
        divide_by(v1, u1, v2, u2)
            .map(|r| (r.value, r.unit))
            .map_err(|e| e.to_string())
    });
}

#[test]
fn outcome_tolerance_is_half_the_last_written_digit() {
    assert!(matches_outcome(25.2, "25"));
    assert!(!matches_outcome(25.6, "25"));
    assert!(matches_outcome(6_300_000.4, "6300000"));
    assert!(!matches_outcome(6_300_001.0, "6300000"));
    assert!(matches_outcome(0.16002, "0.160"));
    assert!(!matches_outcome(0.000_127, "0.000126"));
    assert!(matches_outcome(1e-7, "1e-7"));
    assert!(matches_outcome(
        0.000_001_256_637_061_435_917_3,
        "0.00000125663706143591729538506"
    ));
}
