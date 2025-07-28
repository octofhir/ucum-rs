use octofhir_ucum_core::precision::{NumericOps, Number, from_f64, to_f64};
use octofhir_ucum_core::{evaluate_owned, generate_display_name_owned, parse_expression};
use std::fs;
use std::path::Path;

#[derive(Debug)]
struct TestCase {
    id: String,
    unit: String,
    valid: bool,
    reason: Option<String>,
}

#[derive(Debug)]
struct ConversionTest {
    id: String,
    value: Number,
    source_unit: String,
    target_unit: String,
    outcome: Number,
}

#[derive(Debug)]
struct DisplayNameTest {
    id: String,
    unit: String,
    display: String,
}

#[derive(Debug)]
struct MultiplicationTest {
    id: String,
    v1: Number,
    u1: String,
    v2: Number,
    u2: String,
    v_res: Number,
    u_res: String,
}

#[derive(Debug)]
struct DivisionTest {
    id: String,
    v1: Number,
    u1: String,
    v2: Number,
    u2: String,
    v_res: Number,
    u_res: String,
}

#[derive(Debug)]
struct TestResults {
    passed: usize,
    failed: usize,
    failed_cases: Vec<String>,
}

impl TestResults {
    fn new() -> Self {
        Self {
            passed: 0,
            failed: 0,
            failed_cases: Vec::new(),
        }
    }

    fn add_pass(&mut self) {
        self.passed += 1;
    }

    fn add_fail(&mut self, case_info: String) {
        self.failed += 1;
        self.failed_cases.push(case_info);
    }

    fn total(&self) -> usize {
        self.passed + self.failed
    }

    fn print_summary(&self, test_type: &str) {
        println!("[DEBUG_LOG] Official {} test results:", test_type);
        println!("[DEBUG_LOG] Passed: {}", self.passed);
        println!("[DEBUG_LOG] Failed: {}", self.failed);
        println!("[DEBUG_LOG] Total: {}", self.total());

        if !self.failed_cases.is_empty() {
            println!("[DEBUG_LOG] Failed cases:");
            for case in &self.failed_cases {
                println!("[DEBUG_LOG]   {}", case);
            }
        }
    }
}

// Parse functions (reusing existing logic)
fn parse_xml_test_file(file_path: &str) -> Result<Vec<TestCase>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let mut test_cases = Vec::new();
    let mut in_validation_section = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("<validation>") {
            in_validation_section = true;
            continue;
        }

        if trimmed.starts_with("</validation>") {
            in_validation_section = false;
            continue;
        }

        if in_validation_section && trimmed.starts_with("<case") && trimmed.contains("id=") {
            if let (Some(id), Some(unit)) = (
                extract_attribute(trimmed, "id"),
                extract_attribute(trimmed, "unit"),
            ) {
                let valid = extract_attribute(trimmed, "valid")
                    .map(|v| v == "true")
                    .unwrap_or(false);
                let reason = extract_attribute(trimmed, "reason");

                test_cases.push(TestCase {
                    id,
                    unit,
                    valid,
                    reason,
                });
            }
        }
    }

    Ok(test_cases)
}

fn extract_attribute(line: &str, attr_name: &str) -> Option<String> {
    let pattern = format!("{}=\"", attr_name);
    if let Some(start) = line.find(&pattern) {
        let start = start + pattern.len();
        if let Some(end) = line[start..].find('"') {
            return Some(line[start..start + end].to_string());
        }
    }
    None
}

fn parse_display_name_tests(
    file_path: &str,
) -> Result<Vec<DisplayNameTest>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let mut test_cases = Vec::new();
    let mut in_display_section = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("<displayNameGeneration>") {
            in_display_section = true;
            continue;
        }

        if trimmed.starts_with("</displayNameGeneration>") {
            in_display_section = false;
            continue;
        }

        if in_display_section && trimmed.starts_with("<case") {
            if let (Some(id), Some(unit), Some(display)) = (
                extract_attribute(trimmed, "id"),
                extract_attribute(trimmed, "unit"),
                extract_attribute(trimmed, "display"),
            ) {
                test_cases.push(DisplayNameTest { id, unit, display });
            }
        }
    }

    Ok(test_cases)
}

