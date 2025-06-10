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
    pub fn new(
        name: &'static str,
        description: &'static str,
        version: &'static str,
        license: &'static str,
        author: &'static str,
        homepage: &'static str,
        command_line: &'static str,
        textdomain: &'static str,
    ) -> Self {
        Self {
            struct_type: "IBusComponent".to_owned(),

            name: name.to_owned(),
            description: description.to_owned(),
            version: version.to_owned(),
            license: license.to_owned(),
            author: author.to_owned(),
            homepage: homepage.to_owned(),
            command_line: command_line.to_owned(),
            textdomain: textdomain.to_owned(),

            ..Default::default()
        }
    }

    pub fn add_engine(&mut self, engine: EngineDesc) {
        self.engines
            .push(OwnedValue::try_from(engine).expect("cannot add engine"));
    }
}
