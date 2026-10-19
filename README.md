# UCUM-RS

[![CI](https://github.com/octofhir/ucum-rs/workflows/CI/badge.svg)](https://github.com/octofhir/ucum-rs/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/octofhir-ucum.svg)](https://crates.io/crates/octofhir-ucum)
[![npm](https://img.shields.io/npm/v/@octofhir/ucum-wasm.svg)](https://www.npmjs.com/package/@octofhir/ucum-wasm)
[![Docs.rs](https://docs.rs/octofhir-ucum/badge.svg)](https://docs.rs/octofhir-ucum)

An implementation of the [Unified Code for Units of Measure](https://ucum.org/) (UCUM) in
Rust: validate, compare and convert unit expressions exactly as the UCUM specification
defines them.

## Specification conformance

**This library follows the UCUM specification, version 2.2 (tables of 2024-06-17).
Conformance is mandatory, not best effort.** When the code and the specification disagree,
the code is wrong: every deviation is a bug, and a fix to it takes priority over backward
compatibility. The specification text, its tables (`ucum-essence.xml`) and the official
functional tests are kept in [`spec/`](spec/) and [`tests/official/`](tests/official/).

What this means in practice:

- **Full conformance (§2.2)**: expressions are compared by their semantics, not their
  spelling. `kg.m/s2` equals `N`, `mg{total}` equals `mg`.
- **The seven UCUM base units (§28)**: meter, second, gram, radian, kelvin, coulomb and
  candela. The radian has a dimension of its own (`Hz` is not `rad/s`, `sr` is `rad2`) and
  the mole is a number, 6.02214076·10²³ (`mol` converts to `10*23`).
- **Exact decimal arithmetic**: factors are decimal numbers of any magnitude, not `f64`.
  `[in_i]` is exactly 2.54 cm, `[m_e]` keeps all its digits at 9.1093837139·10⁻²⁸ g, and
  `[h]` or `Ym3` are valid.
- **Special units (§21-23)** are measured on their own scales through their functions:
  0 `Cel` is 273.15 `K`, 20 `dB[SPL]` is 2·10⁻⁴ `Pa`, 7 `[pH]` is 10⁻⁷ `mol/L`, 100
  `%[slope]` is 45 `deg`. A prefix or a number scales them (`mCel`, `20.dB` is 2 `B`); they
  take part in no product, quotient or power (`Cel/h`, `B2` are errors, §22.1).
- **Arbitrary units (§24-26)** are commensurable with no other unit: `[IU]` does not
  convert to `1` or to `[arb'U]`. A term with an arbitrary unit converts only to a term with
  the same arbitrary units (`[IU]/mL` to `[IU]/L`), as the reference implementation does.
- **Strict syntax (§1-15)**: 7-bit ASCII only, no white space, no `^` operator, no `µ`,
  positive integers only, one non-empty annotation per component.
- **Case insensitive codes (§3.4)** are supported with `Case::Insensitive` (`PAL`, `MG`).
- **100% of the official UCUM functional tests pass** (validation, display names,
  conversion, multiplication, division), checked to the precision each outcome is written in.

Where the sources disagree, this is what the library does:

| Question | Sources | Choice |
|---|---|---|
| Leading solidus: `/m.s` | §7.3 says it inverts the unit right after it; the grammar inverts the whole term | §7.3: `(1/m).s` |
| Annotation on a number: `1{c}` | the grammar forbids it; official test 1-117 accepts it | accepted |
| `%[slope]` | the `<function>` element of the XML says `deg`; its definition and the specification say `rad` | `100tan(1 rad)` |
| White space | §3 allows implementations to reject it or to ignore it | rejected |
| Domain of `100tan` (`%[slope]`) | §21.2 requires `f` and `f⁻¹` to be mutually inverse, which `tan` is only on one branch | principal branch, \|x\| < π/2 rad |
| Empty string | not a term (§7); official test 2-101 gives it the display name "(unity)" | invalid, except for `display_name` |

Found a deviation? Please [open an issue](https://github.com/octofhir/ucum-rs/issues) with
the expression, the result and the section of the specification.

## Quick Start

### Rust Library

```sh
cargo add octofhir-ucum

# no_std + alloc (embedded / bare-metal): disable default features
cargo add octofhir-ucum --no-default-features
```

Without the default `std` feature the crate is `no_std` + `alloc`: parsing, validation,
analysis, conversion and unit arithmetic all work, float math goes through `libm`. Only
`search_units_regex` and `search_units_fuzzy` require `std`.

```rust
use octofhir_ucum::{analyse, convert, is_comparable, unit_multiply, validate};

// Validate UCUM expressions
assert!(validate("mg/dL").is_ok());
assert!(validate("Cel/h").is_err()); // a special unit in a quotient (§22.1)

// Convert between commensurable units
let mmhg = convert(100.0, "kPa", "mm[Hg]")?; // 750.06...
let fahrenheit = convert(37.0, "Cel", "[degF]")?; // 98.6
let concentration = convert(7.4, "[pH]", "nmol/L")?; // 39.8...

// Commensurability
assert!(is_comparable("[psi]", "Pa")?);
assert!(!is_comparable("[IU]", "[arb'U]")?);

// Analysis: magnitude in base units and dimension
let info = analyse("mg/dL")?;
assert_eq!(info.dimension.to_string(), "L-3.M");
assert!((info.factor - 10.0).abs() < 1e-12); // 10 g/m3

// Unit arithmetic
let force = unit_multiply("kg", "m/s2")?;
assert_eq!(force.expression, "kg.m/s2");
# Ok::<(), octofhir_ucum::UcumError>(())
```

### Command-Line Interface

```sh
cargo install octofhir-ucum --features cli

octofhir-ucum validate "mg/dL"
octofhir-ucum convert 100 kPa "mm[Hg]"
octofhir-ucum analyze "[pH]"
octofhir-ucum canonical N
octofhir-ucum comparable Cel "[degF]"
octofhir-ucum search pascal
octofhir-ucum -i validate PAL      # case insensitive codes
```

### JavaScript/TypeScript (WASM)

```sh
npm install @octofhir/ucum-wasm
```

```typescript
import { start, validate, convert, analyze, comparable } from '@octofhir/ucum-wasm';

start();

validate('mg/dL');                    // true
convert(100, 'kPa', 'mm[Hg]');        // 750.06...
comparable('Cel', '[degF]');          // true
analyze('[pH]').special;              // "PH"
```

The package also exports `get_canonical`, `get_unit_info`, `search`, `list_units`,
`multiply_units` and `divide_units`.

## Usage

### Conversion

```rust
use octofhir_ucum::{convert, convert_with_context, AdvancedConversionContext, Case, DecimalPrecision};

convert(1.0, "kg", "g")?;               // 1000
convert(100.0, "Cel", "K")?;            // 373.15
convert(1.0, "[IU]/mL", "[IU]/L")?;     // 1000
convert(0.0, "B[V]", "dB[uV]")?;        // 120
convert(1.0, "mol", "10*23")?;          // 6.02214076
assert!(convert(1.0, "[IU]", "1").is_err());

let context = AdvancedConversionContext {
    precision: DecimalPrecision::Fixed(2),
    case: Case::Insensitive,
    ..Default::default()
};
let result = convert_with_context(1.0, "[IN_I]", "CM", &context)?;
assert_eq!(result.value, 2.54);
assert_eq!(result.factor, Some(2.54)); // None for a non-linear conversion
# Ok::<(), octofhir_ucum::UcumError>(())
```

### Canonical form and expression rewriting

```rust
use octofhir_ucum::{get_canonical_units, optimize_expression, simplify_expression};

let canonical = get_canonical_units("N")?;
assert_eq!(canonical.unit, "g.m.s-2");
assert_eq!(canonical.factor, 1000.0);

assert_eq!(optimize_expression("g.m2.s-2")?, "mJ");
assert_eq!(simplify_expression("km.s/s")?, "km");
# Ok::<(), octofhir_ucum::UcumError>(())
```

### Validation errors

```rust
use octofhir_ucum::validate;

let error = validate("mg/invalid").unwrap_err();
println!("{error}"); // "Unknown unit: 'invalid'", with suggestions
```

## FHIR Integration

```rust
use octofhir_ucum::fhir::{FhirQuantity, are_equivalent, convert_quantity};

let quantity = FhirQuantity::with_ucum_code(1000.0, "mg");
let converted = convert_quantity(&quantity, "g")?;
assert_eq!(converted.value, 1.0);

let gram = FhirQuantity::with_ucum_code(1.0, "g");
assert!(are_equivalent(&gram, &converted)?);
# Ok::<(), octofhir_ucum::fhir::FhirError>(())
```

```sh
cargo add octofhir-ucum --features fhir
```

## Interactive Playground

```sh
cd playground
pnpm install
npm run dev  # http://localhost:6000
```

## Performance

Measured with `cargo bench` (criterion) on an Apple M1, Rust 1.99, release profile:

| Operation | Time |
|---|---|
| Parse a simple unit (`m`, `mg`) | ~62 ns |
| Parse a medical unit (`mg/dL`) | ~170 ns |
| Validate a valid expression | ~0.7 µs |
| Validate an invalid expression, with suggestions | ~38 µs |
| Analyse an expression | ~0.8 µs |
| Convert between proper units (`km` to `m`) | ~1.3 µs |
| Convert a temperature (`Cel` to `[degF]`) | ~2.6 µs |

Exact decimal arithmetic keeps results exact without making the library slower: on the same
expressions 0.7.0 validates, analyses and converts about 2.3 times faster than 0.6.1.

## Documentation

- [API Documentation](https://docs.rs/octofhir-ucum)
- [User Guide](USER_GUIDE.md)
- [Tutorial](TUTORIAL.md)
- [Contributing Guide](CONTRIBUTING.md)
- [Changelog](CHANGELOG.md)

## Project Structure

```
ucum-rs/
├── build.rs                # Generates the unit registry from ucum-essence.xml
├── src/
│   ├── lib.rs              # Public API
│   ├── parser.rs           # Syntax (UCUM §1-15)
│   ├── evaluator.rs        # Semantics and conversion (UCUM §16-26)
│   ├── number.rs           # Exact decimal numbers of any magnitude
│   ├── bin/cli.rs          # Command-line interface
│   └── fhir.rs             # FHIR Quantity integration
├── wasm/                   # WebAssembly bindings (npm: @octofhir/ucum-wasm)
├── playground/             # Interactive web playground
├── spec/                   # UCUM specification text and grammar
└── tests/official/         # Official UCUM functional tests
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). A change that touches semantics must cite the
section of the specification it implements.

## License

Apache-2.0
