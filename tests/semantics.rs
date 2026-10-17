//! The semantics of proper units (UCUM §16-20, §28-31): the seven base units, exact decimal
//! factors of any magnitude, case insensitive symbols and the lookup of unit atoms.

use octofhir_ucum::{
    Case, Dimension, ErrorKind, Number, UnitExpr, convert, evaluate, evaluate_with_case,
    find_prefix, find_unit, get_all_prefixes, get_all_units, is_comparable, lookup_unit,
    parse_expression, validate, validate_with_case,
};

fn eval(input: &str) -> octofhir_ucum::EvalResult {
    evaluate(&parse_expression(input).unwrap()).unwrap_or_else(|e| panic!("{input}: {e}"))
}

fn n(text: &str) -> Number {
    Number::parse(text).unwrap()
}

fn assert_close(actual: f64, expected: f64, what: &str) {
    assert!(
        (actual - expected).abs() <= 1e-12 * expected.abs(),
        "{what}: {actual}, expected {expected}"
    );
}

// §28 base units

#[test]
fn the_seven_base_units_of_ucum() {
    assert_eq!(
        Dimension::BASE_UNITS,
        ["m", "s", "g", "rad", "K", "C", "cd"]
    );
    for (slot, code) in Dimension::BASE_UNITS.iter().enumerate() {
        let unit = find_unit(code).unwrap();
        assert!(unit.is_base, "{code}");
        assert_eq!(unit.factor, Number::ONE, "{code}");
        let mut dim = [0; 7];
        dim[slot] = 1;
        assert_eq!(unit.dim, Dimension(dim), "{code}");
    }
    let base: Vec<&str> = get_all_units()
        .iter()
        .filter(|u| u.is_base)
        .map(|u| u.code)
        .collect();
    assert_eq!(base.len(), 7);
}

#[test]
fn the_kilogram_is_a_thousand_base_units() {
    assert_eq!(eval("kg").factor, Number::from(1000));
    assert_eq!(eval("kg").dim, find_unit("g").unwrap().dim);
}

#[test]
fn the_ampere_is_a_coulomb_per_second() {
    let ampere = eval("A");
    assert_eq!(ampere.dim, Dimension([0, -1, 0, 0, 0, 1, 0]));
    assert_eq!(convert(1.0, "A", "C/s").unwrap(), 1.0);
}

#[test]
fn the_radian_is_a_base_unit() {
    // Angles are not dimensionless: a frequency is not an angular velocity
    assert_eq!(eval("rad").dim, Dimension([0, 0, 0, 1, 0, 0, 0]));
    assert!(!is_comparable("Hz", "rad/s").unwrap());
    assert!(!is_comparable("rad", "1").unwrap());
    assert!(!is_comparable("sr", "rad").unwrap());
    assert!(convert(1.0, "deg", "%").is_err());
    assert_eq!(convert(1.0, "sr", "rad2").unwrap(), 1.0);
    assert_close(
        convert(180.0, "deg", "rad").unwrap(),
        core::f64::consts::PI,
        "180 deg",
    );
    assert_eq!(convert(1.0, "gon", "deg").unwrap(), 0.9);
}

#[test]
fn the_mole_is_a_number() {
    // §30: mol is 6.02214076 10*23, dimensionless
    let mole = eval("mol");
    assert!(mole.dim.is_dimensionless());
    assert_eq!(mole.factor, n("6.02214076e23"));
    assert_eq!(convert(1.0, "mol", "10*23").unwrap(), 6.02214076);
    assert!(is_comparable("mol", "1").unwrap());
    assert!(is_comparable("mmol/L", "/L").unwrap());
    assert_close(
        convert(1.0, "mmol/L", "/L").unwrap(),
        6.02214076e20,
        "1 mmol/L",
    );
    assert_eq!(convert(1.0, "kat", "mol/s").unwrap(), 1.0);
}

// Exact decimal factors

