#![feature(get_mut_unchecked)]
mod io;
mod stack;
mod modes;
mod data;

#[cfg(not(target_arch = "wasm32"))]
mod interface;

#[cfg(target_arch = "wasm32")]
mod terminal;
#[cfg(target_arch = "wasm32")]
mod wasm_interface;
#[cfg(target_arch = "wasm32")]
pub use wasm_interface::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
#[allow(dead_code)]
pub fn wasm_start() {
    
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub fn start() {
    interface::interface();
}