fn parse_conversion_tests(
    file_path: &str,
) -> Result<Vec<ConversionTest>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let mut test_cases = Vec::new();
    let mut in_conversion_section = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("<conversion>") {
            in_conversion_section = true;
            continue;
        }

        if trimmed.starts_with("</conversion>") {
            in_conversion_section = false;
            continue;
        }

        if in_conversion_section && trimmed.starts_with("<case") {
            if let (Some(id), Some(value_str), Some(src_unit), Some(dst_unit), Some(outcome_str)) = (
                extract_attribute(trimmed, "id"),
                extract_attribute(trimmed, "value"),
                extract_attribute(trimmed, "srcUnit"),
                extract_attribute(trimmed, "dstUnit"),
                extract_attribute(trimmed, "outcome"),
            ) {
                if let (Ok(value), Ok(outcome)) =
                    (value_str.parse::<f64>(), outcome_str.parse::<f64>())
                {
                    test_cases.push(ConversionTest {
                        id,
                        value: from_f64(value),
                        source_unit: src_unit,
                        target_unit: dst_unit,
                        outcome: from_f64(outcome),
                    });
                }
            }
        }
    }

    Ok(test_cases)
}

fn parse_multiplication_tests(
    file_path: &str,
) -> Result<Vec<MultiplicationTest>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let mut test_cases = Vec::new();
    let mut in_multiplication_section = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("<multiplication>") {
            in_multiplication_section = true;
            continue;
        }

        if trimmed.starts_with("</multiplication>") {
            in_multiplication_section = false;
            continue;
        }

        if in_multiplication_section && trimmed.starts_with("<case") {
            if let (
                Some(id),
                Some(v1_str),
                Some(u1),
                Some(v2_str),
                Some(u2),
                Some(v_res_str),
                Some(u_res),
            ) = (
                extract_attribute(trimmed, "id"),
                extract_attribute(trimmed, "v1"),
                extract_attribute(trimmed, "u1"),
                extract_attribute(trimmed, "v2"),
                extract_attribute(trimmed, "u2"),
                extract_attribute(trimmed, "vRes"),
                extract_attribute(trimmed, "uRes"),
            ) {
                if let (Ok(v1), Ok(v2), Ok(v_res)) = (
                    v1_str.parse::<f64>(),
                    v2_str.parse::<f64>(),
                    v_res_str.parse::<f64>(),
                ) {
                    test_cases.push(MultiplicationTest {
                        id,
                        v1: from_f64(v1),
                        u1,
                        v2: from_f64(v2),
                        u2,
                        v_res: from_f64(v_res),
                        u_res,
                    });
                }
            }
        }
    }

    Ok(test_cases)
}

fn parse_division_tests(file_path: &str) -> Result<Vec<DivisionTest>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let mut test_cases = Vec::new();
    let mut in_division_section = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("<division>") {
            in_division_section = true;
            continue;
        }

        if trimmed.starts_with("</division>") {
            in_division_section = false;
            continue;
        }

        if in_division_section && trimmed.starts_with("<case") {
            if let (
                Some(id),
                Some(v1_str),
                Some(u1),
                Some(v2_str),
                Some(u2),
                Some(v_res_str),
                Some(u_res),
            ) = (
                extract_attribute(trimmed, "id"),
                extract_attribute(trimmed, "v1"),
                extract_attribute(trimmed, "u1"),
                extract_attribute(trimmed, "v2"),
                extract_attribute(trimmed, "u2"),
                extract_attribute(trimmed, "vRes"),
                extract_attribute(trimmed, "uRes"),
            ) {
                if let (Ok(v1), Ok(v2), Ok(v_res)) = (
                    v1_str.parse::<f64>(),
                    v2_str.parse::<f64>(),
                    v_res_str.parse::<f64>(),
                ) {
                    test_cases.push(DivisionTest {
                        id,
                        v1: from_f64(v1),
                        u1,
                        v2: from_f64(v2),
                        u2,
                        v_res: from_f64(v_res),
                        u_res,
                    });
                }
            }
        }
    }

    Ok(test_cases)
}

