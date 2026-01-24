use super::component::Component;
use crate::{engine::IBusEngine, factory::Factory, proxies::IBusProxy};
use zbus::Connection;
use zvariant::Value;

pub struct Bus<'a> {
    connection: Connection,
    ibus_proxy: IBusProxy<'a>,
}

impl Bus<'_> {
    pub async fn new(connection: Connection) -> Self {
        let ibus_proxy = IBusProxy::new(&connection)
            .await
            .expect("failed create ibus proxy");

        Self {
            connection,
            ibus_proxy,
        }
    }

    pub async fn register_component(&self, component: Component) {
        self.ibus_proxy
            .register_component(Value::from(component))
            .await
            .expect("failed to register_component");
    }

    pub async fn set_global_engine(&self, engine_name: String) {
        self.ibus_proxy
            .set_global_engine(engine_name)
            .await
            .expect("failed to set global engine");
    }

    pub async fn export_factory(&self, factory: Factory) {
        if !self
            .connection
            .object_server()
            .at("/org/freedesktop/IBus/Factory", factory)
            .await
            .expect("failed to export factory")
        {
            println!("factory already exists");
        }
    }

    pub async fn export_engine(&self, engine: IBusEngine, object_path: &'_ str) {
        if !self
            .connection
            .object_server()
            .at(object_path, engine)
            .await
            .expect("failed to export engine")
        {
            println!("egnine already exists");
        }
    }
}
