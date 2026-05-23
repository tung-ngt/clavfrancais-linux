use std::{env, future};

use clavfrancais_linux::{
    bus::Bus, component::Component, engine::IBusEngine, engine_desc::EngineDesc, factory::Factory,
};

use zvariant::OwnedObjectPath;

#[tokio::main]
async fn main() {
    let bus = Bus::builder().default_ibus_address().build().await;

    let engine_desc = EngineDesc::new("clavfrancais")
        .longname("clavfrancais")
        .description("French input method for qwert keyboard layout")
        .language("fr")
        .license("None")
        .author("Nguyen Thanh Tung")
        .icon("icon.jpg")
        .layout("us");

    let component = Component::new("clavfrancais")
        .description("French input method for qwert keyboard layout")
        .version("0.1.0")
        .license("None")
        .author("Nguyen Thanh Tung")
        .homepage("")
        .command_line("")
        .textdomain("")
        .add_engine(engine_desc);

    let object_path = "/org/freedesktop/IBus/Engine/Clavfrancais/engine";
    let engine = IBusEngine::default();
    bus.export_engine(engine, object_path).await;
    let factory = Factory::new(OwnedObjectPath::try_from(object_path).unwrap());
    bus.export_factory(factory).await;

    bus.register_component(component).await;

    println!("Engine itialized");

    // Only imediately set the engine if called in standalone mode (not from ibus)
    if let Some(arg) = env::args().nth(1)
        && arg == "standalone"
    {
        println!("Running in standalone mode, setting the engine to clavfrancais");
        bus.set_global_engine("clavfrancais".to_string()).await;
    } else {
        println!("Waiting for user to change clavfrancais engine")
    }

    loop {
        future::pending::<()>().await;
    }
}
