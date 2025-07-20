use octofhir_ucum_core::{EvalResult, UnitExpr, UnitFactor, evaluate};

fn eval_ratio(expr: UnitExpr) -> f64 {
    evaluate(&expr).unwrap().factor
}

fn eval_expr(expr: UnitExpr) -> EvalResult {
    evaluate(&expr).unwrap()
}

fn product_expr(value: f64, unit: &str) -> UnitExpr {
    UnitExpr::Product(vec![
        UnitFactor {
            expr: UnitExpr::Numeric(value),
            exponent: 1,
        },
        UnitFactor {
            expr: UnitExpr::Symbol(unit.into()),
            exponent: 1,
        },
    ])
}

#[test]
fn test_decibel() {
    // 20 dB should equal ratio 10^2 = 100
    let expr = UnitExpr::Product(vec![
        UnitFactor {
            expr: UnitExpr::Numeric(20.0),
            exponent: 1,
        },
        UnitFactor {
            expr: UnitExpr::Symbol("dB".into()),
            exponent: 1,
        },
    ]);
    let r = eval_ratio(expr);
    assert!((r - 100.0).abs() < 1e-6);
}

#[test]
fn test_neper() {
    // 1 Np == e^1
    let expr = UnitExpr::Product(vec![
        UnitFactor {
            expr: UnitExpr::Numeric(1.0),
            exponent: 1,
        },
        UnitFactor {
            expr: UnitExpr::Symbol("Np".into()),
            exponent: 1,
        },
    ]);
    let r = eval_ratio(expr);
    assert!((r - std::f64::consts::E).abs() < 1e-12);
}

#[test]
fn test_prism_diopter() {
    // 100 [p'diop] == tan(1 rad) ≈ 1.5574
    let expr = UnitExpr::Product(vec![
        UnitFactor {
            expr: UnitExpr::Numeric(100.0),
            exponent: 1,
        },
        UnitFactor {
            expr: UnitExpr::Symbol("[p'diop]".into()),
            exponent: 1,
        },
    ]);
    let r = eval_ratio(expr);
    let expected = 1.0_f64.tan();
    assert!((r - expected).abs() < 1e-6);
}

#[test]
fn test_decibel_variations() {
    // Test different values
    let test_cases = [
        (0.0, 1.0),           // 0 dB = 10^0 = 1
        (10.0, 10.0),         // 10 dB = 10^1 = 10
        (20.0, 100.0),        // 20 dB = 10^2 = 100
        (30.0, 1000.0),       // 30 dB = 10^3 = 1000
        (-10.0, 0.1),         // -10 dB = 10^-1 = 0.1
        (3.0, 2.0f64.sqrt()), // 3 dB ≈ 1.4142 (√2)
    ];

    for (db, expected) in test_cases.iter() {
        let expr = product_expr(*db, "dB");
        let r = eval_ratio(expr);
        assert!(
            (r - expected).abs() < 1e-6,
            "{} dB: expected {}, got {}",
            db,
            expected,
            r
        );
    }
}

#[test]
fn test_neper_variations() {
    // Test different values
    let test_cases = [
        (0.0, 1.0),                                       // 0 Np = e^0 = 1
        (1.0, std::f64::consts::E),                       // 1 Np = e^1 = e
        (2.0, std::f64::consts::E * std::f64::consts::E), // 2 Np = e^2
        (-1.0, 1.0 / std::f64::consts::E),                // -1 Np = e^-1
    ];

    for (np, expected) in test_cases.iter() {
        let expr = product_expr(*np, "Np");
        let r = eval_ratio(expr);
        assert!(
            (r - expected).abs() < 1e-12,
            "{} Np: expected {}, got {}",
            np,
            expected,
            r
        );
    }
}

#[test]
fn test_prism_diopter_variations() {
    // Test different values
    let test_cases = [
        (0.0, 0.0),                                      // 0 [p'diop] = tan(0) = 0
        (100.0, 1.0_f64.tan()),                          // 100 [p'diop] = tan(1 radian)
        (200.0, 2.0_f64.tan()),                          // 200 [p'diop] = tan(2 radians)
        (-100.0, (-1.0_f64).tan()),                      // -100 [p'diop] = tan(-1 radian)
        (314.1592653589793, std::f64::consts::PI.tan()), // 100π [p'diop] ≈ tan(π)
    ];

    for (pd, expected) in test_cases.iter() {
        let expr = product_expr(*pd, "[p'diop]");
        let r = eval_ratio(expr);
        assert!(
            (r - expected).abs() < 1e-6,
            "{} [p'diop]: expected {}, got {}",
            pd,
            expected,
            r
        );
    }
}

#[test]
fn test_special_unit_prefixes() {
    // Test that prefixes work with special units
    let test_cases = [
        ("dB", 10.0, 10.0),                 // 10 dB = 10^1 = 10
        ("B", 1.0, 10.0),                   // 1 B = 10^1 = 10 (Bel is base unit)
        ("dB", 20.0, 100.0),                // 20 dB = 10^2 = 100
        ("B", 2.0, 100.0),                  // 2 B = 10^2 = 100
        ("dNp", 10.0, std::f64::consts::E), // 10 dNp = 1 Np = e^1
    ];

    for (unit, value, expected) in test_cases.iter() {
        let expr = product_expr(*value, unit);
        let r = eval_ratio(expr);
        assert!(
            (r - expected).abs() < 1e-6,
            "{} {}: expected {}, got {}",
            value,
            unit,
            expected,
            r
        );
    }
}

#[test]
fn test_special_unit_combinations() {
    // Test combining special units with other units
    let expr = UnitExpr::Product(vec![
        UnitFactor {
            expr: UnitExpr::Numeric(10.0),
            exponent: 1,
        },
        UnitFactor {
            expr: UnitExpr::Symbol("dB".into()),
            exponent: 1,
        },
        UnitFactor {
            expr: UnitExpr::Symbol("m".into()),
            exponent: -1,
        },
    ]);

    println!("Testing expression: 10 dB/m");
    let result = eval_expr(expr);
    println!(
        "Result: factor={}, dim={:?}, offset={}",
        result.factor, result.dim.0, result.offset
    );

    // 10 dB/m should have a factor of 10 (from dB) and dimension of m^-1
    println!("Checking factor: {} (expected ~10.0)", result.factor);
    assert!(
        (result.factor - 10.0).abs() < 1e-6,
        "Expected factor ~10.0, got {}",
        result.factor
    );

    // Dimension order is [M, L, T, I, Θ, N, J], so length (L) is at index 1
    println!(
        "Checking dimension: {:?} (expected [0, -1, 0, 0, 0, 0, 0])",
        result.dim.0
    );
    assert_eq!(
        result.dim.0[1], -1,
        "Length dimension should be -1 (m^-1) at index 1"
    );
    assert_eq!(result.offset, 0.0, "Offset should be 0.0");
}

#[test]
fn test_zero_special_units() {
    // Test that special units work with zero
    // Note: [p'diop] is a special case where tan(0) = 0
    let test_cases = [
        ("dB", 1.0),
        ("Np", 1.0),
        ("[p'diop]", 0.0), // tan(0) = 0
    ];

    for (unit, expected) in test_cases.iter() {
        let expr = product_expr(0.0, unit);
        let r = eval_ratio(expr);
        assert!(
            (r - expected).abs() < 1e-12,
            "0 {}: expected {}, got {}",
            unit,
            expected,
            r
        );
    }
}