// Test runners for each group
fn run_validation_tests_group() -> TestResults {
    let mut results = TestResults::new();
    let test_files = [
        "tests/official/UcumFunctionalTests.xml",
        "tests/official/UcumFunctionalTests.2.xml",
    ];

    for (file_index, test_file_path) in test_files.iter().enumerate() {
        if !Path::new(test_file_path).exists() {
            println!("[DEBUG_LOG] Test file not found: {}", test_file_path);
            continue;
        }

        let test_cases = match parse_xml_test_file(test_file_path) {
            Ok(cases) => cases,
            Err(e) => {
                println!("[DEBUG_LOG] Failed to parse {}: {:?}", test_file_path, e);
                continue;
            }
        };

        println!(
            "[DEBUG_LOG] Processing {} validation tests from file {}",
            test_cases.len(),
            file_index + 1
        );

        for test_case in test_cases {
            if test_case.unit.is_empty() {
                continue;
            }

            let parse_result = parse_expression(&test_case.unit);
            let is_valid = parse_result.is_ok();

            if is_valid == test_case.valid {
                results.add_pass();
            } else {
                let reason = test_case
                    .reason
                    .clone()
                    .unwrap_or_else(|| "No reason provided".to_string());
                let fail_info = format!(
                    "{} - {} (expected: {}, got: {}) - {}",
                    test_case.id, test_case.unit, test_case.valid, is_valid, reason
                );
                results.add_fail(fail_info);
            }
        }
    }

    results
}

fn run_conversion_tests_group() -> TestResults {
    let mut results = TestResults::new();
    let test_files = [
        "tests/official/UcumFunctionalTests.xml",
        "tests/official/UcumFunctionalTests.2.xml",
    ];

    for (file_index, test_file_path) in test_files.iter().enumerate() {
        if !Path::new(test_file_path).exists() {
            continue;
        }

        let test_cases = match parse_conversion_tests(test_file_path) {
            Ok(cases) => cases,
            Err(e) => {
                println!(
                    "[DEBUG_LOG] Failed to parse conversion tests from {}: {:?}",
                    test_file_path, e
                );
                continue;
            }
        };

        println!(
            "[DEBUG_LOG] Processing {} conversion tests from file {}",
            test_cases.len(),
            file_index + 1
        );

        for test_case in test_cases {
            // Parse source unit
            let source_expr = match parse_expression(&test_case.source_unit) {
                Ok(expr) => expr,
                Err(e) => {
                    let fail_info = format!(
                        "{} - Failed to parse source unit '{}': {:?}",
                        test_case.id, test_case.source_unit, e
                    );
                    results.add_fail(fail_info);
                    continue;
                }
            };

            // Parse target unit
            let target_expr = match parse_expression(&test_case.target_unit) {
                Ok(expr) => expr,
                Err(e) => {
                    let fail_info = format!(
                        "{} - Failed to parse target unit '{}': {:?}",
                        test_case.id, test_case.target_unit, e
                    );
                    results.add_fail(fail_info);
                    continue;
                }
            };

            // Evaluate both units
            let source_result = match evaluate_owned(&source_expr) {
                Ok(result) => result,
                Err(e) => {
                    let fail_info = format!(
                        "{} - Failed to evaluate source unit '{}': {:?}",
                        test_case.id, test_case.source_unit, e
                    );
                    results.add_fail(fail_info);
                    continue;
                }
            };

            let target_result = match evaluate_owned(&target_expr) {
                Ok(result) => result,
                Err(e) => {
                    let fail_info = format!(
                        "{} - Failed to evaluate target unit '{}': {:?}",
                        test_case.id, test_case.target_unit, e
                    );
                    results.add_fail(fail_info);
                    continue;
                }
            };

            // Check dimension compatibility
            if source_result.dim != target_result.dim {
                let fail_info = format!(
                    "{} - Dimension mismatch: {} -> {}",
                    test_case.id, test_case.source_unit, test_case.target_unit
                );
                results.add_fail(fail_info);
                continue;
            }

            // Perform conversion
            let conversion_factor = source_result.factor.div(target_result.factor);
            let converted_value = test_case.value.mul(conversion_factor);

            // Check result with tolerance
            let tolerance =
                from_f64(1e-10).mul(test_case.outcome.abs().max(from_f64(1.0)));

            if (converted_value.sub(test_case.outcome)).abs() <= tolerance {
                results.add_pass();
            } else {
                let fail_info = format!(
                    "{} - {} {} -> {}: expected {}, got {} (diff: {})",
                    test_case.id,
                    to_f64(test_case.value),
                    test_case.source_unit,
                    test_case.target_unit,
                    to_f64(test_case.outcome),
                    to_f64(converted_value),
                    to_f64((converted_value.sub(test_case.outcome)).abs())
                );
                results.add_fail(fail_info);
            }
        }
    }

    results
}

