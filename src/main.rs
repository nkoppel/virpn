mod io;
mod stack;
mod modes;

use crate::stack::*;
use crate::io::*;
use crate::modes::*;

use crate::modes::{
    number::Number_mode,
    ops::Op_mode,
    // var::Var_mode
};

use pancurses::{initscr, endwin};

fn main() {
    let window = initscr();
    window.keypad(true);

    // let tmp = Stack::from_nums((1..20).map(|n| n as f64).collect());
    // let v = vec![List(tmp.into_vec())];

    // let stack = Stack::from_vec(vec![List(v); 10]);

    let ui = Ui::build(window, vec![
        Box::new(Number_mode{}),
        Box::new(Op_mode::new()),
        // Box::new(Var_mode::new()),
    ]);
    let ui = Rc::new(ui);

    let mut helper = ui.build_helper();

    helper.add_escape_binding(bind_from_str("\\"));

    let out = helper.call_mode_by_next_binding(Vec::new());

    endwin();

    println!("{:?}", out)
}
