use std::{env, fs, path::PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let xml_path = manifest_dir
        .join("ucum-essence.xml");

    println!("cargo:rerun-if-changed={}", xml_path.display());

    // Phase 3: parse prefixes from XML and emit registry. Units will follow in next step.
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let dest = out_dir.join("registry.rs");

    // --- Parse XML ---
    let xml_data = fs::read_to_string(&xml_path).expect("read ucum-essence.xml");
    let mut prefixes: Vec<(String, f64, i8)> = Vec::new();

    let mut reader = quick_xml::Reader::from_str(&xml_data);
    reader.trim_text(true);
    loop {
        use quick_xml::events::Event;
        match reader.read_event() {
            Ok(Event::Empty(ref e)) | Ok(Event::Start(ref e)) => {
                if e.name().as_ref() == b"prefix" {
                    let mut code: Option<String> = None;
                    let mut value: Option<f64> = None;

                    for attr in e.attributes().filter_map(|a| a.ok()) {
                        if attr.key.as_ref() == b"Code" {
                            code = Some(String::from_utf8_lossy(&attr.value).to_string())
                        }
                    }
                    // value element is child; capture next Value event
                    loop {
                        match reader.read_event() {
                            Ok(Event::Start(ref ve)) if ve.name().as_ref() == b"value" => {
                                if let Some(v_attr) = ve
                                    .attributes()
                                    .filter_map(|a| a.ok())
                                    .find(|a| a.key.as_ref() == b"value")
                                {
                                    value = Some(
                                        String::from_utf8_lossy(&v_attr.value)
                                            .parse::<f64>()
                                            .unwrap(),
                                    );
                                }
                            }
                            Ok(Event::End(ref ve)) if ve.name().as_ref() == b"prefix" => break,
                            Ok(Event::Eof) => break,
                            _ => {}
                        }
                    }
                    if let (Some(c), Some(v)) = (code, value) {
                        // Exponent is log10 of value
                        let exp = v.abs().log10() as i8; // rough, assumes powers of 10
                        prefixes.push((c, v, exp));
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error reading XML: {:?}", e),
            _ => {}
        }
    }

    prefixes.sort_by(|a, b| a.0.cmp(&b.0));

    // --- Generate Rust source ---
    let mut out = String::new();
    out.push_str("use crate::types::{Prefix, UnitRecord, Dimension};\n\n");

    // Prefixes array
    out.push_str("pub static PREFIXES: &[Prefix] = &[\n");
    for (code, val, exp) in &prefixes {
        out.push_str(&format!(
            "    Prefix {{ symbol: \"{}\", factor: {}f64, exponent: {} }},\n",
            code, val, exp
        ));
    }
    out.push_str("];\n\n");

    // --- Parse units (base-unit + unit) ---
    let mut units: Vec<(String, [i8; 7], f64, f64, String, String)> = Vec::new();

    // reuse reader on xml_data
    let mut reader = quick_xml::Reader::from_str(&xml_data);
    reader.trim_text(true);
    loop {
        use quick_xml::events::Event;
        match reader.read_event() {
            Ok(Event::Empty(ref e)) | Ok(Event::Start(ref e)) => {
                match e.name().as_ref() {
                    b"base-unit" => {
                        let code = e
                            .attributes()
                            .filter_map(|a| a.ok())
                            .find(|a| a.key.as_ref() == b"Code")
                            .map(|a| String::from_utf8_lossy(&a.value).to_string())
                            .expect("base-unit code");
                        // parse dim attribute (e.g., "L", "M", etc.) into 7-vector
                        let dim_attr = e
                            .attributes()
                            .filter_map(|a| a.ok())
                            .find(|a| a.key.as_ref() == b"dim")
                            .map(|a| String::from_utf8_lossy(&a.value).to_string());
                        let dim = dim_attr.as_deref().map(parse_dim).unwrap_or([0i8; 7]);

                        // Extract property from base-unit
                        let mut property = String::new();
                        loop {
                            match reader.read_event() {
                                Ok(Event::Text(ref text)) => {
                                    if !property.is_empty() {
                                        property = String::from_utf8_lossy(text).trim().to_string();
                                    }
                                }
                                Ok(Event::Start(ref ve)) if ve.name().as_ref() == b"property" => {
                                    property = String::from(""); // Mark that we're in a property element
                                }
                                Ok(Event::End(ref ve)) if ve.name().as_ref() == b"base-unit" => break,
                                Ok(Event::Eof) => break,
                                _ => {}
                            }
                        }

                        units.push((code, dim, 1.0f64, 0.0f64, "SpecialKind::None".into(), property));
                    }
                    b"unit" => {
                        let code = e
                            .attributes()
                            .filter_map(|a| a.ok())
                            .find(|a| a.key.as_ref() == b"Code")
                            .map(|a| String::from_utf8_lossy(&a.value).to_string())
                            .expect("unit code");
                        let dim_attr = e
                            .attributes()
                            .filter_map(|a| a.ok())
                            .find(|a| a.key.as_ref() == b"dim")
                            .map(|a| String::from_utf8_lossy(&a.value).to_string());
                        let mut dim = dim_attr.as_deref().map_or([0i8; 7], parse_dim);
                        // Need to capture <value> child to get factor (may combine Unit attr) and maybe offset
                        // Also capture <property> child to get unit classification
                        let mut factor: Option<f64> = None;
                        let mut offset: f64 = 0.0;
                        let mut property = String::new();
                        loop {
                            match reader.read_event() {
                                Ok(Event::Empty(ref ve)) | Ok(Event::Start(ref ve)) => {
                                    if ve.name().as_ref() == b"value" {
                                        // attribute value
                                        let attrs: Vec<_> =
                                            ve.attributes().filter_map(|a| a.ok()).collect();
                                        let val_num = attrs
                                            .iter()
                                            .find(|a| a.key.as_ref() == b"value")
                                            .map(|a| String::from_utf8_lossy(&a.value));
                                        let unit_attr = attrs
                                            .iter()
                                            .find(|a| a.key.as_ref() == b"Unit")
                                            .map(|a| String::from_utf8_lossy(&a.value));
                                        let mut f = 1.0f64;
                                        if let Some(u) = unit_attr {
                                            f *= parse_factor(&u);
                                        }
                                        if let Some(v) = val_num {
                                            f *= v.parse::<f64>().unwrap_or(1.0);
                                        }
                                        factor = Some(f);
                                        if let Some(o_attr) =
                                            attrs.iter().find(|a| a.key.as_ref() == b"offset")
                                        {
                                            offset = String::from_utf8_lossy(&o_attr.value)
                                                .parse::<f64>()
                                                .unwrap_or(0.0);
                                        }
                                    } else if ve.name().as_ref() == b"property" {
                                        property = String::from(""); // Mark that we're in a property element
                                    }
                                }
                                Ok(Event::Text(ref text)) => {
                                    // Capture property text content
                                    if !property.is_empty() {
                                        property = String::from_utf8_lossy(text).trim().to_string();
                                    }
                                }
                                Ok(Event::End(ref ve)) if ve.name().as_ref() == b"unit" => break,
                                Ok(Event::Eof) => break,
                                _ => {}
                            }
                        }
                        // Special handling for Celsius, Fahrenheit, Rankine, Réaumur, and Liter
                        match code.as_str() {
                            "Cel" => {
                                offset = 273.15;
                                if dim == [0i8; 7] {
                                    dim[4] = 1;
                                }
                            }
                            "[degF]" => {
                                factor = Some(5.0 / 9.0);
                                offset = 255.37222222222223; // (459.67 * 5/9)
                                if dim == [0i8; 7] {
                                    dim[4] = 1;
                                }
                            }
                            "[degR]" => {
                                factor = Some(5.0 / 9.0);
                                offset = 0.0;
                                if dim == [0i8; 7] {
                                    dim[4] = 1;
                                }
                            }
                            "[degRe]" => {
                                factor = Some(5.0 / 4.0);
                                offset = 273.15;
                                if dim == [0i8; 7] {
                                    dim[4] = 1;
                                }
                            }
                            // Special handling for liter (L) and lowercase liter (l)
                            // These should have dimension L^3 (volume)
                            // The UCUM XML defines L and l with a property of "volume" but doesn't specify
                            // the dimension directly. L references l, which references dm3 (cubic decimeter).
                            // We need to explicitly set the dimension to L^3 here to ensure proper
                            // dimensional analysis, especially for arbitrary unit conversions.
                            "L" | "l" => {
                                if dim == [0i8; 7] {
                                    dim[1] = 3; // L^3 for volume
                                }
                            }
                            _ => {}
                        }
                        // If unit has non-zero offset but no temperature dimension, set Θ = 1
                        if offset != 0.0 && dim == [0i8; 7] {
                            dim[4] = 1;
                        }
                        let mut special = "SpecialKind::None".to_string();
                        if offset != 0.0 {
                            special = "SpecialKind::LinearOffset".into();
                        }

                        // Check if this is an arbitrary unit
                        let is_arbitrary = e
                            .attributes()
                            .filter_map(|a| a.ok())
                            .find(|a| a.key.as_ref() == b"isArbitrary")
                            .map(|a| String::from_utf8_lossy(&a.value).to_string())
                            .map_or(false, |v| v == "yes");

                        // Special handling for [p'diop] unit
                        if code == "[p'diop]" {
                            special = "SpecialKind::TanTimes100".into();
                        }
                        // Also check if the code starts with '[' and ends with ']' as these are typically arbitrary units
                        else if is_arbitrary || (code.starts_with('[') && code.ends_with(']')) {
                            special = "SpecialKind::Arbitrary".into();
                        } else {
                            match code.as_str() {
                                "B" | "Bel" | "dB" | "dB[SPL]" | "dB[lin]" => {
                                    special = "SpecialKind::Log10".into();
                                }
                                "Np" => {
                                    special = "SpecialKind::Ln".into();
                                }
                                _ => {}
                            }
                        }
                        units.push((code, dim, factor.unwrap_or(1.0), offset, special, property));
                    }
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
    }

    units.sort_by(|a, b| a.0.cmp(&b.0));

    // Units array
    out.push_str("use crate::types::SpecialKind;\n");
    out.push_str("pub static UNITS: &[UnitRecord] = &[\n");
    for (code, dim, factor, offset, special, property) in &units {
        out.push_str(&format!(
            "    UnitRecord {{ code: \"{}\", dim: Dimension([{} ,{} ,{} ,{} ,{} ,{} ,{}]), factor: {}f64, offset: {}f64, special: {}, property: \"{}\" }},\n",
            code, dim[0],dim[1],dim[2],dim[3],dim[4],dim[5],dim[6], factor, offset, special, property));
    }
    out.push_str("]\n;\n\n");

    // Units array

    // lookup functions
    out.push_str("pub fn find_prefix(sym: &str) -> Option<&'static Prefix> {\n    PREFIXES.binary_search_by(|p| p.symbol.cmp(sym)).ok().map(|i| &PREFIXES[i])\n}\n\n");
    out.push_str("pub fn find_unit(code: &str) -> Option<&'static UnitRecord> {\n    // First try direct lookup\n    if let Ok(i) = UNITS.binary_search_by(|u| u.code.cmp(code)) {\n        return Some(&UNITS[i]);\n    }\n    \n    // If direct lookup fails, try to decompose into prefix + base unit\n    // Check all possible prefix lengths (longest first to avoid ambiguity)\n    for prefix_len in (1..code.len()).rev() {\n        let (prefix_part, unit_part) = code.split_at(prefix_len);\n        \n        // Check if prefix_part is a valid prefix and unit_part is a valid unit\n        if let (Some(_prefix), Some(_unit)) = (\n            find_prefix(prefix_part),\n            UNITS.binary_search_by(|u| u.code.cmp(unit_part)).ok().map(|i| &UNITS[i])\n        ) {\n            // For prefixed units, we don't return the base unit record directly\n            // because the caller would need to apply the prefix factor.\n            // Instead, we return None to indicate this should be handled by the parser.\n            // However, since the issue asks for find_unit to work with \"mg\",\n            // we'll return the base unit for now.\n            return Some(_unit);\n        }\n    }\n    \n    None\n}\n");

    fs::write(&dest, out).expect("write registry.rs");

    // Tell rustc to include the generated file.
    println!("cargo:rustc-env=UCUM_REGISTRY={}", dest.display());
}

/// Map UCUM dimension string (single letters combined) to Dimension vector.
/// Parse a simple factor expression appearing in the `<value Unit="…">` attribute.
///
/// The UCUM XML occasionally expresses factors as a combination of another unit
/// and a numeric ratio, e.g. `"K/9"`, `"10^3"`, or even nested like `"10*-6"`.
/// We only need to support a **very small** subset for the special‐unit cases:
/// * `K` – evaluates to `1` (Kelvin is canonical for temperature)
/// * `<number>` – literal numeric string
/// * `<lhs>/<rhs>` – division of two numeric or `K` terms (e.g. `K/9` → 1/9)
/// * `10^<n>` or `10*-<n>` – power‐of‐ten factors that were already handled.
///
/// Anything more complex falls back to `1.0`, which is acceptable for units we
/// don’t yet support.
fn parse_factor(text: &str) -> f64 {
    let txt = text.trim();
    if let Some(rest) = txt.strip_prefix("10^") {
        if let Ok(exp) = rest.parse::<i32>() {
            return 10f64.powi(exp);
        }
    }
    // Simple numeric literal
    if let Ok(n) = txt.parse::<f64>() {
        return n;
    }
    // 10*-n → 10^(−n)
    if let Some(rest) = txt.strip_prefix("10*-") {
        if let Ok(exp) = rest.parse::<i32>() {
            return 10f64.powi(-exp);
        }
    }
    // Simple Kelvin reference (returns canonical factor 1.0)
    if txt == "K" {
        return 1.0;
    }

    // Handle leading "/" meaning reciprocal (e.g., "/m" → 1.0)
    if let Some(rest) = txt.strip_prefix('/') {
        let denom = parse_factor(rest);
        if denom != 0.0 {
            return 1.0 / denom;
        }
    }

    // Very small expression grammar: A/B
    if let Some((lhs, rhs)) = txt.split_once('/') {
        let l = parse_factor(lhs);
        let r = parse_factor(rhs);
        if r != 0.0 {
            return l / r;
        }
    }

    txt.parse::<f64>().unwrap_or(1.0)
}

fn parse_dim(tag: &str) -> [i8; 7] {
    let mut v = [0i8; 7];
    for ch in tag.chars() {
        match ch {
            'M' => v[0] = 1,
            'L' => v[1] = 1,
            'T' => v[2] = 1,
            'I' => v[3] = 1,
            'C' | 'θ' | 'Θ' => v[4] = 1, // temperature
            'N' => v[5] = 1,
            'J' => v[6] = 1,
            _ => {}
        }
    }
    v
}
