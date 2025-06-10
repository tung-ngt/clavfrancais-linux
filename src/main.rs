use std::future;

use clavfrancais_linux::{
    bus::Bus, component::Component, engine::Engine, engine_desc::EngineDesc, factory::Factory,
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
    let mut component = Component::new("abc", "abc", "abc", "abc", "abc", "abc", "abc", "abc");
    let engine_desc = EngineDesc::new("abcd", "abc", "abc", "abc", "abc", "abc", "abc", "abc");
    component.add_engine(engine_desc);

    let object_path = "/org/freedesktop/IBus/Engine/test/test";
    let engine = Engine;
    bus.export_engine(engine, object_path).await;
    let factory = Factory::new(OwnedObjectPath::try_from(object_path).unwrap());
    bus.export_factory(factory).await;

    bus.register_component(component).await;

    bus.set_global_engine("abcd".to_string()).await;
    println!("fuck");

    loop {
        future::pending::<()>().await;
    }
}
