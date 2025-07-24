//! Suggestion engine for error corrections and unit alternatives (Phase 5).

use crate::{get_all_units, UnitRecord};
use std::collections::HashMap;

/// Suggestion engine for providing error corrections and alternatives
pub struct SuggestionEngine {
    /// Pre-computed unit similarity map for fast lookups
    unit_similarity: HashMap<String, Vec<String>>,
    /// All available units for fuzzy matching
    all_units: Vec<&'static UnitRecord>,
}

impl SuggestionEngine {
    /// Create a new suggestion engine
    pub fn new() -> Self {
        let all_units: Vec<_> = get_all_units().iter().collect();
        let unit_similarity = Self::build_similarity_map(&all_units);
        
        Self {
            unit_similarity,
            all_units,
        }
    }
    
    /// Build a similarity map for fast unit lookups
    fn build_similarity_map(units: &[&'static UnitRecord]) -> HashMap<String, Vec<String>> {
        let mut similarity_map = HashMap::new();
        
        for unit in units {
            let mut similar = Vec::new();
            
            // Find units with similar codes
            for other in units {
                if unit.code != other.code {
                    let similarity = Self::string_similarity(&unit.code, &other.code);
                    if similarity > 0.6 {  // 60% similarity threshold
                        similar.push(other.code.to_string());
                    }
                }
            }
            
            // Find units with similar display names
            for other in units {
                if unit.code != other.code {
                    let similarity = Self::string_similarity(&unit.display_name, &other.display_name);
                    if similarity > 0.7 {  // 70% similarity threshold for display names
                        similar.push(other.code.to_string());
                    }
                }
            }
            
            // Sort by similarity and remove duplicates
            similar.sort();
            similar.dedup();
            
            if !similar.is_empty() {
                similarity_map.insert(unit.code.to_string(), similar);
            }
        }
        
        similarity_map
    }
    
    /// Calculate string similarity using Levenshtein distance
    pub fn string_similarity(s1: &str, s2: &str) -> f64 {
        if s1.is_empty() && s2.is_empty() {
            return 1.0;
        }
        
        if s1.is_empty() || s2.is_empty() {
            return 0.0;
        }
        
        let len1 = s1.chars().count();
        let len2 = s2.chars().count();
        
        if len1 == 0 {
            return len2 as f64;
        }
        if len2 == 0 {
            return len1 as f64;
        }
        
        let mut matrix = vec![vec![0usize; len2 + 1]; len1 + 1];
        
        // Initialize first row and column
        for i in 0..=len1 {
            matrix[i][0] = i;
        }
        for j in 0..=len2 {
            matrix[0][j] = j;
        }
        
        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();
        
        // Fill the matrix
        for i in 1..=len1 {
            for j in 1..=len2 {
                let cost = if s1_chars[i - 1] == s2_chars[j - 1] { 0 } else { 1 };
                matrix[i][j] = (matrix[i - 1][j] + 1)
                    .min(matrix[i][j - 1] + 1)
                    .min(matrix[i - 1][j - 1] + cost);
            }
        }
        
        let distance = matrix[len1][len2];
        let max_len = len1.max(len2);
        
        1.0 - (distance as f64 / max_len as f64)
    }
    
    /// Suggest corrections for an invalid unit
    pub fn suggest_corrections(&self, invalid_unit: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        // First, check for exact case-insensitive matches
        for unit in &self.all_units {
            if unit.code.to_lowercase() == invalid_unit.to_lowercase() && unit.code != invalid_unit {
                suggestions.push(format!("Did you mean '{}'? (case mismatch)", unit.code));
            }
        }
        
        // Check for common typos and similar units
        let mut scored_suggestions = Vec::new();
        for unit in &self.all_units {
            let similarity = Self::string_similarity(&unit.code, invalid_unit);
            if similarity > 0.5 {  // 50% similarity threshold
                scored_suggestions.push((similarity, unit.code.to_string(), unit.display_name.to_string()));
            }
            
            // Also check display names
            let display_similarity = Self::string_similarity(&unit.display_name.to_lowercase(), &invalid_unit.to_lowercase());
            if display_similarity > 0.6 {  // 60% similarity for display names
                scored_suggestions.push((display_similarity, unit.code.to_string(), unit.display_name.to_string()));
            }
        }
        
        // Sort by similarity score (highest first)
        scored_suggestions.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        
        // Add top suggestions
        let mut added = std::collections::HashSet::new();
        for (score, code, display) in scored_suggestions.into_iter().take(5) {
            if added.insert(code.clone()) {
                suggestions.push(format!("Did you mean '{}'? ({}) [similarity: {:.1}%]", code, display, score * 100.0));
            }
        }
        
        // Check for common prefixed versions
        self.suggest_prefixed_alternatives(invalid_unit, &mut suggestions);
        
        // Check for common unit patterns
        self.suggest_pattern_corrections(invalid_unit, &mut suggestions);
        
        suggestions
    }
    
    /// Suggest alternatives based on unit property
    pub fn suggest_alternatives(&self, unit: &str, property: &str) -> Vec<String> {
        let mut alternatives = Vec::new();
        
        // Find units with the same property
        for other_unit in &self.all_units {
            if other_unit.property.to_lowercase() == property.to_lowercase() && other_unit.code != unit {
                alternatives.push(format!("{} ({})", other_unit.code, other_unit.display_name));
            }
        }
        
        // Sort and limit results
        alternatives.sort();
        alternatives.truncate(10);
        
        alternatives
    }
    
    /// Suggest prefixed alternatives for a unit
    fn suggest_prefixed_alternatives(&self, invalid_unit: &str, suggestions: &mut Vec<String>) {
        // Check if this might be a missing or incorrect prefix
        for unit in &self.all_units {
            // Check if the invalid unit ends with a valid unit code
            if invalid_unit.len() > unit.code.len() && invalid_unit.ends_with(&unit.code) {
                let potential_prefix = &invalid_unit[..invalid_unit.len() - unit.code.len()];
                if potential_prefix.len() <= 3 {  // Reasonable prefix length
                    suggestions.push(format!("Did you mean a prefixed version of '{}'? (e.g., 'k{}', 'm{}', 'u{}')", 
                                           unit.code, unit.code, unit.code, unit.code));
                    break;
                }
            }
            
            // Check if this might be missing a prefix
            if unit.code.len() > invalid_unit.len() && unit.code.ends_with(invalid_unit) {
                let prefix = &unit.code[..unit.code.len() - invalid_unit.len()];
                if prefix.len() <= 3 {
                    suggestions.push(format!("Did you mean '{}' ({})?", unit.code, unit.display_name));
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
            if lower_invalid == pattern.to_lowercase() || 
               lower_invalid.contains(&pattern.to_lowercase()) {
                suggestions.push(format!("Did you mean '{}'?", replacement));
            }
        }
        
        // Common bracket errors
        if invalid_unit.starts_with('[') && !invalid_unit.ends_with(']') {
            suggestions.push(format!("Missing closing bracket: '{}]'", invalid_unit));
        } else if !invalid_unit.starts_with('[') && invalid_unit.ends_with(']') {
            suggestions.push(format!("Missing opening bracket: '[{}'", invalid_unit));
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
                        suggestions.push(format!("Did you mean '{}'? (capitalization)", capitalized));
                        break;
                    }
                }
            }
        }
    }
    
