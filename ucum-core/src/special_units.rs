//! Special unit handlers for extensible UCUM special unit processing.
//!
//! This module provides a pluggable system for handling special units that require
//! non-linear conversions, such as temperature units with offsets, logarithmic units,
//! and other special cases defined in the UCUM specification.

use crate::error::UcumError;
use crate::precision::{Number, NumericOps, from_f64};
use crate::types::Dimension;
use std::collections::HashMap;

/// Trait for handling special unit conversions.
///
/// Special units are those that require non-linear conversions or have special
/// properties that cannot be handled by simple multiplication factors.
pub trait SpecialUnitHandler: Send + Sync {
    /// Get the name of this handler (for debugging and registration).
    fn name(&self) -> &'static str;

    /// Check if this handler can process the given unit code.
    fn can_handle(&self, unit_code: &str) -> bool;

    /// Convert a value from this special unit to its base unit.
    ///
    /// # Arguments
    /// * `value` - The numeric value to convert
    /// * `unit_code` - The unit code being converted from
    /// * `context` - Additional context for the conversion
    ///
    /// # Returns
    /// The converted value in base units, or an error if conversion fails.
    fn convert_from(
        &self,
        value: Number,
        unit_code: &str,
        context: &ConversionContext,
    ) -> Result<Number, UcumError>;

    /// Convert a value from base units to this special unit.
    ///
    /// # Arguments
    /// * `value` - The numeric value in base units
    /// * `unit_code` - The unit code being converted to
    /// * `context` - Additional context for the conversion
    ///
    /// # Returns
    /// The converted value in the special unit, or an error if conversion fails.
    fn convert_to(
        &self,
        value: Number,
        unit_code: &str,
        context: &ConversionContext,
    ) -> Result<Number, UcumError>;

    /// Get the dimension vector for this special unit.
    fn get_dimension(&self, unit_code: &str) -> Dimension;

    /// Get the base conversion factor (for units that have a linear component).
    fn get_base_factor(&self, unit_code: &str) -> Number;
}

/// Context information for special unit conversions.
#[derive(Debug, Clone)]
pub struct ConversionContext {
    /// The source unit code (if converting between special units)
    pub source_unit: Option<String>,
    /// The target unit code (if converting between special units)
    pub target_unit: Option<String>,
    /// Additional parameters for the conversion
    pub parameters: HashMap<String, Number>,
}

impl ConversionContext {
    /// Create a new conversion context.
    pub fn new() -> Self {
        Self {
            source_unit: None,
            target_unit: None,
            parameters: HashMap::new(),
        }
    }

    /// Set the source unit for the conversion.
    pub fn with_source_unit(mut self, unit: String) -> Self {
        self.source_unit = Some(unit);
        self
    }

    /// Set the target unit for the conversion.
    pub fn with_target_unit(mut self, unit: String) -> Self {
        self.target_unit = Some(unit);
        self
    }

    /// Add a parameter to the conversion context.
    pub fn with_parameter(mut self, key: String, value: Number) -> Self {
        self.parameters.insert(key, value);
        self
    }
}

impl Default for ConversionContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Registry for special unit handlers.
///
/// This registry allows for pluggable special unit handling, making it easy to
/// add new special unit types or customize existing ones.
pub struct SpecialUnitRegistry {
    handlers: Vec<Box<dyn SpecialUnitHandler>>,
    handler_map: HashMap<String, usize>, // unit_code -> handler_index
}

impl SpecialUnitRegistry {
    /// Create a new special unit registry.
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
            handler_map: HashMap::new(),
        }
    }

    /// Register a new special unit handler.
    ///
    /// # Arguments
    /// * `handler` - The handler to register
    ///
    /// # Returns
    /// The index of the registered handler.
    pub fn register(&mut self, handler: Box<dyn SpecialUnitHandler>) -> usize {
        let index = self.handlers.len();
        self.handlers.push(handler);
        index
    }

    /// Find a handler for the given unit code.
    pub fn find_handler(&self, unit_code: &str) -> Option<&dyn SpecialUnitHandler> {
        // First check the cached mapping
        if let Some(&index) = self.handler_map.get(unit_code) {
            return self.handlers.get(index).map(|h| h.as_ref());
        }

        // Search through handlers
        for handler in &self.handlers {
            if handler.can_handle(unit_code) {
                return Some(handler.as_ref());
            }
        }

        None
    }

    /// Get all registered handlers.
    pub fn handlers(&self) -> &[Box<dyn SpecialUnitHandler>] {
        &self.handlers
    }

    /// Cache a unit code to handler mapping for performance.
    pub fn cache_mapping(&mut self, unit_code: String, handler_index: usize) {
        self.handler_map.insert(unit_code, handler_index);
    }
}

