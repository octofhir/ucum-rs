# UCUM-RS Advanced Tutorial

This advanced tutorial covers complex use cases for the UCUM-RS library, building on the concepts introduced in the [Getting Started Tutorial](TUTORIAL.md).

## Prerequisites

- Completion of the [Getting Started Tutorial](TUTORIAL.md)
- Intermediate knowledge of Rust programming
- Understanding of dimensional analysis and unit conversions

## Part 1: Custom Error Handling

### Step 1: Create a new Rust project

```bash
cargo new ucum-advanced
cd ucum-advanced
```

### Step 2: Add the UCUM-RS dependency

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
octofhir-ucum-core = "0.3.0"
```

### Step 3: Implement custom error handling

Create `src/main.rs` with the following code:

```rust
use octofhir_ucum_core::{evaluate, parse_expression, UcumError};
use std::error::Error;
use std::fmt;

// Define a custom error type
#[derive(Debug)]
enum ConversionError {
    ParseError(String),
    EvaluationError(String),
    IncommensurableUnits(String),
    Other(String),
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConversionError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ConversionError::EvaluationError(msg) => write!(f, "Evaluation error: {}", msg),
            ConversionError::IncommensurableUnits(msg) => write!(f, "Incommensurable units: {}", msg),
            ConversionError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

impl Error for ConversionError {}

// Conversion function with custom error handling
fn convert(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, ConversionError> {
    // Parse the unit expressions
    let from_expr = parse_expression(from_unit)
        .map_err(|e| ConversionError::ParseError(format!("Failed to parse '{}': {}", from_unit, e)))?;
    
    let to_expr = parse_expression(to_unit)
        .map_err(|e| ConversionError::ParseError(format!("Failed to parse '{}': {}", to_unit, e)))?;
    
    // Evaluate the expressions
    let from_result = evaluate(&from_expr)
        .map_err(|e| ConversionError::EvaluationError(format!("Failed to evaluate '{}': {}", from_unit, e)))?;
    
    let to_result = evaluate(&to_expr)
        .map_err(|e| ConversionError::EvaluationError(format!("Failed to evaluate '{}': {}", to_unit, e)))?;
    
    // Check if the units are commensurable
    if from_result.dim != to_result.dim {
        return Err(ConversionError::IncommensurableUnits(
            format!("Cannot convert between '{}' and '{}': different dimensions", from_unit, to_unit)
        ));
    }
    
    // Perform the conversion
    Ok(value * from_result.factor / to_result.factor)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Example conversions with error handling
    match convert(100.0, "kPa", "mm[Hg]") {
        Ok(result) => println!("100 kPa = {:.2} mm[Hg]", result),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    match convert(1.0, "kg", "m") {
        Ok(result) => println!("1 kg = {} m", result),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    match convert(1.0, "invalid", "m") {
        Ok(result) => println!("1 invalid = {} m", result),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    Ok(())
}
```

### Step 4: Run the program

```bash
cargo run
```

You should see output similar to:

```
100 kPa = 750.06 mm[Hg]
Error: Incommensurable units: Cannot convert between 'kg' and 'm': different dimensions
Error: Parse error: Failed to parse 'invalid': Unrecognized unit code: invalid
```

## Part 2: Working with Arbitrary Units

Arbitrary units are units that are not defined in terms of any other unit, such as international units (IU).

### Step 1: Create a program to work with arbitrary units

Edit `src/main.rs` to include the following code:

```rust
use octofhir_ucum_core::{evaluate, parse_expression};

fn main() {
    // Parse and evaluate an arbitrary unit
    let expr_str = "[IU]";
    match parse_expression(expr_str).and_then(|expr| evaluate(&expr)) {
        Ok(result) => {
            println!("Expression: {}", expr_str);
            println!("Factor: {}", result.factor);
            println!("Dimensions: {:?}", result.dim);
            println!("Is arbitrary: {}", result.is_arbitrary);
            if let Some(base) = result.arbitrary_base {
                println!("Arbitrary base: {}", base);
            }
        },
        Err(e) => println!("Error: {}", e),
    }
    
    // Arbitrary unit with prefix
    let expr_str = "k[IU]";
    match parse_expression(expr_str).and_then(|expr| evaluate(&expr)) {
        Ok(result) => {
            println!("\nExpression: {}", expr_str);
            println!("Factor: {}", result.factor);
            println!("Is arbitrary: {}", result.is_arbitrary);
        },
        Err(e) => println!("Error: {}", e),
    }
    
    // Arbitrary unit combined with other units
    let expr_str = "[IU]/mL";
    match parse_expression(expr_str).and_then(|expr| evaluate(&expr)) {
        Ok(result) => {
            println!("\nExpression: {}", expr_str);
            println!("Factor: {}", result.factor);
            println!("Dimensions: {:?}", result.dim);
            println!("Is arbitrary: {}", result.is_arbitrary);
        },
        Err(e) => println!("Error: {}", e),
    }
    
    // Convert between arbitrary units with the same base
    convert_arbitrary(1000.0, "[IU]", "k[IU]");
    convert_arbitrary(5.0, "[IU]/mL", "[IU]/L");
    
    // Try to convert between different arbitrary units
    convert_arbitrary(1.0, "[IU]", "[arb'U]");
}

// Function to convert between arbitrary units
fn convert_arbitrary(value: f64, from_unit: &str, to_unit: &str) {
    let from_expr = match parse_expression(from_unit) {
        Ok(expr) => expr,
        Err(e) => {
            println!("Parse error for '{}': {}", from_unit, e);
            return;
        }
    };
    
    let to_expr = match parse_expression(to_unit) {
        Ok(expr) => expr,
        Err(e) => {
            println!("Parse error for '{}': {}", to_unit, e);
            return;
        }
    };
    
    let from_result = match evaluate(&from_expr) {
        Ok(result) => result,
        Err(e) => {
            println!("Evaluation error for '{}': {}", from_unit, e);
            return;
        }
    };
    
    let to_result = match evaluate(&to_expr) {
        Ok(result) => result,
        Err(e) => {
            println!("Evaluation error for '{}': {}", to_unit, e);
            return;
        }
    };
    
    if from_result.is_arbitrary && to_result.is_arbitrary {
        // Check if they have the same arbitrary base
        if from_result.arbitrary_base == to_result.arbitrary_base {
            let result = value * from_result.factor / to_result.factor;
            println!("\n{} {} = {} {}", value, from_unit, result, to_unit);
        } else {
            println!("\nCannot convert between different arbitrary units: {} and {}", from_unit, to_unit);
        }
    } else if from_result.is_arbitrary || to_result.is_arbitrary {
        println!("\nCannot convert between arbitrary and non-arbitrary units");
    } else {
        // For non-arbitrary units, check dimensions
        if from_result.dim != to_result.dim {
            println!("\nUnits have different dimensions: {} and {}", from_unit, to_unit);
        } else {
            let result = value * from_result.factor / to_result.factor;
            println!("\n{} {} = {} {}", value, from_unit, result, to_unit);
        }
    }
}
```

### Step 2: Run the program

```bash
cargo run
```

You should see output similar to:

```
Expression: [IU]
Factor: 1
Dimensions: [0, 0, 0, 0, 0, 0, 0]
Is arbitrary: true
Arbitrary base: IU

Expression: k[IU]
Factor: 1000
Is arbitrary: true

Expression: [IU]/mL
Factor: 1000000
Dimensions: [0, -3, 0, 0, 0, 0, 0]
Is arbitrary: true

1000 [IU] = 1 k[IU]

5 [IU]/mL = 5000 [IU]/L

Cannot convert between different arbitrary units: [IU] and [arb'U]
```

## Part 3: Custom Unit Registry

UCUM-RS allows you to create custom unit registries and add your own units.

### Step 1: Create a program with a custom unit registry

Edit `src/main.rs` to include the following code:

```rust
use octofhir_ucum_core::{registry, Unit, Dimension, parse_expression, evaluate};

fn main() {
    // Define a custom unit
    let custom_unit = Unit {
        code: "myUnit".to_string(),
        name: "My Custom Unit".to_string(),
        print_symbol: None,
        property: Some("custom".to_string()),
        is_metric: false,
        is_special: false,
        is_arbitrary: false,
        class: "custom".to_string(),
        factor: 42.0, // Conversion factor to canonical unit
        dim: Dimension([0, 0, 0, 0, 0, 0, 0]), // Dimensionless
        offset: 0.0,
        value: None,
    };

    // Register the custom unit
    registry::register_unit(custom_unit);

    // Now you can use the custom unit
    let expr_str = "myUnit";
    match parse_expression(expr_str).and_then(|expr| evaluate(&expr)) {
        Ok(result) => {
            println!("Expression: {}", expr_str);
            println!("Factor: {}", result.factor); // Should be 42.0
            println!("Dimensions: {:?}", result.dim);
        },
        Err(e) => println!("Error: {}", e),
    }

    // Convert between the custom unit and another unit
    let expr_str = "2*myUnit";
    match parse_expression(expr_str).and_then(|expr| evaluate(&expr)) {
        Ok(result) => {
            println!("\nExpression: {}", expr_str);
            println!("Factor: {}", result.factor); // Should be 84.0
            println!("Dimensions: {:?}", result.dim);
        },
        Err(e) => println!("Error: {}", e),
    }
}
```

### Step 2: Run the program

```bash
cargo run
```

You should see output similar to:

```
Expression: myUnit
Factor: 42
Dimensions: [0, 0, 0, 0, 0, 0, 0]

Expression: 2*myUnit
Factor: 84
Dimensions: [0, 0, 0, 0, 0, 0, 0]
```

## Part 4: Integration with FHIR

UCUM-RS provides integration with FHIR (Fast Healthcare Interoperability Resources) through the `octofhir-ucum-fhir` crate.

### Step 1: Add the FHIR integration dependency

Edit your `Cargo.toml` file to add the FHIR integration:

```toml
[dependencies]
octofhir-ucum-core = "0.3.0"
octofhir-ucum-fhir = "0.3.0"
```

### Step 2: Create a program that uses FHIR integration

Edit `src/main.rs` to include the following code:

```rust
use octofhir_ucum_fhir::{FhirQuantity, convert_quantity, are_equivalent};

fn main() {
    // Create a FHIR Quantity with a UCUM code
    let quantity = FhirQuantity::with_ucum_code(1000.0, "mg");
    println!("Created FHIR Quantity: {:?}", quantity);

    // Convert to a different unit
    match convert_quantity(&quantity, "g") {
        Ok(converted) => {
            println!("\nConverted to grams:");
            println!("Value: {}", converted.value);
            println!("Unit: {}", converted.unit.unwrap_or_default());
            println!("Code: {}", converted.code.unwrap_or_default());
        },
        Err(e) => println!("Conversion error: {}", e),
    }

    // Check if two quantities are equivalent
    let quantity2 = FhirQuantity::with_ucum_code(1.0, "g");
    match are_equivalent(&quantity, &quantity2) {
        Ok(result) => println!("\nQuantities are equivalent: {}", result),
        Err(e) => println!("Equivalence check error: {}", e),
    }

    // Try with incompatible units
    let quantity3 = FhirQuantity::with_ucum_code(1.0, "m");
    match are_equivalent(&quantity, &quantity3) {
        Ok(result) => println!("Quantities are equivalent: {}", result),
        Err(e) => println!("Equivalence check error: {}", e),
    }

    // Working with arbitrary units in FHIR
    let iu_quantity = FhirQuantity::with_ucum_code(1000.0, "[IU]");
    let kiu_quantity = FhirQuantity::with_ucum_code(1.0, "k[IU]");
    
    match are_equivalent(&iu_quantity, &kiu_quantity) {
        Ok(result) => println!("\n1000 [IU] is equivalent to 1 k[IU]: {}", result),
        Err(e) => println!("Equivalence check error: {}", e),
    }
}
```

### Step 3: Run the program

```bash
cargo run
```

You should see output similar to:

```
Created FHIR Quantity: FhirQuantity { value: 1000.0, unit: Some("mg"), system: Some("http://unitsofmeasure.org"), code: Some("mg"), comparator: None }

Converted to grams:
Value: 1
Unit: g
Code: g

Quantities are equivalent: true
Equivalence check error: Units are not commensurable: mg and m have different dimensions

1000 [IU] is equivalent to 1 k[IU]: true
```

## Part 5: WebAssembly Integration

UCUM-RS can be used in web applications via WebAssembly.

### Step 1: Set up a WebAssembly project

First, install wasm-pack:

```bash
cargo install wasm-pack
```

Then, create a new directory for your WebAssembly project:

```bash
mkdir ucum-wasm-example
cd ucum-wasm-example
```

Create an `index.html` file:

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>UCUM-RS WebAssembly Example</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
        }
        .result {
            margin-top: 10px;
            padding: 10px;
            background-color: #f0f0f0;
            border-radius: 5px;
        }
        input, select, button {
            margin: 5px;
            padding: 5px;
        }
        .section {
            margin-bottom: 20px;
            padding: 15px;
            border: 1px solid #ddd;
            border-radius: 5px;
        }
    </style>
</head>
<body>
    <h1>UCUM-RS WebAssembly Example</h1>
    
    <div class="section">
        <h2>Validate UCUM Expression</h2>
        <input type="text" id="validate-input" placeholder="Enter UCUM expression (e.g., mg/dL)">
        <button id="validate-button">Validate</button>
        <div id="validate-result" class="result"></div>
    </div>
    
    <div class="section">
        <h2>Convert Between Units</h2>
        <input type="number" id="convert-value" placeholder="Value" value="100">
        <input type="text" id="convert-from" placeholder="From unit (e.g., kPa)">
        <input type="text" id="convert-to" placeholder="To unit (e.g., mm[Hg])">
        <button id="convert-button">Convert</button>
        <div id="convert-result" class="result"></div>
    </div>
    
    <div class="section">
        <h2>Get Unit Information</h2>
        <input type="text" id="info-input" placeholder="Enter unit code (e.g., kg)">
        <button id="info-button">Get Info</button>
        <div id="info-result" class="result"></div>
    </div>
    
    <script type="module">
        import { start, validate, convert, get_unit_info } from 'https://cdn.jsdelivr.net/npm/@octofhir/ucum-wasm/ucum_wasm.js';
        
        // Initialize the WASM module
        start();
        
        // Validate UCUM expression
        document.getElementById('validate-button').addEventListener('click', () => {
            const expr = document.getElementById('validate-input').value;
            try {
                const isValid = validate(expr);
                document.getElementById('validate-result').textContent = 
                    isValid ? `✅ Valid UCUM expression: ${expr}` : `❌ Invalid UCUM expression: ${expr}`;
            } catch (error) {
                document.getElementById('validate-result').textContent = `❌ Error: ${error.message}`;
            }
        });
        
        // Convert between units
        document.getElementById('convert-button').addEventListener('click', () => {
            const value = parseFloat(document.getElementById('convert-value').value);
            const fromUnit = document.getElementById('convert-from').value;
            const toUnit = document.getElementById('convert-to').value;
            
            try {
                const result = convert(value, fromUnit, toUnit);
                document.getElementById('convert-result').textContent = 
                    `${value} ${fromUnit} = ${result} ${toUnit}`;
            } catch (error) {
                document.getElementById('convert-result').textContent = `❌ Error: ${error.message}`;
            }
        });
        
        // Get unit information
        document.getElementById('info-button').addEventListener('click', () => {
            const unit = document.getElementById('info-input').value;
            
            try {
                const info = get_unit_info(unit);
                let resultText = `Code: ${info.code}\n`;
                resultText += `Display Name: ${info.display_name}\n`;
                resultText += `Factor: ${info.factor}\n`;
                resultText += `Dimensions: ${JSON.stringify(info.dimensions)}\n`;
                resultText += `Is Special: ${info.is_special}\n`;
                resultText += `Is Arbitrary: ${info.is_arbitrary}\n`;
                resultText += `Property: ${info.property || 'N/A'}`;
                
                document.getElementById('info-result').textContent = resultText;
            } catch (error) {
                document.getElementById('info-result').textContent = `❌ Error: ${error.message}`;
            }
        });
    </script>
</body>
</html>
```

### Step 2: Serve the HTML file

You can use any local server to serve the HTML file. For example, with Python:

```bash
# Python 3
python -m http.server

# Python 2
python -m SimpleHTTPServer
```

Then open your browser to http://localhost:8000 to see the UCUM-RS WebAssembly example.

## Next Steps

Now that you've completed this advanced tutorial, you can:

1. Explore the [UCUM-RS User Guide](https://github.com/octofhir/ucum-rs/blob/main/USER_GUIDE.md) for more detailed information
2. Contribute to the [UCUM-RS project](https://github.com/octofhir/ucum-rs) by adding features or fixing bugs
3. Integrate UCUM-RS into your own healthcare or scientific applications
4. Extend the library with custom units for your specific domain

## Conclusion

In this advanced tutorial, you've learned how to:
- Implement custom error handling
- Work with arbitrary units
- Create and use custom unit registries
- Integrate with FHIR
- Use UCUM-RS in web applications via WebAssembly

These advanced features make UCUM-RS a powerful tool for working with units of measure in a variety of contexts, from healthcare applications to scientific computing.
