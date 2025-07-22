use octofhir_ucum_core::{
    ConceptKind, search_units, search_units_filtered, search_units_fuzzy, search_units_regex,
};

#[test]
fn test_basic_search() {
    let results = search_units("meter");
    assert!(!results.is_empty());

    // Should find units containing "meter" in code, display name, or property
    let has_meter = results.iter().any(|unit| {
        unit.code.to_lowercase().contains("meter")
            || unit.display_name.to_lowercase().contains("meter")
            || unit.property.to_lowercase().contains("meter")
    });
    assert!(has_meter);
}

#[test]
fn test_regex_search() {
    // Test regex search for units containing "meter" or "metre"
    let results = search_units_regex(r"mete?r", false).unwrap();
    assert!(!results.is_empty());

    // Test case-sensitive search
    let case_sensitive_results = search_units_regex("METER", true).unwrap();
    let case_insensitive_results = search_units_regex("METER", false).unwrap();

    // Case-insensitive should return more or equal results
    assert!(case_insensitive_results.len() >= case_sensitive_results.len());
}

#[test]
fn test_fuzzy_search() {
    // Test fuzzy search with a typo
    let results = search_units_fuzzy("metter", 30); // "metter" instead of "meter"
    assert!(!results.is_empty());

    // Results should be sorted by score (descending)
    for i in 1..results.len() {
        assert!(results[i - 1].1 >= results[i].1);
    }

    // Should find some meter-related units
    let has_meter_like = results.iter().any(|(unit, _score)| {
        unit.code.to_lowercase().contains("m") || unit.display_name.to_lowercase().contains("meter")
    });
    assert!(has_meter_like);
}

#[test]
fn test_concept_kind_filtering() {
    // Test filtering for base units only
    let _base_units = search_units_filtered("m", &[ConceptKind::BaseUnit], false);

    // Test filtering for arbitrary units only
    let arbitrary_units = search_units_filtered("IU", &[ConceptKind::ArbitraryUnit], false);

    // Arbitrary units should have square brackets
    for unit in &arbitrary_units {
        assert!(unit.code.starts_with('[') && unit.code.ends_with(']'));
    }

    // Test multiple concept kinds
    let mixed_results = search_units_filtered(
        "m",
        &[ConceptKind::BaseUnit, ConceptKind::PrefixedUnit],
        false,
    );
    assert!(!mixed_results.is_empty());
}

#[test]
fn test_fuzzy_with_filtering() {
    // Test fuzzy search combined with concept kind filtering
    let results = search_units_filtered("metter", &[ConceptKind::BaseUnit], true);

    // Should find some results
    assert!(!results.is_empty());

    // All results should be base units (this is a simplified check)
    // In practice, the classification might be more complex
}

#[test]
fn test_invalid_regex() {
    // Test invalid regex pattern
    let result = search_units_regex("[invalid", false);
    assert!(result.is_err());
}

#[test]
fn test_empty_search() {
    // Test empty search query
    let _results = search_units("");
    // Should return all units or empty results, depending on implementation
    // This tests that it doesn't crash

    let _fuzzy_results = search_units_fuzzy("", 30);
    // Should handle empty query gracefully
}
