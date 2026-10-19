//! Special units (UCUM §21-23): units on non-ratio scales, each defined by its
//! corresponding proper unit `u`, a function `f` and its inverse, and a scale `α` (a
//! prefix or numeric factors, §22.3-4). A value `r` in the special unit is the proper
//! quantity `f⁻¹(α r) × u`.
//!
//! - §21.2-3: a special unit is commensurable with its corresponding proper unit and
//!   converts to and from it through its function.
//! - §22.1: it takes part in no product, quotient or power.
//! - §22.3-4: it may be scaled by a prefix or a numeric factor, which scales the value.

use octofhir_ucum::{
    AdvancedConversionContext, ErrorKind, SpecialKind, analyse, convert, convert_with_context,
    divide_by, get_canonical_units, is_comparable, multiply, unit_divide, unit_multiply, validate,
};

fn conv(value: f64, from: &str, to: &str) -> f64 {
    convert(value, from, to).unwrap_or_else(|err| panic!("{value} {from} to {to}: {err}"))
}

fn assert_close(actual: f64, expected: f64, what: &str) {
    assert!(
        (actual - expected).abs() <= 1e-12 * expected.abs().max(1.0),
        "{what}: {actual}, expected {expected}"
    );
}

fn assert_special_error<T: std::fmt::Debug>(
    result: Result<T, octofhir_ucum::UcumError>,
    what: &str,
) {
    let err = result.expect_err(what);
    assert!(
        matches!(err.kind, ErrorKind::SpecialUnitError { .. }),
        "{what}: {:?}",
        err.kind
    );
}

mod temperatures {
    //! Interval scales: `cel(1 K)`, `degf(5 K/9)`, `degre(5 K/4)`.
    use super::*;

    #[test]
    fn celsius_and_fahrenheit_are_exact() {
        assert_eq!(conv(100.0, "Cel", "[degF]"), 212.0);
        assert_eq!(conv(32.0, "[degF]", "Cel"), 0.0);
        assert_eq!(conv(212.0, "[degF]", "Cel"), 100.0);
        assert_eq!(conv(0.0, "Cel", "[degF]"), 32.0);
        assert_eq!(conv(-40.0, "Cel", "[degF]"), -40.0);
        assert_close(conv(98.6, "[degF]", "Cel"), 37.0, "98.6 [degF]");
    }

    #[test]
    fn kelvin_rankine_and_reaumur() {
        assert_eq!(conv(0.0, "Cel", "K"), 273.15);
        assert_eq!(conv(273.15, "K", "Cel"), 0.0);
        assert_close(conv(32.0, "[degF]", "K"), 273.15, "32 [degF] to K");
        // `[degR]` is a proper unit, 5/9 K
        assert_close(conv(0.0, "Cel", "[degR]"), 491.67, "0 Cel to [degR]");
        assert_eq!(conv(80.0, "[degRe]", "Cel"), 100.0);
        assert_eq!(conv(1.0, "mK", "K"), 0.001);
    }

    #[test]
    fn a_prefix_scales_the_value() {
        // §22.4: 1000 mCel is 1 Cel, not 1 Cel minus a thousandth of 273.15
        assert_eq!(conv(1000.0, "mCel", "Cel"), 1.0);
        assert_eq!(conv(1.0, "kCel", "K"), 1273.15);
        assert_eq!(conv(300.0, "K", "mCel"), 26850.0);
    }

    #[test]
    fn a_number_scales_the_value() {
        assert_eq!(conv(1.0, "2.Cel", "Cel"), 2.0);
        assert_eq!(conv(1.0, "Cel.2", "Cel"), 2.0);
        assert_eq!(conv(1.0, "Cel/2", "Cel"), 0.5);
        assert_eq!(conv(1.0, "10*3.Cel", "Cel"), 1000.0);
    }

