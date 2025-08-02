//! Performance optimization module for Performance and Scalability
//!
//! This module provides enhanced caching and optimization features:
//! - Multi-level evaluation cache with hash-based keys
//! - Optimized registry access with O(1) unit lookup
//! - Conversion result caching
//! - Performance monitoring and metrics

use crate::{
    UcumError,
    ast::UnitExpr,
    evaluator::EvalResult,
    types::{Prefix, UnitRecord},
};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Enhanced evaluation cache with multiple cache types
pub struct EvaluationCache {
    /// Cache for expression evaluation results
    expression_cache: HashMap<u64, EvalResult>,
    /// Cache for conversion results (from_unit, to_unit) -> factor
    conversion_cache: HashMap<(String, String), f64>,
    /// Cache for dimension lookups
    dimension_cache: HashMap<String, [i8; 7]>,
    /// Performance statistics
    stats: CacheStats,
}

/// Performance statistics for cache usage
#[derive(Debug, Default, Clone)]
pub struct CacheStats {
    pub expression_hits: u64,
    pub expression_misses: u64,
    pub conversion_hits: u64,
    pub conversion_misses: u64,
    pub dimension_hits: u64,
    pub dimension_misses: u64,
}

impl CacheStats {
    /// Calculate a hit ratio for expressions
    pub fn expression_hit_ratio(&self) -> f64 {
        if self.expression_hits + self.expression_misses == 0 {
            0.0
        } else {
            self.expression_hits as f64 / (self.expression_hits + self.expression_misses) as f64
        }
    }

    /// Calculate hit ratio for conversions
    pub fn conversion_hit_ratio(&self) -> f64 {
        if self.conversion_hits + self.conversion_misses == 0 {
            0.0
        } else {
            self.conversion_hits as f64 / (self.conversion_hits + self.conversion_misses) as f64
        }
    }

    /// Calculate overall hit ratio
    pub fn overall_hit_ratio(&self) -> f64 {
        let total_hits = self.expression_hits + self.conversion_hits + self.dimension_hits;
        let total_misses = self.expression_misses + self.conversion_misses + self.dimension_misses;

        if total_hits + total_misses == 0 {
            0.0
        } else {
            total_hits as f64 / (total_hits + total_misses) as f64
        }
    }
}

impl Default for EvaluationCache {
    fn default() -> Self {
        Self::new()
    }
}

impl EvaluationCache {
    /// Create a new evaluation cache
    pub fn new() -> Self {
        Self {
            expression_cache: HashMap::new(),
            conversion_cache: HashMap::new(),
            dimension_cache: HashMap::new(),
            stats: CacheStats::default(),
        }
    }

    /// Generate a hash key for a UnitExpr using a fast hash function
    pub fn hash_expr(expr: &UnitExpr) -> u64 {
        let mut hasher = DefaultHasher::new();
        hash_unit_expr(expr, &mut hasher);
        hasher.finish()
    }

    /// Get or compute an expression evaluation result with caching
    pub fn get_or_compute_expression<F>(
        &mut self,
        expr: &UnitExpr,
        compute: F,
    ) -> Result<EvalResult, UcumError>
    where
        F: FnOnce() -> Result<EvalResult, UcumError>,
    {
        let key = Self::hash_expr(expr);

        if let Some(result) = self.expression_cache.get(&key) {
            self.stats.expression_hits += 1;
            Ok(result.clone())
        } else {
            self.stats.expression_misses += 1;
            let result = compute()?;

            // Only cache if the cache isn't too large (prevent memory bloat)
            if self.expression_cache.len() < 10000 {
                self.expression_cache.insert(key, result.clone());
            }

            Ok(result)
        }
    }

    /// Get or compute a conversion result with caching
    pub fn get_or_compute_conversion<F>(
        &mut self,
        from_unit: &str,
        to_unit: &str,
        compute: F,
    ) -> Result<f64, UcumError>
    where
        F: FnOnce() -> Result<f64, UcumError>,
    {
        let key = (from_unit.to_string(), to_unit.to_string());

        if let Some(&result) = self.conversion_cache.get(&key) {
            self.stats.conversion_hits += 1;
            Ok(result)
        } else {
            self.stats.conversion_misses += 1;
            let result = compute()?;

            // Only cache if the cache isn't too large
            if self.conversion_cache.len() < 5000 {
                self.conversion_cache.insert(key, result);
            }

            Ok(result)
        }
    }