#[test]
fn factors_are_exact_decimals() {
    assert_eq!(eval("mg").factor, n("0.001"));
    assert_eq!(eval("ns").factor, n("1e-9"));
    assert_eq!(eval("YL").factor, n("1e21"));
    assert_eq!(eval("%").factor, n("0.01"));
    assert_eq!(eval("[in_i]").factor, n("0.0254"));
    assert_eq!(eval("[ft_i]").factor, n("0.3048"));
    assert_eq!(eval("[lb_av]").factor, n("453.59237"));
}

#[test]
fn conversions_between_decimal_units_are_exact() {
    for (value, from, to, expected) in [
        (1.0, "mg", "g", 0.001),
        (1.0, "uL", "mL", 0.001),
        (1.0, "s", "ns", 1e9),
        (1.0, "[lb_av]", "g", 453.59237),
        (1.0, "[oz_tr]", "g", 31.1034768),
        (250.0, "mg", "g", 0.25),
        (1.0, "[in_i]", "cm", 2.54),
        (1.0, "[ft_i]", "m", 0.3048),
        (1.0, "GiBy", "By", 1073741824.0),
        (6.3, "mm", "m", 0.0063),
        (1.0, "[k]", "J/K", 1.380649e-23),
        (1.0, "eV", "J", 1.602176634e-19),
        (1.0, "[gal_us]", "L", 3.785411784),
        (1.0, "[cup_us]", "mL", 236.5882365),
        (1.0, "[foz_us]", "mL", 29.5735295625),
        (1.0, "[oz_av]", "g", 28.349523125),
        (1.0, "[pt_br]", "L", 0.56826125),
        (1.0, "mo", "d", 30.4375),
        (50.0, "%", "1", 0.5),
        (1.0, "[ppm]", "1", 1e-6),
        (1.0, "b", "m2", 1e-28),
        (1.0, "P", "Pa.s", 0.1),
    ] {
        assert_eq!(
            convert(value, from, to).unwrap(),
            expected,
            "{value} {from} to {to}"
        );
    }
}

#[test]
fn every_prefix_converts_exactly() {
    for base in ["m", "g", "s", "L", "mol", "Pa", "Hz"] {
        for prefix in get_all_prefixes()
            .iter()
            .filter(|p| !p.symbol.ends_with('i') || p.symbol.len() == 1)
        {
            let prefixed = format!("{}{base}", prefix.symbol);
            let factor = prefix.factor.to_f64();
            assert_eq!(convert(1.0, &prefixed, base).unwrap(), factor, "{prefixed}");
            assert_eq!(
                convert(1.0, base, &prefixed).unwrap(),
                Number::ONE.checked_div(prefix.factor).unwrap().to_f64(),
                "{prefixed}"
            );
        }
    }
}

#[test]
fn referenced_factors_apply_once() {
    for (value, from, to, expected) in [
        (1.0, "[ppth]", "1", 1e-3),
        (1.0, "[ppb]", "1", 1e-9),
        (60.0, "'", "deg", 1.0),
        (1.0, "a", "mo", 12.0),
        (1.0, "[ft_us]", "m", 1200.0 / 3937.0),
        (1.0, "[mi_us]", "[ft_us]", 5280.0),
        (1.0, "[psi]", "Pa", 6894.757293168361),
        (1.0, "[den]", "g/km", 1.0 / 9.0),
    ] {
        assert_close(convert(value, from, to).unwrap(), expected, from);
    }
}

// Any magnitude

#[test]
fn factors_of_any_magnitude_are_valid() {
    for input in [
        "[h]", "k[h]", "[m_e]", "[m_p]", "u", "uu", "u2", "pu", "Yt", "10*29", "10*-29", "10*30.m",
        "10*-30", "Tm3", "Ym2", "ym2", "fm3", "[ly]2", "pc3", "mol/[h]", "km10", "[pi]100",
        "m.km10",
    ] {
        assert!(validate(input).is_ok(), "{input}: {:?}", validate(input));
    }
}

#[test]
fn small_factors_keep_their_digits() {
    assert_eq!(eval("[m_e]").factor, n("9.1093837139e-28"));
    assert_eq!(eval("[h]").factor, n("6.62607015e-31"));
    assert_close(
        convert(1.0, "[m_p]", "[m_e]").unwrap(),
        1.672_621_925_95e-24 / 9.109_383_713_9e-28,
        "1 [m_p]",
    );
    assert_close(
        convert(1.0, "1/[ly]", "cm-1").unwrap(),
        1.057_000_834_024_615_5e-18,
        "1/[ly]",
    );
    assert_eq!(convert(1.0, "eV", "kJ").unwrap(), 1.602176634e-22);
}

