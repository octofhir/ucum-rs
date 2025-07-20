use crate::grammar::{UcumUnit, UcumQuantity, UcumCategory};

/// Extension traits for UCUM units
pub trait UcumUnitExt {
    /// Check if this unit is compatible with another unit for conversion
    fn is_compatible_with(&self, other: &UcumUnit) -> bool;
    
    /// Get the dimension of this unit
    fn get_dimension(&self) -> &str;
    
    /// Check if this unit is a base unit
    fn is_base_unit(&self) -> bool;
    
    /// Get the conversion factor to base unit
    fn get_conversion_factor(&self) -> f64;
    
    /// Get the conversion offset to base unit
    fn get_conversion_offset(&self) -> f64;
}

impl UcumUnitExt for UcumUnit {
    fn is_compatible_with(&self, other: &UcumUnit) -> bool {
        self.dimension == other.dimension
    }
    
    fn get_dimension(&self) -> &str {
        &self.dimension
    }
    
    fn is_base_unit(&self) -> bool {
        self.is_base_unit
    }
    
    fn get_conversion_factor(&self) -> f64 {
        self.conversion_factor
    }
    
    fn get_conversion_offset(&self) -> f64 {
        self.conversion_offset
    }
}

/// Extension traits for UCUM quantities
pub trait UcumQuantityExt {
    /// Convert this quantity to a different unit
    fn convert_to(&self, target_unit: &UcumUnit) -> Option<UcumQuantity>;
    
    /// Check if this quantity equals another quantity
    fn equals(&self, other: &UcumQuantity) -> Option<bool>;
    
    /// Compare this quantity with another quantity
    fn compare(&self, other: &UcumQuantity) -> Option<std::cmp::Ordering>;
    
    /// Add another quantity to this one
    fn add(&self, other: &UcumQuantity) -> Option<UcumQuantity>;
    
    /// Subtract another quantity from this one
    fn subtract(&self, other: &UcumQuantity) -> Option<UcumQuantity>;
    
    /// Multiply this quantity by another quantity
    fn multiply(&self, other: &UcumQuantity) -> Option<UcumQuantity>;
    
    /// Divide this quantity by another quantity
    fn divide(&self, other: &UcumQuantity) -> Option<UcumQuantity>;
}

impl UcumQuantityExt for UcumQuantity {
    fn convert_to(&self, target_unit: &UcumUnit) -> Option<UcumQuantity> {
        if !self.unit.is_compatible_with(target_unit) {
            return None;
        }
        
        // Convert to base unit first, then to target unit
        let base_value = self.value * self.unit.conversion_factor + self.unit.conversion_offset;
        let target_value = (base_value - target_unit.conversion_offset) / target_unit.conversion_factor;
        
        Some(UcumQuantity {
            value: target_value,
            unit: target_unit.clone(),
            precision: self.precision,
        })
    }
    
    fn equals(&self, other: &UcumQuantity) -> Option<bool> {
        if !self.unit.is_compatible_with(&other.unit) {
            return None;
        }
        
        let converted = other.convert_to(&self.unit)?;
        Some((self.value - converted.value).abs() < f64::EPSILON)
    }
    
    fn compare(&self, other: &UcumQuantity) -> Option<std::cmp::Ordering> {
        if !self.unit.is_compatible_with(&other.unit) {
            return None;
        }
        
        let converted = other.convert_to(&self.unit)?;
        self.value.partial_cmp(&converted.value)
    }
    
    fn add(&self, other: &UcumQuantity) -> Option<UcumQuantity> {
        if !self.unit.is_compatible_with(&other.unit) {
            return None;
        }
        
        let converted = other.convert_to(&self.unit)?;
        Some(UcumQuantity {
            value: self.value + converted.value,
            unit: self.unit.clone(),
            precision: self.precision,
        })
    }
    
    fn subtract(&self, other: &UcumQuantity) -> Option<UcumQuantity> {
        if !self.unit.is_compatible_with(&other.unit) {
            return None;
        }
        
        let converted = other.convert_to(&self.unit)?;
        Some(UcumQuantity {
            value: self.value - converted.value,
            unit: self.unit.clone(),
            precision: self.precision,
        })
    }
    
    fn multiply(&self, other: &UcumQuantity) -> Option<UcumQuantity> {
        // For multiplication, we need to handle unit arithmetic
        // This is a simplified version - in practice, you'd need more complex unit arithmetic
        Some(UcumQuantity {
            value: self.value * other.value,
            unit: self.unit.clone(), // Simplified - should combine units
            precision: self.precision,
        })
    }
    
    fn divide(&self, other: &UcumQuantity) -> Option<UcumQuantity> {
        if other.value == 0.0 {
            return None;
        }
        
        // For division, we need to handle unit arithmetic
        // This is a simplified version - in practice, you'd need more complex unit arithmetic
        Some(UcumQuantity {
            value: self.value / other.value,
            unit: self.unit.clone(), // Simplified - should combine units
            precision: self.precision,
        })
    }
}

/// Utility functions for working with UCUM units
pub mod utils {
    use super::*;
    
    /// Create a length unit
    pub fn length_unit(code: &str, name: &str, factor: f64) -> UcumUnit {
        UcumUnit {
            code: code.to_string(),
            name: name.to_string(),
            symbol: Some(code.to_string()),
            dimension: "L".to_string(),
            conversion_factor: factor,
            conversion_offset: 0.0,
            base_unit: None,
            is_base_unit: factor == 1.0,
            is_metric: true,
            is_imperial: false,
            category: UcumCategory::Length,
        }
    }
    
    /// Create a mass unit
    pub fn mass_unit(code: &str, name: &str, factor: f64) -> UcumUnit {
        UcumUnit {
            code: code.to_string(),
            name: name.to_string(),
            symbol: Some(code.to_string()),
            dimension: "M".to_string(),
            conversion_factor: factor,
            conversion_offset: 0.0,
            base_unit: None,
            is_base_unit: factor == 1.0,
            is_metric: true,
            is_imperial: false,
            category: UcumCategory::Mass,
        }
    }
    
    /// Create a time unit
    pub fn time_unit(code: &str, name: &str, factor: f64) -> UcumUnit {
        UcumUnit {
            code: code.to_string(),
            name: name.to_string(),
            symbol: Some(code.to_string()),
            dimension: "T".to_string(),
            conversion_factor: factor,
            conversion_offset: 0.0,
            base_unit: None,
            is_base_unit: factor == 1.0,
            is_metric: true,
            is_imperial: false,
            category: UcumCategory::Time,
        }
    }
    
    /// Create a temperature unit
    pub fn temperature_unit(code: &str, name: &str, factor: f64, offset: f64) -> UcumUnit {
        UcumUnit {
            code: code.to_string(),
            name: name.to_string(),
            symbol: Some(code.to_string()),
            dimension: "Θ".to_string(),
            conversion_factor: factor,
            conversion_offset: offset,
            base_unit: None,
            is_base_unit: factor == 1.0 && offset == 0.0,
            is_metric: true,
            is_imperial: false,
            category: UcumCategory::Temperature,
        }
    }
} 