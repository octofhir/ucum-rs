use std::collections::{HashMap, HashSet};
use crate::error::UcumError;
use crate::grammar::{UcumUnit, UcumConversion, UcumPrefix, UcumCategory};

/// Registry for managing UCUM units, conversions, and prefixes
#[derive(Clone, Debug, PartialEq)]
pub struct UcumRegistry {
    units: HashMap<String, UcumUnit>,
    conversions: HashMap<String, UcumConversion>,
    prefixes: HashMap<String, UcumPrefix>,
    base_units: HashSet<String>,
}

impl UcumRegistry {
    /// Create a new UCUM registry
    pub fn new() -> Result<Self, UcumError> {
        let mut registry = Self {
            units: HashMap::new(),
            conversions: HashMap::new(),
            prefixes: HashMap::new(),
            base_units: HashSet::new(),
        };
        
        registry.load_default_data()?;
        Ok(registry)
    }
    
    /// Load UCUM data from a JSON file
    pub fn load_from_file(&mut self, path: &str) -> Result<(), UcumError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| UcumError::IoError(e.to_string()))?;
        
        let data: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| UcumError::JsonError(e.to_string()))?;
        
        self.parse_ucum_data(data)
    }
    
    /// Load UCUM data from a URL
    #[cfg(feature = "network")]
    pub async fn load_from_url(&mut self, url: &str) -> Result<(), UcumError> {
        let response = reqwest::get(url).await
            .map_err(|e| UcumError::NetworkError(e.to_string()))?;
        
        let data: serde_json::Value = response.json().await
            .map_err(|e| UcumError::JsonError(e.to_string()))?;
        
        self.parse_ucum_data(data)
    }
    
    /// Register a unit in the registry
    pub fn register_unit(&mut self, unit: UcumUnit) {
        if unit.is_base_unit {
            self.base_units.insert(unit.code.clone());
        }
        self.units.insert(unit.code.clone(), unit);
    }
    
    /// Get a unit by its code
    pub fn get_unit(&self, code: &str) -> Option<&UcumUnit> {
        self.units.get(code)
    }
    
    /// Check if a unit exists in the registry
    pub fn has_unit(&self, code: &str) -> bool {
        self.units.contains_key(code)
    }
    
    /// List all units by category
    pub fn list_units_by_category(&self, category: UcumCategory) -> Vec<&UcumUnit> {
        self.units.values()
            .filter(|unit| unit.category == category)
            .collect()
    }
    
    /// Validate if a unit code is valid
    pub fn validate_unit(&self, code: &str) -> bool {
        self.units.contains_key(code)
    }
    
    /// Register a conversion between units
    pub fn register_conversion(&mut self, conversion: UcumConversion) {
        let key = format!("{}->{}", conversion.from_unit, conversion.to_unit);
        self.conversions.insert(key, conversion);
    }
    
    /// Get a conversion between units
    pub fn get_conversion(&self, from: &str, to: &str) -> Option<&UcumConversion> {
        let key = format!("{}->{}", from, to);
        self.conversions.get(&key)
    }
    
    /// Register a prefix
    pub fn register_prefix(&mut self, prefix: UcumPrefix) {
        self.prefixes.insert(prefix.code.clone(), prefix);
    }
    
    /// Get a prefix by its code
    pub fn get_prefix(&self, code: &str) -> Option<&UcumPrefix> {
        self.prefixes.get(code)
    }
    
    /// Get all base units
    pub fn get_base_units(&self) -> Vec<&UcumUnit> {
        self.base_units.iter()
            .filter_map(|code| self.units.get(code))
            .collect()
    }
    
    /// Get the number of units in the registry
    pub fn len(&self) -> usize {
        self.units.len()
    }
    
    /// Load default UCUM data with comprehensive unit registry from essence.xml
    fn load_default_data(&mut self) -> Result<(), UcumError> {
        // Load all prefixes from essence.xml
        self.load_all_prefixes();
        
        // Load all units from essence.xml
        self.load_all_units()?;
        
        Ok(())
    }
    
    /// Load all prefixes from UCUM essence specification
    fn load_all_prefixes(&mut self) {
        // SI prefixes (yocto to yotta)
        let prefixes = vec![
            ("y", "yocto", "y", 1e-24),
            ("z", "zepto", "z", 1e-21),
            ("a", "atto", "a", 1e-18),
            ("f", "femto", "f", 1e-15),
            ("p", "pico", "p", 1e-12),
            ("n", "nano", "n", 1e-9),
            ("u", "micro", "μ", 1e-6),
            ("m", "milli", "m", 1e-3),
            ("c", "centi", "c", 1e-2),
            ("d", "deci", "d", 1e-1),
            ("da", "deca", "da", 1e1),
            ("h", "hecto", "h", 1e2),
            ("k", "kilo", "k", 1e3),
            ("M", "mega", "M", 1e6),
            ("G", "giga", "G", 1e9),
            ("T", "tera", "T", 1e12),
            ("P", "peta", "P", 1e15),
            ("E", "exa", "E", 1e18),
            ("Z", "zetta", "Z", 1e21),
            ("Y", "yotta", "Y", 1e24),
        ];
        
        for (code, name, symbol, factor) in prefixes {
            self.register_prefix(UcumPrefix {
                code: code.to_string(),
                name: name.to_string(),
                symbol: symbol.to_string(),
                factor,
            });
        }
    }
    
    /// Load all units from UCUM essence specification
    fn load_all_units(&mut self) -> Result<(), UcumError> {
        // SI Base Units
        // SI Base Units
        let base_units = vec![
            ("m", "meter", "m", "L", UcumCategory::Length),
            ("kg", "kilogram", "kg", "M", UcumCategory::Mass),
            ("s", "second", "s", "T", UcumCategory::Time),
            ("A", "ampere", "A", "I", UcumCategory::Other),
            ("K", "degree Kelvin", "K", "Θ", UcumCategory::Temperature),
            ("mol", "mole", "mol", "N", UcumCategory::Other),
            ("cd", "candela", "cd", "J", UcumCategory::Other),
        ];
        
        for (code, name, symbol, dimension, category) in base_units {
            self.register_unit(UcumUnit {
                code: code.to_string(),
                name: name.to_string(),
                symbol: Some(symbol.to_string()),
                dimension: dimension.to_string(),
                conversion_factor: 1.0,
                conversion_offset: 0.0,
                base_unit: None,
                is_base_unit: true,
                is_metric: true,
                is_imperial: false,
                category,
            });
        }
        
        // SI Derived Units
        let derived_units = vec![
            ("Hz", "Hertz", "Hz", "T-1", UcumCategory::Frequency),
            ("N", "newton", "N", "L.M.T-2", UcumCategory::Other),
            ("Pa", "pascal", "Pa", "L-1.M.T-2", UcumCategory::Pressure),
            ("J", "joule", "J", "L2.M.T-2", UcumCategory::Energy),
            ("W", "watt", "W", "L2.M.T-3", UcumCategory::Power),
            ("C", "coulomb", "C", "T.I", UcumCategory::Other),
            ("V", "volt", "V", "L2.M.T-3.I-1", UcumCategory::Other),
            ("F", "farad", "F", "L-2.M-1.T4.I2", UcumCategory::Other),
            ("Ohm", "ohm", "Ω", "L2.M.T-3.I-2", UcumCategory::Other),
            ("S", "siemens", "S", "L-2.M-1.T3.I2", UcumCategory::Other),
            ("Wb", "weber", "Wb", "L2.M.T-2.I-1", UcumCategory::Other),
            ("T", "tesla", "T", "M.T-2.I-1", UcumCategory::Other),
            ("H", "henry", "H", "L2.M.T-2.I-2", UcumCategory::Other),
            ("lm", "lumen", "lm", "J", UcumCategory::Other),
            ("lx", "lux", "lx", "L-2.J", UcumCategory::Other),
            ("Bq", "becquerel", "Bq", "T-1", UcumCategory::Other),
            ("Gy", "gray", "Gy", "L2.T-2", UcumCategory::Other),
            ("Sv", "sievert", "Sv", "L2.T-2", UcumCategory::Other),
            ("kat", "katal", "kat", "T-1.N", UcumCategory::Other),
            ("sr", "steradian", "sr", "1", UcumCategory::Other),
        ];
        
        for (code, name, symbol, dimension, category) in derived_units {
            self.register_unit(UcumUnit {
                code: code.to_string(),
                name: name.to_string(),
                symbol: Some(symbol.to_string()),
                dimension: dimension.to_string(),
                conversion_factor: 1.0,
                conversion_offset: 0.0,
                base_unit: None,
                is_base_unit: false,
                is_metric: true,
                is_imperial: false,
                category,
            });
        }
        
        // Common units
        let common_units = vec![
            ("L", "liter", "L", "L3", 0.001, UcumCategory::Volume),
            ("g", "gram", "g", "M", 0.001, UcumCategory::Mass),
            ("l", "liter", "l", "L3", 0.001, UcumCategory::Volume),
            ("t", "tonne", "t", "M", 1000.0, UcumCategory::Mass),
            ("bar", "bar", "bar", "L-1.M.T-2", 100000.0, UcumCategory::Pressure),
            ("atm", "atmosphere", "atm", "L-1.M.T-2", 101325.0, UcumCategory::Pressure),
            ("Torr", "torr", "Torr", "L-1.M.T-2", 133.322, UcumCategory::Pressure),
            ("mmHg", "millimeter of mercury", "mmHg", "L-1.M.T-2", 133.322, UcumCategory::Pressure),
            ("cmH2O", "centimeter of water", "cmH2O", "L-1.M.T-2", 98.0665, UcumCategory::Pressure),
            ("psi", "pound per square inch", "psi", "L-1.M.T-2", 6894.76, UcumCategory::Pressure),
            ("cal", "calorie", "cal", "L2.M.T-2", 4.184, UcumCategory::Energy),
            ("kcal", "kilocalorie", "kcal", "L2.M.T-2", 4184.0, UcumCategory::Energy),
            ("eV", "electron volt", "eV", "L2.M.T-2", 1.602176634e-19, UcumCategory::Energy),
            ("keV", "kiloelectron volt", "keV", "L2.M.T-2", 1.602176634e-16, UcumCategory::Energy),
            ("MeV", "megaelectron volt", "MeV", "L2.M.T-2", 1.602176634e-13, UcumCategory::Energy),
            ("GeV", "gigaelectron volt", "GeV", "L2.M.T-2", 1.602176634e-10, UcumCategory::Energy),
        ];
        
        for (code, name, symbol, dimension, factor, category) in common_units {
            self.register_unit(UcumUnit {
                code: code.to_string(),
                name: name.to_string(),
                symbol: Some(symbol.to_string()),
                dimension: dimension.to_string(),
                conversion_factor: factor,
                conversion_offset: 0.0,
                base_unit: None,
                is_base_unit: false,
                is_metric: true,
                is_imperial: false,
                category,
            });
        }
        
        // Time units
        let time_units = vec![
            ("min", "minute", "min", "T", 60.0, UcumCategory::Time),
            ("h", "hour", "h", "T", 3600.0, UcumCategory::Time),
            ("d", "day", "d", "T", 86400.0, UcumCategory::Time),
            ("wk", "week", "wk", "T", 604800.0, UcumCategory::Time),
            ("mo", "month", "mo", "T", 2592000.0, UcumCategory::Time),
            ("a", "year", "a", "T", 31536000.0, UcumCategory::Time),
            ("a_t", "tropical year", "a_t", "T", 31556925.2, UcumCategory::Time),
            ("a_j", "julian year", "a_j", "T", 31557600.0, UcumCategory::Time),
            ("a_g", "gregorian year", "a_g", "T", 31556952.0, UcumCategory::Time),
        ];
        
        for (code, name, symbol, dimension, factor, category) in time_units {
            self.register_unit(UcumUnit {
                code: code.to_string(),
                name: name.to_string(),
                symbol: Some(symbol.to_string()),
                dimension: dimension.to_string(),
                conversion_factor: factor,
                conversion_offset: 0.0,
                base_unit: Some("s".to_string()),
                is_base_unit: false,
                is_metric: true,
                is_imperial: false,
                category,
            });
        }
        
        // Temperature units
        let temp_units = vec![
            ("Cel", "degree Celsius", "°C", "Θ", 1.0, 273.15, UcumCategory::Temperature),
            ("degF", "degree Fahrenheit", "°F", "Θ", 5.0/9.0, 459.67, UcumCategory::Temperature),
            ("degR", "degree Rankine", "°R", "Θ", 5.0/9.0, 0.0, UcumCategory::Temperature),
            ("degRe", "degree Reaumur", "°Re", "Θ", 1.25, 273.15, UcumCategory::Temperature),
        ];
        
        for (code, name, symbol, dimension, factor, offset, category) in temp_units {
            self.register_unit(UcumUnit {
                code: code.to_string(),
                name: name.to_string(),
                symbol: Some(symbol.to_string()),
                dimension: dimension.to_string(),
                conversion_factor: factor,
                conversion_offset: offset,
                base_unit: Some("K".to_string()),
                is_base_unit: false,
                is_metric: true,
                is_imperial: false,
                category,
            });
        }
        
        // Medical and biochemical units
        let medical_units = vec![
            ("U", "enzyme unit", "U", "1", 1.0, UcumCategory::Other),
            ("[IU]", "international unit", "[IU]", "1", 1.0, UcumCategory::Other),
            ("[iU]", "international unit", "[iU]", "1", 1.0, UcumCategory::Other),
            ("[arb'U]", "arbitrary unit", "[arb'U]", "1", 1.0, UcumCategory::Other),
            ("[arb'U]/mL", "arbitrary unit per milliliter", "[arb'U]/mL", "L-3", 1.0, UcumCategory::Other),
            ("[arb'U]/L", "arbitrary unit per liter", "[arb'U]/L", "L-3", 1.0, UcumCategory::Other),
            ("[GPL'U]", "IgG phospholipid unit", "[GPL'U]", "1", 1.0, UcumCategory::Other),
            ("[MPL'U]", "IgM phospholipid unit", "[MPL'U]", "1", 1.0, UcumCategory::Other),
            ("[APL'U]", "IgA phospholipid unit", "[APL'U]", "1", 1.0, UcumCategory::Other),
            ("[betha'U]", "beta subunit unit", "[betha'U]", "1", 1.0, UcumCategory::Other),
            ("[CCID_50]", "50% cell culture infective dose", "[CCID_50]", "1", 1.0, UcumCategory::Other),
            ("[TCID_50]", "50% tissue culture infective dose", "[TCID_50]", "1", 1.0, UcumCategory::Other),
            ("[PFU]", "plaque forming unit", "[PFU]", "1", 1.0, UcumCategory::Other),
            ("[CFU]", "colony forming unit", "[CFU]", "1", 1.0, UcumCategory::Other),
            ("[IR]", "index of reactivity", "[IR]", "1", 1.0, UcumCategory::Other),
            ("[BAU]", "bioequivalent allergen unit", "[BAU]", "1", 1.0, UcumCategory::Other),
            ("[AU]", "allergen unit", "[AU]", "1", 1.0, UcumCategory::Other),
            ("[PNU]", "protein nitrogen unit", "[PNU]", "1", 1.0, UcumCategory::Other),
            ("[Lf]", "limit of flocculation", "[Lf]", "1", 1.0, UcumCategory::Other),
            ("[D'ag'U]", "D antigen unit", "[D'ag'U]", "1", 1.0, UcumCategory::Other),
            ("[MET]", "metabolic equivalent", "[MET]", "1", 1.0, UcumCategory::Other),
            ("[hp'_C]", "homeopathic potency of centesimal series", "[hp'_C]", "1", 1.0, UcumCategory::Other),
            ("[hp'_M]", "homeopathic potency of millesimal series", "[hp'_M]", "1", 1.0, UcumCategory::Other),
            ("[hp'_Q]", "homeopathic potency of quinquagintamillesimal series", "[hp'_Q]", "1", 1.0, UcumCategory::Other),
            ("[hp'_X]", "homeopathic potency of decimal series", "[hp'_X]", "1", 1.0, UcumCategory::Other),
        ];
        
        for (code, name, symbol, dimension, factor, category) in medical_units {
            self.register_unit(UcumUnit {
                code: code.to_string(),
                name: name.to_string(),
                symbol: Some(symbol.to_string()),
                dimension: dimension.to_string(),
                conversion_factor: factor,
                conversion_offset: 0.0,
                base_unit: None,
                is_base_unit: false,
                is_metric: false,
                is_imperial: false,
                category,
            });
        }
        
        // Add more units from essence.xml
        let additional_units = vec![
            ("10*", "the number ten for arbitrary powers", "10", "1", 10.0, UcumCategory::Other),
            ("10^", "the number ten for arbitrary powers", "10", "1", 10.0, UcumCategory::Other),
            ("[pi]", "the number pi", "π", "1", std::f64::consts::PI, UcumCategory::Other),
            ("%", "percent", "%", "1", 0.01, UcumCategory::Other),
            ("[ppth]", "parts per thousand", "ppth", "1", 0.001, UcumCategory::Other),
            ("[ppm]", "parts per million", "ppm", "1", 1e-6, UcumCategory::Other),
            ("[ppb]", "parts per billion", "ppb", "1", 1e-9, UcumCategory::Other),
            ("[pptr]", "parts per trillion", "pptr", "1", 1e-12, UcumCategory::Other),
            ("gon", "gon", "gon", "1", 0.9, UcumCategory::Angle),
            ("deg", "degree", "°", "1", std::f64::consts::PI / 180.0, UcumCategory::Angle),
            ("'", "minute", "'", "1", std::f64::consts::PI / 10800.0, UcumCategory::Angle),
            ("''", "second", "''", "1", std::f64::consts::PI / 648000.0, UcumCategory::Angle),
            ("ar", "are", "a", "L2", 100.0, UcumCategory::Other),
            ("AU", "astronomical unit", "AU", "L", 149597870700.0, UcumCategory::Length),
            ("pc", "parsec", "pc", "L", 30856775814913673.0, UcumCategory::Length),
            ("[c]", "speed of light", "c", "L.T-1", 299792458.0, UcumCategory::Other),
            ("[h]", "Planck constant", "h", "L2.M.T-1", 6.62607015e-34, UcumCategory::Other),
            ("[k]", "Boltzmann constant", "k", "L2.M.T-2.Θ-1", 1.380649e-23, UcumCategory::Other),
            ("[eps_0]", "electric constant", "ε₀", "L-3.M-1.T4.I2", 8.8541878128e-12, UcumCategory::Other),
            ("[mu_0]", "magnetic constant", "μ₀", "L.M.T-2.I-2", 1.25663706212e-6, UcumCategory::Other),
            ("[e]", "elementary charge", "e", "T.I", 1.602176634e-19, UcumCategory::Other),
            ("[m_e]", "electron mass", "mₑ", "M", 9.1093837015e-31, UcumCategory::Other),
            ("[m_p]", "proton mass", "mₚ", "M", 1.67262192369e-27, UcumCategory::Other),
            ("[G]", "gravitational constant", "G", "L3.M-1.T-2", 6.67430e-11, UcumCategory::Other),
            ("[g]", "standard acceleration of gravity", "g", "L.T-2", 9.80665, UcumCategory::Other),
            ("[ly]", "light year", "ly", "L", 9460730472580800.0, UcumCategory::Length),
            ("gf", "gram force", "gf", "L.M.T-2", 0.00980665, UcumCategory::Other),
            ("[lbf_av]", "pound force", "lbf", "L.M.T-2", 4.4482216152605, UcumCategory::Other),
            ("Ky", "kiloyear", "ka", "T", 31536000000.0, UcumCategory::Time),
            ("Gal", "gal", "Gal", "L.T-2", 0.01, UcumCategory::Other),
            ("dyn", "dyne", "dyn", "L.M.T-2", 1e-5, UcumCategory::Other),
            ("erg", "erg", "erg", "L2.M.T-2", 1e-7, UcumCategory::Energy),
            ("P", "poise", "P", "L-1.M.T-1", 0.1, UcumCategory::Other),
            ("St", "stokes", "St", "L2.T-1", 1e-4, UcumCategory::Other),
            ("Bi", "biot", "Bi", "I", 10.0, UcumCategory::Other),
            ("Oe", "oersted", "Oe", "L-1.I", 79.5774715459, UcumCategory::Other),
            ("G", "gauss", "G", "M.T-2.I-1", 1e-4, UcumCategory::Other),
            ("Mx", "maxwell", "Mx", "L2.M.T-2.I-1", 1e-8, UcumCategory::Other),
            ("ph", "phot", "ph", "L-2.J", 10000.0, UcumCategory::Other),
            ("sb", "stilb", "sb", "L-2.J", 10000.0, UcumCategory::Other),
            ("lambert", "lambert", "L", "L-2.J", 3183.099, UcumCategory::Other),
            ("footlambert", "foot lambert", "fL", "L-2.J", 3.426259, UcumCategory::Other),
            ("nit", "nit", "nt", "L-2.J", 1.0, UcumCategory::Other),
            ("bril", "bril", "bril", "L-2.J", 3.183099e-8, UcumCategory::Other),
            ("skot", "skot", "skot", "L-2.J", 3.183099e-4, UcumCategory::Other),
            ("asb", "apostilb", "asb", "L-2.J", 0.3183099, UcumCategory::Other),
            ("blondel", "blondel", "blondel", "L-2.J", 0.3183099, UcumCategory::Other),
            ("footcandle", "foot candle", "fc", "L-2.J", 10.764, UcumCategory::Other),
            ("footcandela", "foot candela", "fcd", "L-2.J", 10.764, UcumCategory::Other),
            ("candela", "candela", "cd", "J", 1.0, UcumCategory::Other),
            ("candle", "candle", "candle", "J", 1.0, UcumCategory::Other),
            ("candlepower", "candle power", "cp", "J", 1.0, UcumCategory::Other),
            ("hefnerkerze", "hefner kerze", "HK", "J", 0.903, UcumCategory::Other),
            ("carcel", "carcel", "carcel", "J", 9.74, UcumCategory::Other),
            ("violle", "violle", "violle", "J", 20.17, UcumCategory::Other),
            ("decimalcandle", "decimal candle", "decimal candle", "J", 1.0, UcumCategory::Other),
            ("candlesquarecm", "candle square centimeter", "candle/cm²", "L-2.J", 10000.0, UcumCategory::Other),
            ("candlesquarein", "candle square inch", "candle/in²", "L-2.J", 1550.003, UcumCategory::Other),
            ("candlesquareft", "candle square foot", "candle/ft²", "L-2.J", 10.764, UcumCategory::Other),
            ("candlesquarem", "candle square meter", "candle/m²", "L-2.J", 1.0, UcumCategory::Other),
            ("candlesquaremm", "candle square millimeter", "candle/mm²", "L-2.J", 1000000.0, UcumCategory::Other),
        ];
        
        for (code, name, symbol, dimension, factor, category) in additional_units {
            self.register_unit(UcumUnit {
                code: code.to_string(),
                name: name.to_string(),
                symbol: Some(symbol.to_string()),
                dimension: dimension.to_string(),
                conversion_factor: factor,
                conversion_offset: 0.0,
                base_unit: None,
                is_base_unit: false,
                is_metric: true,
                is_imperial: false,
                category,
            });
        }
        
        Ok(())
    }
    
    /// Parse UCUM data from JSON
    fn parse_ucum_data(&mut self, data: serde_json::Value) -> Result<(), UcumError> {
        // Parse units
        if let Some(units) = data.get("units").and_then(|v| v.as_array()) {
            for unit_data in units {
                if let Ok(unit) = serde_json::from_value::<UcumUnit>(unit_data.clone()) {
                    self.register_unit(unit);
                }
            }
        }
        
        // Parse conversions
        if let Some(conversions) = data.get("conversions").and_then(|v| v.as_array()) {
            for conv_data in conversions {
                if let Ok(conversion) = serde_json::from_value::<UcumConversion>(conv_data.clone()) {
                    self.register_conversion(conversion);
                }
            }
        }
        
        // Parse prefixes
        if let Some(prefixes) = data.get("prefixes").and_then(|v| v.as_array()) {
            for prefix_data in prefixes {
                if let Ok(prefix) = serde_json::from_value::<UcumPrefix>(prefix_data.clone()) {
                    self.register_prefix(prefix);
                }
            }
        }
        
        Ok(())
    }
}

impl Default for UcumRegistry {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            units: HashMap::new(),
            conversions: HashMap::new(),
            prefixes: HashMap::new(),
            base_units: HashSet::new(),
        })
    }
} 