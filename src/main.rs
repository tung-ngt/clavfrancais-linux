use std::future;

use clavfrancais_linux::{
    bus::Bus, component::Component, engine::IBusEngine, engine_desc::EngineDesc, factory::Factory,
    utils::get_ibus_address,
};

use zbus::connection;
use zvariant::OwnedObjectPath;

#[tokio::main]
async fn main() {
    let address = get_ibus_address();
    let connection = connection::Builder::address(address.as_str())
        .expect("Cannot create connection to the address")
        .auth_mechanism(zbus::AuthMechanism::External)
        .build()
        .await
        .expect("Cannot build the connection");

    let bus = Bus::new(connection).await;

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

    bus.set_global_engine("clavfrancais".to_string()).await;
    loop {
        future::pending::<()>().await;
    }
}