fn run_display_name_tests_group() -> TestResults {
    let mut results = TestResults::new();
    let test_files = [
        "tests/official/UcumFunctionalTests.xml",
        "tests/official/UcumFunctionalTests.2.xml",
    ];

    for (file_index, test_file_path) in test_files.iter().enumerate() {
        if !Path::new(test_file_path).exists() {
            continue;
        }

        let test_cases = match parse_display_name_tests(test_file_path) {
            Ok(cases) => cases,
            Err(e) => {
                println!(
                    "[DEBUG_LOG] Failed to parse display name tests from {}: {:?}",
                    test_file_path, e
                );
                continue;
            }
        };

        println!(
            "[DEBUG_LOG] Processing {} display name tests from file {}",
            test_cases.len(),
            file_index + 1
        );

        for test_case in test_cases {
            let parse_result = parse_expression(&test_case.unit);

            match parse_result {
                Ok(expr) => {
                    let generated_display = generate_display_name_owned(&expr);

                    if generated_display == test_case.display {
                        results.add_pass();
                    } else {
                        let fail_info = format!(
                            "{} - '{}' -> '{}' (expected: '{}')",
                            test_case.id, test_case.unit, generated_display, test_case.display
                        );
                        results.add_fail(fail_info);
                    }
                }
                Err(e) => {
                    let fail_info = format!(
                        "{} - '{}' failed to parse: {:?}",
                        test_case.id, test_case.unit, e
                    );
                    results.add_fail(fail_info);
                }
            }
        }
    }

    results
}

fn run_multiplication_tests_group() -> TestResults {
    let mut results = TestResults::new();
    let test_files = [
        "tests/official/UcumFunctionalTests.xml",
        "tests/official/UcumFunctionalTests.2.xml",
    ];

    for (file_index, test_file_path) in test_files.iter().enumerate() {
        if !Path::new(test_file_path).exists() {
            continue;
        }

        let test_cases = match parse_multiplication_tests(test_file_path) {
            Ok(cases) => cases,
            Err(e) => {
                println!(
                    "[DEBUG_LOG] Failed to parse multiplication tests from {}: {:?}",
                    test_file_path, e
                );
                continue;
            }
        };

        println!(
            "[DEBUG_LOG] Processing {} multiplication tests from file {}",
            test_cases.len(),
            file_index + 1
        );

        for test_case in test_cases {
            // Parse and evaluate all units
            let u1_expr = match parse_expression(&test_case.u1) {
                Ok(expr) => expr,
                Err(e) => {
                    let fail_info = format!(
                        "{} - Failed to parse u1 '{}': {:?}",
                        test_case.id, test_case.u1, e
                    );
                    results.add_fail(fail_info);
                    continue;
                }
            };

            let u1_result = match evaluate_owned(&u1_expr) {
                Ok(result) => result,
                Err(e) => {
                    let fail_info = format!(
                        "{} - Failed to evaluate u1 '{}': {:?}",
                        test_case.id, test_case.u1, e
                    );
                    results.add_fail(fail_info);
                    continue;
                }
            };

            let u2_expr = match parse_expression(&test_case.u2) {
                Ok(expr) => expr,
                Err(e) => {
                    let fail_info = format!(
                        "{} - Failed to parse u2 '{}': {:?}",
                        test_case.id, test_case.u2, e
                    );
                    results.add_fail(fail_info);
                    continue;
                }
            };

            let u2_result = match evaluate_owned(&u2_expr) {
                Ok(result) => result,
                Err(e) => {
                    let fail_info = format!(
                        "{} - Failed to evaluate u2 '{}': {:?}",
                        test_case.id, test_case.u2, e
                    );
                    results.add_fail(fail_info);
                    continue;
                }
            };

            let u_res_expr = match parse_expression(&test_case.u_res) {
                Ok(expr) => expr,
                Err(e) => {
                    let fail_info = format!(
                        "{} - Failed to parse u_res '{}': {:?}",
                        test_case.id, test_case.u_res, e
                    );
                    results.add_fail(fail_info);
                    continue;
                }
            };

            let u_res_result = match evaluate_owned(&u_res_expr) {
                Ok(result) => result,
                Err(e) => {
                    let fail_info = format!(
                        "{} - Failed to evaluate u_res '{}': {:?}",
                        test_case.id, test_case.u_res, e
                    );
                    results.add_fail(fail_info);
                    continue;
                }
            };

            // Calculate multiplication
            let calculated_value = test_case.v1.mul(test_case.v2);
            let calculated_factor = u1_result.factor.mul(u2_result.factor);
            let expected_factor = u_res_result.factor.mul(test_case.v_res);

            // Check dimension compatibility
            let mut expected_dim = [0i8; 7];
            for i in 0..7 {
                expected_dim[i] = u1_result.dim.0[i] + u2_result.dim.0[i];
            }

            if expected_dim != u_res_result.dim.0 {
                let fail_info = format!(
                    "{} - Dimension mismatch: {:?} * {:?} = {:?}, expected {:?}",
                    test_case.id,
                    u1_result.dim.0,
                    u2_result.dim.0,
                    expected_dim,
                    u_res_result.dim.0
                );
                results.add_fail(fail_info);
                continue;
            }

            // Check result
            let tolerance = from_f64(1e-10).mul(expected_factor.abs().max(from_f64(1.0)));
            let actual_result = calculated_value.mul(calculated_factor);

            if (actual_result.sub(expected_factor)).abs() <= tolerance {
                results.add_pass();
            } else {
                let fail_info = format!(
                    "{} - {} {} * {} {} = {} {}: expected {}, got {} (diff: {})",
                    test_case.id,
                    to_f64(test_case.v1),
                    test_case.u1,
                    to_f64(test_case.v2),
                    test_case.u2,
                    to_f64(test_case.v_res),
                    test_case.u_res,
                    to_f64(expected_factor),
                    to_f64(actual_result),
                    to_f64((actual_result.sub(expected_factor)).abs())
                );
                results.add_fail(fail_info);
            }
        }
    }

    results
}

