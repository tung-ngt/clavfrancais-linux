use clavfrancais_engine::{
    char_buffer::ResizableCharBuffer,
    engine::{setup_key_combination_map, CombinationTarget, Engine},
};
use xkbcommon::xkb::{
    keysym_to_utf32,
    keysyms::{KEY_BackSpace, KEY_space},
};
use zbus::{interface, object_server::SignalEmitter};
use zvariant::{OwnedValue, Value};

use crate::{
    flags::{Capability, KeyState},
    preedit_lose_focus::PreeditLoseFocusMode,
    special_keys::UPPER_KEYS,
    text::Text,
};

pub struct IBusEngine {
    engine: Engine<ResizableCharBuffer>,
    buffer: String,
    //skip_sync_text: bool,
    //skip_del_text: bool,
}

impl Default for IBusEngine {
    fn default() -> Self {
        Self {
            engine: Engine::new(setup_key_combination_map(), ResizableCharBuffer::new()),
            buffer: Default::default(),
            //skip_sync_text: false,
            //skip_del_text: false,
        }
    }
}

impl IBusEngine {
    pub async fn sync_preedit_buffer(
        &mut self,
        emitter: &SignalEmitter<'_>,
    ) -> Result<(), zbus::Error> {
        //if !self.buffer.is_empty() {
        //    println!("will skip");
        //    self.skip_sync_text = true;
        //}

        emitter
            .update_preedit_text(
                Value::from(Text::new(&self.buffer)),
                self.buffer.len() as u32,
                !self.buffer.is_empty(),
                if !self.buffer.is_empty() {
                    PreeditLoseFocusMode::Commit.value()
                } else {
                    PreeditLoseFocusMode::Clear.value()
                },
            )
            .await
    }

    pub async fn clear_preedit(&mut self, emitter: &SignalEmitter<'_>) {
        self.engine.clear_char_buffer();
        self.buffer.clear();
        self.sync_preedit_buffer(emitter)
            .await
            .expect("cannot clear");
    }

    pub async fn commit_buffer(&mut self, emitter: &SignalEmitter<'_>) -> bool {
        if self.buffer.is_empty() {
            return false;
        }
        //self.skip_sync_text = true;

        emitter
            .commit_text(Value::from(Text::new(&self.buffer)))
            .await
            .expect("Cannot emmit");

        self.clear_preedit(emitter).await;
        true
    }

    pub async fn handle_backspace(&mut self, emitter: &SignalEmitter<'_>) -> bool {
        if self.buffer.is_empty() {
            return false;
        }

        self.buffer.pop();
        self.engine.backspace();

        self.sync_preedit_buffer(emitter).await.expect("failed");

        true
    }

    pub async fn handle_combination(&mut self, emitter: &SignalEmitter<'_>, unicode_char: char) {
        if let Some(combination_target) = self.engine.add_char(unicode_char) {
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
            self.buffer.push(unicode_char);
        }

        println!("buffer: {}", self.buffer.as_bytes()[0]);

        self.sync_preedit_buffer(emitter)
            .await
            .expect("cannot update preedit");
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
        println!(
            "keyval: {}, key_code: {}, state: {}",
            keyval, keycode, state
        );
        //return false;

        //Skip key up event
        if (state & KeyState::RELEASE) != 0 {
            println!("skip key release");
            return false;
        }

        //Skip control and alt combination
        if (state & KeyState::LOOSE_FOCUS) != 0 {
            println!("skip ctrl alt super combination");
            return false;
        }

        //Skip shift and cap
        if UPPER_KEYS.contains(&keyval) {
            println!("skip upper keys");
            return false;
        }

        if keyval == KEY_BackSpace {
            println!("handle backspace");
            return self.handle_backspace(&emitter).await;
        }

        if keyval == KEY_space {
            println!("handle space");
            self.buffer.push(' ');
            return self.commit_buffer(&emitter).await;
        }

        let unicode_char = keyval_to_unicode_char(keyval);

        println!(
            "keyval: {}, unicode: {:?}, key_code: {}, state: {}",
            keyval, unicode_char, keycode, state
        );

        let Some(unicode_char) = unicode_char else {
            println!("cannot convert unicode, commit buffer and skip");
            self.commit_buffer(&emitter).await;
            return false;
        };

        self.handle_combination(&emitter, unicode_char).await;

        true
    }

    pub async fn focus_in(&mut self) {
        //self.skip_sync_text = true;
        println!("focus in");
    }

    pub async fn focus_out(&self) {
        println!("focus out\n\n\n");
    }

