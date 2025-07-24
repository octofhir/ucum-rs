use octofhir_ucum_core::{
    EvalResult,
    Quantity as UcumQuantity,
    UcumError,
    UnitRecord,
    analyse,
    evaluate,
    find_unit,
    get_all_units,
    get_canonical_units,
    get_defined_forms as core_get_defined_forms,
    is_comparable,
    parse_expression,
    search_units as core_search_units,
    search_units_by_property as core_search_by_property,
    search_units_fuzzy as core_search_fuzzy,
    search_units_regex as core_search_regex,
    unit_divide,
    unit_multiply,
    validate as core_validate,
    validate_in_property,
    precision::to_f64,
    // Model Introspection API
    get_model,
    validate_ucum,
    get_properties,
    validate_canonical_units,
    get_common_display,
    // Advanced Conversion API
    convert_with_context,
    AdvancedConversionContext,
    DecimalPrecision,
    RoundingMode,
    TemperatureScale,
    // Performance Optimizations
    get_cache_stats,
    clear_global_cache,
    get_cache_sizes,
    // Enhanced Error Handling
    ErrorKind,
    SuggestionEngine,
    // Extended Functionality
    optimize_expression,
    canonicalize_expression,
    simplify_expression,
    MeasurementContext,

};
use octofhir_ucum_fhir::{
    FhirError, FhirQuantity, ToFhirQuantity, are_equivalent, convert_quantity,
};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Initialize panic hook for better error messages
pub fn init_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen(start)]
pub fn start() {
    init_panic_hook();
}

// JavaScript-friendly error type with enhanced error details
#[derive(Serialize, Deserialize)]
pub struct JsError {
    message: String,
    error_type: String,
    suggestions: Vec<String>,
    context: Vec<String>,
    span: Option<JsSpan>,
    kind_details: Option<JsErrorKind>,
}

// JavaScript-friendly span information
#[derive(Serialize, Deserialize)]
pub struct JsSpan {
    start: usize,
    end: usize,
    source: String,
    text: String,
}

// JavaScript-friendly error kind details
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum JsErrorKind {
    UnitNotFound { unit: String, similar: Vec<String> },
    ConversionError { from: String, to: String, reason: String },
    DimensionMismatch { expected: Vec<i8>, found: Vec<i8>, operation: String },
    ParseError { expected: String, found: String },
    InvalidExpression { reason: String },
    InvalidPercentPlacement { position: usize },
    PrecisionOverflow { operation: String, value: String },
    InvalidProperty { property: String, available: Vec<String> },
    MultipleSlash,
    SpecialUnitError { unit: String, reason: String },
}

// JavaScript-friendly unit information
#[derive(Serialize, Deserialize)]
pub struct UnitInfo {
    code: String,
    display_name: String,
    factor: f64,
    offset: f64,
    is_special: bool,
    is_arbitrary: bool,
    dimensions: Vec<i8>,
    property: String,
}

// JavaScript-friendly evaluation result
#[derive(Serialize, Deserialize)]
pub struct EvaluationResult {
    factor: f64,
    offset: f64,
    dimensions: Vec<i8>,
}

// JavaScript-friendly unit analysis result
#[derive(Serialize, Deserialize)]
pub struct JsUnitAnalysis {
    expression: String,
    factor: f64,
    offset: f64,
    dimensions: Vec<i8>,
    is_dimensionless: bool,
    has_offset: bool,
}

// JavaScript-friendly canonical unit result
#[derive(Serialize, Deserialize)]
pub struct JsCanonicalUnit {
    unit: String,
    factor: f64,
    offset: f64,
    dimensions: Vec<i8>,
}

// JavaScript-friendly unit arithmetic result
#[derive(Serialize, Deserialize)]
pub struct JsUnitArithmeticResult {
    expression: String,
    factor: f64,
    dimensions: Vec<i8>,
    offset: f64,
    is_dimensionless: bool,
}

// JavaScript-friendly search result
#[derive(Serialize, Deserialize)]
pub struct JsSearchResult {
    units: Vec<UnitInfo>,
}

// JavaScript-friendly fuzzy search result
#[derive(Serialize, Deserialize)]
pub struct JsFuzzySearchResult {
    results: Vec<JsFuzzyMatch>,
}

#[derive(Serialize, Deserialize)]
pub struct JsFuzzyMatch {
    unit: UnitInfo,
    score: i64,
}

