#![no_main]

use libfuzzer_sys::fuzz_target;
use octofhir_ucum_core::{evaluate, parse_expression};

// List of valid UCUM units to use as building blocks
const VALID_UNITS: &[&str] = &[
    "m", "g", "s", "A", "K", "mol", "cd", "rad", "sr",
    "Hz", "N", "Pa", "J", "W", "C", "V", "F", "Ohm", "S",
    "Wb", "T", "H", "lm", "lx", "Bq", "Gy", "Sv", "kat",
    "l", "L", "ar", "min", "h", "d", "a", "wk", "mo", "y",
    "deg", "'", "\"", "gon", "g", "t", "u", "eV", "pc", "in",
    "ft", "[in_i]", "[ft_i]", "[yd_i]", "[mi_i]", "[fth_i]",
    "[nmi_i]", "[kn_i]", "[sin_i]", "[sft_i]", "[syd_i]",
    "[cin_i]", "[cft_i]", "[cyd_i]", "[Btu_i]", "[hp_i]",
    "[oz_av]", "[lb_av]", "[dr_av]", "[gr]", "[scwt_av]",
    "[lcwt_av]", "[ston_av]", "[lton_av]", "[stone_av]",
    "[pwt_tr]", "[oz_tr]", "[lb_tr]", "[sc_ap]", "[dr_ap]",
    "[min_ap]", "[fl_oz_us]", "[fl_dr_us]", "[min_us]",
    "[pt_us]", "[qt_us]", "[gal_us]", "[bbl_us]", "[pt_br]",
    "[qt_br]", "[gal_br]", "[pk_br]", "[bu_br]", "[gal_wi]",
    "[pk_us]", "[bu_us]", "[tsp_us]", "[tbs_us]", "[cup_us]",
    "[foz_br]", "[gil_br]", "[fdr_br]", "[min_br]", "[crd_us]",
    "[bd_i]", "[ch_us]", "[rd_us]", "[fur_us]", "[mi_us]",
    "[acr_us]", "[sct]", "[cml_i]", "[hnsf_i]", "[p_i]",
    "[diop]", "[prism]", "[pica_i]", "[pnt_i]", "[pca_pr]",
    "[pnt_pr]", "[pied_pr]", "[pouce_pr]", "[ligne_pr]",
    "[didot_pr]", "[cicero_pr]", "[degF]", "[degR]", "[degRe]",
    "[cal_15]", "[cal_20]", "[cal_m]", "[cal_IT]", "[cal_th]",
    "[cal]", "[Btu_39]", "[Btu_59]", "[Btu_60]", "[Btu_m]",
    "[Btu_IT]", "[Btu_th]", "[Btu]", "[HP]", "[tex]", "[den]",
    "[eq]", "[osm]", "[g%]", "[g/l]", "[S]", "[mho]", "[G]",
    "[Mx]", "[gb]", "[sb]", "[Oe]", "[Gb]", "[Wb/m2]", "[At]",
    "[St]", "[P]", "[Gs]", "[Gy]", "[RAD]", "[REM]", "[Sv]",
    "[Ci]", "[Ro]", "[RAD]", "[rem]", "[[Bq]]", "[Rd]", "[Rn]",
    "[Bi]", "[Hg]", "[atm]", "[bar]", "[mbar]", "[Torr]",
    "[mm Hg]", "[in_i'H2O]", "[in_i'Hg]", "[PRU]", "[Wood]",
    "[dyn.s/cm5]", "[USP]", "[IU]", "[arb'U]", "[U]", "[iU]",
    "[IU/l]", "[GPL]", "[MPL]", "[APL]", "[beth]", "[AU]",
    "[hnsf]", "[ly]", "[pc]", "[gf]", "[lbf]", "[kip]",
    "[dyn]", "[pond]", "[GLM]", "[MPL]", "[GPL]", "[APL]",
    "[%]", "[ppth]", "[ppm]", "[ppb]", "[pptr]", "[mol/m3]",
    "[eq/l]", "[osm/l]", "[pH]", "[g/dl]", "[S/m]", "[C/m2]",
    "[mho/m]", "[H/m]", "[A/m]", "[cd/m2]", "[lm/m2]", "[lx]",
    "[Np]", "[B]", "[dB]", "[st]", "[Ao]", "[b]", "[att]",
    "[mho]", "[Ci]", "[R]", "[RAD]", "[REM]", "[Sv]", "[Gy]",
    "[St]", "[P]", "[Gs]", "[Wb]", "[T]", "[G]", "[Oe]",
    "[Mx]", "[gb]", "[sb]", "[Gb]", "[At]", "[Bi]", "[Hg]",
    "[Rd]", "[Rn]", "[rem]", "[[Bq]]", "[Ro]", "[atm]",
    "[bar]", "[mbar]", "[Torr]", "[mm Hg]", "[in_i'H2O]",
    "[in_i'Hg]", "[PRU]", "[Wood]", "[dyn.s/cm5]", "[USP]",
    "[IU]", "[arb'U]", "[U]", "[iU]", "[IU/l]", "[GPL]",
    "[MPL]", "[APL]", "[beth]", "[AU]", "[hnsf]", "[ly]",
    "[pc]", "[gf]", "[lbf]", "[kip]", "[dyn]", "[pond]",
    "[GLM]", "[MPL]", "[GPL]", "[APL]", "[%]", "[ppth]",
    "[ppm]", "[ppb]", "[pptr]", "[mol/m3]", "[eq/l]",
    "[osm/l]", "[pH]", "[g/dl]", "[S/m]", "[C/m2]", "[mho/m]",
    "[H/m]", "[A/m]", "[cd/m2]", "[lm/m2]", "[lx]", "[Np]",
    "[B]", "[dB]", "[st]", "[Ao]", "[b]", "[att]", "[mho]",
];

