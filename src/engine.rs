use zbus::{interface, object_server::SignalEmitter};
use zvariant::Value;

use crate::text::Text;

pub struct Engine;

#[interface(name = "org.freedesktop.IBus.Engine")]
impl Engine {
    pub async fn process_key_event(
        &self,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
        keyval: u32,
        keycode: u32,
        state: u32,
    ) -> bool {
        let cedille_code: u32 = 'ç'.into();
        let b_code: u32 = 'b'.into();

        println!("{}, {}, {}", keyval, keycode, state);

        return false;

        //let release_mask = 1 << 30;
        //
        //if (state & release_mask) != 0 {
        //    println!("release");
        //    return false;
        //}
        //
        //if let Some(c) = char::from_u32(keyval) {
        //    println!("yo {}", c);
        //    emitter
        //        .update_preedit_text(Value::from(Text::new(c.to_string().as_str())), 2, true, 1)
        //        .await
        //        .expect("cannot update preedit");
        //    return false;
        //} else {
        //    return false;
        //}

        //if keyval == b_code {
        //    //println!("abc");
        //    emitter
        //        .commit_text(Value::from(Text::new("ç")))
        //        .await
        //        .expect("Cannot emmit");
        //    true
        //} else {
        //    false
        //}
    }

    pub async fn focus_in(&self) {
        println!("focus in");
    }

    #[zbus(signal)]
    pub async fn update_preedit_text(
        emitter: &SignalEmitter<'_>,
        text: Value<'_>,
        cursor_pos: u32,
        visible: bool,
        mode: u32,
    ) -> Result<(), zbus::Error>;

    #[zbus(signal)]
    pub async fn commit_text(
        emitter: &SignalEmitter<'_>,
        text: Value<'_>,
    ) -> Result<(), zbus::Error>;
}
