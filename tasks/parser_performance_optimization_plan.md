# UCUM Parser Performance Optimization Plan

## Current Analysis (Baseline: ~200,000 ops/second parsing)

### Performance Bottlenecks Identified

1. **String Allocations**: Excessive `String` allocations in AST construction
2. **Unicode Normalization**: `µ` → `u` replacement creates new strings unnecessarily 
3. **Pattern Validation**: Multiple string scans for invalid patterns
4. **Recursive Parsing**: Deep recursion in quotient parsing
5. **ASCII Lookups**: Character validation can be optimized further
6. **Nom Combinator Overhead**: Parser combinator allocations and backtracking

### Target Performance Goal
**Increase parsing performance from ~200,000 ops/second to ~500,000+ ops/second** (2.5x improvement)

---

## Phase 1: Zero-Copy String Optimizations (High Impact)

### 1.1 Lifetime-Based AST (`<'a>`)
**Goal**: Eliminate string allocations during parsing by borrowing from input

```rust
// Current
pub enum UnitExpr {
    Symbol(String),         // Allocates
    Numeric(f64),
    // ...
}

// Optimized
pub enum UnitExpr<'a> {
    Symbol(&'a str),        // Zero-copy
    SymbolOwned(String),    // Only when needed
    Numeric(f64),
    // ...
}
```

**Implementation Strategy**:
- Create `UnitExpr<'a>` for zero-copy parsing
- Keep `UnitExpr` (owned) for final results
- Add conversion: `UnitExpr<'a>` → `UnitExpr`

### 1.2 Lazy Unicode Normalization
**Goal**: Avoid allocations when no `µ` character present

```rust
// Current: Always allocates Cow
let normalized = if s.contains('µ') {
    Cow::Owned(s.replace('µ', "u"))
} else {
    Cow::Borrowed(s)
};

// Optimized: Defer normalization
fn normalize_if_needed(s: &str) -> Option<String> {
    if s.contains('µ') {
        Some(s.replace('µ', "u"))
    } else {
        None
    }
}
```

### 1.3 String Interning
**Goal**: Reuse common unit symbols to reduce memory pressure

```rust
use string_cache::DefaultAtom;

// For frequently used symbols like "m", "g", "s", etc.
pub enum UnitExpr<'a> {
    Symbol(&'a str),
    InternedSymbol(DefaultAtom),  // For common units
    Numeric(f64),
}
```

---

## Phase 2: Parser Structure Optimizations (Medium-High Impact)

### 2.1 Custom Tokenizer (Pre-lexing)
**Goal**: Single-pass tokenization to avoid repeated character scans

```rust
#[derive(Debug, Clone)]
pub enum Token<'a> {
    Symbol(&'a str),
    Number(f64),
    Operator(char),  // '.', '/', '^', '(', ')'
    Annotation(&'a str),
    Whitespace,
}

pub fn tokenize(input: &str) -> Vec<Token<'_>> {
    // Single pass through input creating tokens
    // Handles all character classification once
}
```

**Benefits**:
- Single character scan instead of multiple
- Better error reporting with token positions
- Enables parser memoization at token level

### 2.2 Optimized ASCII Validation
**Goal**: Vectorized character validation using SIMD-friendly techniques

```rust
// Current: Individual character checks
fn is_symbol_char(c: char) -> bool {
    if c.is_ascii() {
        SYMBOL_CHARS[c as usize]
    } else {
        c == 'µ'
    }
}

// Optimized: Batch validation
fn validate_symbol_fast(s: &str) -> bool {
    // Use slice operations for ASCII validation
    if s.is_ascii() {
        s.bytes().all(|b| SYMBOL_CHARS[b as usize])
    } else {
        s.chars().all(is_symbol_char)
    }
}
```

### 2.3 Pattern Validation Consolidation
**Goal**: Combine multiple pattern checks into single scan

```rust
#[derive(Default)]
struct ValidationState {
    has_invalid_time_pattern: bool,
    has_invalid_annotation: bool,
    has_parenthesis_issues: bool,
    invalid_position: Option<usize>,
}

fn validate_patterns_once(input: &str) -> Result<(), ValidationState> {
    // Single scan detecting all invalid patterns
    // Return early on first invalid pattern found
}
```

---

