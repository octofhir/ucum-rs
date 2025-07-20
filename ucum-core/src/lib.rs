//! UCUM Core Library – Rust 2024 Edition
//!
//! This crate provides parsing, validation and conversion utilities for the
//! Unified Code for Units of Measure (UCUM). It aims to be `no_std`-optional
//! and suitable for both embedded and server environments.

#![cfg_attr(not(feature = "std"), no_std)]

mod ast;
mod error;
mod evaluator;
mod expr;
mod parser;
mod registry;
mod types;

pub use crate::ast::{UnitExpr, UnitFactor};
pub use crate::error::UcumError;
pub use crate::evaluator::{EvalResult, evaluate};
pub use crate::expr::parse_expression;
pub use crate::types::{BaseUnit, DerivedUnit, Dimension, Prefix, Quantity, UnitRecord};

/// Lookup a unit by code using the generated registry.
pub fn find_unit(code: &str) -> Option<&'static crate::types::UnitRecord> {
    registry::find_unit(code)
}

/// Lookup a prefix by symbol.
pub fn find_prefix(sym: &str) -> Option<&'static Prefix> {
    registry::find_prefix(sym)
}