    /// Get or compute dimension information with caching
    pub fn get_or_compute_dimension<F>(&mut self, unit_code: &str, compute: F) -> [i8; 7]
    where
        F: FnOnce() -> [i8; 7],
    {
        if let Some(&result) = self.dimension_cache.get(unit_code) {
            self.stats.dimension_hits += 1;
            result
        } else {
            self.stats.dimension_misses += 1;
            let result = compute();

            // Only cache if the cache isn't too large
            if self.dimension_cache.len() < 1000 {
                self.dimension_cache.insert(unit_code.to_string(), result);
            }

            result
        }
    }

    /// Clear all caches
    pub fn clear(&mut self) {
        self.expression_cache.clear();
        self.conversion_cache.clear();
        self.dimension_cache.clear();
        self.stats = CacheStats::default();
    }

    /// Get cache statistics
    pub fn stats(&self) -> &CacheStats {
        &self.stats
    }

    /// Get cache sizes
    pub fn cache_sizes(&self) -> (usize, usize, usize) {
        (
            self.expression_cache.len(),
            self.conversion_cache.len(),
            self.dimension_cache.len(),
        )
    }
}

/// Hash function for UnitExpr that creates consistent hash values
fn hash_unit_expr<H: Hasher>(expr: &UnitExpr, hasher: &mut H) {
    match expr {
        UnitExpr::Numeric(n) => {
            0u8.hash(hasher); // discriminant
            n.to_bits().hash(hasher);
        }
        UnitExpr::Symbol(s) => {
            1u8.hash(hasher); // discriminant
            s.hash(hasher);
        }
        UnitExpr::SymbolOwned(s) => {
            2u8.hash(hasher); // discriminant
            s.hash(hasher);
        }
        UnitExpr::Product(factors) => {
            3u8.hash(hasher); // discriminant

            // Sort factors by their hash to ensure consistent ordering
            // This is important for commutative operations
            let mut factor_hashes: Vec<_> = factors
                .iter()
                .map(|f| {
                    let mut sub_hasher = DefaultHasher::new();
                    hash_unit_expr(&f.expr, &mut sub_hasher);
                    f.exponent.hash(&mut sub_hasher);
                    sub_hasher.finish()
                })
                .collect();
            factor_hashes.sort_unstable();

            factor_hashes.hash(hasher);
        }
        UnitExpr::Quotient(num, den) => {
            4u8.hash(hasher); // discriminant
            hash_unit_expr(num, hasher);
            hash_unit_expr(den, hasher);
        }
        UnitExpr::Power(base, exp) => {
            5u8.hash(hasher); // discriminant
            hash_unit_expr(base, hasher);
            exp.hash(hasher);
        }
    }
}

// Optimized registry lookup using pre-computed HashMaps for O(1) access
lazy_static! {
    /// Pre-computed HashMap for O(1) unit lookup
    static ref UNIT_MAP: HashMap<&'static str, &'static UnitRecord> = {
        let mut map = HashMap::new();
        for unit in crate::registry::UNITS.iter() {
            map.insert(unit.code, unit);
        }
        map
    };

    /// Pre-computed HashMap for O(1) prefix lookup (enhanced version)
    static ref PREFIX_MAP_ENHANCED: HashMap<&'static str, &'static Prefix> = {
        let mut map = HashMap::new();
        for prefix in crate::registry::PREFIXES.iter() {
            map.insert(prefix.symbol, prefix);
        }
        map
    };

}

/// Optimized unit lookup with O(1) HashMap access
pub fn find_unit_optimized(code: &str) -> Option<&'static UnitRecord> {
    // First try direct O(1) lookup
    if let Some(&unit) = UNIT_MAP.get(code) {
        return Some(unit);
    }

    // If direct lookup fails, try to decompose into prefix + base unit
    // Check all possible prefix lengths (longest first to avoid ambiguity)
    for prefix_len in (1..code.len()).rev() {
        let (prefix_part, unit_part) = code.split_at(prefix_len);

        // Check if prefix_part is a valid prefix and unit_part is a valid unit
        if let (Some(_prefix), Some(&unit)) = (
            PREFIX_MAP_ENHANCED.get(prefix_part),
            UNIT_MAP.get(unit_part),
        ) {
            return Some(unit);
        }
    }

    None
}

/// Optimized prefix lookup with O(1) HashMap access
pub fn find_prefix_optimized(sym: &str) -> Option<&'static Prefix> {
    PREFIX_MAP_ENHANCED.get(sym).copied()
}

/// Prefix trie for efficient prefix matching (Phase 4 enhancement)
#[derive(Debug)]
pub struct PrefixTrie {
    /// Root node of the trie
    root: TrieNode,
}

#[derive(Debug)]
struct TrieNode {
    /// Children nodes indexed by character
    children: HashMap<char, TrieNode>,
    /// Prefix stored at this node (if any)
    prefix: Option<&'static Prefix>,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            prefix: None,
        }
    }
}

impl Default for PrefixTrie {
    fn default() -> Self {
        Self::new()
    }
}

