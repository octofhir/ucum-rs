//! WebAssembly bindings for UCUM operations.
//!
//! This crate provides JavaScript-compatible bindings for `octofhir-ucum` and is
//! published to npm as `@octofhir/ucum-wasm` via `wasm-pack`.

use octofhir_ucum::{
    ErrorKind, Number, UcumError, UnitArithmeticResult, UnitRecord, analyse,
    convert as core_convert, find_unit, get_canonical_units, is_comparable,
    search_units as core_search_units, unit_divide, unit_multiply, validate as core_validate,
};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[derive(Serialize, Deserialize)]
pub struct JsError {
    pub message: String,
    pub error_type: String,
    /// The stable code of the kind (`ErrorKind::code`): `invalid_expression`,
    /// `unit_not_found`, `dimension_mismatch`...
    pub code: String,
    /// For a syntax error, the byte offset in the expression where parsing stopped
    pub position: Option<usize>,
    /// For a syntax error, the expression `position` refers to
    pub expression: Option<String>,
    /// Corrections to propose (`Did you mean 'Pa'?`)
    pub suggestions: Vec<String>,
    /// Where the error happened (`In expression: 'mg/xyz'`)
    pub context: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UnitInfo {
    pub code: String,
    pub display_name: String,
    pub property: String,
}

/// A magnitude as a JavaScript number, rounded, and `null` when it is out of the range of
/// one (`10*1000`); the `*_exact` fields hold it with all its digits ("1e1000", "0.0254").
fn js_number(n: Number) -> Option<f64> {
    let x = n.to_f64();
    (x.is_finite() && (x != 0.0 || n.is_zero())).then_some(x)
}

#[derive(Serialize, Deserialize)]
pub struct JsUnitAnalysis {
    pub expression: String,
    pub factor: Option<f64>,
    pub factor_exact: String,
    /// Exponents of m, s, g, rad, K, C, cd
    pub dimension: [i8; 7],
    pub scale: Option<f64>,
    pub scale_exact: String,
    pub offset: Option<f64>,
    pub offset_exact: String,
    pub arbitrary: Vec<(String, i32)>,
    /// Kind of the special unit (`Celsius`, `Lg`), if the expression is one
    pub special: Option<String>,
    pub is_dimensionless: bool,
}

#[derive(Serialize, Deserialize)]
pub struct JsUnitArithmetic {
    pub expression: String,
    pub factor: Option<f64>,
    pub factor_exact: String,
    pub dimension: [i8; 7],
    pub is_dimensionless: bool,
}

#[derive(Serialize, Deserialize)]
pub struct JsCanonicalUnit {
    pub unit: String,
    pub factor: Option<f64>,
    pub factor_exact: String,
    pub offset: Option<f64>,
    pub offset_exact: String,
    pub dimension: [i8; 7],
}

#[derive(Serialize, Deserialize)]
pub struct JsSearchResult {
    pub units: Vec<UnitInfo>,
}

fn convert_error(err: UcumError) -> JsError {
    let (expression, position) = match &err.kind {
        ErrorKind::InvalidExpression {
            expression,
            position,
            ..
        } => (Some(expression.clone()), *position),
        _ => (None, None),
    };
    JsError {
        // The suggestions have a field of their own
        message: err.message,
        error_type: "UcumError".to_string(),
        code: err.kind.code().to_string(),
        position,
        expression,
        suggestions: err.suggestions,
        context: err.context,
    }
}

fn create_simple_js_error(message: String, error_type: String, code: &str) -> JsError {
    JsError {
        message,
        error_type,
        code: code.to_string(),
        position: None,
        expression: None,
        suggestions: Vec::new(),
        context: Vec::new(),
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
                factor: js_number(analysis.factor),
                factor_exact: analysis.factor.to_string(),
                dimension: analysis.dimension.0,
                scale: js_number(analysis.scale),
                scale_exact: analysis.scale.to_string(),
                offset: js_number(analysis.offset),
                offset_exact: analysis.offset.to_string(),
                arbitrary: analysis
                    .arbitrary
                    .iter()
                    .map(|(atom, exp)| (atom.to_string(), *exp))
                    .collect(),
                special: analysis.special.map(|kind| format!("{kind:?}")),
                is_dimensionless: analysis.is_dimensionless,
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
                factor: js_number(canonical.factor),
                factor_exact: canonical.factor.to_string(),
                offset: js_number(canonical.offset),
                offset_exact: canonical.offset.to_string(),
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
            let unit_info = convert_unit_record(unit);
            Ok(to_value(&unit_info)?)
        }
        None => {
            let js_error = create_simple_js_error(
                format!("Unit '{}' not found", code),
                "NotFound".to_string(),
                "unit_not_found",
            );
            Err(to_value(&js_error)?)
        }
    }
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

fn arithmetic_result(result: Result<UnitArithmeticResult, UcumError>) -> Result<JsValue, JsValue> {
    match result {
        Ok(result) => Ok(to_value(&JsUnitArithmetic {
            expression: result.expression,
            factor: js_number(result.factor),
            factor_exact: result.factor.to_string(),
            dimension: result.dimension.0,
            is_dimensionless: result.is_dimensionless,
        })?),
        Err(err) => Err(to_value(&convert_error(err))?),
    }
}

#[wasm_bindgen]
pub fn multiply_units(unit1: &str, unit2: &str) -> Result<JsValue, JsValue> {
    arithmetic_result(unit_multiply(unit1, unit2))
}

#[wasm_bindgen]
pub fn divide_units(unit1: &str, unit2: &str) -> Result<JsValue, JsValue> {
    arithmetic_result(unit_divide(unit1, unit2))
}

#[wasm_bindgen]
pub fn convert(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, JsValue> {
    core_convert(value, from_unit, to_unit)
        .map_err(|err| to_value(&convert_error(err)).unwrap_or(JsValue::NULL))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magnitudes_out_of_the_range_of_a_javascript_number_are_null() {
        let n = |text: &str| Number::parse(text).unwrap();
        assert_eq!(js_number(n("0.0254")), Some(0.0254));
        assert_eq!(js_number(Number::ZERO), Some(0.0));
        assert_eq!(js_number(n("1e1000")), None);
        assert_eq!(js_number(n("1e-1000")), None);
        let analysis = analyse("10*1000").unwrap();
        assert_eq!(js_number(analysis.factor), None);
        assert_eq!(analysis.factor.to_string(), "1e1000");
    }

    #[test]
    fn errors_carry_their_code_position_and_suggestions() {
        let err = convert_error(core_validate("m//s").unwrap_err());
        assert_eq!(
            (err.code.as_str(), err.position),
            ("invalid_expression", Some(2))
        );
        assert_eq!(err.expression.as_deref(), Some("m//s"));
        assert!(!err.context.is_empty());
        let err = convert_error(core_validate("metr").unwrap_err());
        assert_eq!((err.code.as_str(), err.position), ("unit_not_found", None));
        assert!(!err.suggestions.is_empty());
        // Once, in their own field
        assert!(!err.message.contains("Suggestions"), "{}", err.message);
        let err = convert_error(core_convert(1.0, "m", "kg").unwrap_err());
        assert_eq!(err.code, "dimension_mismatch");
    }
}
