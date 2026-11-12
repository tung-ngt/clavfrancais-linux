use std::collections::HashMap;

use zvariant::{OwnedValue, Value};

use crate::text_attribute::AttrList;

#[derive(OwnedValue, Value)]
pub struct Text {
    struct_type: String,
    attachments: HashMap<String, OwnedValue>,
    text: String,
    attr_list: OwnedValue,
}

impl Text {
    pub const IBUS_CLASS: &str = "IBusText";

    pub fn new(text: &'_ str) -> Self {
        let text_len = text.chars().count() as u32;
        Self {
            struct_type: Self::IBUS_CLASS.to_string(),
            attachments: HashMap::new(),
            text: text.to_string(),
            attr_list: OwnedValue::try_from(AttrList::with_underline(text_len))
                .expect("failed to create text"),
        }
    }

    pub fn get(&self) -> &str {
        &self.text
    }
}