## Phase 3: Algorithm Optimizations (Medium Impact)

### 3.1 Iterative Parsing (Replace Recursion)
**Goal**: Convert recursive quotient parsing to iterative with explicit stack

```rust
pub fn parse_quotient_iterative(input: &str) -> IResult<&str, UnitExpr> {
    let mut stack = Vec::new();
    let mut operators = Vec::new();
    
    // Shunting-yard style parsing
    // Avoids function call overhead of recursion
    // Better cache locality
}
```

### 3.2 Specialized Number Parsing
**Goal**: Fast path for common numeric patterns

```rust
fn parse_number_fast(input: &str) -> Option<(f64, &str)> {
    // Fast path for integers: "10", "2", "1"
    if let Some((int_str, rest)) = try_parse_small_int(input) {
        return Some((int_str as f64, rest));
    }
    
    // Fast path for "10*n" and "10^n"
    if input.starts_with("10") {
        return try_parse_scientific(input);
    }
    
    // Fallback to nom's recognize_float
    None
}
```

### 3.3 Expression Builder Optimization
**Goal**: Reduce AST node allocations during construction

```rust
// Use object pooling for AST nodes
pub struct ExpressionBuilder {
    node_pool: Vec<UnitExpr>,
    factor_pool: Vec<UnitFactor>,
}

impl ExpressionBuilder {
    fn create_product(&mut self, factors: Vec<UnitFactor>) -> UnitExpr {
        // Reuse allocated nodes when possible
        // Inline single-factor products
    }
}
```

---

## Phase 4: Parser Memoization (Medium Impact)

### 4.1 LRU Parse Cache
**Goal**: Cache parsing results for repeated expressions

```rust
use lru::LruCache;
use std::num::NonZeroUsize;

pub struct MemoizedParser {
    cache: LruCache<String, UnitExpr>,
    hit_count: u64,
    miss_count: u64,
}

impl MemoizedParser {
    pub fn parse_cached(&mut self, input: &str) -> Result<UnitExpr, ParseError> {
        if let Some(cached) = self.cache.get(input) {
            self.hit_count += 1;
            return Ok(cached.clone());
        }
        
        let result = parse_expression(input)?;
        self.cache.put(input.to_string(), result.clone());
        self.miss_count += 1;
        Ok(result)
    }
}
```

**Configuration**:
- Cache size: 1000-5000 entries
- LRU eviction policy
- Optional: Persistent cache for CLI usage

### 4.2 Sub-expression Caching
**Goal**: Cache parsing of common sub-expressions like "mg", "dL", "m/s"

```rust
// Cache factor-level parsing for maximum reuse
fn parse_factor_cached(&mut self, input: &str) -> IResult<&str, UnitFactor> {
    let cache_key = input;
    if let Some(cached) = self.factor_cache.get(cache_key) {
        return cached.clone();
    }
    
    let result = parse_factor(input);
    if result.is_ok() {
        self.factor_cache.put(cache_key.to_string(), result.clone());
    }
    result
}
```

---

## Phase 5: Nom Combinator Optimizations (Low-Medium Impact)

### 5.1 Custom Combinators
**Goal**: Replace general nom combinators with specialized versions

```rust
// Instead of generic `alt` combinator, create specialized versions
fn parse_atomic_fast(input: &str) -> IResult<&str, UnitExpr> {
    // Manual dispatch based on first character
    match input.chars().next() {
        Some('(') => parse_paren_expr(input),
        Some('1') if input.starts_with("10") => parse_numeric(input),
        Some(c) if c.is_ascii_digit() => parse_decimal(input),
        Some('{') => parse_standalone_annotation(input),
        _ => parse_symbol(input),
    }
}
```

### 5.2 Reduced Backtracking
**Goal**: Design parser to minimize nom's backtracking overhead

```rust
// Current: Multiple alternatives that can backtrack
alt((
    parse_paren_expr,
    parse_numeric,      // Can fail and backtrack
    parse_decimal,      // Can fail and backtrack  
    parse_symbol,       // Fallback
))

// Optimized: Lookahead to avoid backtracking
fn parse_base_atomic_optimized(input: &str) -> IResult<&str, UnitExpr> {
    // Single character lookahead determines parser path
    // No backtracking needed
}
```

---

## Phase 6: Micro-optimizations (Low Impact)

