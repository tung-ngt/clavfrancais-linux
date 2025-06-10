use std::collections::HashMap;

use zvariant::{OwnedValue, Value};

#[derive(OwnedValue, Value)]
pub struct Text {
    struct_type: String,
    attachments: HashMap<String, OwnedValue>,
    text: String,
    attr_list: OwnedValue,
}

#[derive(OwnedValue, Value)]
struct AtrrList {
    struct_type: String,
    attachments: HashMap<String, OwnedValue>,
    attributes: Vec<OwnedValue>,
}

#[derive(OwnedValue, Value)]
struct Attribute {
    struct_type: String,
    attachments: HashMap<String, OwnedValue>,
    attribute_type: u32,
    value: u32,
    start_index: u32,
    end_index: u32,
}

impl AtrrList {
    pub fn new() -> Self {
        Self {
            struct_type: "IBusAttrList".to_string(),
            attachments: HashMap::new(),
            attributes: Vec::new(),
        }
    }
}

impl Text {
    pub fn new(text: &'_ str) -> Self {
        Self {
            struct_type: "IBusText".to_string(),
            attachments: HashMap::new(),
            text: text.to_string(),
            attr_list: OwnedValue::try_from(AtrrList::new()).expect("failed to create text"),
        }
    }
}