    /// Get similar units based on pre-computed similarity map
    pub fn get_similar_units(&self, unit: &str) -> Vec<String> {
        self.unit_similarity.get(unit).cloned().unwrap_or_default()
    }
    
    /// Suggest corrections for dimension mismatches
    pub fn suggest_dimension_fixes(&self, expected_property: &str, found_unit: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        // Find units with the expected property
        for unit in &self.all_units {
            if unit.property.to_lowercase() == expected_property.to_lowercase() {
                let similarity = Self::string_similarity(&unit.code, found_unit);
                if similarity > 0.3 {
                    suggestions.push(format!("Use '{}' ({}) for {}", unit.code, unit.display_name, expected_property));
                }
            }
        }
        
        // Sort by relevance and limit
        suggestions.sort();
        suggestions.truncate(5);
        
        suggestions
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
        assert!(SuggestionEngine::string_similarity("kg", "g") > 0.5);
        assert!(SuggestionEngine::string_similarity("meter", "metre") > 0.8);
    }
    
    #[test]
    fn test_suggest_corrections() {
        let engine = SuggestionEngine::new();
        let suggestions = engine.suggest_corrections("kilo");
        assert!(!suggestions.is_empty());
    }
    
    #[test]
    fn test_suggest_alternatives() {
        let engine = SuggestionEngine::new();
        let alternatives = engine.suggest_alternatives("m", "length");
        assert!(!alternatives.is_empty());
    }
    
    #[test]
    fn test_case_sensitivity_suggestions() {
        let engine = SuggestionEngine::new();
        let suggestions = engine.suggest_corrections("pa");  // Should suggest "Pa"
        assert!(suggestions.iter().any(|s| s.contains("Pa")));
    }
}