### 6.1 Branch Prediction Optimization
```rust
// Hot path optimization for common cases
#[inline(always)]  
fn is_common_unit_char(c: u8) -> bool {
    // Optimize for most common characters: a-z, A-Z
    matches!(c, b'a'..=b'z' | b'A'..=b'Z')
}

// Cold path for special characters
#[cold]
fn is_special_unit_char(c: char) -> bool {
    matches!(c, '%' | '_' | '[' | ']' | '\'' | '-' | '+' | 'µ')
}
```

### 6.2 Memory Pool for Temporary Allocations
```rust
pub struct ParseContext {
    string_pool: Vec<String>,  // Reuse string allocations
    factor_pool: Vec<UnitFactor>, // Reuse factor vectors
}

impl ParseContext {
    fn get_temp_string(&mut self) -> String {
        self.string_pool.pop().unwrap_or_default()
    }
    
    fn return_temp_string(&mut self, mut s: String) {
        s.clear();
        self.string_pool.push(s);
    }
}
```

---

## Implementation Priority & Timeline

### High Priority (Target: 2-3x performance gain)
1. **Phase 1**: Zero-copy string optimizations
2. **Phase 2.1**: Custom tokenizer 
3. **Phase 2.3**: Pattern validation consolidation
4. **Phase 3.1**: Iterative parsing

### Medium Priority (Target: additional 20-50% gain)
1. **Phase 4**: Parser memoization (if expression reuse is high)
2. **Phase 2.2**: ASCII validation optimization
3. **Phase 3.2**: Specialized number parsing

### Low Priority (Target: additional 5-15% gain)
1. **Phase 5**: Nom combinator optimizations
2. **Phase 6**: Micro-optimizations

---

## Development Best Practices

### 1. Benchmarking Strategy
```rust
// Comprehensive benchmark suite
mod benches {
    use criterion::{black_box, criterion_group, criterion_main, Criterion};
    
    fn bench_parser_suite(c: &mut Criterion) {
        let test_cases = [
            "mg/dL",           // Simple quotient
            "kg.m/s2",         // Product with quotient
            "10*23",           // Scientific notation
            "{annotation}",    // Annotation
            "complex/expression/with/multiple/divisions",
        ];
        
        c.bench_function("parse_current", |b| {
            b.iter(|| {
                for case in &test_cases {
                    black_box(parse_expression(case));
                }
            })
        });
    }
}
```

### 2. Regression Testing
- Maintain 100% compatibility with current parser behavior
- All existing tests must pass unchanged
- Add performance regression tests in CI

### 3. Profiling Integration
```rust
#[cfg(feature = "profiling")]
mod profiling {
    use std::time::Instant;
    
    pub fn profile_parse_step<T>(name: &str, f: impl FnOnce() -> T) -> T {
        let start = Instant::now();
        let result = f();
        println!("{}: {:?}", name, start.elapsed());
        result
    }
}
```

### 4. Memory Usage Monitoring
- Track allocation patterns during parsing
- Monitor peak memory usage with complex expressions
- Ensure optimizations don't increase memory pressure

### 5. Error Handling Preservation
- Maintain exact same error messages and types
- Preserve error position information
- Keep validation behavior identical

---

## Success Metrics

### Performance Targets
- **Parsing**: 200K → 500K+ ops/second (2.5x improvement)
- **Memory**: Reduce allocations by 60%+ 
- **Latency**: Sub-microsecond parsing for simple expressions

### Quality Targets
- **Zero regressions**: All existing tests pass
- **Memory safety**: No unsafe code
- **Maintainability**: Code remains readable and documented
- **API compatibility**: No breaking changes to public interface

---

## Risk Mitigation

### Technical Risks
1. **Complexity increase**: Mitigate with comprehensive tests and documentation
2. **Memory leaks**: Use Rust's ownership system and thorough testing
3. **Performance regressions**: Benchmark every change
4. **Unicode handling**: Ensure proper µ normalization in zero-copy design

### Implementation Risks  
1. **Over-optimization**: Focus on high-impact changes first
2. **API changes**: Keep optimizations internal to parser module
3. **Testing burden**: Automate performance testing in CI

This plan provides a systematic approach to achieving significant parser performance improvements while maintaining code quality and safety.