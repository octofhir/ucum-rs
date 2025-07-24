# @octofhir/ucum-wasm

WebAssembly bindings for the UCUM (Unified Code for Units of Measure) library, providing fast and reliable unit conversion and validation for healthcare applications.

## Features

- 🚀 **Fast**: WebAssembly-powered performance
- 🔒 **Type-safe**: Full TypeScript support
- 🏥 **Healthcare-focused**: Built for FHIR and medical applications
- 📏 **Complete UCUM support**: Handles all UCUM units and expressions
- 🌐 **Universal**: Works in browsers, Node.js, and bundlers
- 🔍 **Extended Functionality**: Unit expression optimization, measurement contexts, model introspection, self-validation, and advanced conversion with precision control

## Installation

```bash
npm install @octofhir/ucum-wasm
```

## Quick Start

```javascript
import { 
  start, 
  validate, 
  convert, 
  get_unit_info,
  get_ucum_model,
  get_unit_display_name,
  convert_advanced_simple,
  optimize_unit_expression,
  create_measurement_context
} from '@octofhir/ucum-wasm';

// Initialize the WASM module
start();

// Validate UCUM expressions
console.log(validate('kg')); // true
console.log(validate('invalid')); // false

// Convert between units
const result = convert(100, 'g', 'kg');
console.log(result); // 0.1

// Get unit information
const unitInfo = get_unit_info('kg');
console.log(unitInfo.code); // 'kg'
console.log(unitInfo.factor); // 1000

// Model introspection
const model = get_ucum_model();
console.log(model.version); // '2.1'
console.log(model.total_units); // 312

// Enhanced display names
console.log(get_unit_display_name('kg')); // 'kilogram'
console.log(get_unit_display_name('m/s')); // '(meter) / (second)'

// Advanced conversion with precision
const advResult = convert_advanced_simple(1000, 'g', 'kg', 3);
console.log(advResult.value); // 1.000
console.log(advResult.precision_info); // '3 decimal places'
```

## API Reference

### Core Functions

#### `start(): void`
Initialize the WASM module. Must be called before using other functions.

#### `validate(expression: string): boolean`
Validate a UCUM expression.

```javascript
validate('kg.m/s2'); // true - valid UCUM expression
validate('invalid'); // false - invalid expression
```

#### `convert(value: number, from_unit: string, to_unit: string): number`
Convert a value from one unit to another.

```javascript
convert(1000, 'g', 'kg'); // 1
convert(100, 'cm', 'm'); // 1
convert(32, '[degF]', 'Cel'); // 0 (Fahrenheit to Celsius)
```

#### `get_unit_info(code: string): UnitInfo`
Get detailed information about a unit.

```javascript
const info = get_unit_info('kg');
// Returns:
// {
//   code: 'kg',
//   factor: 1000,
//   offset: 0,
//   is_special: false,
//   is_arbitrary: false,
//   dimensions: [1, 0, 0, 0, 0, 0, 0] // Mass dimension
// }
```

### Advanced Functions

#### `evaluate_expression(expression: string): EvaluationResult`
Evaluate a UCUM expression and get its properties.

```javascript
const result = evaluate_expression('kg.m/s2');
// Returns:
// {
//   factor: 1000000,
//   offset: 0,
//   dimensions: [1, 1, -2, 0, 0, 0, 0] // Force dimensions
// }
```

#### `arithmetic(left_unit: string, operation: "mul" | "div", right_unit: string, value: number): EvaluationResult`
Perform arithmetic operations on units.

```javascript
const result = arithmetic('kg', 'mul', 'm/s2', 10);
// Multiplies kg by m/s2 with value 10
```

#### `list_units(filter?: string): UnitInfo[]`
List all available units, optionally filtered.

```javascript
const allUnits = list_units();
const massUnits = list_units('mass');
```

### Extended Functionality

#### Unit Expression Optimization

