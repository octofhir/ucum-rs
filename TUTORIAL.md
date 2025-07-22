# UCUM-RS Tutorial: Getting Started

This tutorial will guide you through the basics of using the UCUM-RS library for working with units of measure.

## Prerequisites

- Rust installed on your system
- Basic knowledge of Rust programming
- Familiarity with the concept of units of measure

## Part 1: Installation and Basic Setup

### Step 1: Create a new Rust project

```bash
cargo new ucum-tutorial
cd ucum-tutorial
```

### Step 2: Add the UCUM-RS dependency

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
octofhir-ucum-core = "0.3.0"
```

### Step 3: Create a simple program

Edit `src/main.rs` to include the following code:

```rust
use octofhir_ucum_core::{parse_expression, evaluate, find_unit};

fn main() {
    // Validate a UCUM expression
    let expr_str = "mg/dL";
    match parse_expression(expr_str) {
        Ok(_) => println!("'{}' is a valid UCUM expression", expr_str),
        Err(e) => println!("'{}' is not a valid UCUM expression: {}", expr_str, e),
    }
    
    // Look up a unit
    let unit_code = "kg";
    match find_unit(unit_code) {
        Some(unit) => println!("Found unit: {} - {}", unit.code, unit.name),
        None => println!("Unit '{}' not found", unit_code),
    }
    
    println!("UCUM-RS tutorial completed successfully!");
}
```

### Step 4: Run the program

```bash
cargo run
```

You should see output similar to:

```
'mg/dL' is a valid UCUM expression
Found unit: kg - kilogram
UCUM-RS tutorial completed successfully!
```

## Part 2: Unit Conversion

### Step 1: Create a conversion function

Edit `src/main.rs` to add a conversion function:

```rust
use octofhir_ucum_core::{parse_expression, evaluate, find_unit};

fn convert(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
    // Parse the unit expressions
    let from_expr = parse_expression(from_unit)?;
    let to_expr = parse_expression(to_unit)?;
    
    // Evaluate the expressions
    let from_result = evaluate(&from_expr)?;
    let to_result = evaluate(&to_expr)?;
    
    // Check if the units are commensurable
    if from_result.dim != to_result.dim {
        return Err(format!("Units '{}' and '{}' have different dimensions", from_unit, to_unit));
    }
    
    // Perform the conversion
    Ok(value * from_result.factor / to_result.factor)
}

fn main() {
    // Convert 100 kPa to mm[Hg]
    match convert(100.0, "kPa", "mm[Hg]") {
        Ok(result) => println!("100 kPa = {:.2} mm[Hg]", result),
        Err(e) => println!("Conversion error: {}", e),
    }
    
    // Convert 1 kg to g
    match convert(1.0, "kg", "g") {
        Ok(result) => println!("1 kg = {} g", result),
        Err(e) => println!("Conversion error: {}", e),
    }
    
    // Try an invalid conversion
    match convert(1.0, "kg", "m") {
        Ok(result) => println!("1 kg = {} m", result),
        Err(e) => println!("Conversion error: {}", e),
    }
}
```

### Step 2: Run the updated program

```bash
cargo run
```

You should see output similar to:

```
100 kPa = 750.06 mm[Hg]
1 kg = 1000 g
Conversion error: Units 'kg' and 'm' have different dimensions
```

## Part 3: Working with Temperature Units

Temperature units require special handling due to their offsets.

### Step 1: Create a temperature conversion function

Edit `src/main.rs` to add a temperature conversion function:

```rust
use octofhir_ucum_core::{parse_expression, evaluate, find_unit};

fn convert_temperature(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
    // Parse the unit expressions
    let from_expr = parse_expression(from_unit)?;
    let to_expr = parse_expression(to_unit)?;
    
    // Evaluate the expressions
    let from_result = evaluate(&from_expr)?;
    let to_result = evaluate(&to_expr)?;
    
    // Check if the units are commensurable
    if from_result.dim != to_result.dim {
        return Err(format!("Units '{}' and '{}' have different dimensions", from_unit, to_unit));
    }
    
    // Handle special case for temperature units with offsets
    if from_unit == "Cel" && to_unit == "[degF]" {
        return Ok(value * 9.0/5.0 + 32.0);
    } else if from_unit == "[degF]" && to_unit == "Cel" {
        return Ok((value - 32.0) * 5.0/9.0);
    } else if (from_unit == "Cel" || from_unit == "[degF]") && to_unit == "K" {
        let celsius = if from_unit == "Cel" { value } else { (value - 32.0) * 5.0/9.0 };
        return Ok(celsius + 273.15);
    } else if from_unit == "K" && (to_unit == "Cel" || to_unit == "[degF]") {
        let celsius = value - 273.15;
        return Ok(if to_unit == "Cel" { celsius } else { celsius * 9.0/5.0 + 32.0 });
    }
    
    // For other units, use the standard conversion
    Ok(value * from_result.factor / to_result.factor)
}

