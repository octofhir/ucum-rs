use octofhir_ucum_core::{
    EvalResult,
    UcumError,
    UnitRecord,
    analyse,
    evaluate,
    find_unit,
    get_canonical_units,
    is_comparable,
    parse_expression,
    search_units as core_search_units,
    validate as core_validate,
    precision::to_f64,
};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[derive(Serialize, Deserialize)]
pub struct JsError {
    message: String,
    error_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct UnitInfo {
    code: String,
    display_name: String,
    property: String,
}

#[derive(Serialize, Deserialize)]
pub struct JsUnitAnalysis {
    expression: String,
    factor: f64,
    offset: f64,
    dimensions: Vec<i8>,
    is_dimensionless: bool,
    has_offset: bool,
}

#[derive(Serialize, Deserialize)]
pub struct JsCanonicalUnit {
    unit: String,
    factor: f64,
    offset: f64,
    dimensions: Vec<i8>,
}

#[derive(Serialize, Deserialize)]
pub struct JsSearchResult {
    units: Vec<UnitInfo>,
}


fn convert_error(err: UcumError) -> JsError {
    JsError {
        message: err.to_string(),
        error_type: "UcumError".to_string(),
    }
}

fn create_simple_js_error(message: String, error_type: String) -> JsError {
    JsError {
        message,
        error_type,
    }
}

fn convert_unit_record(record: &UnitRecord) -> UnitInfo {
    UnitInfo {
        code: record.code.to_string(),
        display_name: record.display_name.to_string(),
        property: record.property.to_string(),
    }
}

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

#[wasm_bindgen]
pub fn analyze(expression: &str) -> Result<JsValue, JsValue> {
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

#[wasm_bindgen]
pub fn comparable(unit1: &str, unit2: &str) -> Result<bool, JsValue> {
    match is_comparable(unit1, unit2) {
        Ok(comparable) => Ok(comparable),
        Err(err) => {
            let js_error = convert_error(err);
            Err(to_value(&js_error).unwrap())
        }
    }
}

#[wasm_bindgen]
pub fn search(query: &str) -> JsValue {
    let results = core_search_units(query);
    let unit_infos: Vec<UnitInfo> = results
        .iter()
        .map(|unit| convert_unit_record(unit))
        .collect();
    let js_result = JsSearchResult { units: unit_infos };
    to_value(&js_result).unwrap()
}

#[wasm_bindgen]
pub fn get_unit_info(code: &str) -> Result<JsValue, JsValue> {
    match find_unit(code) {
        Some(unit) => {
            let unit_info = convert_unit_record(unit);
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

#[wasm_bindgen]
pub fn convert(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, JsValue> {
    let from_expr = match parse_expression(from_unit) {
        Ok(expr) => expr,
        Err(err) => return Err(to_value(&convert_error(err)).unwrap()),
    };

    let to_expr = match parse_expression(to_unit) {
        Ok(expr) => expr,
        Err(err) => return Err(to_value(&convert_error(err)).unwrap()),
    };

    let from_result = match evaluate(&from_expr) {
        Ok(result) => result,
        Err(err) => return Err(to_value(&convert_error(err)).unwrap()),
    };

    let to_result = match evaluate(&to_expr) {
        Ok(result) => result,
        Err(err) => return Err(to_value(&convert_error(err)).unwrap()),
    };

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

    let canonical_value = value * to_f64(from_result.factor);
    let result_value = canonical_value / to_f64(to_result.factor);

    Ok(result_value)
}