##### `optimize_unit_expression(expression: string): string`
Optimize a unit expression for better readability by recognizing common derived units.

```javascript
const optimized = optimize_unit_expression('kg.m.s-2');
console.log(optimized); // 'N' (newton)

const powerOpt = optimize_unit_expression('kg.m2.s-3');
console.log(powerOpt); // 'W' (watt)
```

##### `canonicalize_unit_expression(expression: string): string`
Convert a unit expression to its canonical (base units) form.

```javascript
const canonical = canonicalize_unit_expression('N');
console.log(canonical); // 'kg.m.s-2'

const pressureCanonical = canonicalize_unit_expression('Pa');
console.log(pressureCanonical); // 'kg.m-1.s-2'
```

##### `simplify_unit_expression(expression: string): string`
Simplify a unit expression by combining like terms and reducing complexity.

```javascript
const simplified = simplify_unit_expression('m.m/s');
console.log(simplified); // 'm2/s'

const complexSimplified = simplify_unit_expression('kg.m.s-2.s');
console.log(complexSimplified); // 'kg.m.s-1'
```

#### Measurement Context Support

##### `create_measurement_context(domain: string): MeasurementContext`
Create a measurement context for domain-specific unit preferences.

```javascript
// Create contexts for different domains
const medicalContext = create_measurement_context('medical');
const engineeringContext = create_measurement_context('engineering');
const physicsContext = create_measurement_context('physics');
const chemistryContext = create_measurement_context('chemistry');

console.log(medicalContext.domain); // 'Medical'
console.log(medicalContext.preferred_units); // ['mg', 'kg', 'L', 'mL', 'mmol', ...]
console.log(medicalContext.avoided_units); // ['[stone_av]', '[lb_av]', '[gal_us]', ...]
```

##### `is_unit_preferred(domain: string, unit: string): boolean`
Check if a unit is preferred in a specific measurement context.

```javascript
const isPreferred = is_unit_preferred('medical', 'mg');
console.log(isPreferred); // true

const notPreferred = is_unit_preferred('medical', '[stone_av]');
console.log(notPreferred); // false
```

##### `is_unit_avoided(domain: string, unit: string): boolean`
Check if a unit should be avoided in a specific measurement context.

```javascript
const isAvoided = is_unit_avoided('medical', '[lb_av]');
console.log(isAvoided); // true

const notAvoided = is_unit_avoided('engineering', 'kPa');
console.log(notAvoided); // false
```

##### `get_context_unit_suggestions(domain: string, unit: string): UnitSuggestions`
Get unit suggestions for a measurement context.

```javascript
const suggestions = get_context_unit_suggestions('chemistry', 'g');
console.log(suggestions.alternatives); // ['mg', 'kg', 'mol']
console.log(suggestions.is_preferred); // true
console.log(suggestions.is_avoided); // false
```

### Model Introspection Functions

#### `get_ucum_model(): UcumModel`
Get information about the UCUM model.

```javascript
const model = get_ucum_model();
console.log(model.version);        // '2.1'
console.log(model.revision_date);  // '2017-11-21'
console.log(model.total_units);    // 312
console.log(model.total_prefixes); // 24
console.log(model.properties);     // Array of property names
```

#### `validate_ucum_implementation(): ValidationResult`
Validate the UCUM implementation for self-consistency.

```javascript
const validation = validate_ucum_implementation();
console.log(validation.is_valid); // true or false
console.log(validation.issues);   // Array of issue descriptions
```

#### `get_ucum_properties(): string[]`
Get all available properties in the UCUM model.

```javascript
const properties = get_ucum_properties();
// Returns: ['length', 'mass', 'time', 'temperature', ...]
```

#### `validate_canonical_form(unit: string, canonical: string): boolean`
Validate if a unit matches its canonical form.

```javascript
const isCanonical1 = validate_canonical_form('kg', 'g');        // false
const isCanonical2 = validate_canonical_form('m/s', 'm.s-1');  // true
```