// JavaScript-friendly types for model introspection
#[derive(Serialize, Deserialize)]
pub struct JsUcumModel {
    version: String,
    revision_date: String,
    total_units: usize,
    total_prefixes: usize,
    properties: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct JsValidationResult {
    is_valid: bool,
    issues: Vec<String>,
}

// JavaScript-friendly types for advanced conversion
#[derive(Serialize, Deserialize)]
pub struct JsAdvancedConversionResult {
    value: f64,
    unit: String,
    factor: f64,
    offset: f64,
    precision_info: String,
    used_special_units: bool,
}

#[derive(Serialize, Deserialize)]
pub struct JsConversionConfig {
    precision_type: String,      // "default", "fixed", "significant"
    precision_value: Option<u32>, // For fixed/significant precision
    rounding_mode: String,       // "nearest", "up", "down", "truncate"
    temperature_scale: String,   // "kelvin", "celsius", "fahrenheit"
    use_special_units: bool,
}

// JavaScript-friendly types for performance features
#[derive(Serialize, Deserialize)]
pub struct JsCacheStats {
    expression_hits: u64,
    expression_misses: u64,
    conversion_hits: u64,
    conversion_misses: u64,
    dimension_hits: u64,
    dimension_misses: u64,
    expression_hit_ratio: f64,
    conversion_hit_ratio: f64,
    overall_hit_ratio: f64,
}

#[derive(Serialize, Deserialize)]
pub struct JsCacheSizes {
    expressions: usize,
    conversions: usize,
    dimensions: usize,
}

#[derive(Serialize, Deserialize)]
pub struct JsBenchmarkResult {
    operation: String,
    duration_ms: f64,
    iterations: usize,
    operations_per_second: f64,
}

// JavaScript-friendly types for extended functionality
#[derive(Serialize, Deserialize)]
pub struct JsMeasurementContext {
    domain: String,
    precision_requirements: JsPrecisionRequirements,
    preferred_units: Vec<String>,
    avoided_units: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct JsPrecisionRequirements {
    min_significant_figures: u32,
    max_relative_error: f64,
    require_exact: bool,
}

#[derive(Serialize, Deserialize)]
pub struct JsUnitSuggestions {
    alternatives: Vec<String>,
    is_preferred: bool,
    is_avoided: bool,
}

// Convert internal UcumError to JavaScript-friendly JsError with enhanced details
fn convert_error(err: UcumError) -> JsError {
    // Convert span if present
    let js_span = err.span.map(|span| JsSpan {
        start: span.start,
        end: span.end,
        source: span.source.clone(),
        text: span.text().to_string(),
    });
    
    // Convert error kind to JavaScript-friendly format
    let kind_details = match &err.kind {
        ErrorKind::UnitNotFound { unit, similar } => Some(JsErrorKind::UnitNotFound {
            unit: unit.clone(),
            similar: similar.clone(),
        }),
        ErrorKind::ConversionError { from, to, reason } => Some(JsErrorKind::ConversionError {
            from: from.clone(),
            to: to.clone(),
            reason: reason.clone(),
        }),
        ErrorKind::DimensionMismatch { expected, found, operation } => Some(JsErrorKind::DimensionMismatch {
            expected: expected.0.to_vec(),
            found: found.0.to_vec(),
            operation: operation.clone(),
        }),
        ErrorKind::ParseError { expected, found } => Some(JsErrorKind::ParseError {
            expected: expected.clone(),
            found: found.clone(),
        }),
        ErrorKind::InvalidExpression { reason } => Some(JsErrorKind::InvalidExpression {
            reason: reason.clone(),
        }),
        ErrorKind::InvalidPercentPlacement { position } => Some(JsErrorKind::InvalidPercentPlacement {
            position: *position,
        }),
        ErrorKind::PrecisionOverflow { operation, value } => Some(JsErrorKind::PrecisionOverflow {
            operation: operation.clone(),
            value: value.clone(),
        }),
        ErrorKind::InvalidProperty { property, available } => Some(JsErrorKind::InvalidProperty {
            property: property.clone(),
            available: available.clone(),
        }),
        ErrorKind::MultipleSlash => Some(JsErrorKind::MultipleSlash),
        ErrorKind::SpecialUnitError { unit, reason } => Some(JsErrorKind::SpecialUnitError {
            unit: unit.clone(),
            reason: reason.clone(),
        }),
    };
    
    JsError {
        message: err.message,
        error_type: format!("{:?}", err.kind),
        suggestions: err.suggestions,
        context: err.context,
        span: js_span,
        kind_details,
    }
}

// Convert internal FhirError to JavaScript-friendly JsError
fn convert_fhir_error(err: FhirError) -> JsError {
    JsError {
        message: err.to_string(),
        error_type: format!("{:?}", err),
        suggestions: Vec::new(),
        context: Vec::new(),
        span: None,
        kind_details: None,
    }
}

// Helper function to create a simple JsError for basic error cases
fn create_simple_js_error(message: String, error_type: String) -> JsError {
    JsError {
        message,
        error_type,
        suggestions: Vec::new(),
        context: Vec::new(),
        span: None,
        kind_details: None,
    }
}

// Convert internal UnitRecord to JavaScript-friendly UnitInfo
fn convert_unit_record(record: &UnitRecord) -> UnitInfo {
    // Since we can't directly access the SpecialKind enum, we'll use a heuristic:
    // - If offset != 0, it's a special unit (LinearOffset)
    // - We can't reliably detect arbitrary units without access to SpecialKind,
    //   but we can check if the code is enclosed in square brackets
    let is_special = record.offset != 0.0;
    let is_arbitrary = record.code.starts_with('[') && record.code.ends_with(']');

    UnitInfo {
        code: record.code.to_string(),
        display_name: record.display_name.to_string(),
        factor: record.factor,
        offset: record.offset,
        is_special,
        is_arbitrary,
        dimensions: record.dim.0.to_vec(),
        property: record.property.to_string(),
    }
}

// Convert internal EvalResult to JavaScript-friendly EvaluationResult
fn convert_eval_result(result: EvalResult) -> EvaluationResult {
    EvaluationResult {
        factor: to_f64(result.factor),
        offset: to_f64(result.offset),
        dimensions: result.dim.0.to_vec(),
    }
}

// Validate a UCUM expression using enhanced API
#[wasm_bindgen]
pub fn validate(expression: &str) -> Result<bool, JsValue> {
    match core_validate(expression) {
        Ok(_) => Ok(true),
        Err(err) => {
            let js_error = convert_error(err);
            Err(to_value(&js_error).unwrap())
        }
    }
}

// Analyze a UCUM expression - NEW ADR-001 feature
#[wasm_bindgen]
pub fn analyze_unit(expression: &str) -> Result<JsValue, JsValue> {
    match analyse(expression) {
        Ok(analysis) => {
            let js_analysis = JsUnitAnalysis {
                expression: analysis.expression,
                factor: analysis.factor,
                offset: analysis.offset,
                dimensions: analysis.dimension.0.to_vec(),
                is_dimensionless: analysis.is_dimensionless,
                has_offset: analysis.has_offset,
            };
            Ok(to_value(&js_analysis).unwrap())
        }
        Err(err) => {
            let js_error = convert_error(err);
            Err(to_value(&js_error).unwrap())
        }
    }
}

// Validate unit for a specific property - NEW ADR-001 feature
#[wasm_bindgen]
pub fn validate_property(expression: &str, property: &str) -> Result<bool, JsValue> {
    match validate_in_property(expression, property) {
        Ok(is_valid) => Ok(is_valid),
        Err(err) => {
            let js_error = convert_error(err);
            Err(to_value(&js_error).unwrap())
        }
    }
}

// Check if two units are comparable - NEW ADR-001 feature
#[wasm_bindgen]
pub fn units_comparable(unit1: &str, unit2: &str) -> Result<bool, JsValue> {
    match is_comparable(unit1, unit2) {
        Ok(comparable) => Ok(comparable),
        Err(err) => {
            let js_error = convert_error(err);
            Err(to_value(&js_error).unwrap())
        }
    }
}

// Get canonical units - NEW ADR-001 feature
#[wasm_bindgen]
pub fn get_canonical(expression: &str) -> Result<JsValue, JsValue> {
    match get_canonical_units(expression) {
        Ok(canonical) => {
            let js_canonical = JsCanonicalUnit {
                unit: canonical.unit,
                factor: canonical.factor,
                offset: canonical.offset,
                dimensions: canonical.dimension.0.to_vec(),
            };
            Ok(to_value(&js_canonical).unwrap())
        }
        Err(err) => {
            let js_error = convert_error(err);
            Err(to_value(&js_error).unwrap())
        }
    }
}

// Multiply units - NEW ADR-001 feature
#[wasm_bindgen]
pub fn multiply_units(unit1: &str, unit2: &str) -> Result<JsValue, JsValue> {
    match unit_multiply(unit1, unit2) {
        Ok(result) => {
            let js_result = JsUnitArithmeticResult {
                expression: result.expression,
                factor: result.factor,
                dimensions: result.dimension.0.to_vec(),
                offset: result.offset,
                is_dimensionless: result.is_dimensionless,
            };
            Ok(to_value(&js_result).unwrap())
        }
        Err(err) => {
            let js_error = convert_error(err);
            Err(to_value(&js_error).unwrap())
        }
    }
}

// Divide units - NEW ADR-001 feature
#[wasm_bindgen]
pub fn divide_units(numerator: &str, denominator: &str) -> Result<JsValue, JsValue> {
    match unit_divide(numerator, denominator) {
        Ok(result) => {
            let js_result = JsUnitArithmeticResult {
                expression: result.expression,
                factor: result.factor,
                dimensions: result.dimension.0.to_vec(),
                offset: result.offset,
                is_dimensionless: result.is_dimensionless,
            };
            Ok(to_value(&js_result).unwrap())
        }
        Err(err) => {
            let js_error = convert_error(err);
            Err(to_value(&js_error).unwrap())
        }
    }
}

// Search units by text - NEW ADR-001 feature
#[wasm_bindgen]
pub fn search_units_text(query: &str) -> Result<JsValue, JsValue> {
    let results = core_search_units(query);
    let unit_infos: Vec<UnitInfo> = results
        .iter()
        .map(|unit| convert_unit_record(unit))
        .collect();
    let js_result = JsSearchResult { units: unit_infos };
    Ok(to_value(&js_result).unwrap())
}

// Search units by property - NEW ADR-001 feature
#[wasm_bindgen]
pub fn search_units_property(property: &str) -> Result<JsValue, JsValue> {
    let results = core_search_by_property(property);
    let unit_infos: Vec<UnitInfo> = results
        .iter()
        .map(|unit| convert_unit_record(unit))
        .collect();
    let js_result = JsSearchResult { units: unit_infos };
    Ok(to_value(&js_result).unwrap())
}

// Get defined forms of a unit - NEW ADR-001 feature
#[wasm_bindgen]
pub fn get_unit_forms(base_code: &str) -> Result<JsValue, JsValue> {
    let results = core_get_defined_forms(base_code);
    let unit_infos: Vec<UnitInfo> = results
        .iter()
        .map(|unit| convert_unit_record(unit))
        .collect();
    let js_result = JsSearchResult { units: unit_infos };
    Ok(to_value(&js_result).unwrap())
}

// Fuzzy search units - NEW ADR-001 feature
#[wasm_bindgen]
pub fn search_units_fuzzy(query: &str, threshold: i64) -> Result<JsValue, JsValue> {
    let results = core_search_fuzzy(query, threshold);
    let fuzzy_matches: Vec<JsFuzzyMatch> = results
        .iter()
        .map(|(unit, score)| JsFuzzyMatch {
            unit: convert_unit_record(unit),
            score: *score,
        })
        .collect();
    let js_result = JsFuzzySearchResult {
        results: fuzzy_matches,
    };
    Ok(to_value(&js_result).unwrap())
}

// Search units with regex - NEW ADR-001 feature
#[wasm_bindgen]
pub fn search_units_regex(pattern: &str, case_sensitive: bool) -> Result<JsValue, JsValue> {
    match core_search_regex(pattern, case_sensitive) {
        Ok(results) => {
            let unit_infos: Vec<UnitInfo> = results
                .iter()
                .map(|unit| convert_unit_record(unit))
                .collect();
            let js_result = JsSearchResult { units: unit_infos };
            Ok(to_value(&js_result).unwrap())
        }
        Err(err) => {
            let js_error = convert_error(err);
            Err(to_value(&js_error).unwrap())
        }
    }
}

// Get information about a unit
#[wasm_bindgen]
pub fn get_unit_info(code: &str) -> Result<JsValue, JsValue> {
    match find_unit(code) {
        Some(unit) => {
            let mut unit_info = convert_unit_record(unit);
            // Use the new get_common_display for better display names
            unit_info.display_name = get_common_display(code);
            Ok(to_value(&unit_info).unwrap())
        }
        None => {
            let js_error = create_simple_js_error(
                format!("Unit '{}' not found", code),
                "UnitNotFound".to_string()
            );
            Err(to_value(&js_error).unwrap())
        }
    }
}

// Convert a value from one unit to another
#[wasm_bindgen]
pub fn convert(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, JsValue> {
    // Parse the source and target unit expressions
    let from_expr = match parse_expression(from_unit) {
        Ok(expr) => expr,
        Err(err) => return Err(to_value(&convert_error(err)).unwrap()),
    };

    let to_expr = match parse_expression(to_unit) {
        Ok(expr) => expr,
        Err(err) => return Err(to_value(&convert_error(err)).unwrap()),
    };

    // Evaluate the source unit
    let from_result = match evaluate(&from_expr) {
        Ok(result) => result,
        Err(err) => return Err(to_value(&convert_error(err)).unwrap()),
    };

    // Evaluate the target unit
    let to_result = match evaluate(&to_expr) {
        Ok(result) => result,
        Err(err) => return Err(to_value(&convert_error(err)).unwrap()),
    };

    // Check if the units are compatible (have the same dimensions)
    if from_result.dim.0 != to_result.dim.0 {
        let js_error = create_simple_js_error(
            format!(
                "Cannot convert from '{}' to '{}': incompatible dimensions",
                from_unit, to_unit
            ),
            "IncompatibleDimensions".to_string()
        );
        return Err(to_value(&js_error).unwrap());
    }

    // Calculate the conversion
    let canonical_value = value * to_f64(from_result.factor);
    let result_value = canonical_value / to_f64(to_result.factor);

    Ok(result_value)
}

// Evaluate a UCUM expression
#[wasm_bindgen]
pub fn evaluate_expression(expression: &str) -> Result<JsValue, JsValue> {
    // Parse the expression
    let expr = match parse_expression(expression) {
        Ok(expr) => expr,
        Err(err) => return Err(to_value(&convert_error(err)).unwrap()),
    };

    // Evaluate the expression
    let result = match evaluate(&expr) {
        Ok(result) => result,
        Err(err) => return Err(to_value(&convert_error(err)).unwrap()),
    };

    // Convert to JavaScript-friendly result
    let js_result = convert_eval_result(result);

    Ok(to_value(&js_result).unwrap())
}

// Perform arithmetic operations on units
#[wasm_bindgen]
pub fn arithmetic(
    left_unit: &str,
    operation: &str,
    right_unit: &str,
    value: f64,
) -> Result<JsValue, JsValue> {
    // Parse the left and right unit expressions
    let left_expr = match parse_expression(left_unit) {
        Ok(expr) => expr,
        Err(err) => return Err(to_value(&convert_error(err)).unwrap()),
    };

    let right_expr = match parse_expression(right_unit) {
        Ok(expr) => expr,
        Err(err) => return Err(to_value(&convert_error(err)).unwrap()),
    };

    // Evaluate the left and right units
    let left_result = match evaluate(&left_expr) {
        Ok(result) => result,
        Err(err) => return Err(to_value(&convert_error(err)).unwrap()),
    };

    let right_result = match evaluate(&right_expr) {
        Ok(result) => result,
        Err(err) => return Err(to_value(&convert_error(err)).unwrap()),
    };

    // Perform the operation
    let result = match operation {
        "mul" => {
            // For multiplication, add the dimensions
            let mut new_dimensions = [0i8; 7];
            for i in 0..7 {
                new_dimensions[i] = left_result.dim.0[i] + right_result.dim.0[i];
            }

            // Calculate the new value
            let new_factor = value * to_f64(left_result.factor) * to_f64(right_result.factor);

            // Create a new result
            EvaluationResult {
                factor: new_factor,
                offset: 0.0, // Multiplication resets offset
                dimensions: new_dimensions.to_vec(),
            }
        }
        "div" => {
            // For division, subtract the dimensions
            let mut new_dimensions = [0i8; 7];
            for i in 0..7 {
                new_dimensions[i] = left_result.dim.0[i] - right_result.dim.0[i];
            }

            // Calculate the new value
            let new_factor = value * to_f64(left_result.factor) / to_f64(right_result.factor);

            // Create a new result
            EvaluationResult {
                factor: new_factor,
                offset: 0.0, // Division resets offset
                dimensions: new_dimensions.to_vec(),
            }
        }
        _ => {
            let js_error = create_simple_js_error(
                format!("Unsupported operation: {}", operation),
                "UnsupportedOperation".to_string()
            );
            return Err(to_value(&js_error).unwrap());
        }
    };

    Ok(to_value(&result).unwrap())
}

// List all available units
#[wasm_bindgen]
pub fn list_units(filter: Option<String>) -> JsValue {
    let all_units = get_all_units();
    let mut units: Vec<UnitInfo> = Vec::new();

    for unit in all_units {
        // Apply text filter if provided
        if let Some(ref filter_str) = filter {
            let filter_lower = filter_str.to_lowercase();
            if !unit.code.to_lowercase().contains(&filter_lower)
                && !unit.display_name.to_lowercase().contains(&filter_lower)
                && !unit.property.to_lowercase().contains(&filter_lower)
            {
                continue;
            }
        }

        let unit_info = convert_unit_record(&unit);
        units.push(unit_info);
    }

    to_value(&units).unwrap()
}

// JavaScript-friendly FHIR Quantity
#[derive(Serialize, Deserialize)]
pub struct JsFhirQuantity {
    value: f64,
    unit: Option<String>,
    system: Option<String>,
    code: Option<String>,
    comparator: Option<String>,
}

// JavaScript-friendly UCUM Quantity result
#[derive(Serialize, Deserialize)]
pub struct UcumQuantityResult {
    value: f64,
    unit: String,
}

// Convert between JsFhirQuantity and FhirQuantity
fn js_to_fhir_quantity(js_quantity: &JsFhirQuantity) -> FhirQuantity {
    FhirQuantity {
        value: js_quantity.value,
        unit: js_quantity.unit.clone(),
        system: js_quantity.system.clone(),
        code: js_quantity.code.clone(),
        comparator: js_quantity.comparator.clone(),
    }
}

fn fhir_to_js_quantity(fhir_quantity: &FhirQuantity) -> JsFhirQuantity {
    JsFhirQuantity {
        value: fhir_quantity.value,
        unit: fhir_quantity.unit.clone(),
        system: fhir_quantity.system.clone(),
        code: fhir_quantity.code.clone(),
        comparator: fhir_quantity.comparator.clone(),
    }
}

// Create a FHIR Quantity with a UCUM code
#[wasm_bindgen]
pub fn create_fhir_quantity(value: f64, ucum_code: &str) -> JsValue {
    let fhir_quantity = FhirQuantity::with_ucum_code(value, ucum_code);
    let js_quantity = fhir_to_js_quantity(&fhir_quantity);
    to_value(&js_quantity).unwrap()
}

// Convert a FHIR Quantity to a UCUM Quantity
#[wasm_bindgen]
pub fn fhir_to_ucum(js_quantity_val: JsValue) -> Result<JsValue, JsValue> {
    // Deserialize the JavaScript object to our JsFhirQuantity struct
    let js_quantity: JsFhirQuantity = match serde_wasm_bindgen::from_value(js_quantity_val) {
        Ok(q) => q,
        Err(err) => {
            let js_error = create_simple_js_error(
                format!("Failed to deserialize FHIR Quantity: {}", err),
                "DeserializationError".to_string()
            );
            return Err(to_value(&js_error).unwrap());
        }
    };

    // Convert to FhirQuantity
    let fhir_quantity = js_to_fhir_quantity(&js_quantity);

    // Convert to UcumQuantity
    match fhir_quantity.to_ucum_quantity() {
        Ok(ucum_quantity) => {
            // Create a simple object with value and unit string
            let result = UcumQuantityResult {
                value: ucum_quantity.value,
                unit: ucum_quantity.unit.to_string(),
            };
            Ok(to_value(&result).unwrap())
        }
        Err(err) => Err(to_value(&convert_fhir_error(err)).unwrap()),
    }
}

// Convert a UCUM Quantity to a FHIR Quantity
#[wasm_bindgen]
pub fn ucum_to_fhir(value: f64, unit: &str) -> Result<JsValue, JsValue> {
    // Parse the unit expression
    let expr = match parse_expression(unit) {
        Ok(expr) => expr,
        Err(err) => return Err(to_value(&convert_error(err)).unwrap()),
    };

    // Create a UCUM Quantity
    let ucum_quantity = UcumQuantity { value, unit: expr };

    // Convert to FHIR Quantity
    match ToFhirQuantity::to_fhir_quantity(&ucum_quantity) {
        Ok(fhir_quantity) => {
            let js_quantity = fhir_to_js_quantity(&fhir_quantity);
            Ok(to_value(&js_quantity).unwrap())
        }
        Err(err) => Err(to_value(&convert_fhir_error(err)).unwrap()),
    }
}

// Convert a FHIR Quantity from one unit to another
#[wasm_bindgen]
pub fn convert_fhir_quantity(
    js_quantity_val: JsValue,
    target_unit: &str,
) -> Result<JsValue, JsValue> {
    // Deserialize the JavaScript object to our JsFhirQuantity struct
    let js_quantity: JsFhirQuantity = match serde_wasm_bindgen::from_value(js_quantity_val) {
        Ok(q) => q,
        Err(err) => {
            let js_error = create_simple_js_error(
                format!("Failed to deserialize FHIR Quantity: {}", err),
                "DeserializationError".to_string()
            );
            return Err(to_value(&js_error).unwrap());
        }
    };

    // Convert to FhirQuantity
    let fhir_quantity = js_to_fhir_quantity(&js_quantity);

    // Convert to target unit
    match convert_quantity(&fhir_quantity, target_unit) {
        Ok(converted) => {
            let js_converted = fhir_to_js_quantity(&converted);
            Ok(to_value(&js_converted).unwrap())
        }
        Err(err) => Err(to_value(&convert_fhir_error(err)).unwrap()),
    }
}

// Check if two FHIR Quantities are equivalent
#[wasm_bindgen]
pub fn are_fhir_quantities_equivalent(a_val: JsValue, b_val: JsValue) -> Result<bool, JsValue> {
    // Deserialize the first JavaScript object
    let a_js: JsFhirQuantity = match serde_wasm_bindgen::from_value(a_val) {
        Ok(q) => q,
        Err(err) => {
            let js_error = create_simple_js_error(
                format!("Failed to deserialize first FHIR Quantity: {}", err),
                "DeserializationError".to_string()
            );
            return Err(to_value(&js_error).unwrap());
        }
    };

    // Deserialize the second JavaScript object
    let b_js: JsFhirQuantity = match serde_wasm_bindgen::from_value(b_val) {
        Ok(q) => q,
        Err(err) => {
            let js_error = create_simple_js_error(
                format!("Failed to deserialize second FHIR Quantity: {}", err),
                "DeserializationError".to_string()
            );
            return Err(to_value(&js_error).unwrap());
        }
    };

    // Convert to FhirQuantity
    let a_fhir = js_to_fhir_quantity(&a_js);
    let b_fhir = js_to_fhir_quantity(&b_js);

    // Check if they are equivalent
    match are_equivalent(&a_fhir, &b_fhir) {
        Ok(result) => Ok(result),
        Err(err) => Err(to_value(&convert_fhir_error(err)).unwrap()),
    }
}

// Model Introspection API for WASM

/// Get UCUM model information
#[wasm_bindgen]
pub fn get_ucum_model() -> JsValue {
    let model = get_model();
    let properties: Vec<String> = get_properties().into_iter().collect();
    
    let js_model = JsUcumModel {
        version: model.version,
        revision_date: model.revision_date,
        total_units: model.units.len(),
        total_prefixes: model.prefixes.len(),
        properties,
    };
    
    to_value(&js_model).unwrap()
}

/// Validate UCUM implementation for self-consistency
#[wasm_bindgen]
pub fn validate_ucum_implementation() -> JsValue {
    let issues = validate_ucum();
    
    let js_result = JsValidationResult {
        is_valid: issues.is_empty(),
        issues,
    };
    
    to_value(&js_result).unwrap()
}

/// Get all available properties in the UCUM model
#[wasm_bindgen]
pub fn get_ucum_properties() -> JsValue {
    let properties: Vec<String> = get_properties().into_iter().collect();
    to_value(&properties).unwrap()
}

/// Validate canonical unit forms
#[wasm_bindgen]
pub fn validate_canonical_form(unit: &str, canonical: &str) -> Result<bool, JsValue> {
    match validate_canonical_units(unit, canonical) {
        Ok(is_valid) => Ok(is_valid),
        Err(err) => {
            let js_error = convert_error(err);
            Err(to_value(&js_error).unwrap())
        }
    }
}

/// Get display name for a unit code
#[wasm_bindgen]
pub fn get_unit_display_name(code: &str) -> String {
    get_common_display(code)
}

// Advanced Conversion API for WASM

/// Advanced unit conversion with precision control
#[wasm_bindgen]
pub fn convert_advanced(
    value: f64,
    from_unit: &str,
    to_unit: &str,
    config_val: JsValue,
) -> Result<JsValue, JsValue> {
    // Deserialize the conversion configuration
    let config: JsConversionConfig = match serde_wasm_bindgen::from_value(config_val) {
        Ok(c) => c,
        Err(err) => {
            let js_error = create_simple_js_error(
                format!("Failed to deserialize conversion config: {}", err),
                "DeserializationError".to_string()
            );
            return Err(to_value(&js_error).unwrap());
        }
    };
    
    // Parse precision configuration
    let decimal_precision = match config.precision_type.as_str() {
        "default" => DecimalPrecision::Default,
        "fixed" => {
            if let Some(places) = config.precision_value {
                DecimalPrecision::Fixed(places)
            } else {
                return Err(to_value(&create_simple_js_error(
                    "Fixed precision requires precision_value".to_string(),
                    "InvalidConfiguration".to_string()
                )).unwrap());
            }
        }
        "significant" => {
            if let Some(sig_figs) = config.precision_value {
                DecimalPrecision::Significant(sig_figs)
            } else {
                return Err(to_value(&create_simple_js_error(
                    "Significant precision requires precision_value".to_string(),
                    "InvalidConfiguration".to_string()
                )).unwrap());
            }
        }
        _ => {
            return Err(to_value(&create_simple_js_error(
                format!("Invalid precision type: {}", config.precision_type),
                "InvalidConfiguration".to_string()
            )).unwrap());
        }
    };
    
    // Parse rounding mode
    let rounding_mode = match config.rounding_mode.as_str() {
        "nearest" => RoundingMode::Nearest,
        "up" => RoundingMode::Up,
        "down" => RoundingMode::Down,
        "truncate" => RoundingMode::Truncate,
        _ => {
            return Err(to_value(&create_simple_js_error(
                format!("Invalid rounding mode: {}", config.rounding_mode),
                "InvalidConfiguration".to_string()
            )).unwrap());
        }
    };
    
    // Parse temperature scale
    let temp_scale = match config.temperature_scale.as_str() {
        "kelvin" => TemperatureScale::Kelvin,
        "celsius" => TemperatureScale::Celsius,
        "fahrenheit" => TemperatureScale::Fahrenheit,
        _ => {
            return Err(to_value(&create_simple_js_error(
                format!("Invalid temperature scale: {}", config.temperature_scale),
                "InvalidConfiguration".to_string()
            )).unwrap());
        }
    };
    
    // Create conversion context
    let context = AdvancedConversionContext {
        precision: decimal_precision,
        rounding: rounding_mode,
        temperature_scale: temp_scale,
        use_special_units: config.use_special_units,
    };
    
    // Perform conversion
    match convert_with_context(value, from_unit, to_unit, &context) {
        Ok(result) => {
            let js_result = JsAdvancedConversionResult {
                value: result.value,
                unit: result.unit,
                factor: result.factor,
                offset: result.offset,
                precision_info: result.precision_info,
                used_special_units: result.used_special_units,
            };
            Ok(to_value(&js_result).unwrap())
        }
        Err(err) => {
            let js_error = convert_error(err);
            Err(to_value(&js_error).unwrap())
        }
    }
}

/// Simple advanced conversion with default configuration
#[wasm_bindgen]
pub fn convert_advanced_simple(
    value: f64,
    from_unit: &str,
    to_unit: &str,
    precision_places: Option<u32>,
) -> Result<JsValue, JsValue> {
    let decimal_precision = if let Some(places) = precision_places {
        DecimalPrecision::Fixed(places)
    } else {
        DecimalPrecision::Default
    };
    
    let context = AdvancedConversionContext {
        precision: decimal_precision,
        rounding: RoundingMode::Nearest,
        temperature_scale: TemperatureScale::Kelvin,
        use_special_units: true,
    };
    
    match convert_with_context(value, from_unit, to_unit, &context) {
        Ok(result) => {
            let js_result = JsAdvancedConversionResult {
                value: result.value,
                unit: result.unit,
                factor: result.factor,
                offset: result.offset,
                precision_info: result.precision_info,
                used_special_units: result.used_special_units,
            };
            Ok(to_value(&js_result).unwrap())
        }
        Err(err) => {
            let js_error = convert_error(err);
            Err(to_value(&js_error).unwrap())
        }
    }
}

// Performance Optimization Functions for WASM

/// Get performance cache statistics
#[wasm_bindgen]
pub fn get_performance_cache_stats() -> Result<JsValue, JsValue> {
    match get_cache_stats() {
        Ok(stats) => {
            let js_stats = JsCacheStats {
                expression_hits: stats.expression_hits,
                expression_misses: stats.expression_misses,
                conversion_hits: stats.conversion_hits,
                conversion_misses: stats.conversion_misses,
                dimension_hits: stats.dimension_hits,
                dimension_misses: stats.dimension_misses,
                expression_hit_ratio: stats.expression_hit_ratio(),
                conversion_hit_ratio: stats.conversion_hit_ratio(),
                overall_hit_ratio: stats.overall_hit_ratio(),
            };
            Ok(to_value(&js_stats).unwrap())
        }
        Err(err) => {
            let js_error = convert_error(err);
            Err(to_value(&js_error).unwrap())
        }
    }
}

/// Clear the performance cache
#[wasm_bindgen]
pub fn clear_performance_cache() -> Result<bool, JsValue> {
    match clear_global_cache() {
        Ok(_) => Ok(true),
        Err(err) => {
            let js_error = convert_error(err);
            Err(to_value(&js_error).unwrap())
        }
    }
}

/// Get cache sizes
#[wasm_bindgen]
pub fn get_performance_cache_sizes() -> Result<JsValue, JsValue> {
    match get_cache_sizes() {
        Ok((expressions, conversions, dimensions)) => {
            let js_sizes = JsCacheSizes {
                expressions,
                conversions,
                dimensions,
            };
            Ok(to_value(&js_sizes).unwrap())
        }
        Err(err) => {
            let js_error = convert_error(err);
            Err(to_value(&js_error).unwrap())
        }
    }
}

// Enhanced Error Handling and Suggestion Engine Functions for WASM

/// Get suggestions for an invalid unit using the suggestion engine
#[wasm_bindgen]
pub fn get_unit_suggestions(invalid_unit: &str) -> JsValue {
    let suggestion_engine = SuggestionEngine::new();
    let suggestions = suggestion_engine.suggest_corrections(invalid_unit);
    to_value(&suggestions).unwrap()
}

/// Get alternative units for a specific property
#[wasm_bindgen]
pub fn get_property_alternatives(unit: &str, property: &str) -> JsValue {
    let suggestion_engine = SuggestionEngine::new();
    let alternatives = suggestion_engine.suggest_alternatives(unit, property);
    to_value(&alternatives).unwrap()
}

/// Get dimension-based suggestions for fixing property mismatches
#[wasm_bindgen]
pub fn get_dimension_suggestions(expected_property: &str, found_unit: &str) -> JsValue {
    let suggestion_engine = SuggestionEngine::new();
    let suggestions = suggestion_engine.suggest_dimension_fixes(expected_property, found_unit);
    to_value(&suggestions).unwrap()
}

/// Calculate string similarity between two units (for demonstration)
#[wasm_bindgen]
pub fn calculate_unit_similarity(unit1: &str, unit2: &str) -> f64 {
    SuggestionEngine::string_similarity(unit1, unit2)
}

// Extended Functionality Functions

/// Optimize a unit expression for better readability
#[wasm_bindgen]
pub fn optimize_unit_expression(expression: &str) -> Result<String, JsValue> {
    match optimize_expression(expression) {
        Ok(optimized) => Ok(optimized),
        Err(err) => {
            let js_error = convert_error(err);
            Err(to_value(&js_error).unwrap())
        }
    }
}

/// Convert a unit expression to its canonical (base units) form
#[wasm_bindgen]
pub fn canonicalize_unit_expression(expression: &str) -> Result<String, JsValue> {
    match canonicalize_expression(expression) {
        Ok(canonical) => Ok(canonical),
        Err(err) => {
            let js_error = convert_error(err);
            Err(to_value(&js_error).unwrap())
        }
    }
}

/// Simplify a unit expression by combining like terms
#[wasm_bindgen]
pub fn simplify_unit_expression(expression: &str) -> Result<String, JsValue> {
    match simplify_expression(expression) {
        Ok(simplified) => Ok(simplified),
        Err(err) => {
            let js_error = convert_error(err);
            Err(to_value(&js_error).unwrap())
        }
    }
}

/// Create a measurement context for a specific domain
#[wasm_bindgen]
pub fn create_measurement_context(domain: &str) -> Result<JsValue, JsValue> {
    let context = match domain.to_lowercase().as_str() {
        "medical" => MeasurementContext::medical(),
        "engineering" => MeasurementContext::engineering(),
        "physics" => MeasurementContext::physics(),
        "chemistry" => MeasurementContext::chemistry(),
        "general" => MeasurementContext::default(),
        _ => {
            let js_error = create_simple_js_error(
                format!("Invalid domain: {}. Use: medical, engineering, physics, chemistry, general", domain),
                "InvalidDomain".to_string()
            );
            return Err(to_value(&js_error).unwrap());
        }
    };
    
    let js_context = JsMeasurementContext {
        domain: format!("{:?}", context.domain),
        precision_requirements: JsPrecisionRequirements {
            min_significant_figures: context.precision_requirements.min_significant_figures,
            max_relative_error: context.precision_requirements.max_relative_error,
            require_exact: context.precision_requirements.require_exact,
        },
        preferred_units: context.preferred_units,
        avoided_units: context.avoided_units,
    };
    
    Ok(to_value(&js_context).unwrap())
}

/// Check if a unit is preferred in a measurement context
#[wasm_bindgen]
pub fn is_unit_preferred(domain: &str, unit: &str) -> Result<bool, JsValue> {
    let context = match domain.to_lowercase().as_str() {
        "medical" => MeasurementContext::medical(),
        "engineering" => MeasurementContext::engineering(),
        "physics" => MeasurementContext::physics(),
        "chemistry" => MeasurementContext::chemistry(),
        "general" => MeasurementContext::default(),
        _ => {
            let js_error = create_simple_js_error(
                format!("Invalid domain: {}", domain),
                "InvalidDomain".to_string()
            );
            return Err(to_value(&js_error).unwrap());
        }
    };
    
    Ok(context.is_preferred_unit(unit))
}

/// Check if a unit should be avoided in a measurement context
#[wasm_bindgen]
pub fn is_unit_avoided(domain: &str, unit: &str) -> Result<bool, JsValue> {
    let context = match domain.to_lowercase().as_str() {
        "medical" => MeasurementContext::medical(),
        "engineering" => MeasurementContext::engineering(),
        "physics" => MeasurementContext::physics(),
        "chemistry" => MeasurementContext::chemistry(),
        "general" => MeasurementContext::default(),
        _ => {
            let js_error = create_simple_js_error(
                format!("Invalid domain: {}", domain),
                "InvalidDomain".to_string()
            );
            return Err(to_value(&js_error).unwrap());
        }
    };
    
    Ok(context.is_avoided_unit(unit))
}

/// Get unit suggestions for a measurement context
#[wasm_bindgen]
pub fn get_context_unit_suggestions(domain: &str, unit: &str) -> Result<JsValue, JsValue> {
    let context = match domain.to_lowercase().as_str() {
        "medical" => MeasurementContext::medical(),
        "engineering" => MeasurementContext::engineering(),
        "physics" => MeasurementContext::physics(),
        "chemistry" => MeasurementContext::chemistry(),
        "general" => MeasurementContext::default(),
        _ => {
            let js_error = create_simple_js_error(
                format!("Invalid domain: {}", domain),
                "InvalidDomain".to_string()
            );
            return Err(to_value(&js_error).unwrap());
        }
    };
    
    match context.suggest_alternatives(unit) {
        Ok(alternatives) => {
            let js_suggestions = JsUnitSuggestions {
                alternatives,
                is_preferred: context.is_preferred_unit(unit),
                is_avoided: context.is_avoided_unit(unit),
            };
            Ok(to_value(&js_suggestions).unwrap())
        }
        Err(err) => {
            let js_error = convert_error(err);
            Err(to_value(&js_error).unwrap())
        }
    }
}