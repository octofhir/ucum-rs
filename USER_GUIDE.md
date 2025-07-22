# UCUM-RS User Guide

This guide provides an overview of the UCUM-RS library, a Rust implementation of the Unified Code for Units of Measure (UCUM) specification.

## Table of Contents

- [Introduction](#introduction)
- [Core Concepts](#core-concepts)
- [Installation](#installation)
- [Component Overview](#component-overview)
- [UCUM Specification Implementation](#ucum-specification-implementation)
- [Extending the Library](#extending-the-library)
- [Troubleshooting](#troubleshooting)
- [Additional Resources](#additional-resources)

## Introduction

UCUM-RS is a comprehensive implementation of the Unified Code for Units of Measure (UCUM) specification in Rust. It provides tools for parsing, validating, and converting between units of measure, with a focus on healthcare and scientific applications.

### What is UCUM?

The Unified Code for Units of Measure (UCUM) is a system designed to include all units of measures being contemporarily used in international science, engineering, and business. The purpose is to facilitate unambiguous electronic communication of quantities together with their units.

UCUM provides:
- A comprehensive set of base units (e.g., meter, gram, second)
- Derived units (e.g., Newton, Pascal)
- Prefixes (e.g., kilo-, milli-, micro-)
- Rules for combining units
- Special units (e.g., temperature with offsets)
- Arbitrary units (e.g., international units [IU])

### Why UCUM-RS?

UCUM-RS offers several advantages:

- **Performance**: Written in Rust for speed and memory safety
- **Comprehensive**: Full implementation of the UCUM specification
- **Cross-platform**: Works on all platforms supported by Rust
- **WebAssembly support**: Can be used in web applications
- **FHIR integration**: Seamless integration with healthcare applications
- **Robust error handling**: Clear error messages for invalid inputs
- **Extensible**: Can be extended with custom units

## Core Concepts

Understanding these core concepts will help you use UCUM-RS effectively:

### Unit Expressions

UCUM unit expressions can be simple (e.g., `kg`) or complex (e.g., `kg.m/s2`). They follow a specific grammar defined in the UCUM specification.

Components of a unit expression:
- **Atomic units**: Basic units like `m` (meter) or `g` (gram)
- **Prefixed units**: Units with prefixes like `kg` (kilogram) or `ms` (millisecond)
- **Annotations**: Additional information in square brackets like `mm[Hg]` (millimeters of mercury)
- **Operators**: Multiplication (`.`), division (`/`), and exponentiation (e.g., `m2` for square meters)

### Unit Registry

UCUM-RS maintains a registry of all standard UCUM units, including:
- Base units (e.g., `m`, `g`, `s`)
- Derived units (e.g., `Pa`, `N`, `J`)
- Customary units (e.g., `[in_i]`, `[lb_av]`)
- Specialized units (e.g., `[IU]`, `Cel`)

Each unit in the registry has:
- A unique code
- A canonical form
- Conversion factors to base units
- Dimensional information

### Dimensions

UCUM-RS uses dimensional analysis to ensure that unit conversions are valid. Each unit has a dimensional vector representing its physical dimensions:

- Mass (M)
- Length (L)
- Time (T)
- Electric current (I)
- Temperature (Θ)
- Amount of substance (N)
- Luminous intensity (J)

For example:
- `kg` has dimensions [1,0,0,0,0,0,0] (mass)
- `m/s` has dimensions [0,1,-1,0,0,0,0] (length/time)
- `N` has dimensions [1,1,-2,0,0,0,0] (mass·length/time²)

Units can only be converted if they have the same dimensions.

### Special Units

Some units require special handling:
- **Temperature units**: Have offsets (e.g., Celsius, Fahrenheit)
- **Logarithmic units**: Require logarithmic conversions (e.g., decibels)
- **Arbitrary units**: Not defined in terms of other units (e.g., [IU])

### Evaluation Process

When evaluating a unit expression, UCUM-RS:
1. Parses the expression into an abstract syntax tree (AST)
2. Evaluates the AST to determine the canonical form, conversion factor, and dimensions
3. Performs any necessary special conversions (e.g., for temperature units)

## Installation

For detailed installation instructions for each component, please refer to the component-specific documentation:

- [Core Library](ucum-core/README.md#installation)
- [CLI Tool](ucum-cli/README.md#installation)
- [WebAssembly Package](ucum-wasm/README.md#installation)
- [FHIR Integration](ucum-fhir/README.md#installation)
- [Playground](playground/README.md#installation)

## Component Overview

UCUM-RS consists of several components, each serving a specific purpose:

### Core Library (`octofhir-ucum-core`)

The core library provides the fundamental functionality for parsing, evaluating, and converting UCUM expressions.

Key features:
- Parsing UCUM expressions
- Evaluating expressions to determine canonical form, conversion factor, and dimensions
- Converting between units
- Looking up units in the registry
- Handling special units (temperature, logarithmic, arbitrary)

For detailed API documentation, see the [Core Library README](ucum-core/README.md).

### CLI Tool (`octofhir-ucum-cli`)

The CLI tool provides a command-line interface for working with UCUM expressions.

Available commands:
- `validate`: Validate a UCUM expression
- `convert`: Convert a value from one unit to another
- `list-units`: List all supported units
- `explain`: Explain a unit code
- `parse`: Parse a UCUM expression and print the AST
- `evaluate`: Evaluate a UCUM expression and print the result
- `arithmetic`: Perform arithmetic operations on unit expressions

For detailed usage information, see the [CLI Tool README](ucum-cli/README.md).

### WebAssembly Package (`@octofhir/ucum-wasm`)

The WebAssembly package provides JavaScript/TypeScript bindings for the UCUM library.

Key features:
- Validating UCUM expressions
- Converting between units
- Getting information about units
- Evaluating expressions
- Performing arithmetic operations on units
- FHIR integration

For detailed API documentation, see the [WebAssembly Package README](ucum-wasm/README.md).

### FHIR Integration (`octofhir-ucum-fhir`)

The FHIR integration provides integration with FHIR (Fast Healthcare Interoperability Resources).

Key features:
- Converting between FHIR Quantity and UCUM Quantity
- Converting FHIR Quantities between different units
- Checking if two FHIR Quantities are equivalent
- Handling arbitrary units in FHIR Quantities

For detailed API documentation, see the [FHIR Integration README](ucum-fhir/README.md).

### Playground

The playground is an interactive web application for exploring the UCUM library's capabilities.

Features:
- Validating UCUM expressions
- Converting between units
- Getting information about units
- Evaluating expressions
- Performing arithmetic operations on units

For information on running the playground locally, see the [Playground README](playground/README.md).

## UCUM Specification Implementation

UCUM-RS implements the [Unified Code for Units of Measure (UCUM) specification](https://ucum.org/ucum.html), which defines a system for unambiguous representation of units of measure.

### Base Units

UCUM-RS implements all base units defined in the UCUM specification:

| Dimension | Base Unit | Symbol | UCUM Code |
|-----------|-----------|--------|-----------|
| Mass | kilogram | kg | kg |
| Length | meter | m | m |
| Time | second | s | s |
| Electric current | ampere | A | A |
| Temperature | kelvin | K | K |
| Amount of substance | mole | mol | mol |
| Luminous intensity | candela | cd | cd |

### Derived Units

UCUM-RS implements all derived units defined in the UCUM specification, including:

| Unit | Symbol | UCUM Code | Definition |
|------|--------|-----------|------------|
| Newton | N | N | kg·m/s² |
| Pascal | Pa | Pa | N/m² |
| Joule | J | J | N·m |
| Watt | W | W | J/s |
| Coulomb | C | C | A·s |
| Volt | V | V | W/A |
| Farad | F | F | C/V |
| Ohm | Ω | Ohm | V/A |
| Siemens | S | S | A/V |
| Weber | Wb | Wb | V·s |
| Tesla | T | T | Wb/m² |
| Henry | H | H | Wb/A |
| Lumen | lm | lm | cd·sr |
| Lux | lx | lx | lm/m² |
| Becquerel | Bq | Bq | 1/s |
| Gray | Gy | Gy | J/kg |
| Sievert | Sv | Sv | J/kg |
| Katal | kat | kat | mol/s |

### Prefixes

UCUM-RS implements all prefixes defined in the UCUM specification:

| Prefix | Symbol | UCUM Code | Factor |
|--------|--------|-----------|--------|
| yotta | Y | Y | 10²⁴ |
| zetta | Z | Z | 10²¹ |
| exa | E | E | 10¹⁸ |
| peta | P | P | 10¹⁵ |
| tera | T | T | 10¹² |
| giga | G | G | 10⁹ |
| mega | M | M | 10⁶ |
| kilo | k | k | 10³ |
| hecto | h | h | 10² |
| deka | da | da | 10¹ |
| deci | d | d | 10⁻¹ |
| centi | c | c | 10⁻² |
| milli | m | m | 10⁻³ |
| micro | μ | u | 10⁻⁶ |
| nano | n | n | 10⁻⁹ |
| pico | p | p | 10⁻¹² |
| femto | f | f | 10⁻¹⁵ |
| atto | a | a | 10⁻¹⁸ |
| zepto | z | z | 10⁻²¹ |
| yocto | y | y | 10⁻²⁴ |

### Special Units

UCUM-RS implements special units defined in the UCUM specification, including:

| Unit | UCUM Code | Special Handling |
|------|-----------|------------------|
| Celsius | Cel | Temperature with offset |
| Fahrenheit | [degF] | Temperature with offset |
| Decibel | dB | Logarithmic scale |
| pH | [pH] | Logarithmic scale |
| International Unit | [IU] | Arbitrary unit |
| Arbitrary Unit | [arb'U] | Arbitrary unit |

### Grammar

UCUM-RS implements the UCUM grammar for parsing unit expressions, including:

- Simple units (e.g., `m`, `kg`)
- Prefixed units (e.g., `km`, `mg`)
- Annotated units (e.g., `mm[Hg]`)
- Multiplication (e.g., `N.m`)
- Division (e.g., `m/s`)
- Exponentiation (e.g., `m2`, `m3`)
- Grouping with parentheses (e.g., `g/(m.s2)`)

## Extending the Library

UCUM-RS can be extended with custom units for specific domains or applications. For detailed examples of how to extend the library, please refer to:

- [Core Library Extension Guide](ucum-core/README.md#extending-the-library)
- [WebAssembly Extension Guide](ucum-wasm/README.md#extending-the-library)
- [Advanced Tutorial on Custom Units](ADVANCED_TUTORIAL.md#part-3-custom-unit-registry)

## Troubleshooting

For troubleshooting common issues, please refer to:

- [Basic Tutorial Troubleshooting](TUTORIAL.md)
- [Advanced Tutorial Troubleshooting](ADVANCED_TUTORIAL.md)
- [Core Library Troubleshooting](ucum-core/README.md#troubleshooting)
- [WebAssembly Troubleshooting](ucum-wasm/README.md#troubleshooting)

## Additional Resources

- [Getting Started Tutorial](TUTORIAL.md) - Step-by-step guide for beginners
- [Advanced Tutorial](ADVANCED_TUTORIAL.md) - Complex use cases and advanced features
- [UCUM Specification](https://ucum.org/ucum.html) - Official UCUM specification
- [FHIR Quantity](https://www.hl7.org/fhir/datatypes.html#Quantity) - FHIR Quantity data type that uses UCUM
- [GitHub Repository](https://github.com/octofhir/ucum-rs) - Source code and issue tracker
