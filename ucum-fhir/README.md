# UCUM FHIR Integration

This crate provides integration between the UCUM core library and FHIR, allowing for conversion between UCUM units and FHIR Quantity data types.

## Features

- Convert between FHIR Quantity and UCUM Quantity
- Convert FHIR Quantities between different units
- Check if two FHIR Quantities are equivalent
- Proper error handling for invalid inputs
- Support for arbitrary units

## Usage

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
octofhir-ucum-fhir = "0.3.0"
```

### Basic Usage

```rust
use octofhir_ucum_fhir::{FhirQuantity, convert_quantity, are_equivalent};

// Create a FHIR Quantity with a UCUM code
let quantity = FhirQuantity::with_ucum_code(1000.0, "mg");

// Convert to a different unit
let converted = convert_quantity(&quantity, "g").unwrap();
assert_eq!(converted.value, 1.0);
assert_eq!(converted.code, Some("g".to_string()));

// Check if two quantities are equivalent
let quantity2 = FhirQuantity::with_ucum_code(1.0, "g");
assert!(are_equivalent(&quantity2, &converted).unwrap());
```

### Converting Between UCUM and FHIR

```rust
use octofhir_ucum_fhir::{FhirQuantity, ToFhirQuantity, FromFhirQuantity};
use octofhir_ucum_core::{parse_expression, Quantity as UcumQuantity};

// Create a FHIR Quantity
let fhir_quantity = FhirQuantity::with_ucum_code(10.0, "mg");

// Convert to UCUM Quantity
let ucum_quantity = fhir_quantity.to_ucum_quantity().unwrap();
assert_eq!(ucum_quantity.value, 10.0);

// Convert back to FHIR Quantity
let fhir_quantity2 = ucum_quantity.to_fhir_quantity().unwrap();
assert_eq!(fhir_quantity2.value, 10.0);
assert_eq!(fhir_quantity2.code, Some("mg".to_string()));

// Create a UCUM Quantity directly
let expr = parse_expression("kg").unwrap();
let ucum_quantity = UcumQuantity {
    value: 75.0,
    unit: expr,
};

// Convert to FHIR Quantity
let fhir_quantity = ucum_quantity.to_fhir_quantity().unwrap();
assert_eq!(fhir_quantity.value, 75.0);
assert_eq!(fhir_quantity.code, Some("kg".to_string()));
```

### Integration with FHIR Resources

When working with FHIR resources, you can use this library to validate and convert quantities:

```rust
use octofhir_ucum_fhir::{FhirQuantity, convert_quantity, are_equivalent};
use serde_json::json;

// Example FHIR Observation with a weight measurement
let observation = json!({
    "resourceType": "Observation",
    "status": "final",
    "code": {
        "coding": [{
            "system": "http://loinc.org",
            "code": "29463-7",
            "display": "Body Weight"
        }]
    },
    "valueQuantity": {
        "value": 70.0,
        "unit": "kg",
        "system": "http://unitsofmeasure.org",
        "code": "kg"
    }
});

// Extract the quantity from the observation
let value_quantity = observation["valueQuantity"].clone();
let fhir_quantity = FhirQuantity {
    value: value_quantity["value"].as_f64().unwrap(),
    unit: value_quantity["unit"].as_str().map(|s| s.to_string()),
    system: value_quantity["system"].as_str().map(|s| s.to_string()),
    code: value_quantity["code"].as_str().map(|s| s.to_string()),
    comparator: None,
};

// Convert to pounds
let pounds = convert_quantity(&fhir_quantity, "[lb_av]").unwrap();
println!("Weight in pounds: {}", pounds.value);

// Check if the weight is equivalent to 70000 grams
let grams = FhirQuantity::with_ucum_code(70000.0, "g");
assert!(are_equivalent(&fhir_quantity, &grams).unwrap());
```

### Working with Arbitrary Units

UCUM defines arbitrary units as units that are not defined in terms of any other unit, such as international units (IU) or arbitrary units (arb'U). According to the UCUM specification:

- Arbitrary units are enclosed in square brackets, e.g., `[IU]`, `[arb'U]`
- They are dimensionless with a factor of 1.0
- They are not commensurable with any other unit, including other arbitrary units
- They can be combined with other units (e.g., `[IU]/mL`)
- They can be prefixed (e.g., `k[IU]`)

This library provides full support for arbitrary units:

```rust
use octofhir_ucum_fhir::{FhirQuantity, convert_quantity, are_equivalent};

// Create quantities with arbitrary units
let iu = FhirQuantity::with_ucum_code(10.0, "[IU]");
let kiu = FhirQuantity::with_ucum_code(1.0, "k[IU]"); // Prefixed arbitrary unit

// Different values with the same arbitrary unit are not equivalent
let result = are_equivalent(&iu, &kiu);
assert!(result.is_err() || !result.unwrap());

// Create a quantity with 1000 IU
let iu_1000 = FhirQuantity::with_ucum_code(1000.0, "[IU]");
// Create a quantity with 1 kIU (kilo-IU)
let kiu_1 = FhirQuantity::with_ucum_code(1.0, "k[IU]");
// These should be equivalent
assert!(are_equivalent(&iu_1000, &kiu_1).unwrap());

// Different arbitrary units are not commensurable
let iu = FhirQuantity::with_ucum_code(10.0, "[IU]");
let arbu = FhirQuantity::with_ucum_code(10.0, "[arb'U]");
let result = are_equivalent(&iu, &arbu);
assert!(result.is_err());

// Arbitrary units can be combined with other units
let iu_per_ml = FhirQuantity::with_ucum_code(5.0, "[IU]/mL");
let iu_per_l = convert_quantity(&iu_per_ml, "[IU]/L").unwrap();
assert_eq!(iu_per_l.value, 5000.0); // 5 [IU]/mL = 5000 [IU]/L
```

### Error Handling

The library provides comprehensive error handling:

```rust
use octofhir_ucum_fhir::{FhirQuantity, FhirError, are_equivalent};

// Invalid system
let quantity = FhirQuantity {
    value: 10.0,
    unit: Some("mg".to_string()),
    system: Some("http://example.org".to_string()), // Not UCUM
    code: Some("mg".to_string()),
    comparator: None,
};
let result = quantity.to_ucum_quantity();
assert!(result.is_err());
if let Err(FhirError::InvalidSystem(system)) = result {
    println!("Invalid system: {}", system);
}

// Non-commensurable units
let a = FhirQuantity::with_ucum_code(1.0, "g");
let b = FhirQuantity::with_ucum_code(1.0, "s");
let result = are_equivalent(&a, &b);
assert!(result.is_err());
```

## Features

- `std` (default): Standard library support
- `serde`: Serialization/deserialization support

## License

This crate is licensed under the MIT License.
