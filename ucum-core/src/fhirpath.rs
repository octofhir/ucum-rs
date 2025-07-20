use std::cmp::Ordering;
use crate::error::UcumError;
use crate::grammar::UcumUnit;
use crate::parser::UcumGrammarParser;

/// Enhanced FP_Quantity with UCUM grammar support
#[derive(Debug, Clone, PartialEq)]
pub struct FP_Quantity {
    pub value: f64,
    pub unit: String,
    pub system: Option<String>,
    pub code: Option<String>,
    pub ucum_parser: Option<UcumGrammarParser>,
}

impl FP_Quantity {
    /// Create a new FP_Quantity
    pub fn new(value: f64, unit: String) -> Self {
        Self {
            value,
            unit,
            system: Some("http://unitsofmeasure.org".to_string()),
            code: None,
            ucum_parser: None,
        }
    }
    
    /// Create a new FP_Quantity with UCUM parser
    pub fn with_ucum_parser(mut self, parser: UcumGrammarParser) -> Self {
        self.ucum_parser = Some(parser);
        self
    }
    
    /// Parse a unit expression using the UCUM parser
    pub fn parse_unit_expression(&self, expression: &str) -> Result<UcumUnit, UcumError> {
        if let Some(parser) = &self.ucum_parser {
            parser.parse_main_term(expression)
                .and_then(|term| parser.evaluate_term(&term))
        } else {
            Err(UcumError::NoParserAvailable)
        }
    }
    
    /// Check if two quantities are equal (with unit conversion if possible)
    pub fn equals(&self, other: &FP_Quantity) -> Option<bool> {
        if let Some(parser) = &self.ucum_parser {
            // Parse both units and convert to common base
            let self_unit = parser.parse_main_term(&self.unit)
                .and_then(|term| parser.evaluate_term(&term)).ok()?;
            let other_unit = parser.parse_main_term(&other.unit)
                .and_then(|term| parser.evaluate_term(&term)).ok()?;
            
            // For now, just compare if units are the same
            // In a full implementation, this would convert to common units
            if self_unit.code == other_unit.code {
                Some((self.value - other.value).abs() < f64::EPSILON)
            } else {
                None
            }
        } else {
            // Simple comparison if units match exactly
            if self.unit == other.unit {
                Some((self.value - other.value).abs() < f64::EPSILON)
            } else {
                None
            }
        }
    }
    
    /// Compare two quantities (with unit conversion if possible)
    pub fn compare(&self, other: &FP_Quantity) -> Option<Ordering> {
        if let Some(parser) = &self.ucum_parser {
            let self_unit = parser.parse_main_term(&self.unit)
                .and_then(|term| parser.evaluate_term(&term)).ok()?;
            let other_unit = parser.parse_main_term(&other.unit)
                .and_then(|term| parser.evaluate_term(&term)).ok()?;
            
            // For now, only compare if units are the same
            // In a full implementation, this would convert to common units
            if self_unit.code == other_unit.code {
                self.value.partial_cmp(&other.value)
            } else {
                None
            }
        } else {
            if self.unit == other.unit {
                self.value.partial_cmp(&other.value)
            } else {
                None
            }
        }
    }
    
    /// Add two quantities (with unit conversion if possible)
    pub fn plus(&self, other: &FP_Quantity) -> Result<FP_Quantity, UcumError> {
        if let Some(parser) = &self.ucum_parser {
            let self_unit = parser.parse_main_term(&self.unit)
                .and_then(|term| parser.evaluate_term(&term))?;
            let other_unit = parser.parse_main_term(&other.unit)
                .and_then(|term| parser.evaluate_term(&term))?;
            
            // For now, only add if units are the same
            // In a full implementation, this would convert to common units
            if self_unit.code == other_unit.code {
                Ok(FP_Quantity::new(
                    self.value + other.value,
                    self.unit.clone()
                ).with_ucum_parser(parser.clone()))
            } else {
                Err(UcumError::ConversionNotPossible {
                    from: other.unit.clone(),
                    to: self.unit.clone(),
                })
            }
        } else if self.unit == other.unit {
            Ok(FP_Quantity::new(
                self.value + other.value,
                self.unit.clone()
            ))
        } else {
            Err(UcumError::ConversionNotPossible {
                from: other.unit.clone(),
                to: self.unit.clone(),
            })
        }
    }
    