#### `get_unit_display_name(code: string): string`
Get the display name for a unit code (handles prefixed units correctly).

```javascript
const display1 = get_unit_display_name('kg');    // 'kilogram'
const display2 = get_unit_display_name('cm');    // 'centimeter'
const display3 = get_unit_display_name('m/s');   // '(meter) / (second)'
```

### Advanced Conversion Functions

#### `convert_advanced(value: number, from_unit: string, to_unit: string, config: ConversionConfig): AdvancedConversionResult`
Advanced unit conversion with full precision control.

```javascript
const config = {
  precision_type: 'fixed',
  precision_value: 3,
  rounding_mode: 'nearest',
  temperature_scale: 'celsius',
  use_special_units: true
};

const result = convert_advanced(1000, 'g', 'kg', config);
console.log(result.value);              // 1.000
console.log(result.precision_info);     // '3 decimal places'
console.log(result.factor);             // 0.001
console.log(result.used_special_units); // false
```

#### `convert_advanced_simple(value: number, from_unit: string, to_unit: string, precision_places?: number): AdvancedConversionResult`
Simplified advanced conversion with default settings.

```javascript
const result = convert_advanced_simple(100, 'Cel', 'K', 2);
console.log(result.value);              // 373.15
console.log(result.precision_info);     // '2 decimal places'
console.log(result.used_special_units); // true (temperature conversion)
```

## Type Definitions

### `UnitInfo`
```typescript
interface UnitInfo {
  code: string;           // UCUM code
  display_name: string;   // Human-readable display name
  factor: number;         // Conversion factor to canonical unit
  offset: number;         // Offset for linear conversions
  is_special: boolean;    // Non-linear conversion required
  is_arbitrary: boolean;  // Arbitrary unit (e.g., [IU])
  dimensions: number[];   // Dimensional vector [M, L, T, I, Θ, N, J]
  property: string;       // Unit property or class (e.g., "length", "mass")
}
```

### `EvaluationResult`
```typescript
interface EvaluationResult {
  factor: number;       // Conversion factor
  offset: number;       // Linear offset
  dimensions: number[]; // Dimensional vector
}
```

### Extended Functionality Type Definitions

#### `MeasurementContext`
```typescript
interface MeasurementContext {
  domain: string;                           // Domain name (e.g., 'Medical', 'Engineering')
  precision_requirements: PrecisionRequirements;
  preferred_units: string[];                // Units preferred in this domain
  avoided_units: string[];                  // Units to avoid in this domain
}
```

#### `PrecisionRequirements`
```typescript
interface PrecisionRequirements {
  min_significant_figures: number;          // Minimum significant figures required
  max_relative_error: number;               // Maximum acceptable relative error
  require_exact: boolean;                   // Whether exact values are required
}
```

#### `UnitSuggestions`
```typescript
interface UnitSuggestions {
  alternatives: string[];                   // Alternative units for the context
  is_preferred: boolean;                    // Whether the unit is preferred
  is_avoided: boolean;                      // Whether the unit should be avoided
}
```

#### `UcumModel`
```typescript
interface UcumModel {
  version: string;        // UCUM version (e.g., '2.1')
  revision_date: string;  // Revision date (e.g., '2017-11-21')
  total_units: number;    // Total number of units in the model
  total_prefixes: number; // Total number of prefixes in the model
  properties: string[];   // Array of all available properties
}
```

#### `ValidationResult`
```typescript
interface ValidationResult {
  is_valid: boolean;    // Whether the implementation is valid
  issues: string[];     // Array of validation issue descriptions
}
```

#### `AdvancedConversionResult`
```typescript
interface AdvancedConversionResult {
  value: number;             // Converted value
  unit: string;              // Target unit
  factor: number;            // Conversion factor used
  offset: number;            // Offset used (for temperature, etc.)
  precision_info: string;    // Description of precision applied
  used_special_units: boolean; // Whether special unit handling was used
}
```

