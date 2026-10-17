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
}

impl SuggestionEngine {
    /// Create a new suggestion engine
    pub fn new() -> Self {
        let all_units: Vec<&'static UnitRecord> = get_all_units().iter().collect();
        let names_lower = all_units
            .iter()
            .map(|u| u.display_name.to_lowercase())
            .collect();
        Self {
            all_units,
            names_lower,
        }
    }

    /// Similarity of two strings, 1 minus their Levenshtein distance over the length of the
    /// longer one.
    pub fn string_similarity(s1: &str, s2: &str) -> f64 {
        if s1.is_empty() && s2.is_empty() {
            return 1.0;
        }
        if s1.is_empty() || s2.is_empty() {
            return 0.0;
        }
        let len2 = s2.chars().count();
        // One row of the distance matrix: distances from a prefix of s1 to each prefix of s2
        let mut row: Vec<usize> = (0..=len2).collect();
        let mut len1 = 0;
        for (i, c1) in s1.chars().enumerate() {
            len1 += 1;
            let mut diagonal = row[0];
            row[0] = i + 1;
            for (j, c2) in s2.chars().enumerate() {
                let substitution = diagonal + usize::from(c1 != c2);
                diagonal = row[j + 1];
                row[j + 1] = substitution.min(row[j] + 1).min(row[j + 1] + 1);
            }
        }
        1.0 - (row[len2] as f64 / len1.max(len2) as f64)
    }

    /// Suggest corrections for an invalid unit
    pub fn suggest_corrections(&self, invalid_unit: &str) -> Vec<String> {
        let mut suggestions = Vec::new();

        let invalid_lower = invalid_unit.to_lowercase();

        // First, check for exact case-insensitive matches
        for unit in &self.all_units {
            if unit.code.eq_ignore_ascii_case(invalid_unit) && unit.code != invalid_unit {
                suggestions.push(format!("Did you mean '{}'? (case mismatch)", unit.code));
            }
        }

        // Check for common typos and similar units
        let mut scored_suggestions = Vec::new();
        for (unit, name_lower) in self.all_units.iter().zip(&self.names_lower) {
            let similarity = Self::string_similarity(unit.code, invalid_unit);
            if similarity > 0.5 {
                // 50% similarity threshold
                scored_suggestions.push((
                    similarity,
                    unit.code.to_string(),
                    unit.display_name.to_string(),
                ));
            }

            // Also check display names
            let display_similarity = Self::string_similarity(name_lower, &invalid_lower);
            if display_similarity > 0.6 {
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

    #[test]
    fn test_string_similarity() {
        assert!((SuggestionEngine::string_similarity("test", "test") - 1.0).abs() < f64::EPSILON);
        assert!(SuggestionEngine::string_similarity("test", "") < 0.1);
        assert!(SuggestionEngine::string_similarity("kg", "g") >= 0.5);
        assert!(SuggestionEngine::string_similarity("meter", "metre") >= 0.6);
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
