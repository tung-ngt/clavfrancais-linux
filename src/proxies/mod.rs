use zbus::proxy;
use zvariant::Value;

#[proxy(
    default_service = "org.freedesktop.IBus",
    default_path = "/org/freedesktop/IBus"
)]
pub trait IBus {
    fn register_component(&self, component: Value<'_>) -> zbus::Result<()>;
    fn set_global_engine(&self, engine_name: String) -> zbus::Result<()>;
}
