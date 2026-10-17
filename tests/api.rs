//! The helper API around the UCUM tables: search, classification, defined forms,
//! properties, the model, display names and the semantic checks of canonical units and
//! properties.

use octofhir_ucum::{
    ConceptKind, ErrorKind, Number, classify_unit, display_name, find_unit, get_all_units,
    get_common_display, get_defined_forms, get_model, get_properties, search_units,
    search_units_by_property, search_units_filtered, validate_canonical_units,
    validate_in_property, validate_ucum,
};
#[cfg(feature = "std")]
use octofhir_ucum::{search_units_fuzzy, search_units_regex};

fn codes(units: &[&octofhir_ucum::UnitRecord]) -> Vec<&'static str> {
    units.iter().map(|u| u.code).collect()
}

#[test]
fn search_matches_code_name_and_property() {
    let results = search_units("meter");
    assert!(results.iter().any(|u| u.code == "m"));
    assert!(results.iter().all(|u| {
        u.code.to_lowercase().contains("meter")
            || u.display_name.to_lowercase().contains("meter")
            || u.property.to_lowercase().contains("meter")
    }));
    // An exact code comes first
    assert_eq!(search_units("Pa")[0].code, "Pa");
    // An empty query must not panic, whatever it returns
    let _ = search_units("");
}

#[test]
fn search_by_property_uses_the_xml_properties() {
    let length = search_units_by_property("length");
    assert!(length.iter().any(|u| u.code == "m"));
    assert!(length.iter().any(|u| u.code == "[in_i]"));
    assert!(length.iter().all(|u| u.property == "length"));
    let current = codes(&search_units_by_property("Electric Current"));
    assert!(current.contains(&"A"), "{current:?}");
    assert!(search_units_by_property("no such property").is_empty());
}

#[test]
#[cfg(feature = "std")]
fn regex_search() {
    let results = search_units_regex(r"mete?r", false).unwrap();
    assert!(results.iter().any(|u| u.code == "m"));
    let sensitive = search_units_regex("METER", true).unwrap();
    let insensitive = search_units_regex("METER", false).unwrap();
    assert!(sensitive.is_empty());
    assert!(!insensitive.is_empty());
    assert!(search_units_regex("[invalid", false).is_err());
}

#[test]
#[cfg(feature = "std")]
fn fuzzy_search_is_sorted_by_score() {
    let results = search_units_fuzzy("metr", 30);
    assert!(!results.is_empty());
    assert!(results.windows(2).all(|w| w[0].1 >= w[1].1));
    assert!(results.iter().any(|(u, _)| u.display_name == "meter"));
    let _ = search_units_fuzzy("", 30);
}

#[test]
fn base_units_are_the_seven_ucum_base_units() {
    let mut base: Vec<&str> = get_all_units()
        .iter()
        .filter(|u| classify_unit(u) == ConceptKind::BaseUnit)
        .map(|u| u.code)
        .collect();
    base.sort();
    // UCUM §28: meter, second, gram, radian, kelvin, coulomb, candela
    assert_eq!(base, ["C", "K", "cd", "g", "m", "rad", "s"]);
    assert!(get_all_units().iter().filter(|u| u.is_base).count() == 7);
}

#[test]
fn classification_follows_the_tables() {
    let kind = |code| classify_unit(find_unit(code).unwrap());
    assert_eq!(kind("m"), ConceptKind::BaseUnit);
    assert_eq!(kind("rad"), ConceptKind::BaseUnit);
    assert_eq!(kind("A"), ConceptKind::DerivedUnit);
    assert_eq!(kind("mol"), ConceptKind::DerivedUnit);
    assert_eq!(kind("[in_i]"), ConceptKind::DerivedUnit);
    assert_eq!(kind("[psi]"), ConceptKind::DerivedUnit);
    assert_eq!(kind("[IU]"), ConceptKind::ArbitraryUnit);
    assert_eq!(kind("[arb'U]"), ConceptKind::ArbitraryUnit);
    assert_eq!(kind("Cel"), ConceptKind::SpecialUnit);
    assert_eq!(kind("[degF]"), ConceptKind::SpecialUnit);
    assert_eq!(kind("[pH]"), ConceptKind::SpecialUnit);
    assert_eq!(kind("B[V]"), ConceptKind::SpecialUnit);
    // Square brackets do not make a unit arbitrary, nor a leading prefix letter a prefix
    for code in ["[degR]", "cd", "Pa", "Gb", "ph", "kat", "cal"] {
        assert_ne!(kind(code), ConceptKind::ArbitraryUnit, "{code}");
    }
    for unit in get_all_units() {
        assert_eq!(
            classify_unit(unit) == ConceptKind::ArbitraryUnit,
            unit.special.is_arbitrary(),
            "{}",
            unit.code
        );
        assert_eq!(
            classify_unit(unit) == ConceptKind::SpecialUnit,
            unit.special.is_special(),
            "{}",
            unit.code
        );
    }
}

