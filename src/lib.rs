#![feature(get_mut_unchecked)]
mod io;
mod stack;
mod modes;
mod data;

#[cfg(not(target_arch = "wasm32"))]
mod interface;

#[cfg(target_arch = "wasm32")]
mod wasm_interface;

#[cfg(target_arch = "wasm32")]
pub use wasm_interface::*;

#[cfg(not(target_arch = "wasm32"))]
pub fn killwarnings() {
    interface::interface();
}
