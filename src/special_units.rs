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
    #[allow(clippy::result_large_err)]
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
    #[allow(clippy::result_large_err)]
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

impl Default for TemperatureHandler {
    fn default() -> Self {
        Self::new()
    }
}

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
            _ => Err(UcumError::conversion_error(
                "temperature unit",
                "conversion",
                "Unknown temperature unit",
            )),
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
            _ => Err(UcumError::conversion_error(
                "temperature unit",
                "conversion",
                "Unknown temperature unit",
            )),
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

/// Enhanced handler for logarithmic units with comprehensive support.
///
/// Supports:
/// - Bel (B) and decibel (dB) - power ratios
/// - Neper (Np) - amplitude ratios  
/// - pH - hydrogen ion concentration
/// - pK values for various chemical equilibria
/// - Other logarithmic scales commonly used in science
pub struct LogarithmicHandler {
    /// Reference values for different logarithmic scales
    #[allow(dead_code)]
    reference_values: HashMap<String, Number>,
}

impl Default for LogarithmicHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl LogarithmicHandler {
    pub fn new() -> Self {
        let mut reference_values = HashMap::new();

        // Standard reference values
        reference_values.insert("pH".to_string(), from_f64(1e-7)); // [H+] = 10^-7 at pH 7
        reference_values.insert("pOH".to_string(), from_f64(1e-7)); // [OH-] = 10^-7 at pOH 7
        reference_values.insert("pKa".to_string(), Number::one()); // Standard Ka reference
        reference_values.insert("pKw".to_string(), from_f64(1e-14)); // Water dissociation constant

        Self { reference_values }
    }

    /// Get the logarithmic base for a given unit
    fn get_base(&self, unit_code: &str) -> f64 {
        match unit_code {
            "B" | "dB" | "pH" | "pOH" | "pKa" | "pKw" | "pK" => 10.0,
            "B[SPL]" | "B[V]" | "B[mV]" | "B[uV]" | "B[10.nV]" | "B[W]" | "B[kW]" => 10.0,
            "Np" => std::f64::consts::E,
            "ln" => std::f64::consts::E,
            "log" => 10.0,
            "log2" => 2.0,
            _ => 10.0, // Default to base 10
        }
    }

    /// Get the scale factor for a logarithmic unit
    fn get_scale_factor(&self, unit_code: &str) -> f64 {
        match unit_code {
            "dB" => 10.0,                                               // Decibel: 10 * log10
            "pH" | "pOH" | "pKa" | "pKw" | "pK" => -1.0,                // p-scales: -log10
            "B[SPL]" | "B[V]" | "B[mV]" | "B[uV]" | "B[10.nV]" => 20.0, // Field quantities: 20 * log10 (decibel scale)
            "B[W]" | "B[kW]" => 10.0, // Power quantities: 10 * log10 (decibel scale)
            _ => 1.0,
        }
    }

    /// Get reference value for pH-like units
    #[allow(dead_code)]
    fn get_reference_value(&self, unit_code: &str) -> Number {
        self.reference_values
            .get(unit_code)
            .copied()
            .unwrap_or(Number::one())
    }

    /// Get the reference quantity for specialized bel units
    fn get_bel_reference(&self, unit_code: &str) -> f64 {
        match unit_code {
            "B[SPL]" => 2e-5,    // 20 micropascals (2×10^-5 Pa)
            "B[V]" => 1.0,       // 1 volt
            "B[mV]" => 1e-3,     // 1 millivolt
            "B[uV]" => 1e-6,     // 1 microvolt
            "B[10.nV]" => 10e-9, // 10 nanovolts
            "B[W]" => 1.0,       // 1 watt
            "B[kW]" => 1e3,      // 1 kilowatt
            _ => 1.0,
        }
    }
}

