#![no_main]

use libfuzzer_sys::fuzz_target;
use octofhir_ucum_core::{parse_expression, evaluate};
use octofhir_ucum_fhir::{FhirQuantity, convert_quantity, are_equivalent, ToFhirQuantity, FromFhirQuantity};

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

// List of valid prefixes
const PREFIXES: &[&str] = &[
    "Y", "Z", "E", "P", "T", "G", "M", "k", "h", "da", "d", "c", "m", "u", "n", "p", "f", "a", "z", "y",
];

// List of arbitrary units
const ARBITRARY_UNITS: &[&str] = &[
    "[IU]", "[arb'U]", "[USP]", "[U]", "[iU]", "[GPL]", "[MPL]", "[APL]", "[beth]",
];

fuzz_target!(|data: &[u8]| {
    // Convert the byte array to a string if possible
    if let Ok(s) = std::str::from_utf8(data) {
        // Only fuzz if the string is not too long to avoid excessive resource usage
        if s.len() < 100 {
            // Try to use the input as a UCUM code
            let code = if s.is_empty() {
                // If the input is empty, use a random valid unit
                let idx = (data.get(0).unwrap_or(&0) % VALID_UNITS.len() as u8) as usize;
                VALID_UNITS[idx]
            } else {
                s
            };

            // Create a FhirQuantity with the code
            let value = data.get(1).map(|&b| b as f64).unwrap_or(1.0);
            let quantity = FhirQuantity::with_ucum_code(value, code);

            // Test to_ucum_quantity
            if let Ok(ucum_quantity) = quantity.to_ucum_quantity() {
                // Test to_fhir_quantity
                let _ = ucum_quantity.to_fhir_quantity();
            }

            // Test are_equivalent with the same quantity
            let _ = are_equivalent(&quantity, &quantity);

            // Test are_equivalent with a different quantity
            let other_code = if let Some(&b) = data.get(2) {
                let idx = (b % VALID_UNITS.len() as u8) as usize;
                VALID_UNITS[idx]
            } else {
                code
            };
            let other_value = data.get(3).map(|&b| b as f64).unwrap_or(1.0);
            let other_quantity = FhirQuantity::with_ucum_code(other_value, other_code);
            let _ = are_equivalent(&quantity, &other_quantity);

            // Test convert_quantity
            let _ = convert_quantity(&quantity, other_code);

            // Test with arbitrary units
            if let Some(&b) = data.get(4) {
                let idx = (b % ARBITRARY_UNITS.len() as u8) as usize;
                let arb_unit = ARBITRARY_UNITS[idx];
                let arb_quantity = FhirQuantity::with_ucum_code(value, arb_unit);

                // Test to_ucum_quantity
                if let Ok(ucum_quantity) = arb_quantity.to_ucum_quantity() {
                    // Test to_fhir_quantity
                    let _ = ucum_quantity.to_fhir_quantity();
                }

                // Test are_equivalent with the same arbitrary unit
                let _ = are_equivalent(&arb_quantity, &arb_quantity);

                // Test are_equivalent with a different arbitrary unit
                if let Some(&b) = data.get(5) {
                    let idx = (b % ARBITRARY_UNITS.len() as u8) as usize;
                    let other_arb_unit = ARBITRARY_UNITS[idx];
                    let other_arb_quantity = FhirQuantity::with_ucum_code(value, other_arb_unit);
                    let _ = are_equivalent(&arb_quantity, &other_arb_quantity);
                }

                // Test with prefixed arbitrary units
                if let Some(&b) = data.get(6) {
                    let prefix_idx = (b % PREFIXES.len() as u8) as usize;
                    let prefix = PREFIXES[prefix_idx];
                    let prefixed_arb_unit = format!("{}{}", prefix, arb_unit);
                    let prefixed_arb_quantity = FhirQuantity::with_ucum_code(value, &prefixed_arb_unit);

                    // Test to_ucum_quantity
                    if let Ok(ucum_quantity) = prefixed_arb_quantity.to_ucum_quantity() {
                        // Test to_fhir_quantity
                        let _ = ucum_quantity.to_fhir_quantity();
                    }

                    // Test are_equivalent with the non-prefixed arbitrary unit
                    let _ = are_equivalent(&prefixed_arb_quantity, &arb_quantity);
                }
            }
        }
    }
});
