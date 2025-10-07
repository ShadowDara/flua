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