impl SpecialUnitHandler for LogarithmicHandler {
    fn name(&self) -> &'static str {
        "Logarithmic"
    }

    fn can_handle(&self, unit_code: &str) -> bool {
        matches!(
            unit_code,
            "B" | "dB"
                | "Np"
                | "pH"
                | "pOH"
                | "pKa"
                | "pKw"
                | "pK"
                | "ln"
                | "log"
                | "log2"
                | "B[SPL]"
                | "B[V]"
                | "B[mV]"
                | "B[uV]"
                | "B[10.nV]"
                | "B[W]"
                | "B[kW]"
        )
    }

    fn convert_from(
        &self,
        value: Number,
        unit_code: &str,
        _context: &ConversionContext,
    ) -> Result<Number, UcumError> {
        let val = value.to_f64();
        let base = self.get_base(unit_code);
        let scale = self.get_scale_factor(unit_code);

        match unit_code {
            "B" => {
                // Bel: 10^value (power ratio)
                Ok(from_f64(base.powf(val)))
            }
            "dB" => {
                // Decibel: 10^(value/10) (power ratio)
                Ok(from_f64(base.powf(val / scale)))
            }
            "Np" => {
                // Neper: e^value (amplitude ratio)
                Ok(from_f64(base.powf(val)))
            }
            "B[SPL]" | "B[V]" | "B[mV]" | "B[uV]" | "B[10.nV]" | "B[W]" | "B[kW]" => {
                // Specialized bel units: reference_value * 10^(value/scale)
                let reference = self.get_bel_reference(unit_code);
                Ok(from_f64(reference * base.powf(val / scale)))
            }
            "pH" => {
                // pH to hydrogen ion concentration: [H+] = 10^(-pH)
                Ok(from_f64(base.powf(scale * val)))
            }
            "pOH" => {
                // pOH to hydroxide ion concentration: [OH-] = 10^(-pOH)
                Ok(from_f64(base.powf(scale * val)))
            }
            "pKa" | "pKw" | "pK" => {
                // pK to equilibrium constant: K = 10^(-pK)
                Ok(from_f64(base.powf(scale * val)))
            }
            "ln" => {
                // Natural logarithm: e^value
                Ok(from_f64(base.powf(val)))
            }
            "log" => {
                // Common logarithm: 10^value
                Ok(from_f64(base.powf(val)))
            }
            "log2" => {
                // Binary logarithm: 2^value
                Ok(from_f64(base.powf(val)))
            }
            _ => Err(UcumError::conversion_error(
                "logarithmic unit",
                "conversion",
                "Unknown logarithmic unit",
            )),
        }
    }

    fn convert_to(
        &self,
        value: Number,
        unit_code: &str,
        _context: &ConversionContext,
    ) -> Result<Number, UcumError> {
        let val = value.to_f64();

        // Check for valid input ranges
        match unit_code {
            "pH" | "pOH" | "pKa" | "pKw" | "pK" => {
                if val <= 0.0 {
                    return Err(UcumError::conversion_error(
                        "concentration",
                        "p-scale",
                        "Cannot calculate p-scale of non-positive concentration",
                    ));
                }
            }
            "B" | "dB" | "Np" | "ln" | "log" | "log2" => {
                if val <= 0.0 {
                    return Err(UcumError::conversion_error(
                        "value",
                        "logarithm",
                        "Cannot take logarithm of non-positive value",
                    ));
                }
            }
            _ => {}
        }

        let base = self.get_base(unit_code);
        let scale = self.get_scale_factor(unit_code);

        match unit_code {
            "B" => {
                // To Bel: log10(value)
                Ok(from_f64(val.log(base)))
            }
            "dB" => {
                // To Decibel: 10 * log10(value)
                Ok(from_f64(scale * val.log(base)))
            }
            "Np" => {
                // To Neper: ln(value)
                Ok(from_f64(val.ln()))
            }
            "B[SPL]" | "B[V]" | "B[mV]" | "B[uV]" | "B[10.nV]" | "B[W]" | "B[kW]" => {
                // To specialized bel units: scale * log10(value/reference)
                let reference = self.get_bel_reference(unit_code);
                Ok(from_f64(scale * (val / reference).log(base)))
            }
            "pH" => {
                // Hydrogen ion concentration to pH: pH = -log10([H+])
                Ok(from_f64(scale * val.log(base)))
            }
            "pOH" => {
                // Hydroxide ion concentration to pOH: pOH = -log10([OH-])
                Ok(from_f64(scale * val.log(base)))
            }
            "pKa" | "pKw" | "pK" => {
                // Equilibrium constant to pK: pK = -log10(K)
                Ok(from_f64(scale * val.log(base)))
            }
            "ln" => {
                // To natural logarithm: ln(value)
                Ok(from_f64(val.ln()))
            }
            "log" => {
                // To common logarithm: log10(value)
                Ok(from_f64(val.log10()))
            }
            "log2" => {
                // To binary logarithm: log2(value)
                Ok(from_f64(val.log2()))
            }
            _ => Err(UcumError::conversion_error(
                "logarithmic unit",
                "conversion",
                "Unknown logarithmic unit",
            )),
        }
    }

    fn get_dimension(&self, unit_code: &str) -> Dimension {
        match unit_code {
            "pH" | "pOH" => {
                // pH and pOH represent concentration, but are dimensionless in practice
                // In UCUM, they're treated as dimensionless logarithmic scales
                Dimension([0, 0, 0, 0, 0, 0, 0])
            }
            "pKa" | "pKw" | "pK" => {
                // Equilibrium constants are dimensionless
                Dimension([0, 0, 0, 0, 0, 0, 0])
            }
            "B[SPL]" => {
                // Sound pressure level: pressure dimension [M L^-1 T^-2]
                Dimension([1, -1, -2, 0, 0, 0, 0])
            }
            "B[V]" | "B[mV]" | "B[uV]" | "B[10.nV]" => {
                // Electric potential: [M L^2 T^-3 I^-1]
                Dimension([1, 2, -3, -1, 0, 0, 0])
            }
            "B[W]" | "B[kW]" => {
                // Power: [M L^2 T^-3]
                Dimension([1, 2, -3, 0, 0, 0, 0])
            }
            _ => {
                // All other logarithmic units are dimensionless
                Dimension([0, 0, 0, 0, 0, 0, 0])
            }
        }
    }

    fn get_base_factor(&self, _unit_code: &str) -> Number {
        Number::one()
    }
}