impl Default for SpecialUnitRegistry {
    fn default() -> Self {
        let mut registry = Self::new();

        // Register default handlers
        registry.register(Box::new(TemperatureHandler::new()));
        registry.register(Box::new(LogarithmicHandler::new()));
        registry.register(Box::new(ArbitraryHandler::new()));

        registry
    }
}

/// Handler for temperature units with offset support.
///
/// Handles Celsius (°C) and Fahrenheit (°F) conversions which require
/// both scaling and offset operations.
pub struct TemperatureHandler;

impl TemperatureHandler {
    pub fn new() -> Self {
        Self
    }
}

impl SpecialUnitHandler for TemperatureHandler {
    fn name(&self) -> &'static str {
        "Temperature"
    }

    fn can_handle(&self, unit_code: &str) -> bool {
        matches!(unit_code, "Cel" | "[degF]" | "[degR]")
    }

    fn convert_from(
        &self,
        value: Number,
        unit_code: &str,
        _context: &ConversionContext,
    ) -> Result<Number, UcumError> {
        match unit_code {
            "Cel" => {
                // Celsius to Kelvin: K = °C + 273.15
                Ok(value.add(from_f64(273.15)))
            }
            "[degF]" => {
                // Fahrenheit to Kelvin: K = (°F + 459.67) × 5/9
                Ok(value.add(from_f64(459.67)).mul(from_f64(5.0 / 9.0)))
            }
            "[degR]" => {
                // Rankine to Kelvin: K = °R × 5/9
                Ok(value.mul(from_f64(5.0 / 9.0)))
            }
            _ => Err(UcumError::ConversionError("Unknown temperature unit")),
        }
    }

    fn convert_to(
        &self,
        value: Number,
        unit_code: &str,
        _context: &ConversionContext,
    ) -> Result<Number, UcumError> {
        match unit_code {
            "Cel" => {
                // Kelvin to Celsius: °C = K - 273.15
                Ok(value.sub(from_f64(273.15)))
            }
            "[degF]" => {
                // Kelvin to Fahrenheit: °F = K × 9/5 - 459.67
                Ok(value.mul(from_f64(9.0 / 5.0)).sub(from_f64(459.67)))
            }
            "[degR]" => {
                // Kelvin to Rankine: °R = K × 9/5
                Ok(value.mul(from_f64(9.0 / 5.0)))
            }
            _ => Err(UcumError::ConversionError("Unknown temperature unit")),
        }
    }

    fn get_dimension(&self, _unit_code: &str) -> Dimension {
        // All temperature units have the same dimension: thermodynamic temperature
        Dimension([0, 0, 0, 0, 1, 0, 0])
    }

    fn get_base_factor(&self, _unit_code: &str) -> Number {
        // Temperature units don't have a simple linear factor due to offsets
        Number::one()
    }
}

/// Handler for logarithmic units (decibels, nepers, etc.).
pub struct LogarithmicHandler;

impl LogarithmicHandler {
    pub fn new() -> Self {
        Self
    }
}

