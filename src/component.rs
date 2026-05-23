use std::collections::HashMap;

use serde::Serialize;
use zvariant::{OwnedValue, Type, Value};

use crate::engine_desc::EngineDesc;

#[derive(Serialize, Type, Value, OwnedValue, Default)]
pub struct Component {
    struct_type: String,
    attachments: HashMap<String, OwnedValue>,

    name: String,
    description: String,
    version: String,
    license: String,
    author: String,
    homepage: String,
    command_line: String,
    textdomain: String,

    observations: Vec<OwnedValue>,
    engines: Vec<OwnedValue>,
}

impl Component {
    pub const IBUS_CLASS: &str = "IBusComponent";

    pub fn new(name: &str) -> Self {
        Self {
            struct_type: Self::IBUS_CLASS.to_owned(),
            name: name.to_owned(),
            ..Default::default()
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn version(mut self, version: &str) -> Self {
        self.version = version.to_string();
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

    pub fn homepage(mut self, homepage: &str) -> Self {
        self.homepage = homepage.to_string();
        self
    }

    pub fn command_line(mut self, command_line: &str) -> Self {
        self.command_line = command_line.to_string();
        self
    }

    pub fn textdomain(mut self, textdomain: &str) -> Self {
        self.textdomain = textdomain.to_string();
        self
    }

    pub fn add_engine(mut self, engine: EngineDesc) -> Self {
        self.engines
            .push(OwnedValue::try_from(engine).expect("cannot add engine"));
        self
    }
}
