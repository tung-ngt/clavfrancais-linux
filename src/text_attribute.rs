use std::collections::HashMap;

use zvariant::{OwnedValue, Value};

#[derive(OwnedValue, Value)]
pub struct Attribute {
    struct_type: String,
    attachments: HashMap<String, OwnedValue>,
    attribute_type: u32,
    value: u32,
    start_index: u32,
    end_index: u32,
}

#[allow(unused)]
pub enum AtrributeType {
    Underline,
    Forground,
    Background,
    Hint,
}

impl AtrributeType {
    pub fn value(&self) -> u32 {
        match self {
            Self::Underline => 1,
            Self::Forground => 2,
            Self::Background => 3,
            Self::Hint => 4,
        }
    }
}

#[allow(unused)]
pub enum Underline {
    None,
    Single,
    Double,
    Low,
    Error,
}

#[allow(unused)]
impl Underline {
    pub fn value(&self) -> u32 {
        match self {
            Self::None => 0,
            Self::Single => 1,
            Self::Double => 2,
            Self::Low => 3,
            Self::Error => 4,
        }
    }
}

#[allow(unused)]
pub enum PreeditAtribute {
    Default,
    Whole,
    Selection,
    Prediction,
    Prefix,
    Suffix,
    ErrorSpelling,
    ErrorCompose,
    None,
}

impl PreeditAtribute {
    pub fn value(&self) -> u32 {
        match self {
            Self::Default => 0,
            Self::Whole => 1,
            Self::Selection => 2,
            Self::Prediction => 3,
            Self::Prefix => 4,
            Self::Suffix => 5,
            Self::ErrorSpelling => 6,
            Self::ErrorCompose => 7,
            Self::None => 1 << 8,
        }
    }
}

impl Attribute {
    pub const IBUS_CLASS: &str = "IBusAttribute";

    pub fn underline() -> Self {
        Self {
            struct_type: Self::IBUS_CLASS.to_string(),
            attachments: HashMap::new(),
            attribute_type: AtrributeType::Hint.value(),
            value: PreeditAtribute::Whole.value(),
            start_index: 0,
            end_index: u32::MAX,
        }
    }
}

#[derive(OwnedValue, Value)]
pub struct AttrList {
    struct_type: String,
    attachments: HashMap<String, OwnedValue>,
    attributes: Vec<OwnedValue>,
}

impl AttrList {
    pub const IBUS_CLASS: &str = "IBusAttrList";

    pub fn with_underline() -> Self {
        Self {
            struct_type: Self::IBUS_CLASS.to_string(),
            attachments: HashMap::new(),
            attributes: vec![
                OwnedValue::try_from(Attribute::underline()).expect("cannot create Attribute")
            ],
        }
    }
}

impl Default for AttrList {
    fn default() -> Self {
        Self {
            struct_type: Self::IBUS_CLASS.to_string(),
            attachments: HashMap::new(),
            attributes: Vec::new(),
        }
    }
}
