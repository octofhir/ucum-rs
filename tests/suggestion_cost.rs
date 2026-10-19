//! `validate` suggests corrections for an invalid expression by comparing it with the code
//! and the name of every unit, and `validate_in_property` an unknown property with every
//! property. The comparison is a Levenshtein distance, so its cost grew with the length of
//! the input: about 4 µs per character in a release build, 200 ms for 50 KB and seconds for
//! a megabyte, for an input that `analyse` rejects in microseconds. A string much longer than
//! a unit code cannot be similar to it, and is no longer compared.

use octofhir_ucum::{validate, validate_in_property};
use std::time::{Duration, Instant};

fn time_validate(expression: &str) -> Duration {
    let start = Instant::now();
    assert!(validate(expression).is_err());
    start.elapsed()
}

#[test]
fn a_long_invalid_expression_is_rejected_quickly() {
    // Warm the suggestion engine and the registry up
    let _ = validate("metr");
    for expression in [
        // Unknown unit, invalid character, syntax error
        "x".repeat(200_000),
        "m".repeat(200_000) + "\u{7f}",
        "[".repeat(200_000),
        format!("{}{}", "m.".repeat(100_000), "qq"),
    ] {
        // These took about 8 s each in a debug build, 0.8 s in a release one
        let elapsed = time_validate(&expression);
        assert!(
            elapsed < Duration::from_secs(1),
            "{}...: {elapsed:?}",
            &expression[..10]
        );
    }
}

#[test]
fn a_long_unknown_property_is_rejected_quickly() {
    let _ = validate_in_property("m", "lenght");
    let start = Instant::now();
    // This took 9.6 s in a debug build, 0.47 s in a release one
    assert!(validate_in_property("m", &"x".repeat(200_000)).is_err());
    let elapsed = start.elapsed();
    assert!(elapsed < Duration::from_secs(1), "{elapsed:?}");
}

#[test]
fn property_suggestions_are_unchanged() {
    for (property, expected) in [
        ("lenght", &["'length'"][..]),
        ("temprature", &["'temperature'"]),
        ("volum", &["'volume'"]),
        ("presure", &["'pressure'"]),
        ("xyzzy", &[]),
        ("len", &[]),
    ] {
        let err = validate_in_property("m", property).expect_err(property);
        assert_eq!(err.suggestions, expected, "{property}");
    }
}

#[test]
fn suggestions_for_short_inputs_are_unchanged() {
    // The suggestions of 0.7.0, unchanged
    let cases: &[(&str, &[&str])] = &[
        ("metr", &["Did you mean 'm'? (meter) [similarity: 80.0%]"]),
        (
            "pa",
            &[
                "Did you mean 'Pa'? (case mismatch)",
                "Did you mean 'Pa'? (capitalization)",
            ],
        ),
        (
            "kgg",
            &["Did you mean a prefixed version of 'g'? (e.g., 'kg', 'mg', 'ug')"],
        ),
        (
            "[degf]",
            &[
                "Did you mean '[degF]'? (case mismatch)",
                "Did you mean '[degF]'? (degree Fahrenheit) [similarity: 83.3%]",
                "Did you mean '[degR]'? (degree Rankine) [similarity: 83.3%]",
                "Did you mean '[degRe]'? (degree Réaumur) [similarity: 71.4%]",
                "Did you mean '[den]'? (Denier) [similarity: 66.7%]",
                "Did you mean '[dye'U]'? (Dye unit) [similarity: 57.1%]",
                "Did you mean '[degF]'?",
            ],
        ),
        (
            "meters",
            &[
                "Did you mean 'm'? (meter) [similarity: 83.3%]",
                "Did you mean 'm'?",
            ],
        ),
        (
            "Pascal",
            &[
                "Did you mean 'Pa'? (pascal) [similarity: 100.0%]",
                "Did you mean a prefixed version of 'cal'? (e.g., 'kcal', 'mcal', 'ucal')",
                "Did you mean 'Pa'?",
            ],
        ),
        ("celsius", &["Did you mean 'Cel'?"]),
        (
            "[in_x]",
            &[
                "Did you mean '[in_i]'? (inch) [similarity: 83.3%]",
                "Did you mean '[cin_i]'? (cubic inch) [similarity: 71.4%]",
                "Did you mean '[in_br]'? (inch) [similarity: 71.4%]",
                "Did you mean '[in_us]'? (inch) [similarity: 71.4%]",
                "Did you mean '[sin_i]'? (square inch) [similarity: 71.4%]",
            ],
        ),
        (
            "[IU",
            &[
                "Did you mean '[IU]'? (international unit) [similarity: 75.0%]",
                "Did you mean a prefixed version of 'U'? (e.g., 'kU', 'mU', 'uU')",
                "Missing closing bracket: '[IU]'",
            ],
        ),
        (
            "IU]",
            &[
                "Did you mean '[IU]'? (international unit) [similarity: 75.0%]",
                "Did you mean '[IU]' (international unit)?",
                "Missing opening bracket: '[IU]'",
            ],
        ),
        (
            "inch",
            &[
                "Did you mean '[in_br]'? (inch) [similarity: 100.0%]",
                "Did you mean '[in_i]'? (inch) [similarity: 100.0%]",
                "Did you mean '[in_us]'? (inch) [similarity: 100.0%]",
            ],
        ),
        (
            "hourr",
            &[
                "Did you mean 'h'? (hour) [similarity: 80.0%]",
                "Did you mean 'h'?",
            ],
        ),
        ("xyzzy", &[]),
    ];
    for (input, expected) in cases {
        let err = validate(input).expect_err(input);
        assert_eq!(err.suggestions, *expected, "{input}");
    }
}