#### `ConversionConfig`
```typescript
interface ConversionConfig {
  precision_type: 'default' | 'fixed' | 'significant';
  precision_value?: number;  // Required for 'fixed' and 'significant'
  rounding_mode: 'nearest' | 'up' | 'down' | 'truncate';
  temperature_scale: 'kelvin' | 'celsius' | 'fahrenheit';
  use_special_units: boolean;
}
```

## Usage Examples

### Healthcare/FHIR Applications

```javascript
import { start, convert, validate } from '@octofhir/ucum-wasm';

start();

// Validate patient vital signs units
const isValidWeight = validate('kg'); // true
const isValidHeight = validate('cm'); // true
const isValidTemp = validate('Cel'); // true

// Convert lab values
const glucoseInMmol = convert(180, 'mg/dL', 'mmol/L'); // ~10.0
const hemoglobinInG = convert(15, 'g/dL', 'g/L'); // 150

// Handle temperature conversions
const bodyTempC = convert(98.6, '[degF]', 'Cel'); // ~37.0
```

### Extended Functionality and Advanced Conversion

```javascript
import { 
  start, 
  get_ucum_model, 
  validate_ucum_implementation,
  get_unit_display_name,
  convert_advanced,
  convert_advanced_simple
} from '@octofhir/ucum-wasm';

start();

// Get model information for documentation or validation
const model = get_ucum_model();
console.log(`UCUM Version: ${model.version}`);
console.log(`Total Units: ${model.total_units}`);
console.log(`Available Properties: ${model.properties.length}`);

// Validate the implementation
const validation = validate_ucum_implementation();
if (!validation.is_valid) {
  console.warn('UCUM implementation issues:', validation.issues);
}

// Get better display names for units
const weightUnit = get_unit_display_name('kg');     // 'kilogram'
const speedUnit = get_unit_display_name('m/s');     // '(meter) / (second)'
const tempUnit = get_unit_display_name('Cel');      // 'degree Celsius'

// Advanced conversion with precise control
const preciseConversion = convert_advanced(1000, 'g', 'kg', {
  precision_type: 'fixed',
  precision_value: 4,
  rounding_mode: 'nearest',
  temperature_scale: 'celsius',
  use_special_units: true
});
console.log(`${preciseConversion.value} kg`); // 1.0000 kg
console.log(preciseConversion.precision_info); // '4 decimal places'

// Temperature conversion with advanced precision
const tempConversion = convert_advanced_simple(37.5, 'Cel', '[degF]', 1);
console.log(`${tempConversion.value}°F`);        // 99.5°F
console.log(tempConversion.used_special_units);  // true
```

### FHIR Integration

This package includes full support for FHIR Quantity data types, allowing seamless integration with FHIR-based healthcare applications.

#### FHIR Quantity Functions

```javascript
import { 
  start, 
  create_fhir_quantity, 
  convert_fhir_quantity, 
  are_fhir_quantities_equivalent,
  fhir_to_ucum,
  ucum_to_fhir
} from '@octofhir/ucum-wasm';

start();

// Create a FHIR Quantity
const weight = create_fhir_quantity(70, 'kg');
// Returns: { value: 70, unit: 'kg', system: 'http://unitsofmeasure.org', code: 'kg' }

// Convert a FHIR Quantity to a different unit
const weightInGrams = convert_fhir_quantity(weight, 'g');
// Returns: { value: 70000, unit: 'g', system: 'http://unitsofmeasure.org', code: 'g' }

// Check if two FHIR Quantities are equivalent
const weight2 = create_fhir_quantity(70000, 'g');
const areEqual = are_fhir_quantities_equivalent(weight, weight2); // true

// Convert between FHIR and UCUM quantities
const ucumQuantity = fhir_to_ucum(weight);
// Returns: { value: 70, unit: 'kg' }

// Convert from a UCUM value to a FHIR Quantity
const fhirQuantity = ucum_to_fhir(36.6, 'Cel');
// Returns: { value: 36.6, unit: 'Cel', system: 'http://unitsofmeasure.org', code: 'Cel' }
```

