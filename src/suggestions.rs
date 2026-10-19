//! Suggestion engine for error corrections and unit alternatives.

use crate::prelude::*;
use crate::{UnitRecord, get_all_units};
use hashbrown::HashSet;

/// Suggestion engine for providing error corrections and alternatives
pub(crate) struct SuggestionEngine {
    /// All available units for fuzzy matching
    all_units: Vec<&'static UnitRecord>,
    /// Their names in lower case, in the same order
    names_lower: Vec<String>,
    /// The lengths in characters of their codes and of their lower case names
    code_lengths: Vec<usize>,
    name_lengths: Vec<usize>,
}

impl SuggestionEngine {
    /// Create a new suggestion engine
    pub fn new() -> Self {
        let all_units: Vec<&'static UnitRecord> = get_all_units().iter().collect();
        let names_lower: Vec<String> = all_units
            .iter()
            .map(|u| u.display_name.to_lowercase())
            .collect();
        Self {
            code_lengths: all_units.iter().map(|u| u.code.chars().count()).collect(),
            name_lengths: names_lower.iter().map(|n| n.chars().count()).collect(),
            all_units,
            names_lower,
        }
    }

    /// The similarity of two strings of `len1` and `len2` characters, 1 minus their
    /// Levenshtein distance over the length of the longer one, if it is above `threshold`.
    ///
    /// The cost does not grow with a string much longer than the other: the distance is at
    /// least the difference of the lengths, so such a pair is ruled out before any distance
    /// is computed, and the computation stops as soon as every distance in its current row
    /// is too large for the threshold.
    pub fn similarity_above(
        s1: &str,
        len1: usize,
        s2: &str,
        len2: usize,
        threshold: f64,
    ) -> Option<f64> {
        let longer = len1.max(len2);
        if longer == 0 {
            return (1.0 > threshold).then_some(1.0);
        }
        let similarity = |distance: usize| 1.0 - (distance as f64 / longer as f64);
        if similarity(len1.abs_diff(len2)) <= threshold {
            return None;
        }
        // No distance above this one passes; the final check below is the exact one
        let bound = crate::math::ceil((1.0 - threshold) * longer as f64) as usize;
        // One row of the distance matrix: distances from a prefix of s1 to each prefix of s2
        let mut row: Vec<usize> = (0..=len2).collect();
        for (i, c1) in s1.chars().enumerate() {
            let mut diagonal = row[0];
            row[0] = i + 1;
            let mut smallest = row[0];
            for (j, c2) in s2.chars().enumerate() {
                let substitution = diagonal + usize::from(c1 != c2);
                diagonal = row[j + 1];
                row[j + 1] = substitution.min(row[j] + 1).min(row[j + 1] + 1);
                smallest = smallest.min(row[j + 1]);
            }
            // Distances never decrease from one row to the next
            if smallest > bound {
                return None;
            }
        }
        let similarity = similarity(row[len2]);
        (similarity > threshold).then_some(similarity)
    }

    /// Suggest corrections for an invalid unit
    pub fn suggest_corrections(&self, invalid_unit: &str) -> Vec<String> {
        let mut suggestions = Vec::new();

        let invalid_lower = invalid_unit.to_lowercase();
        let invalid_len = invalid_unit.chars().count();
        let invalid_lower_len = invalid_lower.chars().count();

        // First, check for exact case-insensitive matches
        for unit in &self.all_units {
            if unit.code.eq_ignore_ascii_case(invalid_unit) && unit.code != invalid_unit {
                suggestions.push(format!("Did you mean '{}'? (case mismatch)", unit.code));
            }
        }

        // Check for common typos and similar units
        let mut scored_suggestions = Vec::new();
        for (i, unit) in self.all_units.iter().enumerate() {
            if let Some(similarity) = Self::similarity_above(
                unit.code,
                self.code_lengths[i],
                invalid_unit,
                invalid_len,
                0.5,
            ) {
                // 50% similarity threshold
                scored_suggestions.push((
                    similarity,
                    unit.code.to_string(),
                    unit.display_name.to_string(),
                ));
            }

            // Also check display names
            if let Some(display_similarity) = Self::similarity_above(
                &self.names_lower[i],
                self.name_lengths[i],
                &invalid_lower,
                invalid_lower_len,
                0.6,
            ) {
                // 60% similarity for display names
                scored_suggestions.push((
                    display_similarity,
                    unit.code.to_string(),
                    unit.display_name.to_string(),
                ));
            }
        }

        // Sort by similarity score (highest first)
        scored_suggestions
            .sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(core::cmp::Ordering::Equal));

