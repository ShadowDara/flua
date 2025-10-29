use mlua::{Error, Lua, Result, Value};
use serde_json::Value as JsonValue;
use xmltree;
use xmltree::{Element, XMLNode};

use crate::utils::json_utils;

type XmlResult<T> = std::result::Result<T, mlua::Error>;

pub fn register(lua: &Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?;

    // XML decode: String -> Lua Table
    let xml_decode = lua.create_function(|lua, xml_str: String| {
        let root = xmltree::Element::parse(xml_str.as_bytes()).map_err(Error::external)?;
        let json_value = element_to_value(&root);
        json_utils::json_to_lua(lua, &json_value)
    })?;

    // XML encode: Lua Table -> String
    let xml_encode = lua.create_function(|_, value: Value| {
        let json_value = json_utils::lua_to_json(&value)?;
        let root = value_to_element("root", &json_value)?;
        let mut writer = Vec::new();
        root.write(&mut writer).map_err(Error::external)?;
        let xml_str = String::from_utf8(writer).map_err(Error::external)?;
        Ok(xml_str)
    })?;

    table.set("decode", xml_decode)?;
    table.set("encode", xml_encode)?;

    Ok(table)
}

// Helper: xmltree::Element from serde_json::Value
fn value_to_element(name: &str, val: &JsonValue) -> XmlResult<Element> {
    let mut elem = Element::new(name);

    match val {
        JsonValue::Object(map) => {
            for (k, v) in map {
                if k.starts_with("@") {
                    elem.attributes
                        .insert(k[1..].to_string(), v.as_str().unwrap_or("").to_string());
                } else if k == "#text" {
                    elem.children
                        .push(XMLNode::Text(v.as_str().unwrap_or("").to_string()));
                } else {
                    match v {
                        JsonValue::Array(arr) => {
                            for item in arr {
                                let child = value_to_element(k, item)?;
                                elem.children.push(XMLNode::Element(child));
                            }
                        }
                        _ => {
                            let child = value_to_element(k, v)?;
                            elem.children.push(XMLNode::Element(child));
                        }
                    }
                }
            }
        }
        JsonValue::Array(arr) => {
            for item in arr {
                let child = value_to_element(name, item)?;
                elem.children.push(XMLNode::Element(child));
            }
        }
        JsonValue::String(s) => {
            elem.children.push(XMLNode::Text(s.clone()));
        }
        JsonValue::Number(n) => {
            elem.children.push(XMLNode::Text(n.to_string()));
        }
        JsonValue::Bool(b) => {
            elem.children.push(XMLNode::Text(b.to_string()));
        }
        JsonValue::Null => {}
    }

    Ok(elem)
}