fn run_division_tests_group() -> TestResults {
    let mut results = TestResults::new();
    let test_files = [
        "tests/official/UcumFunctionalTests.xml",
        "tests/official/UcumFunctionalTests.2.xml",
    ];

    for (file_index, test_file_path) in test_files.iter().enumerate() {
        if !Path::new(test_file_path).exists() {
            continue;
        }

        let test_cases = match parse_division_tests(test_file_path) {
            Ok(cases) => cases,
            Err(e) => {
                println!(
                    "[DEBUG_LOG] Failed to parse division tests from {}: {:?}",
                    test_file_path, e
                );
                continue;
            }
        };

        println!(
            "[DEBUG_LOG] Processing {} division tests from file {}",
            test_cases.len(),
            file_index + 1
        );

        for test_case in test_cases {
            // Parse and evaluate all units (similar to multiplication but with division logic)
            let u1_expr = match parse_expression(&test_case.u1) {
                Ok(expr) => expr,
                Err(e) => {
                    let fail_info = format!(
                        "{} - Failed to parse u1 '{}': {:?}",
                        test_case.id, test_case.u1, e
                    );
                    results.add_fail(fail_info);
                    continue;
                }
            };

            let u1_result = match evaluate_owned(&u1_expr) {
                Ok(result) => result,
                Err(e) => {
                    let fail_info = format!(
                        "{} - Failed to evaluate u1 '{}': {:?}",
                        test_case.id, test_case.u1, e
                    );
                    results.add_fail(fail_info);
                    continue;
                }
            };

            let u2_expr = match parse_expression(&test_case.u2) {
                Ok(expr) => expr,
                Err(e) => {
                    let fail_info = format!(
                        "{} - Failed to parse u2 '{}': {:?}",
                        test_case.id, test_case.u2, e
                    );
                    results.add_fail(fail_info);
                    continue;
                }
            };

            let u2_result = match evaluate_owned(&u2_expr) {
                Ok(result) => result,
                Err(e) => {
                    let fail_info = format!(
                        "{} - Failed to evaluate u2 '{}': {:?}",
                        test_case.id, test_case.u2, e
                    );
                    results.add_fail(fail_info);
                    continue;
                }
            };

            let u_res_expr = match parse_expression(&test_case.u_res) {
                Ok(expr) => expr,
                Err(e) => {
                    let fail_info = format!(
                        "{} - Failed to parse u_res '{}': {:?}",
                        test_case.id, test_case.u_res, e
                    );
                    results.add_fail(fail_info);
                    continue;
                }
            };

            let u_res_result = match evaluate_owned(&u_res_expr) {
                Ok(result) => result,
                Err(e) => {
                    let fail_info = format!(
                        "{} - Failed to evaluate u_res '{}': {:?}",
                        test_case.id, test_case.u_res, e
                    );
                    results.add_fail(fail_info);
                    continue;
                }
            };

            // Calculate division
            let calculated_value = test_case.v1.div(test_case.v2);
            let calculated_factor = u1_result.factor.div(u2_result.factor);
            let expected_factor = u_res_result.factor.mul(test_case.v_res);

            // Check dimension compatibility
            let mut expected_dim = [0i8; 7];
            for i in 0..7 {
                expected_dim[i] = u1_result.dim.0[i] - u2_result.dim.0[i];
            }

            if expected_dim != u_res_result.dim.0 {
                let fail_info = format!(
                    "{} - Dimension mismatch: {:?} / {:?} = {:?}, expected {:?}",
                    test_case.id,
                    u1_result.dim.0,
                    u2_result.dim.0,
                    expected_dim,
                    u_res_result.dim.0
                );
                results.add_fail(fail_info);
                continue;
            }

            // Check result
            let tolerance = from_f64(1e-10)
                .mul(expected_factor.abs())
                .max(from_f64(1.0));
            let actual_result = calculated_value.mul(calculated_factor);

            if (actual_result.sub(expected_factor)).abs() <= tolerance {
                results.add_pass();
            } else {
                let fail_info = format!(
                    "{} - {} {} / {} {} = {} {}: expected {}, got {} (diff: {})",
                    test_case.id,
                    to_f64(test_case.v1),
                    test_case.u1,
                    to_f64(test_case.v2),
                    test_case.u2,
                    to_f64(test_case.v_res),
                    test_case.u_res,
                    to_f64(expected_factor),
                    to_f64(actual_result),
                    to_f64((actual_result.sub(expected_factor)).abs())
                );
                results.add_fail(fail_info);
            }
        }
    }

    results
}