impl SpecialUnitHandler for LogarithmicHandler {
    fn name(&self) -> &'static str {
        "Logarithmic"
    }

    fn can_handle(&self, unit_code: &str) -> bool {
        matches!(unit_code, "B" | "dB" | "Np")
    }

    fn convert_from(
        &self,
        value: Number,
        unit_code: &str,
        _context: &ConversionContext,
    ) -> Result<Number, UcumError> {
        match unit_code {
            "B" => {
                // Bel: 10^value
                Ok(from_f64(10.0_f64.powf(value.to_f64())))
            }
            "dB" => {
                // Decibel: 10^(value/10)
                Ok(from_f64(10.0_f64.powf(value.to_f64() / 10.0)))
            }
            "Np" => {
                // Neper: e^value
                Ok(from_f64(value.to_f64().exp()))
            }
            _ => Err(UcumError::ConversionError("Unknown logarithmic unit")),
        }
    }

    fn convert_to(
        &self,
        value: Number,
        unit_code: &str,
        _context: &ConversionContext,
    ) -> Result<Number, UcumError> {
        let val = value.to_f64();
        if val <= 0.0 {
            return Err(UcumError::ConversionError(
                "Cannot take logarithm of non-positive value",
            ));
        }

        match unit_code {
            "B" => {
                // To Bel: log10(value)
                Ok(from_f64(val.log10()))
            }
            "dB" => {
                // To Decibel: 10 * log10(value)
                Ok(from_f64(10.0 * val.log10()))
            }
            "Np" => {
                // To Neper: ln(value)
                Ok(from_f64(val.ln()))
            }
            _ => Err(UcumError::ConversionError("Unknown logarithmic unit")),
        }
    }

    fn get_dimension(&self, _unit_code: &str) -> Dimension {
        // Logarithmic units are dimensionless
        Dimension([0, 0, 0, 0, 0, 0, 0])
    }

    fn get_base_factor(&self, _unit_code: &str) -> Number {
        Number::one()
    }
}

/// Handler for arbitrary units (square-bracketed units).
pub struct ArbitraryHandler;

impl ArbitraryHandler {
    pub fn new() -> Self {
        Self
    }
}

impl SpecialUnitHandler for ArbitraryHandler {
    fn name(&self) -> &'static str {
        "Arbitrary"
    }

    fn can_handle(&self, unit_code: &str) -> bool {
        unit_code.starts_with('[') && unit_code.ends_with(']')
    }

    fn convert_from(
        &self,
        value: Number,
        _unit_code: &str,
        _context: &ConversionContext,
    ) -> Result<Number, UcumError> {
        // Arbitrary units are typically dimensionless with factor 1
        Ok(value)
    }

    fn convert_to(
        &self,
        value: Number,
        _unit_code: &str,
        _context: &ConversionContext,
    ) -> Result<Number, UcumError> {
        // Arbitrary units are typically dimensionless with factor 1
        Ok(value)
    }

    fn get_dimension(&self, _unit_code: &str) -> Dimension {
        // Most arbitrary units are dimensionless
        // Specific arbitrary units may override this
        Dimension([0, 0, 0, 0, 0, 0, 0])
    }

    fn get_base_factor(&self, _unit_code: &str) -> Number {
        Number::one()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temperature_handler() {
        let handler = TemperatureHandler::new();
        let context = ConversionContext::new();

        // Test Celsius to Kelvin
        let celsius_to_kelvin = handler
            .convert_from(from_f64(0.0), "Cel", &context)
            .unwrap();
        assert!((celsius_to_kelvin.to_f64() - 273.15).abs() < 1e-10);

        // Test Kelvin to Celsius
        let kelvin_to_celsius = handler
            .convert_to(from_f64(273.15), "Cel", &context)
            .unwrap();
        assert!(kelvin_to_celsius.to_f64().abs() < 1e-10);

        // Test Fahrenheit to Kelvin
        let fahrenheit_to_kelvin = handler
            .convert_from(from_f64(32.0), "[degF]", &context)
            .unwrap();
        assert!((fahrenheit_to_kelvin.to_f64() - 273.15).abs() < 1e-10);
    }

    #[test]
    fn test_logarithmic_handler() {
        let handler = LogarithmicHandler::new();
        let context = ConversionContext::new();

        // Test decibel conversion
        let db_to_ratio = handler
            .convert_from(from_f64(20.0), "dB", &context)
            .unwrap();
        assert!((db_to_ratio.to_f64() - 100.0).abs() < 1e-10);

        let ratio_to_db = handler.convert_to(from_f64(100.0), "dB", &context).unwrap();
        assert!((ratio_to_db.to_f64() - 20.0).abs() < 1e-10);
    }

    #[test]
    fn test_registry() {
        let mut registry = SpecialUnitRegistry::new();
        registry.register(Box::new(TemperatureHandler::new()));

        let handler = registry.find_handler("Cel");
        assert!(handler.is_some());
        assert_eq!(handler.unwrap().name(), "Temperature");

        let no_handler = registry.find_handler("unknown_unit");
        assert!(no_handler.is_none());
    }
}
