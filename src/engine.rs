use clavfrancais_engine::{
    char_buffer::ResizableCharBuffer,
    engine::{setup_key_combination_map, CombinationTarget, Engine},
};
use xkbcommon::xkb::keysym_to_utf32;
use zbus::{interface, object_server::SignalEmitter};
use zvariant::{OwnedValue, Value};

use crate::{
    modifier::{
        ALT_MASK, CAP_KEY, CONTROL_MASK, LOOSE_FOCUS_MASK, RELEASE_MASK, SHIFT_KEY, SHIFT_MASK,
    },
    text::Text,
    utils::find_word,
};

pub struct IBusEngine {
    engine: Engine<ResizableCharBuffer>,
    buffer: String,
}

impl Default for IBusEngine {
    fn default() -> Self {
        Self {
            engine: Engine::new(setup_key_combination_map(), ResizableCharBuffer::new()),
            buffer: Default::default(),
        }
    }
}

impl IBusEngine {
    pub async fn clear_preedit(&mut self, emitter: SignalEmitter<'_>) {
        self.engine.clear_char_buffer();
        self.buffer.clear();
        emitter
            .update_preedit_text(Value::from(Text::new("")), 0, false, 0)
            .await
            .expect("cannot clear");
    }
}

#[interface(name = "org.freedesktop.IBus.Engine")]
impl IBusEngine {
    pub async fn process_key_event(
        &mut self,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
        keyval: u32,
        keycode: u32,
        state: u32,
    ) -> bool {
        //Skip control and alt combination
        if (state & LOOSE_FOCUS_MASK) != 0 {
            self.clear_preedit(emitter).await;
            return false;
        }

        //Skip key up event
        if (state & RELEASE_MASK) != 0 {
            return false;
        }

        //Skip shift and cap
        if (keyval == CAP_KEY) || (keyval == SHIFT_KEY) {
            return false;
        }

        let u = keysym_to_utf32(keyval.into());
        let unicode_char: Option<char> = char::from_u32(u);

        println!("{}, {:?}, {}, {}", keyval, unicode_char, keycode, state);

        if u == 0 {
            emitter
                .commit_text(Value::from(Text::new(&self.buffer)))
                .await
                .expect("Cannot emmit");

            self.clear_preedit(emitter).await;
            return false;
        }

        if unicode_char == Some('\u{8}') {
            if self.buffer.is_empty() {
                return false;
            }

            self.buffer.pop();
            self.engine.backspace();

            emitter
                .update_preedit_text(
                    Value::from(Text::new(&self.buffer)),
                    self.buffer.len() as u32,
                    true,
                    1,
                )
                .await
                .expect("failed");

            return true;
        };

        if unicode_char == Some('\t') {
            if self.buffer.is_empty() {
                return false;
            }

            emitter
                .commit_text(Value::from(Text::new(&self.buffer)))
                .await
                .expect("Cannot emmit");

            self.clear_preedit(emitter).await;

            return false;
        }

        if unicode_char == Some('\r') {
            if self.buffer.is_empty() {
                return false;
            }

            emitter
                .commit_text(Value::from(Text::new(&self.buffer)))
                .await
                .expect("Cannot emmit");

            self.clear_preedit(emitter).await;

            return false;
        }

        if unicode_char == Some(' ') {
            self.buffer.push(' ');

            emitter
                .commit_text(Value::from(Text::new(&self.buffer)))
                .await
                .expect("Cannot emmit");

            self.clear_preedit(emitter).await;

            return true;
        }

        if let Some(combination_target) = self.engine.add_char(unicode_char.unwrap()) {
            match combination_target {
                CombinationTarget::Replace(c) => {
                    self.buffer.push(c);
                }
                CombinationTarget::Combine(c) => {
                    self.buffer.pop();
                    self.buffer.push(c);
                }
                CombinationTarget::Revert(f, s) => {
                    self.buffer.pop();
                    self.buffer.push(f);
                    self.buffer.push(s);
                }
            }
        } else {
            self.buffer.push(unicode_char.unwrap());
        }
        emitter
            .update_preedit_text(
                Value::from(Text::new(&self.buffer)),
                self.buffer.len() as u32,
                true,
                1,
            )
            .await
            .expect("cannot update preedit");

        true
    }

    pub async fn focus_in(&self) {
        println!("focus in");
    }

    pub async fn focus_out(&self) {
        println!("focus out");
    }

    pub async fn disable(&self) {
        println!("disable");
    }

    pub async fn enable(&self) {
        println!("enable");
    }

    pub async fn page_up(&self) {
        println!("page_up");
    }

    pub async fn page_down(&self) {
        println!("page_down");
    }

    pub async fn property_activate(&self) {
        println!("property_activate");
    }

    pub async fn property_hide(&self) {
        println!("property_hide");
    }

    pub async fn property_show(&self) {
        println!("property_show");
    }

    pub async fn reset(&mut self, #[zbus(signal_emitter)] emitter: SignalEmitter<'_>) {
        println!("reset");
        self.clear_preedit(emitter).await;
    }

    pub async fn set_capabilities(&self) {
        println!("set_capabilities");
    }

    pub async fn set_content_type(&self) {
        println!("set content type");
    }

    pub async fn set_cursor_location(&self) {
        println!("set cursor location");
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
