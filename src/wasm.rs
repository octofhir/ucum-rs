//! WebAssembly bindings for UCUM operations.
//!
//! This module provides JavaScript-compatible bindings for UCUM functionality.
//! It's only available when the "wasm" feature is enabled.

use crate::{
    UcumError, UnitRecord, analyse, evaluate_owned, find_unit, get_canonical_units, is_comparable,
    parse_expression, precision::to_f64, search_units as core_search_units,
    validate as core_validate,
};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    #[cfg(feature = "wasm")]
    console_error_panic_hook::set_once();
}

#[derive(Serialize, Deserialize)]
pub struct JsError {
    pub message: String,
    pub error_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct UnitInfo {
    pub code: String,
    pub display_name: String,
    pub property: String,
}

#[derive(Serialize, Deserialize)]
pub struct JsUnitAnalysis {
    pub expression: String,
    pub factor: f64,
    pub dimension: [i8; 7],
    pub offset: f64,
    pub is_dimensionless: bool,
    pub has_offset: bool,
}

#[derive(Serialize, Deserialize)]
pub struct JsCanonicalUnit {
    pub unit: String,
    pub factor: f64,
    pub offset: f64,
    pub dimension: [i8; 7],
}

#[derive(Serialize, Deserialize)]
pub struct JsSearchResult {
    pub units: Vec<UnitInfo>,
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
        Ok(()) => Ok(true),
        Err(err) => {
            let js_error = convert_error(err);
            Err(to_value(&js_error)?)
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
                dimension: analysis.dimension.0,
                offset: analysis.offset,
                is_dimensionless: analysis.is_dimensionless,
                has_offset: analysis.has_offset,
            };
            Ok(to_value(&js_analysis)?)
        }
        Err(err) => {
            let js_error = convert_error(err);
            Err(to_value(&js_error)?)
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
                dimension: canonical.dimension.0,
            };
            Ok(to_value(&js_canonical)?)
        }
        Err(err) => {
            let js_error = convert_error(err);
            Err(to_value(&js_error)?)
        }
    }
}

#[wasm_bindgen]
pub fn comparable(unit1: &str, unit2: &str) -> Result<bool, JsValue> {
    match is_comparable(unit1, unit2) {
        Ok(result) => Ok(result),
        Err(err) => {
            let js_error = convert_error(err);
            Err(to_value(&js_error)?)
        }
    }
}

#[wasm_bindgen]
pub fn search(query: &str) -> JsValue {
    let results = core_search_units(query);
    let unit_infos: Vec<UnitInfo> = results
        .iter()
        .map(|record| convert_unit_record(record))
        .collect();
    let search_result = JsSearchResult { units: unit_infos };
    to_value(&search_result).unwrap_or(JsValue::NULL)
}

#[wasm_bindgen]
pub fn get_unit_info(code: &str) -> Result<JsValue, JsValue> {
    match find_unit(code) {
        Some(unit) => {
            let unit_info = convert_unit_record(&unit);
            Ok(to_value(&unit_info)?)
        }
        None => {
            let js_error = create_simple_js_error(
                format!("Unit '{}' not found", code),
                "NotFound".to_string(),
            );
            Err(to_value(&js_error)?)
        }
    }
}

#[wasm_bindgen]
pub fn analyze_unit(expression: &str) -> Result<JsValue, JsValue> {
    analyze(expression)
}

#[wasm_bindgen]
pub fn search_units_text(query: &str) -> JsValue {
    search(query)
}

#[wasm_bindgen]
pub fn list_units(filter: Option<String>) -> JsValue {
    let all_units = core_search_units("");
    let filtered_units: Vec<UnitInfo> = if let Some(f) = filter {
        all_units
            .iter()
            .filter(|unit| unit.property.contains(&f))
            .map(|record| convert_unit_record(record))
            .collect()
    } else {
        all_units
            .iter()
            .map(|record| convert_unit_record(record))
            .collect()
    };
    let search_result = JsSearchResult {
        units: filtered_units,
    };
    to_value(&search_result).unwrap_or(JsValue::NULL)
}

#[wasm_bindgen]
pub fn units_comparable(unit1: &str, unit2: &str) -> Result<bool, JsValue> {
    comparable(unit1, unit2)
}

#[wasm_bindgen]
pub fn multiply_units(_unit1: &str, _unit2: &str) -> Result<JsValue, JsValue> {
    let js_error = create_simple_js_error(
        "Unit multiplication not yet implemented".to_string(),
        "NotImplemented".to_string(),
    );
    Err(to_value(&js_error)?)
}

#[wasm_bindgen]
pub fn divide_units(_unit1: &str, _unit2: &str) -> Result<JsValue, JsValue> {
    let js_error = create_simple_js_error(
        "Unit division not yet implemented".to_string(),
        "NotImplemented".to_string(),
    );
    Err(to_value(&js_error)?)
}

#[wasm_bindgen]
pub fn evaluate_expression(expression: &str) -> Result<JsValue, JsValue> {
    analyze(expression)
}

#[wasm_bindgen]
pub fn convert(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, JsValue> {
    let from_expr = match parse_expression(from_unit) {
        Ok(expr) => expr,
        Err(err) => {
            let js_error = convert_error(err);
            return Err(to_value(&js_error)?);
        }
    };

    let to_expr = match parse_expression(to_unit) {
        Ok(expr) => expr,
        Err(err) => {
            let js_error = convert_error(err);
            return Err(to_value(&js_error)?);
        }
    };

    let from_res = match evaluate_owned(&from_expr) {
        Ok(res) => res,
        Err(err) => {
            let js_error = convert_error(err);
            return Err(to_value(&js_error)?);
        }
    };

    let to_res = match evaluate_owned(&to_expr) {
        Ok(res) => res,
        Err(err) => {
            let js_error = convert_error(err);
            return Err(to_value(&js_error)?);
        }
    };

    if from_res.dim != to_res.dim {
        let js_error = create_simple_js_error(
            format!(
                "Incompatible dimensions between '{}' and '{}'",
                from_unit, to_unit
            ),
            "IncompatibleDimensions".to_string(),
        );
        return Err(to_value(&js_error)?);
    }

    let canonical = value * to_f64(from_res.factor) + to_f64(from_res.offset);
    let result = (canonical - to_f64(to_res.offset)) / to_f64(to_res.factor);
    Ok(result)
}