    #[test]
    fn the_reported_factor_and_offset_give_the_value() {
        for (value, from, to) in [
            (100.0, "Cel", "[degF]"),
            (32.0, "[degF]", "Cel"),
            (1.0, "kCel", "K"),
            (5.0, "km", "m"),
        ] {
            let result =
                convert_with_context(value, from, to, &AdvancedConversionContext::default())
                    .unwrap();
            let (factor, offset) = (result.factor.unwrap(), result.offset.unwrap());
            assert_close(
                value * factor + offset,
                result.value,
                &format!("{from} to {to}"),
            );
        }
    }

    #[test]
    fn units_without_offset_are_unchanged() {
        assert_eq!(conv(1.0, "km", "m"), 1000.0);
        assert_close(conv(100.0, "kPa", "mm[Hg]"), 750.0637554192107, "kPa");
    }

    #[test]
    fn canonical_unit_of_a_temperature_is_kelvin_with_an_offset() {
        let canonical = get_canonical_units("Cel").unwrap();
        assert_eq!(canonical.unit, "K");
        assert_eq!((canonical.factor, canonical.offset), (1.0, 273.15));
        let canonical = get_canonical_units("mCel").unwrap();
        assert_eq!((canonical.factor, canonical.offset), (0.001, 273.15));
    }

    #[test]
    fn analysis_reports_the_function_scale_and_offset() {
        let analysis = analyse("Cel").unwrap();
        assert_eq!(analysis.special, Some(SpecialKind::Celsius));
        assert_eq!((analysis.scale, analysis.offset), (1.0, 273.15));
        assert_eq!(analyse("mCel").unwrap().scale, 0.001);
        assert_eq!(analyse("K").unwrap().special, None);
    }
}

mod levels {
    //! Logarithmic scales: `lg`, `2lg`, `ln`, `ld`, `pH`, `hpX`... A conversion between two
    //! levels is linear, a conversion to a proper unit goes through the logarithm.
    use super::*;

    #[test]
    fn a_number_scales_a_level() {
        // `20.dB` is a level of 20 dB, 2 B; not the ratio 10^2
        assert_eq!(conv(1.0, "20.dB", "B"), 2.0);
        assert_eq!(conv(1.0, "20.dB", "dB"), 20.0);
        assert_eq!(conv(1.0, "2.B", "B"), 2.0);
        assert_eq!(conv(1.0, "3.bit_s", "bit_s"), 3.0);
        assert!(validate("2.B[V]").is_ok());
    }

    #[test]
    fn levels_convert_linearly_to_each_other() {
        assert_eq!(conv(20.0, "dB", "B"), 2.0);
        assert_eq!(conv(2.0, "B", "dB"), 20.0);
        assert_close(conv(1.0, "Np", "B"), std::f64::consts::LOG10_E, "1 Np");
        assert_close(conv(1.0, "B", "Np"), std::f64::consts::LN_10, "1 B");
        assert_close(
            conv(1.0, "bit_s", "B"),
            std::f64::consts::LOG10_2,
            "1 bit_s",
        );
        let result =
            convert_with_context(20.0, "dB", "B", &AdvancedConversionContext::default()).unwrap();
        assert_eq!((result.factor, result.offset), (Some(0.1), Some(0.0)));
    }

    #[test]
    fn levels_with_references_convert_with_an_offset() {
        // 1 V is 1000 mV: a level of 2 lg 1000 = 6 B[mV]
        assert_eq!(conv(1.0, "B[V]", "B[mV]"), 7.0);
        assert_eq!(conv(0.0, "B[V]", "dB[uV]"), 120.0);
        assert_eq!(conv(0.0, "B[uV]", "B[10.nV]"), 4.0);
        assert_eq!(conv(0.0, "B[kW]", "B[W]"), 3.0);
        assert_eq!(conv(20.0, "dB[SPL]", "B[SPL]"), 2.0);
    }

