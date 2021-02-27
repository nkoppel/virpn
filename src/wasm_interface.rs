pub use wasm_bindgen::prelude::*;
use lazy_static::lazy_static;

use std::sync::Arc;

use crate::io::*;
use crate::modes::*;

lazy_static! {
    pub static ref STATE: Arc<Ui> = Arc::new(Ui::build());
}

#[wasm_bindgen]
pub fn eval_key(s: String) {
    let ui;
    let mut state = STATE.clone();

    unsafe {
        ui = Arc::get_mut_unchecked(&mut state);
    }

    ui.call_history();

    let key =
        match &s[..] {
            "ArrowUp"    => KeyUp              ,
            "ArrowDown"  => KeyDown            ,
            "ArrowLeft"  => KeyLeft            ,
            "ArrowRight" => KeyRight           ,
            "Delete"     => KeyDC              ,
            "Escape"     => Character('\u{1b}'),
            "Backspace"  => KeyBackspace       ,
            "Enter"      => Character('\n')    ,
            _ if s.len() == 1 => Character(s.chars().next().unwrap()),
            _ => return
        };

    ui.eval_key(key);
}

#[wasm_bindgen]
pub fn eval(s: String) {
    let ui;
    let mut state = STATE.clone();

    unsafe {
        ui = Arc::get_mut_unchecked(&mut state);
    }

    ui.eval(s);
}

#[wasm_bindgen]
pub fn render_html(width: usize, height: usize) -> String {
    let ui;
    let mut state = STATE.clone();

    unsafe {
        ui = Arc::get_mut_unchecked(&mut state);
    }

    let mut lines = ui.get_stack()
        .to_disp(width, height)
        .lines()
        .map(|s| s.to_string())
        .rev()
        .take(height - BOTTOM_BUFFER as usize)
        .collect::<Vec<_>>();

    let extras = (height - BOTTOM_BUFFER as usize).saturating_sub(lines.len());

    lines.append(&mut vec![String::new(); extras]);
    lines.reverse();

    lines.push("=".repeat(width));

    let (cmd, loc) = render_command(&ui.print, ui.cursor, width);

    if loc < cmd.len() {
        lines.push(format!(
            "{}<span class=\"cursor\">{}</span>{}",
            &cmd[..loc],
            &cmd[loc..loc + 1],
            &cmd[loc + 1..]
        ));
    } else {
        lines.push(format!("{}<span class=\"cursor\"> </span>", cmd));
    }

    lines.join("\n")
}
