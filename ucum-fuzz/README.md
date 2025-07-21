# UCUM Fuzzing

This crate provides fuzzing infrastructure for the UCUM core library, using [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz).

## Overview

Fuzzing is a testing technique that involves providing random or semi-random data to a program to find bugs, crashes, or unexpected behavior. This crate provides fuzzing targets for the UCUM core library and FHIR integration, focusing on:

1. **Parser Fuzzing**: Testing the `parse_expression` function with arbitrary input strings
2. **Evaluator Fuzzing**: Testing the `evaluate` function with valid UCUM expressions
3. **FHIR Quantity Fuzzing**: Testing the FHIR integration with arbitrary UCUM codes and values

## Setup

To use this fuzzing infrastructure, you need to install `cargo-fuzz`:

```bash
cargo install cargo-fuzz
```

## Running the Fuzzers

### Parser Fuzzer

The parser fuzzer tests the `parse_expression` function with arbitrary input strings:

```bash
# Run from the repository root
cargo fuzz run -p octofhir-ucum-fuzz fuzz_parser
```

This will generate random strings and pass them to the `parse_expression` function, looking for crashes or panics.

### Evaluator Fuzzer

The evaluator fuzzer tests the `evaluate` function with valid UCUM expressions:

```bash
# Run from the repository root
cargo fuzz run -p octofhir-ucum-fuzz fuzz_evaluator
```

This fuzzer:
1. First tries to parse the input string directly
2. If parsing succeeds, it evaluates the expression
3. If parsing fails, it generates a valid expression by combining valid units, operators, and prefixes
4. Then it tries to parse and evaluate the generated expression

### FHIR Quantity Fuzzer

The FHIR quantity fuzzer tests the FHIR integration with arbitrary UCUM codes and values:

```bash
# Run from the repository root
cargo fuzz run -p octofhir-ucum-fuzz fuzz_fhir_quantity
```

This fuzzer:
1. Takes arbitrary input data and tries to use it as a UCUM code
2. If the input is empty, it selects a random valid unit from a predefined list
3. Creates a FhirQuantity with the code and a value derived from the input data
4. Tests the to_ucum_quantity and to_fhir_quantity methods
5. Tests the are_equivalent function with the same quantity and with a different quantity
6. Tests the convert_quantity function
7. Tests with arbitrary units, including prefixed arbitrary units

## Corpus Management

Cargo-fuzz maintains a corpus of interesting inputs that it has found during fuzzing. You can manage this corpus with the following commands:

```bash
# Add a file to the corpus
cargo fuzz add -p octofhir-ucum-fuzz fuzz_parser path/to/file

# List the corpus
cargo fuzz list -p octofhir-ucum-fuzz fuzz_parser

# Clear the corpus
cargo fuzz cmin -p octofhir-ucum-fuzz fuzz_parser
```

## Continuous Fuzzing

For continuous fuzzing, you can set up a CI job that runs the fuzzers for a fixed amount of time:

```bash
# Run the parser fuzzer for 5 minutes
cargo fuzz run -p octofhir-ucum-fuzz fuzz_parser -- -max_total_time=300

# Run the evaluator fuzzer for 5 minutes
cargo fuzz run -p octofhir-ucum-fuzz fuzz_evaluator -- -max_total_time=300
```

## Handling Crashes

When a fuzzer finds a crash, it will save the input that caused the crash to a file in the `artifacts` directory. You can reproduce the crash with:

```bash
# Reproduce a crash
cargo fuzz run -p octofhir-ucum-fuzz fuzz_parser artifacts/fuzz_parser/crash-*
```

## Fuzzing Targets

### fuzz_parser.rs

This target tests the `parse_expression` function with arbitrary input strings. It:

1. Converts the input bytes to a UTF-8 string if possible
2. Limits the string length to avoid excessive resource usage
3. Passes the string to the `parse_expression` function

### fuzz_evaluator.rs

This target tests the `evaluate` function with valid UCUM expressions. It:

1. First tries to parse the input string directly
2. If parsing succeeds, it evaluates the expression
3. If parsing fails, it generates a valid expression by combining valid units, operators, and prefixes
4. Then it tries to parse and evaluate the generated expression

The target includes comprehensive lists of valid UCUM units, operators, and prefixes to use as building blocks for generating valid expressions.

### fuzz_fhir_quantity.rs

This target tests the FHIR integration with arbitrary UCUM codes and values. It:

1. Takes arbitrary input data and tries to use it as a UCUM code
2. If the input is empty, it selects a random valid unit from a predefined list
3. Creates a FhirQuantity with the code and a value derived from the input data
4. Tests the to_ucum_quantity and to_fhir_quantity methods
5. Tests the are_equivalent function with the same quantity and with a different quantity
6. Tests the convert_quantity function
7. Tests with arbitrary units, including:
   - Testing to_ucum_quantity and to_fhir_quantity with arbitrary units
   - Testing are_equivalent with the same arbitrary unit
   - Testing are_equivalent with different arbitrary units
   - Testing with prefixed arbitrary units

The target includes comprehensive lists of valid UCUM units, prefixes, and arbitrary units to use as building blocks for generating valid test cases.

## Best Practices

1. **Run fuzzers regularly**: Incorporate fuzzing into your CI/CD pipeline
2. **Investigate all crashes**: Every crash found by a fuzzer represents a potential bug
3. **Add regression tests**: When a fuzzer finds a bug, add a regression test to prevent it from recurring
4. **Update the corpus**: As you add new features, update the corpus with new interesting inputs
5. **Monitor resource usage**: Fuzzing can be resource-intensive, so monitor CPU and memory usage

## License

This crate is licensed under the MIT License.
