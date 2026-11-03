use octofhir_ucum::{convert, get_all_units};

fn main() {
    let targets = [
        "1", "m", "s", "g", "K", "rad", "C", "cd", "Pa", "m/s", "mol/L", "m-1", "Cel", "[degF]",
        "[pH]", "%[slope]",
    ];
    for unit in get_all_units() {
        for target in targets {
            for value in [0.0_f64, 1.0, 37.0, -2.0, 1e9] {
                match convert(value, unit.code, target) {
                    Ok(result) => println!(
                        "{}|{target}|{}|{}",
                        unit.code,
                        value.to_bits(),
                        result.to_bits()
                    ),
                    Err(error) => println!(
                        "{}|{target}|{}|{}",
                        unit.code,
                        value.to_bits(),
                        error.kind.code()
                    ),
                }
            }
        }
    }
}
