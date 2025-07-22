# UCUM-RS Enhanced API Tutorial

Welcome to the comprehensive tutorial for UCUM-RS's enhanced API! This guide covers all the new features implemented in ADR-001, transforming UCUM-RS from a basic unit parser into a comprehensive UCUM library.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Validation and Analysis](#validation-and-analysis)
3. [Unit Arithmetic](#unit-arithmetic)
4. [Search and Discovery](#search-and-discovery)
5. [Property-Based Validation](#property-based-validation)
6. [Special Units and Temperature](#special-units-and-temperature)
7. [Advanced Features](#advanced-features)
8. [Performance Considerations](#performance-considerations)

## Getting Started

### Installation

Add UCUM-RS to your project:

```toml
[dependencies]
octofhir-ucum-core = "0.1.8"

# Optional: Enable high-precision arithmetic
octofhir-ucum-core = { version = "0.1.8", features = ["precision"] }
```

### Basic Import

```rust
use octofhir_ucum_core::*;
```

## Validation and Analysis

### Simple Validation

The `validate()` function provides comprehensive validation of UCUM expressions:

```rust
// Valid units
assert!(validate("m/s").is_ok());
assert!(validate("kg.m/s2").is_ok());
assert!(validate("Cel").is_ok());

// Invalid units
assert!(validate("invalid_unit").is_err());
assert!(validate("m/s/").is_err()); // Trailing slash
```

### Detailed Analysis

The `analyse()` function provides comprehensive information about units:

```rust
let analysis = analyse("km/h").unwrap();

println!("Original expression: {}", analysis.expression);
println!("Conversion factor: {}", analysis.factor);        // 0.277778
println!("Dimension vector: {:?}", analysis.dimension);    // [0, 1, -1, 0, 0, 0, 0]
println!("Is dimensionless: {}", analysis.is_dimensionless); // false
println!("Has offset: {}", analysis.has_offset);           // false

// Temperature units have offsets
let temp_analysis = analyse("Cel").unwrap();
println!("Temperature offset: {}", temp_analysis.offset);  // 273.15
println!("Has offset: {}", temp_analysis.has_offset);      // true
```

### Understanding Dimension Vectors

Dimension vectors represent the fundamental physical dimensions:
- `[M, L, T, I, Θ, N, J]` where:
  - M = Mass (kg)
  - L = Length (m)  
  - T = Time (s)
  - I = Electric current (A)
  - Θ = Temperature (K)
  - N = Amount of substance (mol)
  - J = Luminous intensity (cd)

```rust
let force = analyse("N").unwrap();
println!("{:?}", force.dimension); // [1, 1, -2, 0, 0, 0, 0] = kg⋅m⋅s⁻²

let energy = analyse("J").unwrap();
println!("{:?}", energy.dimension); // [1, 2, -2, 0, 0, 0, 0] = kg⋅m²⋅s⁻²
```

## Unit Arithmetic

### Multiplication

Multiply units to create compound units:

```rust
let result = unit_multiply("m", "s").unwrap();
println!("{}", result.expression); // "m.s"
println!("Factor: {}", result.factor); // 1.0

let force_result = unit_multiply("kg", "m/s2").unwrap();
println!("{}", force_result.expression); // "kg.m/s2"
println!("Dimension: {:?}", force_result.dimension); // [1, 1, -2, 0, 0, 0, 0]
```

### Division

Divide units to create ratios:

```rust
let velocity = unit_divide("m", "s").unwrap();
println!("{}", velocity.expression); // "m/s"

let power = unit_divide("J", "s").unwrap();
println!("{}", power.expression); // "J/s"
println!("Factor: {}", power.factor); // 1000.0 (J = 1000 g⋅m²⋅s⁻²)
```

### Error Handling

Unit arithmetic has safety checks:

```rust
// Temperature units cannot participate in arithmetic
let result = unit_multiply("Cel", "m");
assert!(result.is_err()); // Error: offset units cannot be used

// Division by zero is prevented
// (This would be caught at the value level, not unit level)
```

## Search and Discovery

### Text Search

Find units by searching their names or codes:

```rust
let results = search_units("meter");
for unit in results.iter().take(5) {
    println!("{}: {}", unit.code, unit.display_name);
}
// Output:
// m: meter
// [BAU]: bioequivalent allergen unit
// m[Hg]: meter of mercury column
// ...
```

### Property-Based Search

Find all units of a specific physical property:

```rust
let length_units = search_units_by_property("length");
println!("Found {} length units", length_units.len());

let mass_units = search_units_by_property("mass");
let time_units = search_units_by_property("time");
```

### Unit Variants

Get all defined forms of a base unit:

```rust
let gram_forms = get_defined_forms("g");
for unit in gram_forms {
    println!("{}: {}", unit.code, unit.display_name);
}
// Output includes: g, kg, mg, µg, etc.
```

### Advanced Search Options

```rust
// Regex search (case-insensitive by default)
let regex_results = search_units_regex(r"mete?r", false).unwrap();

// Fuzzy search with threshold
let fuzzy_results = search_units_fuzzy("meter", 50);
for (unit, score) in fuzzy_results.iter().take(3) {
    println!("{}: {} (score: {})", unit.code, unit.display_name, score);
}

// Filtered search by concept kind
let base_units = search_units_filtered("meter", &[ConceptKind::BaseUnit], false);
```

## Property-Based Validation

### Basic Property Validation

Validate that units match expected physical properties:

```rust
// Valid combinations
assert!(validate_in_property("m", "length").unwrap());
assert!(validate_in_property("kg", "mass").unwrap());
assert!(validate_in_property("s", "time").unwrap());

// Invalid combinations
assert!(!validate_in_property("m", "mass").unwrap());
assert!(!validate_in_property("kg", "length").unwrap());
```

### Supported Properties

The system recognizes these physical properties:

```rust
// Basic SI properties
validate_in_property("m", "length").unwrap();      // true
validate_in_property("kg", "mass").unwrap();       // true
validate_in_property("s", "time").unwrap();        // true
validate_in_property("A", "current").unwrap();     // true
validate_in_property("K", "temperature").unwrap(); // true
validate_in_property("mol", "amount").unwrap();    // true
validate_in_property("cd", "luminosity").unwrap(); // true

// Derived properties
validate_in_property("m2", "area").unwrap();           // true
validate_in_property("m3", "volume").unwrap();         // true
validate_in_property("m/s", "velocity").unwrap();      // true
validate_in_property("m/s2", "acceleration").unwrap(); // true
validate_in_property("N", "force").unwrap();           // true
validate_in_property("J", "energy").unwrap();          // true
validate_in_property("W", "power").unwrap();           // true
validate_in_property("Pa", "pressure").unwrap();       // true
validate_in_property("Hz", "frequency").unwrap();      // true
```

## Special Units and Temperature

### Unit Compatibility

Check if units can be converted between each other:

```rust
// Compatible units (same dimension)
assert!(is_comparable("m", "km").unwrap());     // Both length
assert!(is_comparable("kg", "g").unwrap());     // Both mass
assert!(is_comparable("Cel", "K").unwrap());    // Both temperature

// Incompatible units (different dimensions)
assert!(!is_comparable("m", "kg").unwrap());    // Length vs mass
assert!(!is_comparable("s", "A").unwrap());     // Time vs current
```

### Canonical Units

Get the canonical (base) form of units:

```rust
let canonical = get_canonical_units("km").unwrap();
println!("Canonical: {}", canonical.unit);     // "m"
println!("Factor: {}", canonical.factor);      // 1000.0

let pressure_canonical = get_canonical_units("kPa").unwrap();
println!("Canonical: {}", pressure_canonical.unit); // "kg.m-1.s-2"
println!("Factor: {}", pressure_canonical.factor);  // 1000000.0
```

### Temperature Handling

Temperature units are handled specially due to their offsets:

```rust
let celsius = analyse("Cel").unwrap();
println!("Offset: {}", celsius.offset);        // 273.15
println!("Has offset: {}", celsius.has_offset); // true

// Temperature units are compatible
assert!(is_comparable("Cel", "[degF]").unwrap());
assert!(is_comparable("Cel", "K").unwrap());
```

## Advanced Features

### Special Unit Registry

The library includes an extensible special unit system:

```rust
let registry = SpecialUnitRegistry::default();

// Find handlers for special units
if let Some(handler) = registry.find_handler("Cel") {
    println!("Handler: {}", handler.name()); // "Temperature"
}

// The registry includes:
// - TemperatureHandler: Celsius, Fahrenheit, Rankine
// - LogarithmicHandler: Decibels, bels, nepers  
// - ArbitraryHandler: Square-bracketed arbitrary units
```

### Precision Arithmetic

Enable high-precision arithmetic with the `precision` feature:

```toml
[dependencies]
octofhir-ucum-core = { version = "0.1.8", features = ["precision"] }
```

```rust
// With precision feature enabled, calculations use rust_decimal::Decimal
// instead of f64 for higher precision in complex calculations
```

### Error Handling Patterns

```rust
match validate("invalid_unit") {
    Ok(()) => println!("Valid unit"),
    Err(UcumError::UnknownUnit(unit)) => {
        println!("Unknown unit: {}", unit);
    }
    Err(UcumError::InvalidExpression) => {
        println!("Invalid expression syntax");
    }
    Err(e) => println!("Other error: {}", e),
}
```

## Performance Considerations

### Benchmark Results

UCUM-RS provides excellent performance:

- **Parsing**: ~5.01 µs for multiple unit expressions
- **Evaluation**: ~718 ns for parsed expressions  
- **Validation**: ~3.11 µs for comprehensive validation
- **Analysis**: ~1.65 µs for detailed unit analysis
- **Arithmetic**: ~1.09 µs for multiplication/division

### Optimization Tips

1. **Cache parsed expressions** for repeated use:
   ```rust
   let expr = parse_expression("kg.m/s2").unwrap();
   // Reuse `expr` for multiple evaluations
   ```

2. **Use validation before analysis** for unknown inputs:
   ```rust
   if validate(user_input).is_ok() {
       let analysis = analyse(user_input).unwrap();
       // Process analysis...
   }
   ```

3. **Enable precision feature only when needed** for optimal performance.

## Complete Example

Here's a comprehensive example using multiple features:

```rust
use octofhir_ucum_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Validate user input
    let user_unit = "kg.m/s2";
    validate(user_unit)?;
    
    // Analyze the unit
    let analysis = analyse(user_unit)?;
    println!("Unit: {} (factor: {:.6})", user_unit, analysis.factor);
    
    // Check if it's a force unit
    if validate_in_property(user_unit, "force")? {
        println!("This is a force unit!");
    }
    
    // Find the canonical form
    let canonical = get_canonical_units(user_unit)?;
    println!("Canonical: {} (factor: {})", canonical.unit, canonical.factor);
    
    // Search for related units
    let force_units = search_units_by_property("force");
    println!("Found {} force units", force_units.len());
    
    // Perform unit arithmetic
    let doubled = unit_multiply(user_unit, "2")?;
    println!("Doubled: {}", doubled.expression);
    
    Ok(())
}
```

## Conclusion

The enhanced UCUM-RS API provides comprehensive unit handling capabilities that match the official Java reference implementation while leveraging Rust's type safety and performance. With 98.6% conformance to official UCUM tests and microsecond-level performance, it's ready for production use in any application requiring robust unit handling.

For more examples, see the `comprehensive_api_demo` example in the repository.