impl PrefixTrie {
    /// Create a new prefix trie from the static prefixes
    pub fn new() -> Self {
        let mut trie = Self {
            root: TrieNode::new(),
        };

        // Insert all prefixes into the trie
        for prefix in crate::registry::PREFIXES.iter() {
            trie.insert(prefix);
        }

        trie
    }

    /// Insert a prefix into the trie
    fn insert(&mut self, prefix: &'static Prefix) {
        let mut current = &mut self.root;

        for ch in prefix.symbol.chars() {
            current = current.children.entry(ch).or_insert_with(TrieNode::new);
        }

        current.prefix = Some(prefix);
    }

    /// Find the longest prefix match for a given string
    pub fn find_longest_prefix(&self, text: &str) -> Option<&'static Prefix> {
        let mut current = &self.root;
        let mut last_match = None;

        for ch in text.chars() {
            if let Some(child) = current.children.get(&ch) {
                current = child;
                if current.prefix.is_some() {
                    last_match = current.prefix;
                }
            } else {
                break;
            }
        }

        last_match
    }

    /// Find all possible prefix matches for a given string
    pub fn find_all_prefixes(&self, text: &str) -> Vec<&'static Prefix> {
        let mut current = &self.root;
        let mut matches = Vec::new();

        for ch in text.chars() {
            if let Some(child) = current.children.get(&ch) {
                current = child;
                if let Some(prefix) = current.prefix {
                    matches.push(prefix);
                }
            } else {
                break;
            }
        }

        matches
    }
}

lazy_static! {
    /// Global prefix trie for efficient prefix matching
    static ref PREFIX_TRIE: PrefixTrie = PrefixTrie::new();
}

/// Find prefixes using the optimized trie structure
pub fn find_prefixes_with_trie(text: &str) -> Vec<&'static Prefix> {
    PREFIX_TRIE.find_all_prefixes(text)
}

/// Find the longest prefix match using the trie
pub fn find_longest_prefix_with_trie(text: &str) -> Option<&'static Prefix> {
    PREFIX_TRIE.find_longest_prefix(text)
}

/// Get cache statistics (WASM-compatible - returns dummy values)
/// Note: Caching has been disabled for WASM compatibility
pub fn get_cache_stats() -> Result<CacheStats, UcumError> {
    Ok(CacheStats::default())
}

/// Clear the global cache (WASM-compatible - no-op)
/// Note: Caching has been disabled for WASM compatibility
pub fn clear_global_cache() -> Result<(), UcumError> {
    Ok(())
}

/// Get global cache sizes (WASM-compatible - returns zero sizes)
/// Note: Caching has been disabled for WASM compatibility
pub fn get_cache_sizes() -> Result<(usize, usize, usize), UcumError> {
    Ok((0, 0, 0))
}

/// Access the global cache for custom operations (WASM-compatible - creates temporary cache)
/// Note: Caching has been disabled for WASM compatibility
pub fn with_global_cache<F, R>(f: F) -> Result<R, UcumError>
where
    F: FnOnce(&mut EvaluationCache) -> R,
{
    let mut temp_cache = EvaluationCache::new();
    Ok(f(&mut temp_cache))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::UnitExpr;

    #[test]
    fn test_cache_hash_consistency() {
        let expr1 = UnitExpr::Symbol("kg");
        let expr2 = UnitExpr::Symbol("kg");

        assert_eq!(
            EvaluationCache::hash_expr(&expr1),
            EvaluationCache::hash_expr(&expr2)
        );
    }

    #[test]
    fn test_prefix_trie() {
        let trie = PrefixTrie::new();

        // Test finding prefixes
        let prefixes = trie.find_all_prefixes("kilometer");
        assert!(!prefixes.is_empty());

        // Test longest prefix
        let longest = trie.find_longest_prefix("kilometer");
        assert!(longest.is_some());
    }

    #[test]
    fn test_optimized_lookups() {
        // Test unit lookup - "kg" should decompose to "k" prefix + "g" base unit
        let unit = find_unit_optimized("kg");
        assert!(unit.is_some());
        assert_eq!(unit.unwrap().code, "g");

        // Test prefix lookup
        let prefix = find_prefix_optimized("k");
        assert!(prefix.is_some());
        assert_eq!(prefix.unwrap().symbol, "k");
    }

    #[test]
    fn test_cache_stats() {
        let mut cache = EvaluationCache::new();

        // Initially empty
        assert_eq!(cache.stats().expression_hit_ratio(), 0.0);

        // Add some mock stats
        cache.stats.expression_hits = 7;
        cache.stats.expression_misses = 3;

        assert_eq!(cache.stats().expression_hit_ratio(), 0.7);
    }
}