#### FHIR Quantity Structure

A FHIR Quantity includes:

- `value`: The numeric value
- `unit`: The human-readable unit representation
- `system`: The system that defines the coded unit form (for UCUM, this is "http://unitsofmeasure.org")
- `code`: The coded form of the unit (for UCUM, this is the UCUM code)
- `comparator`: The comparator (<, <=, >=, >) for the value (optional)

#### Error Handling with FHIR Quantities

```javascript
try {
  // Try to convert between incompatible units
  const weight = create_fhir_quantity(70, 'kg');
  const invalidConversion = convert_fhir_quantity(weight, 'Cel');
} catch (error) {
  console.error('Conversion failed:', error.message);
  // Error: Units are not commensurable: kg and Cel have different dimensions
}

try {
  // Try to create a FHIR Quantity with an invalid UCUM code
  const invalidQuantity = create_fhir_quantity(100, 'invalid_unit');
} catch (error) {
  console.error('Creation failed:', error.message);
  // Error: Invalid UCUM code: invalid_unit
}
```

#### Use Cases for FHIR Integration

- **Electronic Health Records (EHR)**: Validate and convert patient measurements stored as FHIR Quantities
- **Clinical Decision Support**: Compare patient values against reference ranges in different units
- **Lab Result Integration**: Convert lab results between different unit systems while maintaining FHIR compatibility
- **FHIR API Development**: Ensure consistent unit handling in FHIR-based APIs
- **Healthcare Analytics**: Process and normalize measurement data from multiple sources

### Complex Unit Expressions

```javascript
// Validate complex expressions
validate('kg.m2/s3'); // true - power units
validate('mol/L'); // true - concentration
validate('1/min'); // true - frequency

// Convert complex units
const pressure = convert(1, 'atm', 'Pa'); // 101325
const energy = convert(1, 'cal', 'J'); // ~4.184
```

### Error Handling

```javascript
try {
  const result = convert(100, 'kg', 'invalid_unit');
} catch (error) {
  console.error('Conversion failed:', error.message);
}

try {
  validate('completely_invalid_expression');
} catch (error) {
  console.error('Validation failed:', error.message);
}
```

## Browser Support

This package works in all modern browsers that support WebAssembly:
- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+

## Node.js Support

Requires Node.js 20+ with WebAssembly support.

## UCUM Specification Implementation

