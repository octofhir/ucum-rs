# FHIR Integration in UCUM-RS

This document describes the integration of FHIR (Fast Healthcare Interoperability Resources) functionality into the UCUM-RS project.

## Overview

The UCUM-RS project now includes FHIR integration, allowing for conversion between UCUM units and FHIR Quantity data types. This functionality is exposed through the WebAssembly package and demonstrated in the playground.

## Components

### 1. UCUM-FHIR Package

The `ucum-fhir` package provides integration between the UCUM core library and FHIR, allowing for conversion between UCUM units and FHIR Quantity data types. Key features include:

- `FhirQuantity` struct: Represents a FHIR Quantity with value, unit, system, code, and comparator
- `ToFhirQuantity` and `FromFhirQuantity` traits: For converting between FHIR and UCUM quantities
- `convert_quantity` function: Converts a FHIR Quantity from one unit to another
- `are_equivalent` function: Checks if two FHIR Quantities are equivalent

### 2. WebAssembly Bindings

The `ucum-wasm` package now includes bindings for the FHIR functionality, exposing the following functions to JavaScript:

- `create_fhir_quantity`: Creates a FHIR Quantity with a UCUM code
- `fhir_to_ucum`: Converts a FHIR Quantity to a UCUM Quantity
- `ucum_to_fhir`: Converts a UCUM Quantity to a FHIR Quantity
- `convert_fhir_quantity`: Converts a FHIR Quantity from one unit to another
- `are_fhir_quantities_equivalent`: Checks if two FHIR Quantities are equivalent

### 3. Playground Integration

The playground now includes a FHIR tab that demonstrates the FHIR functionality. The tab includes:

- A description of the FHIR functionality
- A radio button group to select between different operations:
  - Create FHIR Quantity
  - Convert FHIR Quantity
  - Check Equivalence
- UI components for each operation
- Result display sections
- Error display section

## Usage Examples

### Creating a FHIR Quantity

```javascript
// Create a FHIR Quantity with a value of 10 and a UCUM code of "mg"
const fhirQuantity = create_fhir_quantity(10, "mg");
```

### Converting a FHIR Quantity

```javascript
// Create a FHIR Quantity
const fhirQuantity = create_fhir_quantity(1000, "mg");

// Convert to grams
const convertedQuantity = convert_fhir_quantity(fhirQuantity, "g");
// convertedQuantity.value === 1.0
// convertedQuantity.code === "g"
```

### Checking if Two FHIR Quantities are Equivalent

```javascript
// Create two FHIR Quantities
const quantity1 = create_fhir_quantity(1, "g");
const quantity2 = create_fhir_quantity(1000, "mg");

// Check if they are equivalent
const areEquivalent = are_fhir_quantities_equivalent(quantity1, quantity2);
// areEquivalent === true
```

## Implementation Details

### FHIR Quantity Structure

A FHIR Quantity includes:

- `value`: The numeric value
- `unit`: The human-readable unit representation
- `system`: The system that defines the coded unit form (for UCUM, this is "http://unitsofmeasure.org")
- `code`: The coded form of the unit (for UCUM, this is the UCUM code)
- `comparator`: The comparator (<, <=, >=, >) for the value (optional)

### Error Handling

Errors are handled consistently across the API, with specific error types for:

- Invalid UCUM codes
- Missing required fields
- Invalid system URIs
- Units that are not commensurable (have different dimensions)

## Future Enhancements

Potential future enhancements to the FHIR integration include:

1. Support for FHIR Quantity comparators
2. Integration with FHIR terminology services
3. Support for FHIR extensions
4. Validation against FHIR profiles
