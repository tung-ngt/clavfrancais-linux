use zbus::interface;
use zvariant::OwnedObjectPath;

pub struct Factory {
    object_path: OwnedObjectPath,
}

impl Factory {
    pub fn new(object_path: OwnedObjectPath) -> Self {
        Factory { object_path }
    }
}

#[interface(name = "org.freedesktop.IBus.Factory")]
impl Factory {
    pub async fn create_engine(&self, _engine_name: String) -> OwnedObjectPath {
        println!("wtf");
        self.object_path.clone()
    }

    pub async fn destroy(&self) {}
}