// List of valid operators and modifiers
const OPERATORS: &[&str] = &[
    ".", "/", "^", "(", ")", "{", "}", "[", "]", "10*", "10^",
];

// List of valid prefixes
const PREFIXES: &[&str] = &[
    "Y", "Z", "E", "P", "T", "G", "M", "k", "h", "da", "d", "c", "m", "u", "n", "p", "f", "a", "z", "y",
];

fuzz_target!(|data: &[u8]| {
    // Convert the byte array to a string if possible
    if let Ok(s) = std::str::from_utf8(data) {
        // Only fuzz if the string is not too long to avoid excessive resource usage
        if s.len() < 100 {
            // Try to parse the expression
            if let Ok(expr) = parse_expression(s) {
                // If parsing succeeds, try to evaluate the expression
                let _ = evaluate(&expr);
            } else {
                // If parsing fails, try to generate a valid expression
                // by combining valid units, operators, and prefixes
                let mut valid_expr = String::new();

                // Use the input data to select elements from our valid lists
                let mut bytes = data.iter().cycle();

                // Add 1-3 terms to the expression
                for _ in 0..((data.len() % 3) + 1) {
                    // Maybe add a prefix
                    if let Some(&b) = bytes.next() {
                        if b % 3 == 0 {
                            let prefix_idx = (b as usize) % PREFIXES.len();
                            valid_expr.push_str(PREFIXES[prefix_idx]);
                        }
                    }

                    // Add a unit
                    if let Some(&b) = bytes.next() {
                        let unit_idx = (b as usize) % VALID_UNITS.len();
                        valid_expr.push_str(VALID_UNITS[unit_idx]);
                    }

                    // Maybe add an operator
                    if let Some(&b) = bytes.next() {
                        if b % 4 != 0 && !valid_expr.is_empty() {
                            let op_idx = (b as usize) % OPERATORS.len();
                            valid_expr.push_str(OPERATORS[op_idx]);
                        }
                    }
                }

                // Try to parse and evaluate the generated expression
                if !valid_expr.is_empty() {
                    if let Ok(expr) = parse_expression(&valid_expr) {
                        let _ = evaluate(&expr);
                    }
                }
            }
        }
    }
});
