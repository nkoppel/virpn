mod io;
mod stack;
mod op_helpers;
mod op;
mod modes;

use crate::stack::*;
use crate::io::*;
use crate::modes::*;

use crate::modes::{
    number::Number_mode,
    nil::Nil_mode,
    ops::Op_mode,
    var::Var_mode
};

use pancurses::{initscr, endwin};

fn main() {
    let window = initscr();

    let tmp = Stack::from_nums((1..20).map(|n| n as f64).collect());
    let v = vec![List(tmp.into_vec())];

    let stack = Stack::from_vec(vec![List(v); 10]);

    let mut ui = Ui::build(vec![
        Box::new(Number_mode::new()),
        Box::new(Nil_mode::new()),
        Box::new(Op_mode::new()),
        Box::new(Var_mode::new()),
    ]);

    window.keypad(true);

    ui.run(&window);

    endwin();
}
