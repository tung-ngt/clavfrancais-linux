use std::collections::HashMap;

use serde::Serialize;
use zvariant::{OwnedValue, Type, Value};

#[derive(Serialize, Type, Value, OwnedValue, Default, Clone)]
pub struct EngineDesc {
    struct_type: String,
    attachments: HashMap<String, OwnedValue>,

    name: String,
    longname: String,
    description: String,
    language: String,
    license: String,
    author: String,
    icon: String,
    layout: String,

    rank: u32,

    hotkeys: String,
    symbol: String,
    setup: String,
    layout_variant: String,
    layout_option: String,
    version: String,
    textdomain: String,
}

impl EngineDesc {
    pub fn new(
        name: &'static str,
        longname: &'static str,
        description: &'static str,
        language: &'static str,
        license: &'static str,
        author: &'static str,
        icon: &'static str,
        layout: &'static str,
    ) -> Self {
        Self {
            struct_type: "IBusEngineDesc".to_owned(),

            name: name.to_owned(),
            longname: longname.to_owned(),
            description: description.to_owned(),
            language: language.to_owned(),
            license: license.to_owned(),
            author: author.to_owned(),
            icon: icon.to_owned(),
            layout: layout.to_owned(),
            ..Default::default()
        }
    }
}