    /// Multiply two quantities
    pub fn multiply(&self, other: &FP_Quantity) -> Result<FP_Quantity, UcumError> {
        if let Some(parser) = &self.ucum_parser {
            // Parse units and perform unit multiplication
            let self_unit = parser.parse_main_term(&self.unit)
                .and_then(|term| parser.evaluate_term(&term))?;
            let other_unit = parser.parse_main_term(&other.unit)
                .and_then(|term| parser.evaluate_term(&term))?;
            
            let result_unit = parser.multiply_units(&self_unit, &other_unit)?;
            
            Ok(FP_Quantity::new(
                self.value * other.value,
                result_unit.code
            ).with_ucum_parser(parser.clone()))
        } else {
            // Simple multiplication with unit concatenation
            Ok(FP_Quantity::new(
                self.value * other.value,
                format!("{}.{}", self.unit, other.unit)
            ))
        }
    }
    
    /// Divide two quantities
    pub fn divide(&self, other: &FP_Quantity) -> Result<FP_Quantity, UcumError> {
        if let Some(parser) = &self.ucum_parser {
            // Parse units and perform unit division
            let self_unit = parser.parse_main_term(&self.unit)
                .and_then(|term| parser.evaluate_term(&term))?;
            let other_unit = parser.parse_main_term(&other.unit)
                .and_then(|term| parser.evaluate_term(&term))?;
            
            let result_unit = parser.divide_units(&self_unit, &other_unit)?;
            
            Ok(FP_Quantity::new(
                self.value / other.value,
                result_unit.code
            ).with_ucum_parser(parser.clone()))
        } else {
            // Simple division with unit division
            Ok(FP_Quantity::new(
                self.value / other.value,
                format!("{}/{}", self.unit, other.unit)
            ))
        }
    }
    
    /// Convert quantity to a different unit
    pub fn convert_to(&self, target_unit: &str) -> Result<FP_Quantity, UcumError> {
        if let Some(parser) = &self.ucum_parser {
            let source_unit = parser.parse_main_term(&self.unit)
                .and_then(|term| parser.evaluate_term(&term))?;
            let target_unit_parsed = parser.parse_main_term(target_unit)
                .and_then(|term| parser.evaluate_term(&term))?;
            
            // For now, only convert if units are the same
            // In a full implementation, this would use conversion factors
            if source_unit.code == target_unit_parsed.code {
                Ok(FP_Quantity::new(
                    self.value,
                    target_unit.to_string()
                ).with_ucum_parser(parser.clone()))
            } else {
                Err(UcumError::ConversionNotPossible {
                    from: self.unit.clone(),
                    to: target_unit.to_string(),
                })
            }
        } else {
            Err(UcumError::NoParserAvailable)
        }
    }
    
    /// Validate the unit expression
    pub fn validate_unit(&self) -> Result<(), UcumError> {
        if let Some(parser) = &self.ucum_parser {
            parser.parse_main_term(&self.unit)?;
            Ok(())
        } else {
            Err(UcumError::NoParserAvailable)
        }
    }
    
    /// Get the parsed unit structure
    pub fn get_parsed_unit(&self) -> Result<crate::grammar::UcumTerm, UcumError> {
        if let Some(parser) = &self.ucum_parser {
            parser.parse_main_term(&self.unit)
        } else {
            Err(UcumError::NoParserAvailable)
        }
    }
}

impl std::fmt::Display for FP_Quantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.value, self.unit)
    }
} 