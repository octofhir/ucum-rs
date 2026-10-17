//! The registry holds every prefix and unit atom of `ucum-essence.xml`, and nothing else,
//! with the attributes of the XML: case insensitive codes, the metric, special and
//! arbitrary flags, the class and the name.

use octofhir_ucum::{Number, find_prefix, find_unit, get_all_prefixes, get_all_units};
use quick_xml::Reader;
use quick_xml::events::Event;
use std::collections::{BTreeSet, HashMap};

const XML: &str = include_str!("../ucum-essence.xml");

/// `(element name, attributes, first <name>, <value value=...>)` of every prefix, base unit
/// and unit.
fn elements() -> Vec<(String, HashMap<String, String>, String, String)> {
    let mut reader = Reader::from_str(XML);
    let mut elements = Vec::new();
    let mut current: Option<(String, HashMap<String, String>, String, String)> = None;
    let mut in_name = false;
    // Some units have several names; the first one is the primary
    let mut names = 0;
    loop {
        match reader.read_event().expect("well-formed XML") {
            Event::Start(e) | Event::Empty(e) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                let attributes: HashMap<String, String> = e
                    .attributes()
                    .map(|a| {
                        let a = a.unwrap();
                        (
                            String::from_utf8_lossy(a.key.as_ref()).into_owned(),
                            a.normalized_value(quick_xml::XmlVersion::Implicit1_0)
                                .unwrap()
                                .into_owned(),
                        )
                    })
                    .collect();
                match tag.as_str() {
                    "prefix" | "base-unit" | "unit" => {
                        current = Some((tag, attributes, String::new(), String::new()));
                        names = 0;
                    }
                    "name" => {
                        names += 1;
                        in_name = names == 1;
                    }
                    "value" => {
                        if let Some(c) = current.as_mut() {
                            c.3 = attributes.get("value").cloned().unwrap_or_default();
                        }
                    }
                    _ => {}
                }
            }
            Event::Text(t) if in_name => {
                if let Some(c) = current.as_mut() {
                    c.2.push_str(&t.decode().unwrap());
                }
            }
            Event::GeneralRef(r) if in_name => {
                if let Some(c) = current.as_mut()
                    && let Ok(Some(ch)) = r.resolve_char_ref()
                {
                    c.2.push(ch);
                }
            }
            Event::End(e) => match e.name().as_ref() {
                b"name" => in_name = false,
                b"prefix" | b"base-unit" | b"unit" => elements.extend(current.take()),
                _ => {}
            },
            Event::Eof => return elements,
            _ => {}
        }
    }
}

fn flag(attributes: &HashMap<String, String>, key: &str) -> bool {
    attributes.get(key).is_some_and(|v| v == "yes")
}

#[test]
fn every_unit_of_the_xml_is_in_the_registry_and_nothing_else() {
    let xml: BTreeSet<String> = elements()
        .into_iter()
        .filter(|(tag, ..)| tag != "prefix")
        .map(|(_, attributes, ..)| attributes["Code"].clone())
        .collect();
    let registry: BTreeSet<String> = get_all_units().iter().map(|u| u.code.to_string()).collect();
    assert_eq!(xml.len(), 312);
    assert_eq!(registry, xml);
    // Sorted by code, for the binary search of `find_unit`
    assert!(get_all_units().windows(2).all(|w| w[0].code < w[1].code));
}

#[test]
fn units_have_the_attributes_of_the_xml() {
    for (tag, attributes, name, _) in elements().into_iter().filter(|(tag, ..)| tag != "prefix") {
        let code = &attributes["Code"];
        let unit = find_unit(code).unwrap_or_else(|| panic!("{code}"));
        let base = tag == "base-unit";
        assert_eq!(unit.is_base, base, "{code}");
        assert_eq!(unit.code_ci, attributes["CODE"], "{code}");
        // White space in names is collapsed to single spaces
        let name = name.split_whitespace().collect::<Vec<_>>().join(" ");
        assert_eq!(unit.display_name, name, "{code}");
        assert_eq!(
            unit.class,
            attributes.get("class").map_or("", String::as_str),
            "{code}"
        );
        // Base units are metric (UCUM §11.3)
        assert_eq!(
            unit.is_metric,
            base || flag(&attributes, "isMetric"),
            "{code}"
        );
        assert_eq!(
            unit.special.is_special(),
            flag(&attributes, "isSpecial"),
            "{code}"
        );
        assert_eq!(
            unit.special.is_arbitrary(),
            flag(&attributes, "isArbitrary"),
            "{code}"
        );
        assert_eq!(
            unit.arbitrary.is_empty(),
            !unit.special.is_arbitrary(),
            "{code}"
        );
        assert!(
            !unit.factor.is_zero() && !unit.factor.is_negative(),
            "{code}"
        );
    }
}

#[test]
fn every_prefix_of_the_xml_is_in_the_registry_with_its_value() {
    let prefixes: Vec<_> = elements()
        .into_iter()
        .filter(|(tag, ..)| tag == "prefix")
        .collect();
    assert_eq!(prefixes.len(), 24);
    assert_eq!(get_all_prefixes().len(), 24);
    for (_, attributes, name, value) in prefixes {
        let code = &attributes["Code"];
        let prefix = find_prefix(code).unwrap_or_else(|| panic!("{code}"));
        assert_eq!(prefix.symbol_ci, attributes["CODE"], "{code}");
        assert_eq!(prefix.display_name, name.trim(), "{code}");
        assert_eq!(prefix.factor, Number::parse(&value).unwrap(), "{code}");
    }
    assert_eq!(find_prefix("Ki").unwrap().factor, Number::from_i64(1024));
}

#[test]
fn units_frequently_used_in_practice_exist() {
    for code in [
        "m", "s", "g", "rad", "K", "C", "cd", "A", "mol", "L", "l", "min", "h", "d", "wk", "Cel",
        "bar", "Pa", "N", "J", "W", "V", "Ohm", "Hz", "%", "[NTU]", "[FNU]", "[IU]", "[degF]",
        "m[Hg]", "[in_i]", "[lb_av]",
    ] {
        assert!(find_unit(code).is_some(), "{code}");
    }
}

#[test]
fn the_copy_of_the_tables_in_spec_is_the_one_the_registry_is_built_from() {
    assert_eq!(
        include_str!("../spec/ucum-essence.xml"),
        include_str!("../ucum-essence.xml"),
        "spec/ucum-essence.xml and ucum-essence.xml differ"
    );
}