// Helper: xmltree::Element -> serde_json::Value
fn element_to_value(elem: &xmltree::Element) -> JsonValue {
    use serde_json::json;
    let mut map = serde_json::Map::new();

    // Handle attributes: prefixed with @
    for (k, v) in &elem.attributes {
        map.insert(format!("@{}", k), json!(v));
    }

    // Count child element names to detect duplicates
    let mut tag_counts = std::collections::HashMap::new();
    for child in &elem.children {
        if let XMLNode::Element(child_elem) = child {
            *tag_counts.entry(&child_elem.name).or_insert(0) += 1;
        }
    }

    // Children
    for child in &elem.children {
        if let XMLNode::Element(child_elem) = child {
            let child_value = element_to_value(child_elem);
            let tag = &child_elem.name;
            let count = tag_counts.get(tag).copied().unwrap_or(0);

            if let Some(existing) = map.get_mut(tag) {
                if existing.is_array() {
                    existing.as_array_mut().unwrap().push(child_value);
                } else {
                    *existing = JsonValue::Array(vec![existing.clone(), child_value]);
                }
            } else {
                if count > 1 {
                    map.insert(tag.clone(), JsonValue::Array(vec![child_value]));
                } else {
                    map.insert(tag.clone(), child_value);
                }
            }
        }
    }

    // Collect text nodes
    let text_content = elem
        .children
        .iter()
        .filter_map(|node| {
            if let XMLNode::Text(t) = node {
                Some(t.trim().to_string())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    if !text_content.is_empty() {
        if map.is_empty() {
            // If no attributes or children, return string directly
            return JsonValue::String(text_content);
        } else {
            // Mixed content: include as #text
            map.insert("#text".to_string(), JsonValue::String(text_content));
        }
    }

    JsonValue::Object(map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlua::{Lua, Value};
    use serde_json::json;

    fn setup_lua() -> Lua {
        Lua::new()
    }

    #[test]
    fn test_simple_decode() {
        let lua = setup_lua();

        // Modul manuell registrieren
        let xml_mod = register(&lua).unwrap();
        lua.globals().set("xml", xml_mod).unwrap();

        let xml = "<note><to>Tove</to><from>Jani</from></note>";

        // "xml.decode" aus dem Lua-Kontext holen
        let xml_table: mlua::Table = lua.globals().get("xml").unwrap();
        let decode_func: mlua::Function = xml_table.get("decode").unwrap();

        let result: Value = decode_func.call(xml).unwrap();

        match result {
            Value::Table(t) => {
                assert!(t.contains_key("to").unwrap());
                assert!(t.contains_key("from").unwrap());
            }
            _ => panic!("Expected table"),
        }
    }

    #[test]
    fn test_simple_encode() {
        let json_value = json!({
            "note": {
                "to": "Tove",
                "from": "Jani"
            }
        });
        let elem = value_to_element("root", &json_value).unwrap();
        assert_eq!(elem.name, "root");
        assert_eq!(elem.children.len(), 1);
    }

    #[test]
    fn test_element_to_value_with_attributes() {
        let xml = r#"<book id="1" lang="en"><title>Rust</title></book>"#;
        let elem = xmltree::Element::parse(xml.as_bytes()).unwrap();
        let json_value = element_to_value(&elem);

        assert_eq!(
            json_value,
            json!({
                "@id": "1",
                "@lang": "en",
                "title": "Rust"
            })
        );
    }

    #[test]
    fn test_array_elements() {
        let xml = r#"<root><item>A</item><item>B</item><item>C</item></root>"#;
        let elem = xmltree::Element::parse(xml.as_bytes()).unwrap();
        let json_value = element_to_value(&elem);

        assert_eq!(
            json_value,
            json!({
                "item": ["A", "B", "C"]
            })
        );
    }

    #[test]
    fn test_mixed_content() {
        let xml = r#"<p>Hello <b>World</b>!</p>"#;
        let elem = xmltree::Element::parse(xml.as_bytes()).unwrap();
        let val = element_to_value(&elem);

        assert_eq!(
            val,
            json!({
                "b": "World",
                "#text": "Hello !"
            })
        );
    }

    #[test]
    fn test_round_trip_simple() {
        let xml = r#"<greeting>Hello</greeting>"#;
        let elem = xmltree::Element::parse(xml.as_bytes()).unwrap();
        let json_value = element_to_value(&elem);

        let elem2 = value_to_element("greeting", &json_value).unwrap();
        let mut writer = Vec::new();
        elem2.write(&mut writer).unwrap();
        let xml_back = String::from_utf8(writer).unwrap();

        assert!(xml_back.contains("Hello"));
    }

    #[test]
    fn test_round_trip_with_attributes_and_children() {
        let xml = r#"<person id="42"><name>John</name><age>30</age></person>"#;
        let elem = xmltree::Element::parse(xml.as_bytes()).unwrap();
        let json_value = element_to_value(&elem);

        assert_eq!(
            json_value,
            json!({
                "@id": "42",
                "name": "John",
                "age": "30"
            })
        );

        let elem2 = value_to_element("person", &json_value).unwrap();
        let mut buf = Vec::new();
        elem2.write(&mut buf).unwrap();
        let xml_out = String::from_utf8(buf).unwrap();

        assert!(xml_out.contains(r#"id="42""#));
        assert!(xml_out.contains("<name>John</name>"));
        assert!(xml_out.contains("<age>30</age>"));
    }

    #[test]
    fn test_null_and_bool_values() {
        let json_value = json!({
            "root": {
                "active": true,
                "count": 5,
                "empty": null
            }
        });

        let elem = value_to_element("root", &json_value).unwrap();
        let mut buf = Vec::new();
        elem.write(&mut buf).unwrap();
        let xml_out = String::from_utf8(buf).unwrap();

        assert!(xml_out.contains("<active>true</active>"));
        assert!(xml_out.contains("<count>5</count>"));
        assert!(xml_out.contains("<empty")); // ✅ geändert
    }

    #[test]
    fn test_array_round_trip() {
        let json_value = json!({
            "root": {
                "items": [
                    {"name": "A"},
                    {"name": "B"}
                ]
            }
        });

        // JSON → XML
        let elem = value_to_element("root", &json_value).unwrap();
        let mut buf = Vec::new();
        elem.write(&mut buf).unwrap();
        let xml_str = String::from_utf8(buf).unwrap();

        // XML → JSON
        let elem_back = xmltree::Element::parse(xml_str.as_bytes()).unwrap();
        let json_back = element_to_value(&elem_back);

        println!(
            "json_back = {}",
            serde_json::to_string_pretty(&json_back).unwrap()
        );

        // Zugriff auf root → items
        let items = json_back
            .get("root")
            .and_then(|r| r.get("items"))
            .and_then(|i| i.as_array())
            .unwrap_or_else(|| {
                panic!(
                    "json_back had unexpected structure: {}",
                    serde_json::to_string_pretty(&json_back).unwrap()
                )
            });

        assert_eq!(items.len(), 2, "Expected 2 array items");

        let names: Vec<_> = items
            .iter()
            .filter_map(|v| v.get("name").and_then(|n| n.as_str()))
            .collect();

        assert_eq!(names, vec!["A", "B"]);
    }
}