#[test]
fn filtered_search_keeps_the_requested_kinds() {
    let base = search_units_filtered("m", &[ConceptKind::BaseUnit], false);
    assert!(base.iter().any(|u| u.code == "m"));
    assert!(base.iter().all(|u| u.is_base));
    let arbitrary = search_units_filtered("unit", &[ConceptKind::ArbitraryUnit], false);
    assert!(!arbitrary.is_empty());
    assert!(arbitrary.iter().all(|u| u.special.is_arbitrary()));
    let special = search_units_filtered(
        "degree",
        &[ConceptKind::SpecialUnit, ConceptKind::DerivedUnit],
        false,
    );
    assert!(special.iter().any(|u| u.code == "Cel"));
    assert!(special.iter().any(|u| u.code == "deg"));
    #[cfg(feature = "std")]
    {
        let fuzzy = search_units_filtered("metr", &[ConceptKind::BaseUnit], true);
        assert!(fuzzy.iter().any(|u| u.code == "m"));
    }
}

#[test]
fn defined_forms_are_the_commensurable_atoms() {
    let length = codes(&get_defined_forms("m"));
    for code in ["m", "[in_i]", "[ft_i]", "[mi_i]", "Ao", "[ly]"] {
        assert!(length.contains(&code), "{code} in {length:?}");
    }
    for code in ["g", "s", "m2", "rad"] {
        assert!(!length.contains(&code), "{code} in {length:?}");
    }
    // A prefixed expression has the forms of its atom
    assert_eq!(get_defined_forms("km"), get_defined_forms("m"));

    // "a" is the year: time units, not the pascal or a centi-day
    let time = codes(&get_defined_forms("a"));
    for code in ["a", "s", "min", "h", "d", "wk"] {
        assert!(time.contains(&code), "{code} in {time:?}");
    }
    assert!(!time.contains(&"Pa"));
    assert!(!time.contains(&"cd"));

    // A special unit is commensurable with its corresponding unit (UCUM §21.3)
    let temperature = codes(&get_defined_forms("K"));
    assert!(temperature.contains(&"Cel"));
    assert!(temperature.contains(&"[degF]"));

    // An arbitrary unit with itself only (UCUM §25)
    assert_eq!(codes(&get_defined_forms("[arb'U]")), ["[arb'U]"]);
    let international = codes(&get_defined_forms("[IU]"));
    assert_eq!(international.len(), 2, "{international:?}");
    assert!(international.contains(&"[iU]"));
    assert!(!codes(&get_defined_forms("1")).contains(&"[IU]"));

    assert!(get_defined_forms("not a unit").is_empty());
}

#[test]
fn property_validation_uses_the_xml_properties() {
    for (unit, property) in [
        ("m", "length"),
        ("[in_i]", "length"),
        ("kg", "mass"),
        ("A", "electric current"),
        ("mA", "Electric Current"),
        ("rad", "plane angle"),
        ("deg", "plane angle"),
        ("[psi]", "pressure"),
        ("mm[Hg]", "pressure"),
        ("Cel", "temperature"),
        ("K", "temperature"),
        ("mol", "amount of substance"),
        // A level is commensurable with the unit of its reference (UCUM §21.3)
        ("dB[SPL]", "pressure"),
        ("[IU]", "arbitrary"),
        // Legacy names
        ("mA", "current"),
        ("mmol", "amount"),
        ("cd", "luminosity"),
        ("%", "dimensionless"),
        ("mol", "dimensionless"),
    ] {
        assert!(
            validate_in_property(unit, property).unwrap(),
            "{unit} is {property}"
        );
    }
    for (unit, property) in [
        ("kg", "length"),
        ("s", "mass"),
        // A radian is a base unit, not a number
        ("rad", "dimensionless"),
        // An arbitrary unit is commensurable with nothing else (UCUM §24.3)
        ("[IU]", "dimensionless"),
        ("Hz", "plane angle"),
    ] {
        assert!(
            !validate_in_property(unit, property).unwrap(),
            "{unit} is not {property}"
        );
    }
    let err = validate_in_property("m", "lenght").unwrap_err();
    assert!(
        matches!(err.kind, ErrorKind::InvalidProperty { .. }),
        "{:?}",
        err.kind
    );
    assert!(
        err.suggestions.iter().any(|s| s.contains("length")),
        "{err}"
    );
    assert!(validate_in_property("not a unit", "length").is_err());
}

