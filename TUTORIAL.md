# UCUM-RS Tutorial: Getting Started

This tutorial walks through validating, converting and analysing units of measure with
UCUM-RS. See the [User Guide](USER_GUIDE.md) for the concepts behind each step.

## Prerequisites

- Rust 1.85 or later (edition 2024)

## Part 1: Setup

```sh
cargo new ucum-demo
cd ucum-demo
cargo add octofhir-ucum
```

## Part 2: Validation

```rust
use octofhir_ucum::validate;

fn main() {
    for code in ["mg/dL", "kg.m/s2", "mm[Hg]", "Cel", "ug", "µg", "mg/invalid"] {
        match validate(code) {
            Ok(()) => println!("{code}: valid"),
            Err(e) => println!("{code}: {}", e.message),
        }
    }
}
```

`µg` is invalid: UCUM expressions are ASCII only, and the micro prefix is written `u`.

## Part 3: Conversion

```rust
use octofhir_ucum::convert;

fn main() -> Result<(), octofhir_ucum::UcumError> {
    println!("{}", convert(1.0, "[lb_av]", "kg")?); // 0.45359237
    println!("{}", convert(100.0, "kPa", "mm[Hg]")?); // 750.06...
    println!("{}", convert(1.0, "[IU]/mL", "[IU]/L")?); // 1000

    // Units that are not commensurable do not convert
    assert!(convert(1.0, "kg", "m").is_err());
    Ok(())
}
```

## Part 4: Special units

Temperatures, levels and the pH are measured on their own scales; `convert` applies their
functions.

```rust
use octofhir_ucum::convert;

fn main() -> Result<(), octofhir_ucum::UcumError> {
    println!("{}", convert(37.0, "Cel", "[degF]")?); // 98.6
    println!("{}", convert(0.0, "Cel", "K")?); // 273.15
    println!("{}", convert(20.0, "dB[SPL]", "Pa")?); // 0.0002
    println!("{}", convert(7.4, "[pH]", "nmol/L")?); // 39.8...
    println!("{}", convert(1.0, "20.dB", "B")?); // 2: a number scales a special unit

    // A special unit takes part in no product or quotient
    assert!(octofhir_ucum::validate("Cel/h").is_err());
    Ok(())
}
```

## Part 5: Analysis and canonical form

```rust
use octofhir_ucum::{analyse, get_canonical_units, is_comparable, parse_expression, evaluate};

fn main() -> Result<(), octofhir_ucum::UcumError> {
    let analysis = analyse("mg/dL")?;
    println!("{} × {}", analysis.factor, analysis.dimension); // 10 × L-3.M

    let canonical = get_canonical_units("N")?;
    println!("{} {}", canonical.factor, canonical.unit); // 1000 g.m.s-2

    assert!(is_comparable("[psi]", "Pa")?);
    assert!(!is_comparable("Hz", "rad/s")?); // the radian is a base unit

    // The syntax tree and its semantics
    let expr = parse_expression("4.[pi].10*-7.N/A2")?;
    let result = evaluate(&expr)?;
    println!("{} {}", result.factor, result.dim);
    Ok(())
}
```

## Part 6: The command line

```sh
cargo install octofhir-ucum --features cli

octofhir-ucum validate "mg/dL"
octofhir-ucum convert 37 Cel "[degF]"
octofhir-ucum analyze "[pH]"
octofhir-ucum canonical "mm[Hg]"
octofhir-ucum search pressure --limit 5
```

## Next steps

- [User Guide](USER_GUIDE.md): concepts and the full API
- [API documentation](https://docs.rs/octofhir-ucum)
- The UCUM specification in [`spec/`](spec/)