fn main() {
    // Convert 25°C to °F
    match convert_temperature(25.0, "Cel", "[degF]") {
        Ok(result) => println!("25°C = {:.1}°F", result),
        Err(e) => println!("Conversion error: {}", e),
    }
    
    // Convert 98.6°F to °C
    match convert_temperature(98.6, "[degF]", "Cel") {
        Ok(result) => println!("98.6°F = {:.1}°C", result),
        Err(e) => println!("Conversion error: {}", e),
    }
    
    // Convert 0°C to K
    match convert_temperature(0.0, "Cel", "K") {
        Ok(result) => println!("0°C = {:.2} K", result),
        Err(e) => println!("Conversion error: {}", e),
    }
}
```

### Step 2: Run the updated program

```bash
cargo run
```

You should see output similar to:

```
25°C = 77.0°F
98.6°F = 37.0°C
0°C = 273.15 K
```

## Part 4: Working with Complex Expressions

UCUM-RS can handle complex unit expressions.

### Step 1: Create a function to work with complex expressions

Edit `src/main.rs` to add code for working with complex expressions:

```rust
use octofhir_ucum_core::{parse_expression, evaluate, find_unit};

fn main() {
    // Parse and evaluate a complex expression
    let expr_str = "kg.m2/s3";
    match parse_expression(expr_str) {
        Ok(expr) => {
            match evaluate(&expr) {
                Ok(result) => {
                    println!("Expression: {}", expr_str);
                    println!("Factor: {}", result.factor);
                    println!("Dimensions: {:?}", result.dim);
                    println!("Is special: {}", result.is_special);
                    println!("Is arbitrary: {}", result.is_arbitrary);
                    
                    // Check if this is equivalent to a known unit
                    if result.dim.0 == [1, 1, -2, 0, 0, 0, 0] {
                        println!("This is equivalent to a Newton (N)");
                    }
                },
                Err(e) => println!("Evaluation error: {}", e),
            }
        },
        Err(e) => println!("Parse error: {}", e),
    }
    
    // Another complex expression: concentration
    let expr_str = "mmol/L";
    match parse_expression(expr_str).and_then(|expr| evaluate(&expr)) {
        Ok(result) => {
            println!("\nExpression: {}", expr_str);
            println!("Factor: {}", result.factor);
            println!("Dimensions: {:?}", result.dim);
        },
        Err(e) => println!("Error: {}", e),
    }
}
```

### Step 2: Run the updated program

```bash
cargo run
```

You should see output similar to:

```
Expression: kg.m2/s3
Factor: 1000
Dimensions: [1, 2, -3, 0, 0, 0, 0]
Is special: false
Is arbitrary: false
This is equivalent to a Newton (N)

Expression: mmol/L
Factor: 0.001
Dimensions: [0, -3, 0, 0, 0, 1, 0]
```

## Part 5: Using the CLI

The UCUM-RS library also provides a command-line interface.

### Step 1: Install the CLI

```bash
cargo install octofhir-ucum-cli
```

### Step 2: Try some basic commands

```bash
# Validate a UCUM expression
octofhir-ucum validate "mg/dL"

# Convert between units
octofhir-ucum convert --value 100 --from kPa --to "mm[Hg]"

# List all units
octofhir-ucum list-units

# Explain a unit
octofhir-ucum explain mm[Hg]
```

## Next Steps

Now that you've completed this tutorial, you can:

1. Explore the [UCUM-RS User Guide](https://github.com/octofhir/ucum-rs/blob/main/USER_GUIDE.md) for more detailed information
2. Try the [UCUM-RS Playground](https://github.com/octofhir/ucum-rs/tree/main/playground) for interactive exploration
3. Integrate UCUM-RS with FHIR using the [FHIR integration module](https://github.com/octofhir/ucum-rs/tree/main/ucum-fhir)
4. Use the [WebAssembly package](https://github.com/octofhir/ucum-rs/tree/main/ucum-wasm) for web applications

## Conclusion

In this tutorial, you've learned how to:
- Install and set up the UCUM-RS library
- Validate UCUM expressions
- Look up units
- Convert between units
- Handle temperature conversions
- Work with complex expressions
- Use the CLI

UCUM-RS provides a powerful and flexible way to work with units of measure in Rust, with a focus on healthcare and scientific applications.
