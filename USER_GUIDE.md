# UCUM-RS User Guide

UCUM-RS implements the [Unified Code for Units of Measure](https://ucum.org/) (UCUM),
version 2.2, in Rust. It follows the specification as a requirement: see "Specification
conformance" in the [README](README.md). Section numbers (§) below refer to the
specification, whose text is in [`spec/`](spec/).

## Concepts

### Unit expressions (§1-10)

A UCUM expression is a term of simple units joined by `.` (multiplication) and `/`
(division), read from left to right: `kg.m/s2`, `mg/dL`, `/min`.

- A **simple unit** is a unit atom (`m`, `[in_i]`, `10*`) with an optional prefix (`k`,
  `da`, `Ki`). Only metric atoms take prefixes (§11): `km` is valid, `k[in_i]` is not. The
  prefix is the longest one that leaves a metric atom (§4.4).
- An **exponent** follows a simple unit: `m2`, `s-1`, `10*-3`. There is no `^` operator and
  no exponent on parentheses.
- A **number** is a positive integer: `4.[pi]`, `1000.mL`. `2.5` is 2 × 5.
- An **annotation** in curly braces has no meaning (§6): `mg{total}` equals `mg`, `{rbc}`
  equals `1`.
- **Parentheses** group terms: `kg/(m.s)`.
- Expressions use the characters 33-126 of ASCII only: no white space, no `µ` (write `u`).

### Base units and dimensions (§20, §28)

Every proper unit is a magnitude times a product of powers of the seven base units, in the
order of `Dimension`:

| Base unit | Symbol | Dimension |
|---|---|---|
| meter | `m` | L (length) |
| second | `s` | T (time) |
| gram | `g` | M (mass) |
| radian | `rad` | A (plane angle) |
| kelvin | `K` | Θ (temperature) |
| coulomb | `C` | Q (electric charge) |
| candela | `cd` | F (luminous intensity) |

So the newton `N` is 1000 `g.m.s-2` (dimension `L.T-2.M`), the ampere `A` is `C/s`, and the
steradian `sr` is `rad2`. The mole is a number, 6.02214076·10²³ (§30): `mol/L` has the
dimension `L-3`.

Two units are **commensurable** (§17) when they have the same dimension; only commensurable
units convert to each other. `Hz` and `rad/s` are not commensurable, since the radian is a
base unit.

Magnitudes are exact decimal numbers of any size (`Number`): `[in_i]` is exactly 2.54 cm,
and the Planck constant `[h]` is 6.62607015·10⁻³¹ `g.m2.s-1`. `analyse`,
`get_canonical_units`, `unit_multiply`, `unit_divide` and the linear mapping of
`convert_with_context` return them as `Number`, with 28 significant digits and any
exponent, so `10*1000` has a factor of exactly 10¹⁰⁰⁰; `Number::to_f64` rounds one to the
nearest `f64`. Values of quantities, in `convert` and `multiply`, are `f64`.

### Special units (§21-23)

Some units measure on a scale that is not a ratio scale: the temperatures `Cel`, `[degF]`
and `[degRe]`, the levels `B`, `Np`, `bit_s`, `B[V]`, `B[SPL]` and their prefixed forms
(`dB`), `[pH]`, the homeopathic potencies `[hp'_X]`..., the prism diopter `[p'diop]`, the
percent of slope `%[slope]` and the amplitude spectral density `[m/s2/Hz^(1/2)]`.

A special unit is defined by a function `f`, its inverse, and a corresponding proper unit
`u` (`SpecialKind` names the function). A value `r` in the special unit is the proper
quantity `f⁻¹(r) × u`:

| Unit | Function | Example |
|---|---|---|
| `Cel` | `x - 273.15` of `K` | 0 `Cel` = 273.15 `K` |
| `B` | `lg x` of `1` | 2 `B` = 100 |
| `B[V]` | `2 lg x` of `V` | 0 `B[V]` = 1 `V` |
| `[pH]` | `-lg x` of `mol/L` | 7 `[pH]` = 10⁻⁷ `mol/L` |
| `[p'diop]` | `100 tan x` of `rad` | 100 `[p'diop]` = π/4 `rad` |

A special unit is commensurable with its corresponding unit and converts to it and to the
other special units of the same dimension (`dB` to `Np`, `Cel` to `[degF]`).

A prefix or a number **scales** a special unit (§22.3): `mCel` is a thousandth of a degree
Celsius, `20.dB` is 2 `B`. Special units take part in no other algebra (§22.1): `Cel/h`,
`B2` and `/dB` are errors.

### Arbitrary units (§24-26)

Arbitrary units such as `[IU]`, `[arb'U]` or `[CFU]` depend on a measurement procedure and
are commensurable with no other unit: `[IU]` does not convert to `1` or to `[arb'U]`. A term
with arbitrary units converts only to a term with the same arbitrary units: `[IU]/mL`
converts to `[IU]/L` and `k[IU]` to `[IU]`.

### Case insensitive codes (§3.4)

Every symbol has a case insensitive form for systems that cannot keep case: `PAL` for `Pa`,
`MG` for `mg` (in case sensitive codes `MG` is the megagauss). Pass `Case::Insensitive` to
`validate_with_case`, `evaluate_with_case` or `AdvancedConversionContext`. The two forms
cannot be mixed.

## API overview

| Function | Purpose |
|---|---|
| `validate`, `validate_with_case` | Check syntax and semantics |
| `parse_expression`, `evaluate`, `evaluate_with_case` | Syntax tree and its semantics (`EvalResult`) |
| `analyse` | Dimension, magnitude, arbitrary atoms, special unit |
| `is_comparable` | Commensurability |
| `convert`, `convert_with_context` | Conversion, with rounding and case options |
| `get_canonical_units`, `canonicalize_expression` | Canonical unit in base units |
| `optimize_expression`, `simplify_expression` | Equal, easier to read expressions |
| `multiply`, `divide_by` | Arithmetic on quantities |
| `unit_multiply`, `unit_divide` | Arithmetic on units |
| `find_unit`, `lookup_unit`, `find_prefix`, `get_all_units`, `get_all_prefixes` | The UCUM tables |
| `search_units`, `search_units_by_property`, `search_units_regex`, `search_units_fuzzy`, `search_units_filtered` | Search |
| `get_defined_forms` | Unit atoms commensurable with an expression |
| `validate_in_property`, `get_properties` | Kinds of quantity |
| `validate_canonical_units` | Equality with a canonical form |
| `display_name`, `generate_display_name`, `get_common_display` | Names |
| `get_model`, `validate_ucum` | The tables and their consistency |

### Errors

Errors are `UcumError` values: a `kind` that tells what failed, with its data, a `message`
for people, and `suggestions` (`Did you mean 'Pa'?`): every function suggests corrections
for an unknown unit with case sensitive codes, and `validate` also for a syntax error. A
case insensitive lookup gets no suggestions: `Pa` read case insensitively is the
picoampere (§3.4). `kind.code()` names the kind with a
stable string, which a service can return to its clients. The last column is the HTTP
status such a service would typically answer with; the library itself never panics.

| Kind | `code()` | When | HTTP |
|---|---|---|---|
| `InvalidExpression { reason, expression, position }` | `invalid_expression` | Not UCUM syntax (§1-10): `m//s`, `0.m`, white space; `position` is the byte offset in `expression` where parsing stopped, which tells the two operands of `convert` apart | 400 |
| `NestingTooDeep { limit }` | `nesting_too_deep` | Nested deeper than `MAX_NESTING_DEPTH` levels (a tree built by hand: 3 more nodes), a limit of this library | 400 |
| `UnitNotFound { unit, expression, position }` | `unit_not_found` | An atom that is not in the tables; `position` is the byte offset in `expression` where its symbol starts (`m/qq` at 2) | 400 |
| `InvalidProperty { property }` | `invalid_property` | A property no unit has | 400 |
| `InvalidPattern { pattern, reason }` | `invalid_pattern` | A search pattern that is not a regular expression | 400 |
| `NonFiniteValue` | `non_finite_value` | An operand that is infinite or NaN (§18.5) | 400 |
| `DimensionMismatch { expected, found, from, to, .. }` | `dimension_mismatch` | Proper units of different dimensions (§17, §19.1): `m` to `kg` | 422 |
| `ConversionError` | `conversion_error` | A conversion involving an arbitrary unit (§24.3, §25): `[IU]` to `[arb'U]`, `[IU]` to `m` | 422 |
| `SpecialUnitError` | `special_unit_error` | A special unit in algebra (§22.1), or without a linear canonical form | 422 |
| `OutOfDomain { unit, value }` | `out_of_domain` | A value outside the domain of a special unit's function (§21.2): 0 `mol/L` to `[pH]` | 422 |
| `DivisionByZero` | `division_by_zero` | A divisor value of zero | 422 |
| `PrecisionOverflow` | `precision_overflow` | A value too large or too small for an `f64`, or a factor whose power of ten does not fit in a `Number` | 422 |

`ErrorKind` and each of its variants are `#[non_exhaustive]`: a `match` on the kind needs
a `_` arm and a pattern needs `..` (`ErrorKind::InvalidExpression { position, .. }`), so
that new kinds and fields are not breaking. Errors are built with the constructors of
`UcumError` (`UcumError::invalid_expression`).

## Command-line interface

```sh
cargo install octofhir-ucum --features cli
```

| Command | Purpose |
|---|---|
| `validate <expr>` | Validate an expression |
| `analyze <expr>` | Show dimension, factor, arbitrary atoms and special unit |
| `canonical <expr>` | Show the canonical unit, factor and offset |
| `convert <value> <from> <to>` | Convert a value |
| `comparable <unit1> <unit2>` | Check commensurability |
| `search <query> [--limit N]` | Search the tables |

`-i` (`--case-insensitive`) reads case insensitive codes in `validate` and `convert`.

## WebAssembly

The WebAssembly bindings (`wasm/`, built with wasm-pack) export `validate`, `analyze`, `get_canonical`,
`get_unit_info`, `search`, `list_units`, `comparable`, `convert`, `multiply_units` and
`divide_units`. Errors are thrown as `{ message, error_type, code, expression, position,
suggestions, context }` objects: `code` is `ErrorKind::code()`, `position` the offset of a
syntax error or of an unknown unit in `expression`.

## FHIR

With the `fhir` feature, `octofhir_ucum::fhir` converts FHIR `Quantity` values with UCUM
codes (`convert_quantity`) and compares them (`are_equivalent`), with the same semantics as
`convert`.

## no_std

Without the default `std` feature the crate is `no_std` + `alloc`. Everything but
`search_units_regex` and `search_units_fuzzy` is available.

## Troubleshooting

- **"White space is not allowed"**: UCUM expressions have no spaces; trim input codes.
- **"A character outside ASCII 33-126"**: write `ug`, not `µg`.
- **"Unknown unit"**: codes are case sensitive (`Pa`, not `pa`), unless you use
  `Case::Insensitive`; prefixes only go on metric units (`k[in_i]` is not a unit).
- **"a special unit takes part in no product, quotient or power"**: special units cannot be
  combined (§22.1); convert the value to the proper unit first (`Cel` to `K`).
- **"an arbitrary unit converts only to the same arbitrary unit"**: `[IU]` has no relation to
  other units (§24); only the same arbitrary units convert.
