# @octofhir/ucum-wasm

WebAssembly bindings for the UCUM (Unified Code for Units of Measure) library, providing fast and reliable unit conversion and validation for healthcare applications.

## Features

- 🚀 **Fast**: WebAssembly-powered performance
- 🔒 **Type-safe**: Full TypeScript support
- 🏥 **Healthcare-focused**: Built for FHIR and medical applications
- 📏 **Complete UCUM support**: Handles all UCUM units and expressions
- 🌐 **Universal**: Works in browsers, Node.js, and bundlers

## Installation

```bash
npm install @octofhir/ucum-wasm
```

## Quick Start

```javascript
import { start, validate, convert, get_unit_info } from '@octofhir/ucum-wasm';

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

## Type Definitions

### `UnitInfo`
```typescript
interface UnitInfo {
  code: string;           // UCUM code
  factor: number;         // Conversion factor to canonical unit
  offset: number;         // Offset for linear conversions
  is_special: boolean;    // Non-linear conversion required
  is_arbitrary: boolean;  // Arbitrary unit (e.g., [IU])
  dimensions: number[];   // Dimensional vector [M, L, T, I, Θ, N, J]
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

## Contributing

This package is part of the [ucum-rs](https://github.com/octofhir/ucum-rs) project. Please see the main repository for contribution guidelines.

## License

MIT License - see the [LICENSE](https://github.com/octofhir/ucum-rs/blob/main/LICENSE) file for details.

## Related Packages

- [`@octofhir/ucum-core`](https://crates.io/crates/octofhir-ucum-core) - Rust core library
- [`@octofhir/ucum-cli`](https://crates.io/crates/octofhir-ucum-cli) - Command-line interface

## Support

For issues and questions, please visit the [GitHub repository](https://github.com/octofhir/ucum-rs/issues).