    #[test]
    fn homeopathic_potencies_are_levels_of_a_dilution() {
        assert_eq!(conv(1.0, "[hp'_C]", "[hp'_X]"), 2.0);
        assert_eq!(conv(1.0, "[hp'_M]", "[hp'_X]"), 3.0);
        assert_close(conv(1.0, "[hp'_Q]", "[hp'_X]"), 50000f64.log10(), "1 Q");
        assert_eq!(conv(6.0, "[hp'_X]", "[hp'_C]"), 3.0);
        assert_eq!(conv(1.0, "[hp'_X]", "B"), -1.0);
        assert_eq!(conv(-20.0, "dB", "[hp'_X]"), 2.0);
        // `3 [hp'_C]` is a dilution of 100^-3
        assert_close(conv(3.0, "[hp'_C]", "1"), 1e-6, "3 C");
    }

    #[test]
    fn levels_convert_to_their_proper_units() {
        assert_close(conv(7.0, "[pH]", "mol/L"), 1e-7, "pH 7");
        assert_close(conv(1e-7, "mol/L", "[pH]"), 7.0, "1e-7 mol/L");
        assert_close(conv(20.0, "dB[SPL]", "Pa"), 2e-4, "20 dB[SPL]");
        assert_close(conv(0.0, "B[V]", "V"), 1.0, "0 B[V]");
        assert_close(conv(1.0, "B[W]", "W"), 10.0, "1 B[W]");
        assert_close(conv(1.0, "B", "1"), 10.0, "1 B");
        assert_close(conv(100.0, "1", "B"), 2.0, "100");
        assert_close(conv(3.0, "bit_s", "1"), 8.0, "3 bit_s");
        assert_close(conv(1.0, "Np", "1"), std::f64::consts::E, "1 Np");
        // UCUM §45: pH 7.4 is about 23975 protons per picoliter
        assert_close(conv(7.4, "[pH]", "/pL"), 23974.574186384853, "pH 7.4");
    }

    #[test]
    fn a_conversion_through_a_function_is_not_linear() {
        let result =
            convert_with_context(1.0, "B", "1", &AdvancedConversionContext::default()).unwrap();
        assert_eq!((result.factor, result.offset), (None, None));
        assert!(result.used_special_units);
    }

    #[test]
    fn levels_have_no_canonical_unit() {
        for code in ["B", "dB[SPL]", "[pH]", "Np", "bit_s", "[hp'_X]", "[p'diop]"] {
            assert_special_error(get_canonical_units(code), code);
        }
    }

    #[test]
    fn levels_are_commensurable_with_their_proper_units_only() {
        assert!(is_comparable("B", "1").unwrap());
        assert!(is_comparable("B[V]", "mV").unwrap());
        assert!(is_comparable("[pH]", "mol/L").unwrap());
        assert!(is_comparable("dB[SPL]", "Pa").unwrap());
        assert!(!is_comparable("B[V]", "B").unwrap());
        assert!(!is_comparable("[pH]", "1").unwrap());
        assert!(convert(1.0, "B[V]", "B").is_err());
    }

    #[test]
    fn analysis_of_a_level() {
        let analysis = analyse("dB").unwrap();
        assert_eq!(analysis.special, Some(SpecialKind::Lg));
        assert_eq!(analysis.scale, 0.1);
        assert!(analysis.is_dimensionless);
        assert_eq!(analyse("[pH]").unwrap().special, Some(SpecialKind::PH));
    }
}

mod tangents {
    //! `100tan(1 rad)`: the prism diopter and the percent of slope.
    use super::*;

    #[test]
    fn slope_and_prism_diopter_are_the_same_scale() {
        assert_eq!(conv(1.0, "%[slope]", "[p'diop]"), 1.0);
        assert_eq!(conv(12.5, "[p'diop]", "%[slope]"), 12.5);
        assert_eq!(conv(1.0, "100.[p'diop]", "[p'diop]"), 100.0);
    }

    #[test]
    fn tangents_convert_to_angles() {
        // UCUM §44: the definition is in radians, although the XML <function> says deg
        assert_close(
            conv(100.0, "[p'diop]", "rad"),
            std::f64::consts::FRAC_PI_4,
            "100 [p'diop]",
        );
        assert_close(conv(100.0, "%[slope]", "deg"), 45.0, "100 %[slope]");
        assert_close(conv(45.0, "deg", "%[slope]"), 100.0, "45 deg");
    }