        // Add top suggestions
        let mut added = HashSet::new();
        for (score, code, display) in scored_suggestions.into_iter().take(5) {
            if added.insert(code.clone()) {
                suggestions.push(format!(
                    "Did you mean '{}'? ({}) [similarity: {:.1}%]",
                    code,
                    display,
                    score * 100.0
                ));
            }
        }

        // Check for common prefixed versions
        self.suggest_prefixed_alternatives(invalid_unit, &mut suggestions);

        // Check for common unit patterns
        self.suggest_pattern_corrections(invalid_unit, &mut suggestions);

        suggestions
    }

    /// Suggest prefixed alternatives for a unit
    fn suggest_prefixed_alternatives(&self, invalid_unit: &str, suggestions: &mut Vec<String>) {
        // Check if this might be a missing or incorrect prefix
        for unit in &self.all_units {
            // Check if the invalid unit ends with a valid unit code
            if unit.is_metric
                && invalid_unit.len() > unit.code.len()
                && invalid_unit.ends_with(&unit.code)
            {
                let potential_prefix = &invalid_unit[..invalid_unit.len() - unit.code.len()];
                if potential_prefix.len() <= 3 {
                    // Reasonable prefix length
                    suggestions.push(format!(
                        "Did you mean a prefixed version of '{}'? (e.g., 'k{}', 'm{}', 'u{}')",
                        unit.code, unit.code, unit.code, unit.code
                    ));
                    break;
                }
            }

            // Check if this might be missing a prefix
            if unit.code.len() > invalid_unit.len() && unit.code.ends_with(invalid_unit) {
                let prefix = &unit.code[..unit.code.len() - invalid_unit.len()];
                if prefix.len() <= 3 {
                    suggestions.push(format!(
                        "Did you mean '{}' ({})?",
                        unit.code, unit.display_name
                    ));
                }
            }
        }
    }

    /// Suggest corrections based on common patterns
    fn suggest_pattern_corrections(&self, invalid_unit: &str, suggestions: &mut Vec<String>) {
        // Common typos and patterns
        let patterns = [
            ("degC", "Cel"),
            ("degF", "[degF]"),
            ("celsius", "Cel"),
            ("fahrenheit", "[degF]"),
            ("meter", "m"),
            ("metre", "m"),
            ("gram", "g"),
            ("gramme", "g"),
            ("litre", "L"),
            ("liter", "L"),
            ("second", "s"),
            ("minute", "min"),
            ("hour", "h"),
            ("day", "d"),
            ("year", "a"),
            ("pascal", "Pa"),
            ("newton", "N"),
            ("joule", "J"),
            ("watt", "W"),
            ("volt", "V"),
            ("ampere", "A"),
            ("kelvin", "K"),
            ("mole", "mol"),
            ("candela", "cd"),
        ];

        let lower_invalid = invalid_unit.to_lowercase();
        for (pattern, replacement) in &patterns {
            if lower_invalid == pattern.to_lowercase()
                || lower_invalid.contains(&pattern.to_lowercase())
            {
                suggestions.push(format!("Did you mean '{replacement}'?"));
            }
        }

        // Common bracket errors
        if invalid_unit.starts_with('[') && !invalid_unit.ends_with(']') {
            suggestions.push(format!("Missing closing bracket: '{invalid_unit}]'"));
        } else if !invalid_unit.starts_with('[') && invalid_unit.ends_with(']') {
            suggestions.push(format!("Missing opening bracket: '[{invalid_unit}'"));
        }

        // Common case errors
        if invalid_unit.chars().all(|c| c.is_lowercase()) {
            // Try capitalizing first letter
            let mut chars: Vec<char> = invalid_unit.chars().collect();
            if let Some(first) = chars.get_mut(0) {
                *first = first.to_uppercase().next().unwrap_or(*first);
                let capitalized: String = chars.into_iter().collect();
                for unit in &self.all_units {
                    if unit.code == capitalized {
                        suggestions.push(format!("Did you mean '{capitalized}'? (capitalization)"));
                        break;
                    }
                }
            }
        }
    }
}

