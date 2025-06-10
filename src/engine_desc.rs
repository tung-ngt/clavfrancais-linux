use std::collections::HashMap;

use serde::{Deserialize, Serialize};
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

//impl Arg for EngineDesc {
//    const ARG_TYPE: dbus::arg::ArgType = ArgType::Struct;
//    fn signature() -> dbus::Signature<'static> {
//
//(sa{sv}ssssssssusssssss)
//(sa{sv}ssssssssusssssss)
//        Signature::from("(sa{sv}ssssssssusssssss)")
//    }
//}
//
//impl Append for EngineDesc {
//    fn append(self, i: &mut dbus::arg::IterAppend) {
//        self.append_by_ref(i);
//    }
//
//    fn append_by_ref(&self, i: &mut dbus::arg::IterAppend) {
//        i.append_struct(|s| {
//            s.append("IBusEngineDesc");
//            s.append(HashMap::<String, String>::new());
//
//            s.append(self.name);
//            s.append(self.longname);
//            s.append(self.description);
//            s.append(self.language);
//            s.append(self.license);
//            s.append(self.author);
//            s.append(self.icon);
//            s.append(self.layout);
//            s.append(self.rank);
//
//            s.append(self.hotkeys);
//            s.append(self.symbol);
//            s.append(self.setup);
//            s.append(self.layout_variant);
//            s.append(self.layout_option);
//            s.append(self.version);
//            s.append(self.textdomain);
//        });
//    }
//}
