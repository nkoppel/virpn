mod io;
mod stack;
mod modes;
mod interface;

#[cfg(target_arch = "wasm32")]
mod terminal;

#[cfg(target_arch = "wasm32")]
pub use wasm_bindgen::prelude::*;

use crate::interface::*;

pub fn main() {
    history_interface();
}