impl Default for SuggestionEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn similarity(s1: &str, s2: &str, threshold: f64) -> Option<f64> {
        SuggestionEngine::similarity_above(
            s1,
            s1.chars().count(),
            s2,
            s2.chars().count(),
            threshold,
        )
    }

    #[test]
    fn test_string_similarity() {
        assert_eq!(similarity("test", "test", 0.5), Some(1.0));
        assert_eq!(similarity("", "", 0.5), Some(1.0));
        assert_eq!(similarity("test", "", 0.0), None);
        assert_eq!(similarity("kg", "g", 0.4), Some(0.5));
        assert_eq!(similarity("kg", "g", 0.5), None);
        assert_eq!(similarity("meter", "metre", 0.5), Some(0.6));
        assert_eq!(similarity("meter", "metre", 0.6), None);
    }

    /// The full Levenshtein distance, without bounds
    fn distance(s1: &str, s2: &str) -> usize {
        let b: Vec<char> = s2.chars().collect();
        let mut row: Vec<usize> = (0..=b.len()).collect();
        for (i, c1) in s1.chars().enumerate() {
            let mut diagonal = row[0];
            row[0] = i + 1;
            for (j, c2) in b.iter().enumerate() {
                let substitution = diagonal + usize::from(c1 != *c2);
                diagonal = row[j + 1];
                row[j + 1] = substitution.min(row[j] + 1).min(row[j + 1] + 1);
            }
        }
        row[b.len()]
    }

    #[test]
    fn bounds_do_not_change_the_result() {
        // Every pair of a few unit codes, names and typos, at both thresholds in use
        let words = [
            "",
            "m",
            "g",
            "kg",
            "Pa",
            "pa",
            "metr",
            "meter",
            "metre",
            "meters",
            "[in_i]",
            "[in_x]",
            "inch",
            "degree fahrenheit",
            "[degf]",
            "international unit",
            "[IU",
            "IU]",
            "hourr",
            "hour",
            "xyzzy",
            "réaumur",
            "µ",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        ];
        for a in words {
            for b in words {
                let longer = a.chars().count().max(b.chars().count());
                let full = if longer == 0 {
                    1.0
                } else {
                    1.0 - (distance(a, b) as f64 / longer as f64)
                };
                for threshold in [0.5, 0.6] {
                    let expected = (full > threshold).then_some(full);
                    assert_eq!(
                        similarity(a, b, threshold),
                        expected,
                        "{a:?} {b:?} {threshold}"
                    );
                }
            }
        }
    }

    #[test]
    fn test_suggest_corrections() {
        let engine = SuggestionEngine::new();
        let suggestions = engine.suggest_corrections("metr");
        assert!(!suggestions.is_empty());
    }

    #[test]
    fn test_case_sensitivity_suggestions() {
        let engine = SuggestionEngine::new();
        let suggestions = engine.suggest_corrections("pa"); // Should suggest "Pa"
        assert!(suggestions.iter().any(|s| s.contains("Pa")));
    }
}
