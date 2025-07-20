use ucum_core::{UcumParser, UcumRegistry, UcumError};

/// Example of how UCUM could be integrated with FHIRPath quantity operations
struct FhirPathQuantity {
    value: f64,
    unit: String,
    ucum_parsed: Option<ucum_core::UcumTerm>,
}

impl FhirPathQuantity {
    fn new(value: f64, unit: &str) -> Result<Self, UcumError> {
        let parser = UcumParser::new();
        let ucum_parsed = parser.parse(unit).ok();
        
        Ok(Self {
            value,
            unit: unit.to_string(),
            ucum_parsed,
        })
    }
    
    fn is_valid_unit(&self) -> bool {
        self.ucum_parsed.is_some()
    }
    
    fn to_string(&self) -> String {
        format!("{} {}", self.value, self.unit)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("UCUM Core Library - FHIRPath Integration Example");
    println!("===============================================\n");

    // Example FHIRPath quantities
    let quantities = vec![
        FhirPathQuantity::new(100.0, "mg")?,
        FhirPathQuantity::new(70.0, "kg")?,
        FhirPathQuantity::new(5.5, "mmol/L")?,
        FhirPathQuantity::new(120.0, "mmHg")?,
    ];
    
    for quantity in quantities {
        println!("Quantity: {}", quantity.to_string());
        println!("  Valid UCUM: {}", quantity.is_valid_unit());
        if let Some(parsed) = &quantity.ucum_parsed {
            println!("  Parsed: {:?}", parsed);
        }
        println!();
    }
    
    // Example of unit validation
    let test_units = vec!["mg", "invalid_unit", "kg/m2", "cm3"];
    let parser = UcumParser::new();
    
    println!("Unit Validation:");
    for unit in test_units {
        match parser.parse(unit) {
            Ok(_) => println!("  ✓ {} is valid", unit),
            Err(_) => println!("  ✗ {} is invalid", unit),
        }
    }
    
    Ok(())
} 