/// Handler for arbitrary units (square-bracketed units).
pub struct ArbitraryHandler;

impl Default for ArbitraryHandler {
    fn default() -> Self {
        Self::new()
    }
}

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
    fn test_specialized_bel_units() {
        let handler = LogarithmicHandler::new();
        let context = ConversionContext::new();

        // Test B[SPL] (sound pressure level)
        // 0 dB SPL = 20 μPa reference pressure
        let spl_0db = handler
            .convert_from(from_f64(0.0), "B[SPL]", &context)
            .unwrap();
        println!("0 dB SPL = {} Pa (expected 2e-5)", spl_0db.to_f64());
        assert!((spl_0db.to_f64() - 2e-5).abs() < 1e-10);

        // Debug: check the calculation step by step
        println!("Debug B[SPL] calculation:");
        println!("  Base: {}", handler.get_base("B[SPL]"));
        println!("  Scale: {}", handler.get_scale_factor("B[SPL]"));
        println!("  Reference: {}", handler.get_bel_reference("B[SPL]"));

        // 60 dB SPL should be 20 μPa * 10^(60/20) = 20 μPa * 1000 = 0.02 Pa
        let spl_60db = handler
            .convert_from(from_f64(60.0), "B[SPL]", &context)
            .unwrap();
        println!("60 dB SPL = {} Pa (expected 0.02)", spl_60db.to_f64());

        // Manual calculation should be: 2e-5 * 10^(60/20) = 2e-5 * 10^3 = 0.02
        let manual_calc = 2e-5 * 10f64.powf(60.0 / 20.0);
        println!("Correct manual calc: {manual_calc} Pa");

        assert!((spl_0db.to_f64() - 2e-5).abs() < 1e-10);
        assert!((spl_60db.to_f64() - 0.02).abs() < 1e-10);

        // Test B[V] (voltage level)
        // 0 dB V = 1 V reference
        let v_0db = handler
            .convert_from(from_f64(0.0), "B[V]", &context)
            .unwrap();
        assert!((v_0db.to_f64() - 1.0).abs() < 1e-10);

        // 20 dB V should be 1 V * 10^(20/20) = 10 V
        let v_20db = handler
            .convert_from(from_f64(20.0), "B[V]", &context)
            .unwrap();
        println!("20 dB V = {} V (expected 10.0)", v_20db.to_f64());
        // Actually with our scale of 20, it should be 1 V * 10^(20/20) = 10 V
        assert!((v_20db.to_f64() - 10.0).abs() < 1e-10);

        // Test B[mV] (millivolt level)
        // 0 dB mV = 1 mV reference
        let mv_0db = handler
            .convert_from(from_f64(0.0), "B[mV]", &context)
            .unwrap();
        assert!((mv_0db.to_f64() - 1e-3).abs() < 1e-10);

        // Test B[W] (power level)
        // 0 dB W = 1 W reference
        let w_0db = handler
            .convert_from(from_f64(0.0), "B[W]", &context)
            .unwrap();
        assert!((w_0db.to_f64() - 1.0).abs() < 1e-10);

        // 10 dB W should be 1 W * 10^(10/10) = 10 W
        let w_10db = handler
            .convert_from(from_f64(10.0), "B[W]", &context)
            .unwrap();
        println!("10 dB W = {} W (expected 10.0)", w_10db.to_f64());
        assert!((w_10db.to_f64() - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_specialized_bel_units_conversion_to() {
        let handler = LogarithmicHandler::new();
        let context = ConversionContext::new();

        // Test conversion to B[SPL]
        // 0.02 Pa should be 60 dB SPL: 20*log10(0.02/2e-5) = 20*log10(1000) = 20*3 = 60
        let pa_to_spl = handler
            .convert_to(from_f64(0.02), "B[SPL]", &context)
            .unwrap();
        println!("0.02 Pa to dB SPL = {} (expected 60.0)", pa_to_spl.to_f64());
        assert!((pa_to_spl.to_f64() - 60.0).abs() < 1e-10);

        // Test conversion to B[V]
        // 10 V should be 20 dB V: 20*log10(10/1) = 20*1 = 20
        let v_to_dbv = handler
            .convert_to(from_f64(10.0), "B[V]", &context)
            .unwrap();
        println!("10 V to dB V = {} (expected 20.0)", v_to_dbv.to_f64());
        assert!((v_to_dbv.to_f64() - 20.0).abs() < 1e-10);

        // Test conversion to B[W]
        // 10 W should be 10 dB W: 10*log10(10/1) = 10*1 = 10
        let w_to_dbw = handler
            .convert_to(from_f64(10.0), "B[W]", &context)
            .unwrap();
        println!("10 W to dB W = {} (expected 10.0)", w_to_dbw.to_f64());
        assert!((w_to_dbw.to_f64() - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_ph_units() {
        let handler = LogarithmicHandler::new();
        let context = ConversionContext::new();

        // Test pH = 7 (neutral) should give [H+] = 10^-7 mol/L
        let ph7_to_conc = handler.convert_from(from_f64(7.0), "pH", &context).unwrap();
        assert!((ph7_to_conc.to_f64() - 1e-7).abs() < 1e-15);

        // Test [H+] = 10^-7 mol/L should give pH = 7
        let conc_to_ph7 = handler.convert_to(from_f64(1e-7), "pH", &context).unwrap();
        assert!((conc_to_ph7.to_f64() - 7.0).abs() < 1e-10);

        // Test pH = 3 (acidic) should give [H+] = 10^-3 mol/L
        let ph3_to_conc = handler.convert_from(from_f64(3.0), "pH", &context).unwrap();
        assert!((ph3_to_conc.to_f64() - 1e-3).abs() < 1e-15);
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
