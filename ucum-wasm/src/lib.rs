use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::to_value;
use octofhir_ucum_core::{
    parse_expression, evaluate, find_unit, UnitRecord, EvalResult, UcumError, Quantity as UcumQuantity
};
use octofhir_ucum_fhir::{
    FhirQuantity, ToFhirQuantity, FromFhirQuantity, convert_quantity, are_equivalent, FhirError
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

// Convert internal UcumError to JavaScript-friendly JsError
fn convert_error(err: UcumError) -> JsError {
    JsError {
        message: err.to_string(),
        error_type: format!("{:?}", err),
    }
}

// Convert internal FhirError to JavaScript-friendly JsError
fn convert_fhir_error(err: FhirError) -> JsError {
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
            let js_error = JsError {
                message: format!("Failed to deserialize FHIR Quantity: {}", err),
                error_type: "DeserializationError".to_string(),
            };
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
        },
        Err(err) => {
            Err(to_value(&convert_fhir_error(err)).unwrap())
        }
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
    let ucum_quantity = UcumQuantity {
        value,
        unit: expr,
    };

    // Convert to FHIR Quantity
    match ucum_quantity.to_fhir_quantity() {
        Ok(fhir_quantity) => {
            let js_quantity = fhir_to_js_quantity(&fhir_quantity);
            Ok(to_value(&js_quantity).unwrap())
        },
        Err(err) => {
            Err(to_value(&convert_fhir_error(err)).unwrap())
        }
    }
}

// Convert a FHIR Quantity from one unit to another
#[wasm_bindgen]
pub fn convert_fhir_quantity(js_quantity_val: JsValue, target_unit: &str) -> Result<JsValue, JsValue> {
    // Deserialize the JavaScript object to our JsFhirQuantity struct
    let js_quantity: JsFhirQuantity = match serde_wasm_bindgen::from_value(js_quantity_val) {
        Ok(q) => q,
        Err(err) => {
            let js_error = JsError {
                message: format!("Failed to deserialize FHIR Quantity: {}", err),
                error_type: "DeserializationError".to_string(),
            };
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
        },
        Err(err) => {
            Err(to_value(&convert_fhir_error(err)).unwrap())
        }
    }
}

// Check if two FHIR Quantities are equivalent
#[wasm_bindgen]
pub fn are_fhir_quantities_equivalent(a_val: JsValue, b_val: JsValue) -> Result<bool, JsValue> {
    // Deserialize the first JavaScript object
    let a_js: JsFhirQuantity = match serde_wasm_bindgen::from_value(a_val) {
        Ok(q) => q,
        Err(err) => {
            let js_error = JsError {
                message: format!("Failed to deserialize first FHIR Quantity: {}", err),
                error_type: "DeserializationError".to_string(),
            };
            return Err(to_value(&js_error).unwrap());
        }
    };

    // Deserialize the second JavaScript object
    let b_js: JsFhirQuantity = match serde_wasm_bindgen::from_value(b_val) {
        Ok(q) => q,
        Err(err) => {
            let js_error = JsError {
                message: format!("Failed to deserialize second FHIR Quantity: {}", err),
                error_type: "DeserializationError".to_string(),
            };
            return Err(to_value(&js_error).unwrap());
        }
    };

    // Convert to FhirQuantity
    let a_fhir = js_to_fhir_quantity(&a_js);
    let b_fhir = js_to_fhir_quantity(&b_js);

    // Check if they are equivalent
    match are_equivalent(&a_fhir, &b_fhir) {
        Ok(result) => Ok(result),
        Err(err) => {
            Err(to_value(&convert_fhir_error(err)).unwrap())
        }
    }
}