    #[test]
    fn tangents_are_angles_not_numbers() {
        assert!(!is_comparable("%[slope]", "%").unwrap());
        assert!(!is_comparable("%[slope]", "1").unwrap());
        assert!(convert(1.0, "%[slope]", "%").is_err());
    }
}

mod square_root {
    //! `[m/s2/Hz^(1/2)]` is `sqrt(1 m2/s4/Hz)` (UCUM §45).
    use super::*;

    const ASD: &str = "[m/s2/Hz^(1/2)]";

    #[test]
    fn converts_through_the_square_root() {
        assert_eq!(conv(3.0, ASD, "m2/s4/Hz"), 9.0);
        assert_eq!(conv(9.0, "m2/s4/Hz", ASD), 3.0);
        assert_eq!(conv(2.0, ASD, ASD), 2.0);
        assert_eq!(analyse(ASD).unwrap().special, Some(SpecialKind::Sqrt));
    }

    #[test]
    fn is_not_a_number() {
        // These used to convert 1:1, or 1:100 for %
        assert!(convert(1.0, ASD, "1").is_err());
        assert!(convert(1.0, ASD, "%").is_err());
        assert!(!is_comparable(ASD, "1").unwrap());
        assert!(is_comparable(ASD, "m2.s-3").unwrap());
    }
}

mod algebra {
    //! §22.1: special units take part in no product, quotient or power, but they can be
    //! scaled by a number (§22.3).
    use super::*;

    #[test]
    fn special_units_take_no_part_in_terms() {
        for code in [
            "B2",
            "/dB",
            "1/B",
            "dB/s",
            "Np.s",
            "dB.dB",
            "[p'diop]2",
            "%[slope].m",
            "[hp'_X]/L",
            "Cel/h",
            "Cel2",
            "/Cel",
            "[degF]/s",
            "B[V].m",
            "[pH]/2.m",
            "[m/s2/Hz^(1/2)]2",
            "[m/s2/Hz^(1/2)].s",
        ] {
            assert_special_error(validate(code), code);
        }
    }

    #[test]
    fn a_numeric_scale_is_allowed() {
        for code in [
            "2.Cel", "Cel.2", "10*3.Cel", "Cel/2", "20.dB", "2.[pH]", "Cel{x}",
        ] {
            assert!(validate(code).is_ok(), "{code}");
        }
    }

    #[test]
    fn special_units_take_no_part_in_arithmetic() {
        for code in ["Cel", "B", "[pH]", "%[slope]", "[hp'_X]", "[m/s2/Hz^(1/2)]"] {
            assert_special_error(unit_multiply(code, "m"), code);
            assert_special_error(unit_divide("m", code), code);
            assert_special_error(multiply(1.0, code, 2.0, "m"), code);
            assert_special_error(divide_by(1.0, code, 2.0, "m"), code);
        }
    }

    #[test]
    fn prefixes_go_on_metric_special_units_only() {
        for code in ["mCel", "kCel", "dB", "mB[V]", "cNp"] {
            assert!(validate(code).is_ok(), "{code}");
        }
        for code in ["k[degF]", "kbit_s", "m[pH]", "k[hp'_X]", "k[p'diop]"] {
            assert!(validate(code).is_err(), "{code}");
        }
    }
}

mod non_finite {
    use super::*;

    fn assert_error(value: f64, from: &str, to: &str, expected: fn(&ErrorKind) -> bool) {
        let err = convert(value, from, to).expect_err(from);
        assert!(
            expected(&err.kind),
            "{value} {from} to {to}: {:?}",
            err.kind
        );
    }

    fn assert_non_finite(value: f64, from: &str, to: &str) {
        assert_error(value, from, to, |kind| {
            matches!(kind, ErrorKind::NonFiniteValue { .. })
        });
    }

