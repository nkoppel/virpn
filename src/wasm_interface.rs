pub use wasm_bindgen::prelude::*;
use lazy_static::lazy_static;

use std::sync::{Arc, Mutex};

use std::borrow::Borrow;

use crate::modes::*;
use crate::terminal::*;

lazy_static! {
    pub static ref STATE: Arc<Ui> = Arc::new(Ui::build());
}

#[wasm_bindgen]
pub fn eval_key(s: String) {
    unsafe {
        let mut state = STATE.clone();
        let ui = Arc::get_mut_unchecked(&mut state);

        ui.call_history();
        let key =
            match &s[..] {
                "ArrowUp"    => KeyUp          ,
                "ArrowDown"  => KeyDown        ,
                "ArrowLeft"  => KeyLeft        ,
                "ArrowRight" => KeyRight       ,
                "Delete"     => KeyDC          ,
                "Backspace"  => KeyBackspace   ,
                "Enter"      => Character('\n'),
                _ if s.len() == 1 => Character(s.chars().next().unwrap()),
                _ => return
            };
        ui.eval_key(key);
        ui.show(&initscr());
    }
}
