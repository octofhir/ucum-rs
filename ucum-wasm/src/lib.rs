use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::to_value;
use octofhir_ucum_core::{
    parse_expression, evaluate, find_unit, UnitRecord, EvalResult, UcumError
};

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

// JavaScript-friendly error type
#[derive(Serialize, Deserialize)]
pub struct JsError {
    message: String,
    error_type: String,
}

// JavaScript-friendly unit information
#[derive(Serialize, Deserialize)]
pub struct UnitInfo {
    code: String,
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

// Convert internal UcumError to JavaScript-friendly JsError
fn convert_error(err: UcumError) -> JsError {
    JsError {
        message: err.to_string(),
        error_type: format!("{:?}", err),
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
        factor: result.factor,
        offset: result.offset,
        dimensions: result.dim.0.to_vec(),
    }
}

// Validate a UCUM expression
#[wasm_bindgen]
pub fn validate(expression: &str) -> Result<bool, JsValue> {
    match parse_expression(expression) {
        Ok(_) => Ok(true),
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
            let unit_info = convert_unit_record(unit);
            Ok(to_value(&unit_info).unwrap())
        },
        None => {
            let js_error = JsError {
                message: format!("Unit '{}' not found", code),
                error_type: "UnitNotFound".to_string(),
            };
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
        let js_error = JsError {
            message: format!("Cannot convert from '{}' to '{}': incompatible dimensions", from_unit, to_unit),
            error_type: "IncompatibleDimensions".to_string(),
        };
        return Err(to_value(&js_error).unwrap());
    }

    // Calculate the conversion
    let canonical_value = value * from_result.factor;
    let result_value = canonical_value / to_result.factor;

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
pub fn arithmetic(left_unit: &str, operation: &str, right_unit: &str, value: f64) -> Result<JsValue, JsValue> {
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
            let new_factor = value * left_result.factor * right_result.factor;

            // Create a new result
            EvaluationResult {
                factor: new_factor,
                offset: 0.0, // Multiplication resets offset
                dimensions: new_dimensions.to_vec(),
            }
        },
        "div" => {
            // For division, subtract the dimensions
            let mut new_dimensions = [0i8; 7];
            for i in 0..7 {
                new_dimensions[i] = left_result.dim.0[i] - right_result.dim.0[i];
            }

            // Calculate the new value
            let new_factor = value * left_result.factor / right_result.factor;

            // Create a new result
            EvaluationResult {
                factor: new_factor,
                offset: 0.0, // Division resets offset
                dimensions: new_dimensions.to_vec(),
            }
        },
        _ => {
            let js_error = JsError {
                message: format!("Unsupported operation: {}", operation),
                error_type: "UnsupportedOperation".to_string(),
            };
            return Err(to_value(&js_error).unwrap());
        }
    };

    Ok(to_value(&result).unwrap())
}

// List all available units
#[wasm_bindgen]
pub fn list_units(filter: Option<String>) -> JsValue {
    let units: Vec<UnitInfo> = Vec::new();

    // This is a simplified implementation that would need to be expanded
    // to actually list all units from the registry
    // For now, we'll just return an empty array

    to_value(&units).unwrap()
}