#[test]
fn properties_are_read_whole_from_the_xml() {
    let properties = get_properties();
    for property in ["length", "electric current", "plane angle", "temperature"] {
        assert!(properties.contains(property), "{property}");
    }
    // A character reference in the property used to cut it short
    let reactivity = find_unit("[IR]").unwrap().property;
    assert!(
        reactivity.ends_with("Stallergenes\u{ae} method"),
        "{reactivity}"
    );
    assert!(properties.contains(reactivity));
    assert_eq!(codes(&search_units_by_property(reactivity)), ["[IR]"]);
}

#[test]
fn model_is_the_bundled_tables() {
    let model = get_model();
    assert_eq!(model.version, "2.2");
    assert_eq!(model.revision_date, "2024-06-17");
    assert_eq!(model.prefixes.len(), 24);
    assert_eq!(model.units.len(), 312);
    assert!(validate_ucum().is_empty(), "{:?}", validate_ucum());
}

#[test]
fn canonical_units_are_checked_by_their_meaning() {
    for (unit, canonical) in [
        ("N", "kg.m.s-2"),
        ("N", "kg.m/s2"),
        ("N", "1000.g.m.s-2"),
        ("C", "A.s"),
        ("A", "C/s"),
        ("km", "1000.m"),
        ("km", "10*3.m"),
        ("rad", "rad"),
        ("sr", "rad2"),
        ("[IU]/mL", "[iU]/mL"),
        ("mol", "602214076.10*15"),
    ] {
        assert!(
            validate_canonical_units(unit, canonical).unwrap(),
            "{unit} = {canonical}"
        );
    }
    for (unit, canonical) in [
        ("km", "m"),
        ("N", "g.m.s-2"),
        ("km", "kg"),
        ("[IU]", "1"),
        ("Cel", "K"),
        ("B", "1"),
    ] {
        assert!(
            !validate_canonical_units(unit, canonical).unwrap(),
            "{unit} != {canonical}"
        );
    }
    assert!(validate_canonical_units("not a unit", "m").is_err());
}

#[test]
fn display_names_follow_the_official_tests() {
    for (code, display) in [
        ("", "(unity)"),
        ("m", "(meter)"),
        ("mm", "(millimeter)"),
        ("m[H2O]", "(meter of water column)"),
        ("10*23", "(the number ten for arbitrary powers ^ 23)"),
        ("rad2", "(radian ^ 2)"),
        (
            "m3.kg-1.s-2",
            "(meter ^ 3) * (kilogram ^ -1) * (second ^ -2)",
        ),
        (
            "4.[pi].10*-7.N/A2",
            "4 * (the number pi) * (the number ten for arbitrary powers ^ -7) * (newton) / (amp\u{e8}re ^ 2)",
        ),
        ("Pa", "(pascal)"),
        // A compound divisor is grouped
        ("kg/(m.s)", "(kilogram) / ((meter) * (second))"),
        ("/min", "1 / (minute)"),
        ("mm[Hg]", "(millimeter of mercury column)"),
        ("kg{dry}", "(kilogram){dry}"),
    ] {
        assert_eq!(display_name(code).unwrap(), display, "{code:?}");
    }
    assert!(display_name("not a unit").is_err());
    assert!(display_name("m/").is_err());
}

#[test]
fn common_display_names() {
    assert_eq!(get_common_display("m"), "meter");
    assert_eq!(get_common_display("kg"), "kilogram");
    assert_eq!(get_common_display("[in_i]"), "inch");
    assert_eq!(get_common_display("mm[Hg]"), "millimeter of mercury column");
    assert_eq!(get_common_display("m/s"), "(meter) / (second)");
    // A prefix on a non-metric unit is not a unit (UCUM §11.1)
    assert_eq!(get_common_display("k[in_i]"), "k[in_i]");
    assert_eq!(get_common_display("k%"), "k%");
    assert_eq!(get_common_display("unknown"), "unknown");
}

#[test]
fn find_unit_finds_atoms_only() {
    assert_eq!(find_unit("m").unwrap().code, "m");
    assert_eq!(find_unit("[in_i]").unwrap().code, "[in_i]");
    // Prefixed codes go through `lookup_unit`: `find_unit` used to return the gram for "mg",
    // dropping the prefix, and the inch for "k[in_i]"
    assert!(find_unit("mg").is_none());
    assert!(find_unit("k[in_i]").is_none());
    assert_eq!(find_unit("da").map(|u| u.code), None);
    let (prefix, unit) = octofhir_ucum::lookup_unit("mg", octofhir_ucum::Case::Sensitive).unwrap();
    assert_eq!((prefix.unwrap().symbol, unit.code), ("m", "g"));
    assert_eq!(prefix.unwrap().factor, Number::parse("0.001").unwrap());
}