This package implements the [Unified Code for Units of Measure (UCUM) specification](https://ucum.org/ucum.html), which defines a system for unambiguous representation of units of measure.

### Relationship to the UCUM Specification

The implementation follows the UCUM specification closely, including:

- **Base Units**: All seven base units (mass, length, time, electric current, temperature, amount of substance, luminous intensity)
- **Derived Units**: All derived units (Newton, Pascal, Joule, etc.)
- **Prefixes**: All SI prefixes (kilo-, milli-, micro-, etc.)
- **Grammar**: The full UCUM grammar for parsing unit expressions
- **Special Units**: Units with special handling (temperature, logarithmic, arbitrary)
- **Annotations**: Support for annotated units (e.g., mm[Hg])

The implementation is designed to be:
- **Complete**: Covers all aspects of the UCUM specification
- **Accurate**: Follows the specification precisely
- **Robust**: Handles edge cases and provides clear error messages
- **Performant**: Optimized for speed and memory efficiency

### Implementation Details

The WebAssembly package is built on top of the Rust core library, which uses:

1. A custom parser to convert string expressions into an abstract syntax tree (AST)
2. An evaluator to traverse the AST and determine canonical form, conversion factor, and dimensions
3. A registry of all standard UCUM units
4. Special handling for units with offsets (e.g., temperature) and arbitrary units

## Extending the Library

The WebAssembly package provides several ways to extend the library for specific domains or applications.

### Custom Units Registry

You can create a custom registry with your own units:

```javascript
import { start, create_registry, add_unit_to_registry, validate_with_registry } from '@octofhir/ucum-wasm';

// Initialize the WASM module
start();

// Create a custom registry
const registry = create_registry();

// Add a custom unit
add_unit_to_registry(registry, {
  code: "myUnit",
  display_name: "My Custom Unit",
  factor: 42.0,
  dimensions: [0, 0, 0, 0, 0, 0, 0], // Dimensionless
  is_metric: false,
  is_special: false,
  is_arbitrary: false,
  property: "custom"
});

// Validate using the custom registry
const isValid = validate_with_registry(registry, "myUnit"); // true
```

### Custom Arbitrary Units

You can define custom arbitrary units for specific domains:

```javascript
import { start, create_registry, add_unit_to_registry, validate_with_registry } from '@octofhir/ucum-wasm';

// Initialize the WASM module
start();

// Create a custom registry
const registry = create_registry();

// Add a custom arbitrary unit
add_unit_to_registry(registry, {
  code: "[myArb]",
  display_name: "My Arbitrary Unit",
  factor: 1.0, // Arbitrary units have a factor of 1.0
  dimensions: [0, 0, 0, 0, 0, 0, 0], // Dimensionless
  is_metric: false,
  is_special: false,
  is_arbitrary: true, // Mark as arbitrary
  property: "arbitrary"
});

// Validate using the custom registry
const isValid = validate_with_registry(registry, "[myArb]"); // true
```

### Custom Conversion Functions

For special units or complex conversions, you can implement custom conversion functions:

```javascript
import { start, validate, get_unit_info } from '@octofhir/ucum-wasm';

// Initialize the WASM module
start();

// Custom conversion function for temperature
function convertTemperature(value, fromUnit, toUnit) {
  // Validate the units
  if (!validate(fromUnit) || !validate(toUnit)) {
    throw new Error("Invalid unit");
  }
  
  // Get unit information
  const fromInfo = get_unit_info(fromUnit);
  const toInfo = get_unit_info(toUnit);
  
  // Check if the units are commensurable
  if (fromInfo.dimensions.join(',') !== toInfo.dimensions.join(',')) {
    throw new Error("Units are not commensurable");
  }
  
  // Handle special case for temperature units with offsets
  if (fromUnit === "Cel" && toUnit === "[degF]") {
    return value * 9.0/5.0 + 32.0;
  } else if (fromUnit === "[degF]" && toUnit === "Cel") {
    return (value - 32.0) * 5.0/9.0;
  } else if ((fromUnit === "Cel" || fromUnit === "[degF]") && toUnit === "K") {
    const celsius = fromUnit === "Cel" ? value : (value - 32.0) * 5.0/9.0;
    return celsius + 273.15;
  } else if (fromUnit === "K" && (toUnit === "Cel" || toUnit === "[degF]")) {
    const celsius = value - 273.15;
    return toUnit === "Cel" ? celsius : celsius * 9.0/5.0 + 32.0;
  }
  
  // For other units, use the standard conversion
  return value * fromInfo.factor / toInfo.factor;
}

// Example usage
const result = convertTemperature(25, "Cel", "[degF]"); // 77
```

## Contributing

This package is part of the [ucum-rs](https://github.com/octofhir/ucum-rs) project. Please see the main repository for contribution guidelines.

## License

MIT License - see the [LICENSE](https://github.com/octofhir/ucum-rs/blob/main/LICENSE) file for details.

## Related Packages

- [`@octofhir/ucum-core`](https://crates.io/crates/octofhir-ucum-core) - Rust core library
- [`@octofhir/ucum-cli`](https://crates.io/crates/octofhir-ucum-cli) - Command-line interface

## Support

For issues and questions, please visit the [GitHub repository](https://github.com/octofhir/ucum-rs/issues).
