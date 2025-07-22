# octofhir-ucum-core

UCUM (Unified Code for Units of Measure) core library for FHIRPath quantity operations, written in Rust (2024 edition).

## Features

### Core Capabilities
- 🔍 **Comprehensive Validation** - Validate UCUM expressions with detailed error reporting
- 📊 **Unit Analysis** - Detailed analysis with dimensions, factors, and properties
- 🧮 **Unit Arithmetic** - Mathematical operations on unit expressions (multiply, divide)
- 🔎 **Advanced Search** - Find units by name, property, or fuzzy matching
- 🏷️ **Property Validation** - Validate units against physical properties (length, mass, etc.)
- 🌡️ **Temperature Support** - Full support for temperature units with offsets
- ⚡ **High Performance** - Microsecond-level operations with optional precision arithmetic

### Technical Features
- Parse, validate, and convert UCUM unit expressions
- SI, customary, specialized, and information units
- Prefix and factor handling with precision arithmetic support
- Extensible special unit system with pluggable handlers
- Robust error messages with context
- 98.6% conformance to official UCUM test suite
- Suitable for both embedded and server environments (`no_std` optional)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
octofhir-ucum-core = "0.2.0"
```

## Basic Usage

The core library provides functions for parsing, evaluating, and converting UCUM expressions. For detailed code examples, please refer to the [UCUM-RS User Guide](https://github.com/octofhir/ucum-rs/blob/main/USER_GUIDE.md).

### Parsing Expressions

Use `parse_expression` to parse a UCUM expression into an abstract syntax tree (AST). This function takes a string representation of a UCUM expression and returns a `Result<UnitExpr, String>`.

### Evaluating Expressions

Use `evaluate` to evaluate a parsed expression and determine its canonical form, conversion factor, and dimensions. This function takes a reference to a `UnitExpr` and returns a `Result<EvaluationResult, String>`.

### Looking Up Units

Use `find_unit` to look up a unit by its code. This function takes a string code and returns an `Option<&Unit>`.

### Converting Between Units

To convert a value from one unit to another, you need to:
1. Parse both unit expressions
2. Evaluate both expressions
3. Check if the units have the same dimensions
4. Apply the conversion factor

## Advanced Usage

### Working with Complex Expressions

The library can handle complex unit expressions, including:
- Power units (e.g., "kg.m2/s3")
- Concentration units (e.g., "mmol/L")
- Count per volume (e.g., "10*3/L")
- Annotated units (e.g., "mm[Hg]")

### Temperature Conversions

Temperature units require special handling due to their offsets:
- Celsius ("Cel")
- Fahrenheit ("[degF]")
- Kelvin ("K")

When converting between temperature units, you need to handle the offsets:
- Celsius to Fahrenheit: °F = °C × 9/5 + 32
- Fahrenheit to Celsius: °C = (°F - 32) × 5/9
- Celsius to Kelvin: K = °C + 273.15
- Kelvin to Celsius: °C = K - 273.15

### Working with Arbitrary Units

Arbitrary units are units that are not defined in terms of any other unit:
- International Unit ("[IU]")
- Arbitrary Unit ("[arb'U]")
- United States Pharmacopeia Unit ("[USP'U]")

Arbitrary units:
- Are enclosed in square brackets
- Have a factor of 1.0
- Are dimensionless
- Cannot be converted to non-arbitrary units
- Can be prefixed (e.g., "k[IU]")
- Can be combined with other units (e.g., "[IU]/mL")

### Error Handling

The library provides comprehensive error handling for:
- Parse errors (invalid syntax or unrecognized unit)
- Evaluation errors (error evaluating the expression)
- Incommensurable units (units with different dimensions)

## API Reference

### Core Functions

#### `parse_expression(expr: &str) -> Result<UnitExpr, UcumError>`

Parse a UCUM expression into an abstract syntax tree (AST).

#### `evaluate(expr: &UnitExpr) -> Result<EvalResult, UcumError>`

Evaluate a parsed UCUM expression to determine its canonical form, conversion factor, and dimensions.

#### `analyse(expression: &str) -> Result<UnitAnalysis, UcumError>`

Perform comprehensive analysis of a UCUM expression, returning detailed information about its properties.

#### `validate(expression: &str) -> Result<bool, UcumError>`

Validate a UCUM expression for correctness.

#### `validate_in_property(expression: &str, property: &str) -> Result<bool, UcumError>`

Validate that a unit expression is appropriate for a given physical property.

#### `is_comparable(expr1: &str, expr2: &str) -> Result<bool, UcumError>`

Check if two unit expressions are commensurable (have the same dimensions).

#### `unit_multiply(left: &str, right: &str) -> Result<String, UcumError>`

Multiply two unit expressions.

#### `unit_divide(left: &str, right: &str) -> Result<String, UcumError>`

Divide two unit expressions.

#### `find_unit(code: &str) -> Option<&UnitRecord>`

Look up a unit by its code.

### Data Structures

#### `UnitExpr`

Represents a parsed UCUM expression.

```
pub enum UnitExpr {
    Simple(String),
    Annotated(String, String),
    Prefixed(String, Box<UnitExpr>),
    Power(Box<UnitExpr>, i32),
    Multiply(Box<UnitExpr>, Box<UnitExpr>),
    Divide(Box<UnitExpr>, Box<UnitExpr>),
}
```

#### `EvalResult`

Represents the result of evaluating a UCUM expression.

```rust
pub struct EvalResult {
    pub factor: Number,
    pub dim: Dimension,
    pub offset: Number,
}
```

#### `UnitAnalysis`

Represents detailed analysis of a UCUM expression.

```rust
pub struct UnitAnalysis {
    pub expression: String,
    pub parsed_ast: UnitExpr,
    pub dimension: Dimension,
    pub factor: f64,
    pub offset: f64,
    pub is_dimensionless: bool,
    pub has_offset: bool,
}
```

#### `UnitRecord`

Represents a UCUM unit record.

```rust
pub struct UnitRecord {
    pub code: String,
    pub name: String,
    pub print_symbol: Option<String>,
    pub property: Option<String>,
    pub is_metric: bool,
    pub is_special: bool,
    pub is_arbitrary: bool,
    pub class: String,
    pub factor: f64,
    pub dim: Dimension,
    pub offset: f64,
}
```

#### `Dimension`

Represents the physical dimensions of a unit.

```
pub struct Dimension(pub [i32; 7]);
```

The seven components represent:
1. Mass (M)
2. Length (L)
3. Time (T)
4. Electric current (I)
5. Temperature (Θ)
6. Amount of substance (N)
7. Luminous intensity (J)

#### `UcumError`

Represents errors that can occur when working with UCUM expressions.

```
pub enum UcumError {
    ParseError(String),
    EvaluationError(String),
    IncommensurableUnits(String),
    // Other error variants...
}
```

## UCUM Specification Implementation

This library implements the [Unified Code for Units of Measure (UCUM) specification](https://ucum.org/ucum.html), which defines a system for unambiguous representation of units of measure.

### Base Units

The library implements all base units defined in the UCUM specification:

| Dimension | Base Unit | Symbol | UCUM Code |
|-----------|-----------|--------|-----------|
| Mass | kilogram | kg | kg |
| Length | meter | m | m |
| Time | second | s | s |
| Electric current | ampere | A | A |
| Temperature | kelvin | K | K |
| Amount of substance | mole | mol | mol |
| Luminous intensity | candela | cd | cd |

### Derived Units

The library implements all derived units defined in the UCUM specification, including:

| Unit | Symbol | UCUM Code | Definition |
|------|--------|-----------|------------|
| Newton | N | N | kg·m/s² |
| Pascal | Pa | Pa | N/m² |
| Joule | J | J | N·m |
| Watt | W | W | J/s |
| Coulomb | C | C | A·s |
| Volt | V | V | W/A |
| Farad | F | F | C/V |
| Ohm | Ω | Ohm | V/A |
| Siemens | S | S | A/V |
| Weber | Wb | Wb | V·s |
| Tesla | T | T | Wb/m² |
| Henry | H | H | Wb/A |
| Lumen | lm | lm | cd·sr |
| Lux | lx | lx | lm/m² |
| Becquerel | Bq | Bq | 1/s |
| Gray | Gy | Gy | J/kg |
| Sievert | Sv | Sv | J/kg |
| Katal | kat | kat | mol/s |

## Extending the Library

### Adding Custom Units

To add a custom unit, you need to define:
- A unique code
- A canonical form
- Conversion factors to base units
- Dimensional information

The library provides a registry system for adding custom units. See the [UCUM-RS User Guide](https://github.com/octofhir/ucum-rs/blob/main/USER_GUIDE.md) for detailed examples of how to extend the library with custom units.

## License

MIT OR Apache-2.0

## See Also

- [UCUM-RS User Guide](https://github.com/octofhir/ucum-rs/blob/main/USER_GUIDE.md) - Comprehensive guide to using the UCUM-RS library
- [UCUM Specification](https://ucum.org/ucum.html) - Official UCUM specification
- [FHIR Quantity](https://www.hl7.org/fhir/datatypes.html#Quantity) - FHIR Quantity data type that uses UCUM
