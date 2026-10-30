//! `search_units_fuzzy` matches the query against the code, the name and the property of
//! every unit. A field shorter than the query cannot match, since each character of the
//! query takes a character of the field, and is no longer compared. A long query used to
//! cost about 0.8 s per megabyte in a release build, and 12 s in a debug one; it now
//! costs one pass over the query, so the bound below is far from both.
#![cfg(feature = "std")]

use octofhir_ucum::{ConceptKind, search_units_filtered, search_units_fuzzy};
use std::time::{Duration, Instant};

#[test]
fn a_long_query_returns_quickly() {
    // Warm the registry up
    let _ = search_units_fuzzy("metr", 30);
    for query in ["x".repeat(1_000_000), "é".repeat(500_000)] {
        let start = Instant::now();
        assert!(search_units_fuzzy(&query, 1).is_empty());
        assert!(search_units_filtered(&query, &[ConceptKind::BaseUnit], true).is_empty());
        // A query longer than every field still lists every unit with a score of 0
        assert!(
            search_units_fuzzy(&query, 0)
                .iter()
                .all(|(_, score)| *score == 0)
        );
        let elapsed = start.elapsed();
        assert!(
            elapsed < Duration::from_secs(2),
            "{}...: {elapsed:?}",
            &query[..4]
        );
    }
}
