mod io;
mod stack;
mod modes;

use crate::stack::*;
use crate::io::*;
use crate::modes::*;

use crate::modes::{
    number::Number_mode,
    ops::Op_mode,
    var::Var_mode
};

use pancurses::{initscr, endwin};
use pancurses::Input::*;

fn main() {
    let window = initscr();
    window.keypad(true);

    // let tmp = Stack::from_nums((1..20).map(|n| n as f64).collect());
    // let v = vec![List(tmp.into_vec())];

    // let stack = Stack::from_vec(vec![List(v); 10]);

    let ui = Ui::build(window, vec![
        Box::new(Number_mode{}),
        Box::new(Op_mode::new()),
        Box::new(Var_mode::new()),
    ]);
    let mut ui = Rc::new(ui);

    let mut inputs = Vec::new();

    // ui.print_stack();

    loop {
        let mut helper = ui.clone().build_helper();

        helper.add_escape_binding(bind_from_str("Q"));
        helper.add_escape_binding(bind_from_str(" "));
        helper.add_escape_binding(vec![KeyDC]);

        let out = helper.call_mode_by_next_binding(inputs);
        inputs = Vec::new();

        // println!("{:?}", out);

        match &out {
            ((_, _, _, true), Some((_, b))) if b == &bind_from_str(" ") => {},
            ((_, _, _, true), Some((_, b))) if b == &vec![KeyDC] => {
                print_command(&ui.window, "", 0);
                continue;
            },
            ((_, _, _, true), _) => break,
            (_, Some((_, binds))) => inputs = binds.clone(),
            _ => {}
        }

        let ((_, op, _, _), _) = out;

        std::mem::drop(helper);

        Rc::get_mut(&mut ui).unwrap().eval(op);
        ui.print_stack();
    }

    endwin();
}