    fn assert_overflow(value: f64, from: &str, to: &str) {
        assert_error(value, from, to, |kind| {
            matches!(kind, ErrorKind::PrecisionOverflow { .. })
        });
    }

    #[test]
    fn non_finite_values_are_errors() {
        for (from, to) in [("Cel", "K"), ("B", "1"), ("dB", "B"), ("[pH]", "mol/L")] {
            assert_non_finite(f64::NAN, from, to);
            assert_non_finite(f64::INFINITY, from, to);
            assert_non_finite(f64::NEG_INFINITY, from, to);
        }
    }

    #[test]
    fn an_infinite_value_through_a_function_is_an_error() {
        // 10^-∞ would be 0 mol/L
        assert_non_finite(f64::INFINITY, "[pH]", "mol/L");
    }

    #[test]
    fn a_level_beyond_the_f64_range_is_an_error() {
        assert_overflow(400.0, "B", "1");
        assert_overflow(1e5, "B", "1");
    }

    #[test]
    fn a_level_below_the_f64_range_is_an_error() {
        // 10^-400 mol/L is not zero, and no f64 is closer to it than zero
        assert_overflow(400.0, "[pH]", "mol/L");
        assert_overflow(-400.0, "B", "1");
    }
}

mod domain {
    //! §21.2: `f` and `f⁻¹` are mutually inverse real functions. A value outside the domain
    //! of the function has no value in the special unit, or in its corresponding unit.
    use super::*;

    const ASD: &str = "[m/s2/Hz^(1/2)]";

    fn assert_out_of_domain(value: f64, from: &str, to: &str, special: &str) {
        let err = convert(value, from, to).expect_err(from);
        match &err.kind {
            ErrorKind::OutOfDomain { unit, .. } => {
                assert_eq!(unit, special, "{value} {from} to {to}")
            }
            kind => panic!("{value} {from} to {to}: {kind:?}"),
        }
    }

    #[test]
    fn a_logarithm_takes_positive_values() {
        assert_out_of_domain(0.0, "mol/L", "[pH]", "[pH]");
        assert_out_of_domain(-1e-7, "mol/L", "[pH]", "[pH]");
        assert_out_of_domain(0.0, "W", "B[W]", "B[W]");
        assert_out_of_domain(-1.0, "W", "B[W]", "B[W]");
        assert_out_of_domain(-1.0, "mV", "dB[mV]", "dB[mV]");
        assert_out_of_domain(0.0, "1", "Np", "Np");
        assert_out_of_domain(0.0, "1", "[hp'_X]", "[hp'_X]");
        assert_out_of_domain(-4.0, "1", "[hp'_X]", "[hp'_X]");
        // Any level is a value: it maps to a positive ratio
        assert_close(conv(-3.0, "B", "1"), 1e-3, "-3 B");
        assert_eq!(conv(1.0, "1", "B"), 0.0);
    }

    #[test]
    fn a_square_root_is_never_negative() {
        assert_out_of_domain(-1.0, "m2/s4/Hz", ASD, ASD);
        assert_out_of_domain(-3.0, ASD, "m2/s4/Hz", ASD);
        assert_eq!(conv(0.0, "m2/s4/Hz", ASD), 0.0);
        assert_eq!(conv(0.0, ASD, "m2/s4/Hz"), 0.0);
    }

    #[test]
    fn a_tangent_takes_its_principal_branch() {
        // Beyond a right angle, 100 tan x is not invertible: 2 rad would be -218 %[slope],
        // which is -1.14 rad
        assert_out_of_domain(2.0, "rad", "%[slope]", "%[slope]");
        assert_out_of_domain(-2.0, "rad", "[p'diop]", "[p'diop]");
        assert_out_of_domain(100.0, "deg", "%[slope]", "%[slope]");
        assert_close(conv(-45.0, "deg", "%[slope]"), -100.0, "-45 deg");
        assert_eq!(conv(0.0, "rad", "%[slope]"), 0.0);
        assert!(conv(1.57, "rad", "%[slope]") > 1e5);
        // Any value in the special unit is an angle of the principal branch
        assert_close(
            conv(-1e300, "%[slope]", "rad"),
            -core::f64::consts::FRAC_PI_2,
            "-1e300",
        );
    }

