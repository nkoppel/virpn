mod io;
mod stack;
mod modes;
mod interface;

#[cfg(target_arch = "wasm32")]
mod terminal;

#[cfg(target_arch = "wasm32")]
pub use wasm_bindgen::prelude::*;

use crate::interface::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_start() {
    history_interface();
}
