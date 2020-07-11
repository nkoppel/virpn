mod io;
mod stack;
mod op_helpers;
mod op;
mod modes;

use crate::stack::*;
use crate::io::*;

use pancurses::{initscr, endwin};

fn main() {
    let window = initscr();

    let tmp = Stack::from_nums((1..20).map(|n| n as f64).collect());
    let v = vec![List(tmp.into_vec())];

    let stack = Stack::from_vec(vec![List(v); 10]);

    // print_command(&window, &"asdf".repeat(50), 173);
    // window.getch();
    // endwin();

    window.keypad(true);
    println!("{:?}", window.getch());
}