// Main grouped test runner
#[test]
fn run_official_tests_by_group() {
    println!("[DEBUG_LOG] ========================================");
    println!("[DEBUG_LOG] Running Official UCUM Tests by Group");
    println!("[DEBUG_LOG] ========================================");

    // Run all test groups
    let validation_results = run_validation_tests_group();
    let conversion_results = run_conversion_tests_group();
    let display_name_results = run_display_name_tests_group();
    let multiplication_results = run_multiplication_tests_group();
    let division_results = run_division_tests_group();

    // Print summaries for each group
    println!("\n[DEBUG_LOG] ========================================");
    validation_results.print_summary("validation");

    println!("\n[DEBUG_LOG] ========================================");
    conversion_results.print_summary("conversion");

    println!("\n[DEBUG_LOG] ========================================");
    display_name_results.print_summary("display name");

    println!("\n[DEBUG_LOG] ========================================");
    multiplication_results.print_summary("multiplication");

    println!("\n[DEBUG_LOG] ========================================");
    division_results.print_summary("division");

    // Print overall summary
    let total_passed = validation_results.passed
        + conversion_results.passed
        + display_name_results.passed
        + multiplication_results.passed
        + division_results.passed;
    let total_failed = validation_results.failed
        + conversion_results.failed
        + display_name_results.failed
        + multiplication_results.failed
        + division_results.failed;
    let total_tests = total_passed + total_failed;

    println!("\n[DEBUG_LOG] ========================================");
    println!("[DEBUG_LOG] OVERALL SUMMARY");
    println!("[DEBUG_LOG] ========================================");
    println!("[DEBUG_LOG] Total Passed: {}", total_passed);
    println!("[DEBUG_LOG] Total Failed: {}", total_failed);
    println!("[DEBUG_LOG] Total Tests: {}", total_tests);
    if total_tests > 0 {
        println!(
            "[DEBUG_LOG] Success Rate: {:.1}% ({}/{})",
            (total_passed as f64 / total_tests as f64) * 100.0,
            total_passed,
            total_tests
        );
    }
    println!("[DEBUG_LOG] ========================================");
}