    #[test]
    fn a_prefix_or_a_factor_does_not_change_the_domain() {
        assert_out_of_domain(0.0, "W", "dB[W]", "dB[W]");
        assert_out_of_domain(0.0, "mol/L", "2.[pH]", "2.[pH]");
    }

    #[test]
    fn an_interval_scale_has_no_limit() {
        // The functions of the temperatures are defined on all real numbers
        assert_close(conv(-300.0, "Cel", "K"), -26.85, "-300 Cel");
    }

    #[test]
    fn the_error_names_the_value_and_the_special_unit() {
        let err = convert(0.0, "mol/L", "[pH]").unwrap_err();
        let message = err.to_string();
        assert!(message.contains("0.0 mol/L"), "{message}");
        assert!(message.contains("[pH]"), "{message}");
        assert!(message.contains("§21.2"), "{message}");
    }
}

#[cfg(feature = "fhir")]
mod fhir {
    use super::assert_close;
    use octofhir_ucum::fhir::{FhirQuantity, are_equivalent, convert_quantity};

    #[test]
    fn convert_quantity_applies_the_functions() {
        let celsius = FhirQuantity::with_ucum_code(37.0, "Cel");
        assert_close(
            convert_quantity(&celsius, "[degF]").unwrap().value,
            98.6,
            "37 Cel",
        );
        assert_close(
            convert_quantity(&celsius, "K").unwrap().value,
            310.15,
            "37 Cel",
        );
        let ph = FhirQuantity::with_ucum_code(7.0, "[pH]");
        assert_close(convert_quantity(&ph, "mol/L").unwrap().value, 1e-7, "pH 7");
    }

    #[test]
    fn are_equivalent_applies_the_functions() {
        let celsius = FhirQuantity::with_ucum_code(0.0, "Cel");
        let kelvin = FhirQuantity::with_ucum_code(273.15, "K");
        let fahrenheit = FhirQuantity::with_ucum_code(32.0, "[degF]");
        assert!(are_equivalent(&celsius, &kelvin).unwrap());
        assert!(are_equivalent(&kelvin, &celsius).unwrap());
        assert!(are_equivalent(&fahrenheit, &celsius).unwrap());
        let zero_kelvin = FhirQuantity::with_ucum_code(0.0, "K");
        assert!(!are_equivalent(&celsius, &zero_kelvin).unwrap());
        let level = FhirQuantity::with_ucum_code(20.0, "dB");
        let ratio = FhirQuantity::with_ucum_code(100.0, "1");
        assert!(are_equivalent(&level, &ratio).unwrap());
    }
}

/// `is_comparable` agrees with `convert` on every pair (UCUM §17, §21.3, §25).
#[test]
fn commensurability_agrees_with_conversion() {
    let units = [
        "m",
        "km",
        "[in_i]",
        "g",
        "s",
        "K",
        "Cel",
        "[degF]",
        "mK",
        "B",
        "dB",
        "Np",
        "bit_s",
        "1",
        "%",
        "B[V]",
        "V",
        "mV",
        "B[SPL]",
        "Pa",
        "[pH]",
        "mol/L",
        "[hp'_X]",
        "[p'diop]",
        "%[slope]",
        "rad",
        "deg",
        "[m/s2/Hz^(1/2)]",
        "m2/s4/Hz",
        "[IU]",
        "[iU]",
        "[IU]/mL",
        "[IU]/L",
        "[arb'U]",
        "mol",
        "10*23",
    ];
    for from in units {
        for to in units {
            let comparable = is_comparable(from, to).unwrap();
            let converted = convert(1.0, from, to);
            assert_eq!(
                comparable,
                converted.is_ok(),
                "{from} to {to}: comparable {comparable}, conversion {converted:?}"
            );
        }
    }
}