    pub async fn disable(&self) {
        println!("disable");
    }

    pub async fn enable(&self, #[zbus(signal_emitter)] emitter: SignalEmitter<'_>) {
        println!("enable");
        emitter
            .require_surrounding_text()
            .await
            .expect("cannot tell require surrounding text");
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
        self.clear_preedit(&emitter).await;
    }

    pub async fn set_capabilities(&self, capabilities: u32) {
        println!("set_capabilities: {:b}", capabilities);

        if (capabilities & Capability::PREEDIT_TEXT) != 0 {
            println!("has preedit");
        }

        if (capabilities & Capability::AUXILIARY_TEXT) != 0 {
            println!("has auxiliary");
        }

        if (capabilities & Capability::SURROUNDING_TEXT) != 0 {
            println!("has surrounding");
        }

        if (capabilities & Capability::FOCUS) != 0 {
            println!("has focus");
        }

        if (capabilities & Capability::PROPERTY) != 0 {
            println!("has property");
        }

        if (capabilities & Capability::LOOKUP_TABLE) != 0 {
            println!("has lookup table");
        }
    }

    pub async fn set_surrounding_text(
        &mut self,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
        text: OwnedValue,
        cursor_pos: u32,
        anchor_pos: u32,
    ) {
        //let _ = anchor_pos;
        //println!("set surrounding text");
        //
        //let text: Text = text.try_into().expect("can not get text");
        //let text = text.get();
        //println!("----------------------------------");
        //println!("text: {:?}", text);
        //println!("----------------------------------");
        //println!("cursor pos: {}", cursor_pos);
        //println!(
        //    "cursor on: {:?}",
        //    text.chars().nth(cursor_pos as usize).unwrap_or('\0')
        //);
        //let current_word = word_before(text, cursor_pos as usize);
        //println!("word before'{}'", current_word);
        //if self.skip_del_text {
        //    println!("ignore set text this time\n");
        //    self.skip_del_text = false;
        //    return;
        //}
        //
        //if self.skip_sync_text {
        //    println!("ignore set text this time\n");
        //    self.skip_sync_text = false;
        //    return;
        //}
        //println!();
        //
        //let word_length = current_word.chars().count() as u32;
        //if word_length > 0 {
        //    println!("deleting: {:?}, chars: {}", current_word, word_length);
        //
        //    self.engine.clear_char_buffer();
        //    self.buffer.clear();
        //    self.engine.push_str_without_processing(current_word);
        //    self.buffer.push_str(current_word);
        //    self.sync_preedit_buffer(&emitter).await.expect("failed");
        //    println!("have sync before delete okkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk");
        //
        //    self.skip_del_text = true;
        //    println!("will skip");
        //    emitter
        //        .delete_surrounding_text(-(word_length as i32), word_length)
        //        .await
        //        .expect("cannot delete");
        //    println!("have delete hiiiiiiiiiiiiiiiiiiiiiiiiiii");
        //}
    }

    pub async fn set_content_type(&self) {
        println!("set content type");
    }

    pub async fn set_cursor_location(&self) {
        println!("set new cursor location, ",);
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

    #[zbus(signal)]
    pub async fn delete_surrounding_text(
        emitter: &SignalEmitter<'_>,
        offset: i32,
        nchars: u32,
    ) -> Result<(), zbus::Error>;

    #[zbus(signal)]
    pub async fn require_surrounding_text(emitter: &SignalEmitter<'_>) -> Result<(), zbus::Error>;
}

fn keyval_to_unicode_char(keyval: u32) -> Option<char> {
    let u = keysym_to_utf32(keyval.into());
    if u == 0 {
        return None;
    }
    char::from_u32(u).and_then(|c| if char::is_control(c) { None } else { Some(c) })
}

//fn word_before(text: &str, n: usize) -> &str {
//    let mut last_whitespace_byte_index = None;
//    let mut end_byte_index = None;
//
//    for (i, (byte_index, character)) in text.char_indices().enumerate() {
//        if i == n {
//            break;
//        }
//
//        if character.is_whitespace() {
//            last_whitespace_byte_index = Some(byte_index);
//        }
//
//        end_byte_index = Some(byte_index + character.len_utf8());
//    }
//
//    let end_byte_index = end_byte_index.unwrap_or(0);
//    let last_whitespace_byte_index = last_whitespace_byte_index.map(|x| x + 1).unwrap_or(0);
//
//    &text[last_whitespace_byte_index..end_byte_index]
//}
