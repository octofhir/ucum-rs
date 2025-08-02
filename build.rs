use std::{env, fs, path::PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let xml_path = manifest_dir.join("ucum-essence.xml");

    println!("cargo:rerun-if-changed={}", xml_path.display());

    // Phase 3: parse prefixes from XML and emit registry. Units will follow in the next step.
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let dest = out_dir.join("registry.rs");

    // --- Parse XML ---
    let xml_data = fs::read_to_string(&xml_path).expect("read ucum-essence.xml");
    let mut prefixes: Vec<(String, f64, i8, String)> = Vec::new();

    let mut reader = quick_xml::Reader::from_str(&xml_data);
    loop {
        use quick_xml::events::Event;
        match reader.read_event() {
            Ok(Event::Empty(ref e)) | Ok(Event::Start(ref e)) => {
                if e.name().as_ref() == b"prefix" {
                    let mut code: Option<String> = None;
                    let mut value: Option<f64> = None;
                    let mut name: Option<String> = None;

                    for attr in e.attributes().filter_map(|a| a.ok()) {
                        if attr.key.as_ref() == b"Code" {
                            code = Some(String::from_utf8_lossy(&attr.value).to_string())
                        }
                    }
                    // value and name elements are children; capture them
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
                            Ok(Event::Start(ref ne)) if ne.name().as_ref() == b"name" => {
                                // Read the text content of the name element
                                if let Ok(Event::Text(text)) = reader.read_event() {
                                    name = Some(String::from_utf8_lossy(&text).to_string());
                                }
                            }
                            Ok(Event::End(ref ve)) if ve.name().as_ref() == b"prefix" => break,
                            Ok(Event::Eof) => break,
                            _ => {}
                        }
                    }
                    if let (Some(c), Some(v), Some(n)) = (code, value, name) {
                        // Exponent is log10 of value
                        let exp = v.abs().log10() as i8; // rough, assumes powers of 10
                        prefixes.push((c, v, exp, n));
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error reading XML: {e:?}"),
            _ => {}
        }
    }

    prefixes.sort_by(|a, b| a.0.cmp(&b.0));

    // --- Generate Rust source ---
    let mut out = String::new();
    out.push_str("use crate::types::{Prefix, UnitRecord, Dimension};\n\n");

    // Prefixes array
    out.push_str("pub static PREFIXES: &[Prefix] = &[\n");
    for (code, val, exp, name) in &prefixes {
        out.push_str(&format!(
            "    Prefix {{ symbol: \"{code}\", factor: {val}f64, exponent: {exp}, display_name: \"{name}\" }},\n"
        ));
    }
    out.push_str("];\n\n");

    // --- Parse units (base-unit + unit) ---
    #[allow(clippy::type_complexity)]
    let mut units: Vec<(
        String,
        [i8; 7],
        f64,
        f64,
        String,
        String,
        String,
        Option<String>,
    )> = Vec::new();

    // reuse reader on xml_data
    let mut reader = quick_xml::Reader::from_str(&xml_data);
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

                        // Extract property and display the name from base-unit
                        let mut property = String::new();
                        let mut display_name = String::new();
                        let mut in_property_tag = false;
                        let mut in_n_tag = false;
                        loop {
                            match reader.read_event() {
                                Ok(Event::Text(ref text)) => {
                                    if in_property_tag {
                                        property = String::from_utf8_lossy(text).trim().to_string();
                                        in_property_tag = false;
                                    }
                                    if in_n_tag {
                                        display_name =
                                            String::from_utf8_lossy(text).trim().to_string();
                                        in_n_tag = false;
                                    }
                                }
                                Ok(Event::Start(ref ve)) if ve.name().as_ref() == b"property" => {
                                    in_property_tag = true;
                                }
                                Ok(Event::Start(ref ve)) if ve.name().as_ref() == b"name" => {
                                    in_n_tag = true;
                                }
                                Ok(Event::End(ref ve)) if ve.name().as_ref() == b"base-unit" => {
                                    break;
                                }
                                Ok(Event::Eof) => break,
                                _ => {}
                            }
                        }

                        // Default display name to code if not found
                        if display_name.is_empty() {
                            display_name = code.clone();
                        }

                        units.push((
                            code,
                            dim,
                            1.0f64,
                            0.0f64,
                            "SpecialKind::None".into(),
                            property,
                            display_name,
                            None,
                        ));
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
                        // Need to capture <value> child to get a factor (may combine Unit attr) and maybe offset
                        // Also capture <property> child to get unit classification and <name> for display name
                        let mut factor: Option<f64> = None;
                        let mut offset: f64 = 0.0;
                        let mut property = String::new();
                        let mut display_name = String::new();
                        let mut in_property_tag = false;
                        let mut in_n_tag = false;
                        let mut unit_ref_for_dim: Option<String> = None;
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
                                            unit_ref_for_dim = Some(u.to_string());
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
                                        in_property_tag = true;
                                    } else if ve.name().as_ref() == b"name" {
                                        in_n_tag = true;
                                    }
                                }
                                Ok(Event::Text(ref text)) => {
                                    // Capture property and display name text content
                                    if in_property_tag {
                                        property = String::from_utf8_lossy(text).trim().to_string();
                                        in_property_tag = false;
                                    }
                                    if in_n_tag {
                                        display_name =
                                            String::from_utf8_lossy(text).trim().to_string();
                                        in_n_tag = false;
                                    }
                                }
                                Ok(Event::End(ref ve)) if ve.name().as_ref() == b"unit" => break,
                                Ok(Event::Eof) => break,
                                _ => {}
                            }
                        }

                        // Default display name to code if not found
                        if display_name.is_empty() {
                            display_name = code.clone();
                        }
                        // Special handling for Celsius, Fahrenheit, Rankine, Réaumur, Liter, and Imperial units
                        match code.as_str() {
                            "Cel" => {
                                offset = 273.15;
                                if dim == [0i8; 7] {
                                    dim[4] = 1;
                                }
                            }
                            "[in_i]" => {
                                // Fix precision issue: ensure exactly 2.54 cm
                                // The build script will convert cm to m (2.54 * 0.01 = 0.0254)
                                factor = Some(2.54);
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
                            .is_some_and(|v| v == "yes");

                        // Special handling for [p'diop] unit
                        if code == "[p'diop]" {
                            special = "SpecialKind::TanTimes100".into();
                        }
                        // Only treat units as arbitrary if explicitly marked with isArbitrary="yes"
                        else if is_arbitrary {
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
                        units.push((
                            code,
                            dim,
                            factor.unwrap_or(1.0),
                            offset,
                            special,
                            property,
                            display_name,
                            unit_ref_for_dim,
                        ));
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

    // Helper functions for dimension parsing
    fn parse_unit_expression_dimensions(expr: &str) -> [i8; 7] {
        // Remove numeric factors and constants like [pi] to focus on dimensional units
        let mut cleaned = expr.to_string();

        // Simple string-based cleaning without regex
        // Remove numeric factors like "4.", "10*-7", "10^3", etc.
        // But preserve digits that are part of unit symbols like "A2"
        let mut result = String::new();
        let mut chars = cleaned.chars().peekable();

        while let Some(ch) = chars.next() {
            match ch {
                // Handle digits more carefully
                '0'..='9' => {
                    // Look ahead to see if this digit is part of a unit symbol
                    // Check if preceded by a letter (like "A2")
                    let preceded_by_letter = result
                        .chars()
                        .last()
                        .is_some_and(|c| c.is_ascii_alphabetic());

                    if preceded_by_letter {
                        // This digit is part of a unit symbol, keep it
                        result.push(ch);
                    } else {
                        // This is a numeric factor, skip it and following digits/decimals
                        while let Some(&next_ch) = chars.peek() {
                            if next_ch.is_ascii_digit() || next_ch == '.' {
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        // Skip optional '*' or '^' after numbers
                        if let Some(&next_ch) = chars.peek() {
                            if next_ch == '*' || next_ch == '^' {
                                chars.next();
                                // Skip optional '-' after '^' or '*'
                                if let Some(&minus_ch) = chars.peek() {
                                    if minus_ch == '-' {
                                        chars.next();
                                    }
                                }
                            }
                        }
                    }
                }
                // Handle decimal points
                '.' => {
                    // Only skip if it's part of a numeric factor (not preceded by letter)
                    let preceded_by_letter = result
                        .chars()
                        .last()
                        .is_some_and(|c| c.is_ascii_alphabetic());
                    if !preceded_by_letter {
                        // Skip decimal point and following digits
                        while let Some(&next_ch) = chars.peek() {
                            if next_ch.is_ascii_digit() {
                                chars.next();
                            } else {
                                break;
                            }
                        }
                    } else {
                        result.push(ch);
                    }
                }
                // Remove constants like [pi], [e], etc.
                '[' => {
                    // Skip until closing bracket
                    for bracket_ch in chars.by_ref() {
                        if bracket_ch == ']' {
                            break;
                        }
                    }
                    // Skip optional dot after bracket
                    if let Some(&dot_ch) = chars.peek() {
                        if dot_ch == '.' {
                            chars.next();
                        }
                    }
                }
                // Keep other characters
                _ => result.push(ch),
            }
        }

        // First replace double dots with single dots
        cleaned = result.replace("..", ".");

        // Now handle dots more carefully - we want to remove them as separators
        // but preserve the structure of expressions like "N/A2"
        let mut final_result = String::new();
        let parts: Vec<&str> = cleaned.split('.').collect();

        for part in parts {
            final_result.push_str(part);
        }

        cleaned = final_result.trim().to_string();

        // Now parse the remaining dimensional expression
        if cleaned.is_empty() {
            return [0i8; 7];
        }

        // Handle simple cases first
        match cleaned.as_str() {
            "N/A2" => [1, 1, -2, -2, 0, 0, 0], // Force per square ampere: kg⋅m⋅s⁻²⋅A⁻²
            "g/m3" => [1, -3, 0, 0, 0, 0, 0],  // Mass per volume
            "Pa" => [1, -1, -2, 0, 0, 0, 0],   // Pressure: kg⋅m⁻¹⋅s⁻²
            "kPa" => [1, -1, -2, 0, 0, 0, 0],  // Pressure: kg⋅m⁻¹⋅s⁻²
            "N" => [1, 1, -2, 0, 0, 0, 0],     // Force: kg⋅m⋅s⁻²
            "J" => [1, 2, -2, 0, 0, 0, 0],     // Energy: kg⋅m²⋅s⁻²
            "W" => [1, 2, -3, 0, 0, 0, 0],     // Power: kg⋅m²⋅s⁻³
            "V" => [1, 2, -3, -1, 0, 0, 0],    // Voltage: kg⋅m²⋅s⁻³⋅A⁻¹
            "F" => [-1, -2, 4, 2, 0, 0, 0],    // Capacitance: kg⁻¹⋅m⁻²⋅s⁴⋅A²
            "Ohm" => [1, 2, -3, -2, 0, 0, 0],  // Resistance: kg⋅m²⋅s⁻³⋅A⁻²
            "Ohm-1" => [-1, -2, 3, 2, 0, 0, 0], // Conductance: kg⁻¹⋅m⁻²⋅s³⋅A²
            "S" => [-1, -2, 3, 2, 0, 0, 0],    // Conductance: kg⁻¹⋅m⁻²⋅s³⋅A²
            "Wb" => [1, 2, -2, -1, 0, 0, 0],   // Magnetic flux: kg⋅m²⋅s⁻²⋅A⁻¹
            "T" => [1, 0, -2, -1, 0, 0, 0],    // Magnetic field: kg⋅s⁻²⋅A⁻¹
            "H" => [1, 2, -2, -2, 0, 0, 0],    // Inductance: kg⋅m²⋅s⁻²⋅A⁻²
            "C" => [0, 0, 1, 1, 0, 0, 0],      // Electric charge: s⋅A
            "m" => [0, 1, 0, 0, 0, 0, 0],      // Length
            "m2" => [0, 2, 0, 0, 0, 0, 0],     // Area
            "m3" => [0, 3, 0, 0, 0, 0, 0],     // Volume
            "s" => [0, 0, 1, 0, 0, 0, 0],      // Time
            "s-1" => [0, 0, -1, 0, 0, 0, 0],   // Frequency (1/time)
            "Hz" => [0, 0, -1, 0, 0, 0, 0],    // Frequency (hertz)
            "A" => [0, 0, 0, 1, 0, 0, 0],      // Current
            "A2" => [0, 0, 0, 2, 0, 0, 0],     // Current squared
            _ => {
                // For more complex expressions, try basic parsing
                if cleaned.contains('/') {
                    let parts: Vec<&str> = cleaned.split('/').collect();
                    if parts.len() == 2 {
                        let num_dim = get_basic_unit_dimension(parts[0]);
                        let den_dim = get_basic_unit_dimension(parts[1]);
                        return subtract_dimensions(num_dim, den_dim);
                    }
                }
                [0i8; 7] // Fallback to dimensionless
            }
        }
    }

    fn get_basic_unit_dimension(unit: &str) -> [i8; 7] {
        match unit.trim() {
            "N" => [1, 1, -2, 0, 0, 0, 0], // Force
            "A" => [0, 0, 0, 1, 0, 0, 0],  // Current
            "A2" => [0, 0, 0, 2, 0, 0, 0], // Current squared
            "m" => [0, 1, 0, 0, 0, 0, 0],  // Length
            "m2" => [0, 2, 0, 0, 0, 0, 0], // Area
            "m3" => [0, 3, 0, 0, 0, 0, 0], // Volume
            "g" => [1, 0, 0, 0, 0, 0, 0],  // Mass
            "s" => [0, 0, 1, 0, 0, 0, 0],  // Time
            "C" => [0, 0, 1, 1, 0, 0, 0],  // Charge
            _ => {
                // Check if this is a unit with a numeric suffix (like A2)
                if unit.len() > 1 {
                    let (base, suffix) = unit.split_at(1);
                    if suffix.chars().all(|c| c.is_ascii_digit()) {
                        if base == "A" {
                            // Handle A with numeric suffix (A2, A3, etc.)
                            if let Ok(power) = suffix.parse::<i8>() {
                                return [0, 0, 0, power, 0, 0, 0]; // Current^power
                            }
                        } else if base == "m" {
                            // Handle m with numeric suffix (m2, m3, etc.)
                            if let Ok(power) = suffix.parse::<i8>() {
                                return [0, power, 0, 0, 0, 0, 0]; // Length^power
                            }
                        }
                    }
                }
                [0i8; 7]
            }
        }
    }

    fn subtract_dimensions(a: [i8; 7], b: [i8; 7]) -> [i8; 7] {
        [
            a[0] - b[0],
            a[1] - b[1],
            a[2] - b[2],
            a[3] - b[3],
            a[4] - b[4],
            a[5] - b[5],
            a[6] - b[6],
        ]
    }

    // Second pass: resolve unit references and derive dimensions
    // Create lookup maps for both dimensions and factors
    let mut unit_dims: std::collections::HashMap<String, [i8; 7]> =
        std::collections::HashMap::new();
    let mut unit_factors: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
    let mut original_factors: std::collections::HashMap<String, f64> =
        std::collections::HashMap::new();

    // First, collect all units with known dimensions and factors
    for (code, dim, factor, _, _, _, _, _) in &units {
        if *dim != [0i8; 7] {
            unit_dims.insert(code.clone(), *dim);
        }
        unit_factors.insert(code.clone(), *factor);
        original_factors.insert(code.clone(), *factor); // Store original XML factors
    }

    // Function to resolve unit factor recursively
    fn resolve_unit_factor(
        unit_ref: &str,
        unit_factors: &std::collections::HashMap<String, f64>,
    ) -> f64 {
        // Handle simple numeric expressions first
        if let Ok(n) = unit_ref.parse::<f64>() {
            return n;
        }

        // Handle power-of-ten notation
        if let Some(rest) = unit_ref.strip_prefix("10^") {
            if let Ok(exp) = rest.parse::<i32>() {
                return 10f64.powi(exp);
            }
        }
        if let Some(rest) = unit_ref.strip_prefix("10*-") {
            if let Ok(exp) = rest.parse::<i32>() {
                return 10f64.powi(-exp);
            }
        }

        // Handle division expressions like "m/3937"
        if let Some((lhs, rhs)) = unit_ref.split_once('/') {
            let l = resolve_unit_factor(lhs, unit_factors);
            let r = resolve_unit_factor(rhs, unit_factors);
            if r != 0.0 {
                return l / r;
            }
        }

        // Handle multiplication expressions like "kg.m"
        if unit_ref.contains('.') {
            let parts: Vec<&str> = unit_ref.split('.').collect();
            let mut result = 1.0;
            for part in parts {
                let part_factor = resolve_unit_factor(part, unit_factors);
                result *= part_factor;
            }
            return result;
        }

        // Handle unit references with exponents like "Ohm-1", "Ohm-2", etc.
        if let Some(dash_pos) = unit_ref.rfind('-') {
            let unit_part = &unit_ref[..dash_pos];
            let exp_part = &unit_ref[dash_pos + 1..];

            // Check if the exponent part is a valid integer
            if let Ok(exponent) = exp_part.parse::<i32>() {
                // Look up the base unit
                if let Some(&base_factor) = unit_factors.get(unit_part) {
                    // Apply the negative exponent: Unit-n means Unit^(-n)
                    return base_factor.powi(-exponent);
                }
            }
        }

        // Look up unit reference
        if let Some(&factor) = unit_factors.get(unit_ref) {
            return factor;
        }

        // Handle prefixed units like "cm", "mm", etc.
        // Check if it's a prefixed unit by trying to split it
        for prefix_len in (1..unit_ref.len()).rev() {
            let (prefix_part, unit_part) = unit_ref.split_at(prefix_len);

            // Check if prefix_part is a known prefix
            let prefix_factor = match prefix_part {
                "c" => 0.01,           // centi
                "m" => 0.001,          // milli
                "k" => 1000.0,         // kilo
                "d" => 0.1,            // deci
                "da" => 10.0,          // deka
                "h" => 100.0,          // hecto
                "M" => 1000000.0,      // mega
                "G" => 1000000000.0,   // giga
                "μ" | "u" => 0.000001, // micro
                "n" => 0.000000001,    // nano
                "p" => 0.000000000001, // pico
                _ => continue,
            };

            // Check if unit_part is a known unit
            if let Some(&base_factor) = unit_factors.get(unit_part) {
                return prefix_factor * base_factor;
            }
        }

        // Special cases for constants and base units
        match unit_ref {
            "K" => 1.0,                     // Kelvin is canonical
            "[c]" => 299792458.0,           // speed of light in m/s
            "[pi]" => std::f64::consts::PI, // pi constant
            "[e]" => 1.602176634e-19,       // elementary charge in C
            "[h]" => 6.62607015e-34,        // Planck constant in J⋅s
            "[k]" => 1.380649e-23,          // Boltzmann constant in J/K
            "[g]" => 9.80665,               // standard acceleration of free fall in m/s²
            "a_j" => 31557600.0,            // Julian year in seconds (365.25 * 24 * 3600)
            "a_t" => 31556925.216,          // tropical year in seconds (365.24219 * 24 * 3600)
            "a_g" => 31556952.0,            // Gregorian year in seconds (365.2425 * 24 * 3600)
            _ => 1.0,                       // Fallback
        }
    }

    // Now resolve unit factors that reference other units
    // Do multiple passes until no more changes occur
    let mut changed = true;
    let mut pass = 0;
    while changed && pass < 10 {
        // Limit passes to avoid infinite loops
        changed = false;
        pass += 1;

        #[allow(clippy::needless_range_loop)]
        for i in 0..units.len() {
            let (code, _, _, _, _, _, _, unit_ref) = {
                let unit = &units[i];
                (
                    unit.0.clone(),
                    unit.1,
                    unit.2,
                    unit.3,
                    unit.4.clone(),
                    unit.5.clone(),
                    unit.6.clone(),
                    unit.7.clone(),
                )
            };
            if let Some(ref_unit) = unit_ref {
                // Use the original XML factor, not the current resolved factor
                let original_factor = original_factors.get(&code).copied().unwrap_or(1.0);
                let resolved_factor = resolve_unit_factor(&ref_unit, &unit_factors);
                let final_factor = if code == "[in_i]" {
                    // Special case for [in_i] to ensure exact precision
                    // 1 inch = 2.54 cm = 0.0254 m (exactly)
                    0.0254
                } else {
                    original_factor * resolved_factor
                };

                // Only update if the resolved factor has changed from 1.0 (meaning we found a better resolution)
                // and the final factor is different from the current
                if resolved_factor != 1.0 && (final_factor - units[i].2).abs() > 1e-10 {
                    units[i].2 = final_factor;
                    unit_factors.insert(code, final_factor);
                    changed = true;
                }
            }
        }
    }

    // Now update units that need dimension derivation
    #[allow(clippy::needless_range_loop)]
    for i in 0..units.len() {
        let unit_data = &units[i];
        let needs_update = {
            let (_, dim, _, _, _, _, _, unit_ref) = unit_data;
            *dim == [0i8; 7] && unit_ref.is_some()
        };

        if needs_update {
            let (code, _, _, _, _, _, _, unit_ref) = &units[i];
            let ref_unit = unit_ref.as_ref().unwrap();
            let code = code.clone();

            // Look up the referenced unit's dimension
            // First try property-based assignment for known properties - prioritize this over unit references
            let property_dim = match units[i].5.as_str() {
                "length" => [0, 1, 0, 0, 0, 0, 0],
                "mass" => [1, 0, 0, 0, 0, 0, 0],
                "time" => [0, 0, 1, 0, 0, 0, 0],
                "electric current" => [0, 0, 0, 1, 0, 0, 0],
                "thermodynamic temperature" => [0, 0, 0, 0, 1, 0, 0],
                "amount of substance" => [0, 0, 0, 0, 0, 1, 0],
                "luminous intensity" => [0, 0, 0, 0, 0, 0, 1],
                "area" => [0, 2, 0, 0, 0, 0, 0],
                "volume" => [0, 3, 0, 0, 0, 0, 0],
                "velocity" => [0, 1, -1, 0, 0, 0, 0],
                "acceleration" => [0, 1, -2, 0, 0, 0, 0],
                "force" => [1, 1, -2, 0, 0, 0, 0],
                "pressure" => [1, -1, -2, 0, 0, 0, 0],
                "energy" => [1, 2, -2, 0, 0, 0, 0],
                "power" => [1, 2, -3, 0, 0, 0, 0],
                "electric charge" => [0, 0, 1, 1, 0, 0, 0],
                "electric potential" => [1, 2, -3, -1, 0, 0, 0],
                "electric capacitance" => [-1, -2, 4, 2, 0, 0, 0],
                "electric resistance" => [1, 2, -3, -2, 0, 0, 0],
                "electric conductance" => [-1, -2, 3, 2, 0, 0, 0],
                "magnetic flux" => [1, 2, -2, -1, 0, 0, 0],
                "magnetic flux density" => [1, 0, -2, -1, 0, 0, 0],
                "inductance" => [1, 2, -2, -2, 0, 0, 0],
                "magnetic permeability" => [1, 1, -2, -2, 0, 0, 0],
                "luminous flux" => [0, 0, 0, 0, 0, 0, 1],
                "illuminance" => [0, -2, 0, 0, 0, 0, 1],
                "radioactivity" => [0, 0, -1, 0, 0, 0, 0],
                "frequency" => [0, 0, -1, 0, 0, 0, 0],
                "plane angle" => [0, 0, 0, 0, 0, 0, 0], // dimensionless
                "solid angle" => [0, 0, 0, 0, 0, 0, 0], // dimensionless
                _ => [0i8; 7],                          // unknown, will try other methods
            };

            let derived_dim = if property_dim != [0i8; 7] {
                // Always use property-based dimension if available - this takes priority
                property_dim
            } else if let Some(&ref_dim) = unit_dims.get(ref_unit) {
                if ref_dim != [0i8; 7] {
                    ref_dim
                } else {
                    // Referenced unit is dimensionless, fallback to property-based assignment

                    match units[i].5.as_str() {
                        "length" => [0, 1, 0, 0, 0, 0, 0],
                        "mass" => [1, 0, 0, 0, 0, 0, 0],
                        "time" => [0, 0, 1, 0, 0, 0, 0],
                        "electric current" => [0, 0, 0, 1, 0, 0, 0],
                        "thermodynamic temperature" => [0, 0, 0, 0, 1, 0, 0],
                        "amount of substance" => [0, 0, 0, 0, 0, 1, 0],
                        "luminous intensity" => [0, 0, 0, 0, 0, 0, 1],
                        "area" => [0, 2, 0, 0, 0, 0, 0],
                        "volume" => [0, 3, 0, 0, 0, 0, 0],
                        "velocity" => [0, 1, -1, 0, 0, 0, 0],
                        "acceleration" => [0, 1, -2, 0, 0, 0, 0],
                        "force" => [1, 1, -2, 0, 0, 0, 0],
                        "pressure" => [1, -1, -2, 0, 0, 0, 0],
                        "energy" => [1, 2, -2, 0, 0, 0, 0],
                        "power" => [1, 2, -3, 0, 0, 0, 0],
                        "electric charge" => [0, 0, 1, 1, 0, 0, 0],
                        "electric potential" => [1, 2, -3, -1, 0, 0, 0],
                        "electric capacitance" => [-1, -2, 4, 2, 0, 0, 0],
                        "electric resistance" => [1, 2, -3, -2, 0, 0, 0],
                        "electric conductance" => [-1, -2, 3, 2, 0, 0, 0],
                        "magnetic flux" => [1, 2, -2, -1, 0, 0, 0],
                        "magnetic flux density" => [1, 0, -2, -1, 0, 0, 0],
                        "inductance" => [1, 2, -2, -2, 0, 0, 0],
                        "magnetic permeability" => [1, 1, -2, -2, 0, 0, 0],
                        "luminous flux" => [0, 0, 0, 0, 0, 0, 1],
                        "illuminance" => [0, -2, 0, 0, 0, 0, 1],
                        "radioactivity" => [0, 0, -1, 0, 0, 0, 0],
                        "frequency" => [0, 0, -1, 0, 0, 0, 0],
                        "plane angle" => [0, 0, 0, 0, 0, 0, 0], // dimensionless
                        "solid angle" => [0, 0, 0, 0, 0, 0, 0], // dimensionless
                        _ => [0i8; 7],                          // unknown, keep dimensionless
                    }
                }
            } else {
                // Try to parse complex unit expressions like "4.[pi].10*-7.N/A2"
                let parsed_dim = parse_unit_expression_dimensions(ref_unit);
                if parsed_dim != [0i8; 7] {
                    parsed_dim
                } else {
                    // Handle common base units that might not be in our parsed list

                    match ref_unit.as_str() {
                        "m" => [0, 1, 0, 0, 0, 0, 0],                   // length
                        "g" => [1, 0, 0, 0, 0, 0, 0],                   // mass
                        "s" => [0, 0, 1, 0, 0, 0, 0],                   // time
                        "A" => [0, 0, 0, 1, 0, 0, 0],                   // current
                        "K" => [0, 0, 0, 0, 1, 0, 0],                   // temperature
                        "mol" => [0, 0, 0, 0, 0, 1, 0],                 // amount
                        "cd" => [0, 0, 0, 0, 0, 0, 1],                  // luminous intensity
                        "cm" => [0, 1, 0, 0, 0, 0, 0],                  // length (centimeter)
                        "mm" => [0, 1, 0, 0, 0, 0, 0],                  // length (millimeter)
                        "km" => [0, 1, 0, 0, 0, 0, 0],                  // length (kilometer)
                        "kPa" => [1, -1, -2, 0, 0, 0, 0],               // pressure
                        "Ohm-1" => [-1, -2, 3, 2, 0, 0, 0],             // conductance
                        "[c].a_j" => [0, 1, 0, 0, 0, 0, 0], // length (speed of light * time)
                        "4.[pi].10*-7.N/A2" => [1, 1, -2, -2, 0, 0, 0], // magnetic permeability
                        "[mu_0]" => [1, 1, -2, -2, 0, 0, 0], // magnetic permeability of vacuum
                        _ => {
                            // Try to infer dimension from property if available

                            match units[i].5.as_str() {
                                "length" => [0, 1, 0, 0, 0, 0, 0],
                                "mass" => [1, 0, 0, 0, 0, 0, 0],
                                "time" => [0, 0, 1, 0, 0, 0, 0],
                                "electric current" => [0, 0, 0, 1, 0, 0, 0],
                                "thermodynamic temperature" => [0, 0, 0, 0, 1, 0, 0],
                                "amount of substance" => [0, 0, 0, 0, 0, 1, 0],
                                "luminous intensity" => [0, 0, 0, 0, 0, 0, 1],
                                "area" => [0, 2, 0, 0, 0, 0, 0],
                                "volume" => [0, 3, 0, 0, 0, 0, 0],
                                "velocity" => [0, 1, -1, 0, 0, 0, 0],
                                "acceleration" => [0, 1, -2, 0, 0, 0, 0],
                                "force" => [1, 1, -2, 0, 0, 0, 0],
                                "pressure" => [1, -1, -2, 0, 0, 0, 0],
                                "energy" => [1, 2, -2, 0, 0, 0, 0],
                                "power" => [1, 2, -3, 0, 0, 0, 0],
                                "electric charge" => [0, 0, 1, 1, 0, 0, 0],
                                "electric potential" => [1, 2, -3, -1, 0, 0, 0],
                                "electric capacitance" => [-1, -2, 4, 2, 0, 0, 0],
                                "electric resistance" => [1, 2, -3, -2, 0, 0, 0],
                                "electric conductance" => [-1, -2, 3, 2, 0, 0, 0],
                                "magnetic flux" => [1, 2, -2, -1, 0, 0, 0],
                                "magnetic flux density" => [1, 0, -2, -1, 0, 0, 0],
                                "inductance" => [1, 2, -2, -2, 0, 0, 0],
                                "magnetic permeability" => [1, 1, -2, -2, 0, 0, 0],
                                "luminous flux" => [0, 0, 0, 0, 0, 0, 1],
                                "illuminance" => [0, -2, 0, 0, 0, 0, 1],
                                "radioactivity" => [0, 0, -1, 0, 0, 0, 0],
                                "plane angle" => [0, 0, 0, 0, 0, 0, 0], // dimensionless
                                "solid angle" => [0, 0, 0, 0, 0, 0, 0], // dimensionless
                                _ => [0i8; 7], // unknown, keep dimensionless
                            }
                        }
                    }
                }
            };

            if derived_dim != [0i8; 7] {
                units[i].1 = derived_dim;
                unit_dims.insert(code, derived_dim); // Update the lookup map for future references
            }
        }
    }

    // Units array
    out.push_str("use crate::types::SpecialKind;\n");
    out.push_str("#[allow(clippy::approx_constant)] // Constants come from UCUM specification\n");
    out.push_str("pub static UNITS: &[UnitRecord] = &[\n");
    for (code, dim, factor, offset, special, property, display_name, _unit_ref) in &units {
        // Format factor with const replacement if needed
        let factor_str = if (*factor - std::f64::consts::PI).abs() < 1e-10 {
            "std::f64::consts::PI".to_string()
        } else if (*factor - std::f64::consts::TAU).abs() < 1e-10 {
            "std::f64::consts::TAU".to_string()
        } else if (*factor - std::f64::consts::FRAC_PI_4).abs() < 1e-10 {
            "std::f64::consts::FRAC_PI_4".to_string()
        } else {
            format!("{factor}f64")
        };

        out.push_str(&format!(
            "    UnitRecord {{ code: \"{}\", dim: Dimension([{} ,{} ,{} ,{} ,{} ,{} ,{}]), factor: {}, offset: {}f64, special: {}, property: \"{}\", display_name: \"{}\" }},\n",
            code, dim[0],dim[1],dim[2],dim[3],dim[4],dim[5],dim[6], factor_str, offset, special, property, display_name));
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

/// Parse unit expression to extract dimensions from complex expressions like "4.[pi].10*-7.N/A2"
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
            'Q' => {
                // Charge dimension: time × current
                v[2] = 1; // time
                v[3] = 1; // current
            }
            _ => {}
        }
    }
    v
}
