mod io;
mod stack;
mod modes;
mod data;

#[cfg(not(target_arch = "wasm32"))]
mod interface;

#[cfg(target_arch = "wasm32")]
pub use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    interface::interface();
    // interface::debug_interface();
}
