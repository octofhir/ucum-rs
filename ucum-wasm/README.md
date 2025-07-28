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
import { 
  start, 
  validate, 
  analyze,
  get_canonical,
  comparable,
  convert, 
  search,
  get_unit_info
} from '@octofhir/ucum-wasm';

// Initialize the WASM module
start();

// Validate UCUM expressions
console.log(validate('kg')); // true
console.log(validate('invalid')); // false

// Analyze unit expressions
const analysis = analyze('mg/dL');
console.log(analysis.factor); // 0.00001
console.log(analysis.dimensions); // [1, -3, 0, 0, 0, 0, 0]

// Get canonical forms
const canonical = get_canonical('mg/dL');
console.log(canonical.unit); // kg.m-3

// Check if units are comparable
console.log(comparable('mg', 'g')); // true
console.log(comparable('mg', 's')); // false

// Convert between units
const result = convert(100, 'g', 'kg');
console.log(result); // 0.1

// Search for units
const units = search('gram');
console.log(units.units.length); // Multiple gram-related units

// Get unit information
const unitInfo = get_unit_info('kg');
console.log(unitInfo.code); // 'kg'
console.log(unitInfo.display_name); // 'kilogram'
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

#### `analyze(expression: string): UnitAnalysis`
Analyze a UCUM unit expression and get detailed information.

```javascript
const analysis = analyze('mg/dL');
// Returns:
// {
//   expression: 'mg/dL',
//   factor: 0.00001,
//   offset: 0,
//   dimensions: [1, -3, 0, 0, 0, 0, 0],
//   is_dimensionless: false,
//   has_offset: false
// }
```

#### `get_canonical(expression: string): CanonicalUnit`
Get the canonical (base unit) representation of a UCUM expression.

```javascript
const canonical = get_canonical('N');
console.log(canonical.unit); // 'kg.m.s-2'
console.log(canonical.factor); // 1000000
```

#### `comparable(unit1: string, unit2: string): boolean`
Check if two units are comparable (have the same dimensions).

```javascript
comparable('kg', 'g'); // true - both are mass
comparable('m', 's'); // false - different dimensions
comparable('m/s', 'km/h'); // true - both are velocity
```

#### `convert(value: number, from_unit: string, to_unit: string): number`
Convert a value from one unit to another.

```javascript
convert(1000, 'g', 'kg'); // 1
convert(100, 'cm', 'm'); // 1
convert(32, '[degF]', 'Cel'); // 0 (Fahrenheit to Celsius)
```

#### `search(query: string): SearchResult`
Search for units by name or code.

```javascript
const results = search('gram');
// Returns:
// {
//   units: [
//     { code: 'g', display_name: 'gram', property: 'mass' },
//     { code: 'kg', display_name: 'kilogram', property: 'mass' },
//     // ... more units
//   ]
// }
```

#### `get_unit_info(code: string): UnitInfo`
Get detailed information about a unit.

```javascript
const info = get_unit_info('kg');
// Returns:
// {
//   code: 'kg',
//   display_name: 'kilogram',
//   property: 'mass'
// }
```

## Type Definitions

### `UnitAnalysis`
```typescript
interface UnitAnalysis {
  expression: string;     // The analyzed expression
  factor: number;         // Conversion factor to canonical unit
  offset: number;         // Offset for units with linear offset
  dimensions: number[];   // Dimensional vector [M, L, T, I, Θ, N, J]
  is_dimensionless: boolean; // Whether the unit is dimensionless
  has_offset: boolean;    // Whether the unit has an offset
}
```

### `CanonicalUnit`
```typescript
interface CanonicalUnit {
  unit: string;           // Canonical unit expression
  factor: number;         // Conversion factor
  offset: number;         // Linear offset
  dimensions: number[];   // Dimensional vector
}
```

### `UnitInfo`
```typescript
interface UnitInfo {
  code: string;           // UCUM code
  display_name: string;   // Human-readable display name
  property: string;       // Unit property or class (e.g., "length", "mass")
}
```

### `SearchResult`
```typescript
interface SearchResult {
  units: UnitInfo[];      // Array of matching units
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

Requires Node.js 16+ with WebAssembly support.

## Performance

- **Validation**: ~322,000 ops/second
- **Parsing**: ~200,000 ops/second
- **Evaluation**: ~1,390,000 ops/second
- **Analysis**: ~606,000 ops/second

## License

Apache-2.0

## Related Packages

- [`octofhir-ucum-core`](https://crates.io/crates/octofhir-ucum-core) - Rust core library
- [`octofhir-ucum-cli`](https://crates.io/crates/octofhir-ucum-cli) - Command-line interface
- [`octofhir-ucum-fhir`](https://crates.io/crates/octofhir-ucum-fhir) - FHIR integration

## Support

For issues and questions, please visit the [GitHub repository](https://github.com/octofhir/ucum-rs/issues).