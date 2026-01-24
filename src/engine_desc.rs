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
    pub fn new(name: &str) -> Self {
        Self {
            struct_type: "IBusEngineDesc".to_string(),
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn longname(mut self, longname: &str) -> Self {
        self.longname = longname.to_string();
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn language(mut self, language: &str) -> Self {
        self.language = language.to_string();
        self
    }

    pub fn license(mut self, license: &str) -> Self {
        self.license = license.to_string();
        self
    }

    pub fn author(mut self, author: &str) -> Self {
        self.author = author.to_string();
        self
    }

    pub fn icon(mut self, icon: &str) -> Self {
        self.icon = icon.to_string();
        self
    }

    pub fn layout(mut self, layout: &str) -> Self {
        self.layout = layout.to_string();
        self
    }

    pub fn rank(mut self, rank: u32) -> Self {
        self.rank = rank;
        self
    }

    pub fn hotkeys(mut self, hotkeys: &str) -> Self {
        self.hotkeys = hotkeys.to_string();
        self
    }

    pub fn symbol(mut self, symbol: &str) -> Self {
        self.symbol = symbol.to_string();
        self
    }

    pub fn setup(mut self, setup: &str) -> Self {
        self.setup = setup.to_string();
        self
    }

    pub fn layout_variant(mut self, layout_variant: &str) -> Self {
        self.layout_variant = layout_variant.to_string();
        self
    }

    pub fn layout_option(mut self, layout_option: &str) -> Self {
        self.layout_option = layout_option.to_string();
        self
    }

    pub fn version(mut self, version: &str) -> Self {
        self.version = version.to_string();
        self
    }

    pub fn textdomain(mut self, textdomain: &str) -> Self {
        self.textdomain = textdomain.to_string();
        self
    }
}
