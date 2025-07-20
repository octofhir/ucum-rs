//! Generated registry of UCUM prefixes and units.
//!
//! At compile time, `build.rs` parses `spec/ucum-essence.xml` and emits a Rust
//! source file containing two `&'static` slices: `PREFIXES` and `UNITS`, plus
//! helper lookup functions. That generated file is included here so the rest of
//! the crate can access the data transparently.

// This constant is set by build.rs via `cargo:rustc-env=UCUM_REGISTRY=<path>`.
include!(env!("UCUM_REGISTRY"));
