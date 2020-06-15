use termion::event::Key;
use termion::event::Key::*;

mod io;
use crate::io::*;
mod stack;
mod op_helpers;
mod op;
// mod modes;

fn keys_of_str(s: &str) -> Vec<Key> {
    let mut out = Vec::new();

    for c in s.chars() {
        out.push(Char(c));
    }

    out
}

fn main() {
    print!("{}", "a".repeat(257));
    let s = "a".repeat(257);
    // for len in (0..257).rev() {
        reprint(&s, 257, 84);
        reprint(&s, 84, 85);
        reprint(&s, 85, 84);
    // }
    let bindings = Bindings::from_vec (vec![
        (keys_of_str("aaa"), "a"),
        (keys_of_str("abc"), "b"),
        (keys_of_str("acb"), "c"),
    ], vec![Esc, Backspace, Delete]);
    // println!("{:?}", read_with_bindings(&bindings));
}