#[test]
fn a_unit_of_factor_zero_is_not_a_divisor() {
    // `0` is not a number in UCUM, so there is no unit of factor zero
    assert!(validate("m/0").is_err());
}

// §3.4 case insensitive symbols

#[test]
fn case_insensitive_symbols() {
    assert!(validate("PAL").is_err());
    assert!(validate_with_case("PAL", Case::Insensitive).is_ok());
    assert!(validate_with_case("pal", Case::Insensitive).is_ok());
    // In case insensitive symbols MG is the milligram (mega is MA), not the megagauss
    let mg = evaluate_with_case(&parse_expression("MG").unwrap(), Case::Insensitive).unwrap();
    assert_eq!(mg.factor, n("0.001"));
    assert_eq!(mg.dim, find_unit("g").unwrap().dim);
    let mega_gauss = eval("MG");
    assert_ne!(mega_gauss.dim, mg.dim);
    for (ci, cs) in [("[DEGF]", "[degF]"), ("CEL", "Cel"), ("[IN_I]", "[in_i]")] {
        let (prefix, unit) = lookup_unit(ci, Case::Insensitive).unwrap_or_else(|| panic!("{ci}"));
        assert!(prefix.is_none(), "{ci}");
        assert_eq!(unit.code, cs);
    }
    assert!(lookup_unit("[DEGF]", Case::Sensitive).is_none());
}

// Lookup

#[test]
fn find_unit_takes_atoms_only() {
    let meter = find_unit("m").unwrap();
    assert_eq!(meter.factor, Number::ONE);
    assert!(find_unit("mg").is_none());
    assert!(find_unit("k[in_i]").is_none());
    assert!(find_unit("km").is_none());
    for code in ["[pi]", "[in_i]", "[mu_0]", "[ly]"] {
        assert_eq!(find_unit(code).unwrap().code, code);
    }
    assert_eq!(
        find_unit("[pi]").unwrap().factor.to_f64(),
        core::f64::consts::PI
    );
}

#[test]
fn lookup_unit_splits_the_prefix() {
    let (prefix, unit) = lookup_unit("mg", Case::Sensitive).unwrap();
    assert_eq!(prefix.unwrap().symbol, "m");
    assert_eq!(unit.code, "g");
    let (prefix, unit) = lookup_unit("dam", Case::Sensitive).unwrap();
    assert_eq!(prefix.unwrap().symbol, "da");
    assert_eq!(unit.code, "m");
    let (prefix, unit) = lookup_unit("Pa", Case::Sensitive).unwrap();
    assert!(prefix.is_none());
    assert_eq!(unit.code, "Pa");
    assert!(lookup_unit("k[in_i]", Case::Sensitive).is_none());
    assert!(lookup_unit("kh", Case::Sensitive).is_none());
}

#[test]
fn prefix_lookup() {
    assert_eq!(find_prefix("k").unwrap().factor, Number::from(1000));
    assert_eq!(find_prefix("m").unwrap().factor, n("0.001"));
    assert_eq!(find_prefix("Ki").unwrap().factor, Number::from(1024));
    assert_eq!(find_prefix("da").unwrap().symbol_ci, "DA");
    assert!(find_prefix("x").is_none());
    assert_eq!(get_all_prefixes().len(), 24);
}

#[test]
fn names_are_complete() {
    // Character references are resolved, and the first of several names is kept
    assert_eq!(find_unit("A").unwrap().display_name, "ampère");
    assert_eq!(find_unit("Ao").unwrap().display_name, "Ångström");
    assert_eq!(find_unit("[Ch]").unwrap().display_name, "Charrière");
}

#[test]
fn unknown_symbols_are_unit_not_found() {
    let err = evaluate(&UnitExpr::Symbol("xyz".into())).unwrap_err();
    assert!(matches!(err.kind, ErrorKind::UnitNotFound { .. }));
}
