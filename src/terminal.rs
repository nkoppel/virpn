pub use wasm_bindgen::prelude::*;
use js_sys::*;
use wasm_bindgen_futures::*;
use futures::executor::block_on;

#[wasm_bindgen]
extern "C" {
    pub fn refresh();
    pub fn clrtoeol();

    pub fn get_max_x() -> i32;
    pub fn get_max_y() -> i32;

    pub fn get_cur_yx() -> Vec<i32>;

    pub fn mv(x: i32, y: i32);

    pub fn addstr(s: &str);
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Input {
    Character(char),
    KeyUp,
    KeyDown,
    KeyLeft,
    KeyRight,
    KeyDC,
    KeyBackspace,
}

pub use Input::*;

#[derive(Clone)]
pub struct Window {}

impl Window {
    pub fn refresh(&self) {
        refresh();
    }

    pub fn clrtoeol(&self) {
        clrtoeol();
    }

    pub fn get_max_x(&self) -> i32 {
        get_max_x()
    }

    pub fn get_max_y(&self) -> i32 {
        get_max_y()
    }

    pub fn get_cur_yx(&self) -> (i32, i32) {
        let tmp = get_cur_yx();
        (tmp[0], tmp[1])
    }

    pub fn mv(&self, x: i32, y: i32) {
        mv(x, y)
    }

    pub fn addstr(&self, s: &str) {
        addstr(s);
    }

    pub fn keypad(&self, _: bool) {}
}

pub fn endwin() {}
pub fn noecho() {}

pub fn initscr() -> Window {
    Window{}
}
