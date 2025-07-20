use ucum_core::{Dimension, find_prefix, find_unit};

#[test]
fn prefix_lookup() {
    let k = find_prefix("k").expect("kilo prefix");
    assert_eq!(k.factor, 1e3);
    let m = find_prefix("m").expect("milli prefix");
    assert_eq!(m.factor, 1e-3);
}

#[test]
fn base_unit_lookup() {
    let meter = find_unit("m").expect("meter");
    assert_eq!(meter.factor, 1.0);
    assert_eq!(meter.dim, Dimension([0, 1, 0, 0, 0, 0, 0]));
}

#[test]
fn percent_unit() {
    let pct = find_unit("%").expect("percent");
    assert!((pct.factor - 1e-2).abs() < 1e-12);
}
