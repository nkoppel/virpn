use crate::stack::*;
use crate::io::*;
use crate::modes::*;

use crate::modes::{
    number::Number_mode,
    ops::Op_mode,
    var::Var_mode,
    history::History_mode,
    line_edit::Line_edit_mode
};

use pancurses::{initscr, endwin};
use pancurses::Input::*;

fn new_ui(window: Window) -> Ui {
    Ui::build(window, vec![
        Box::new(Number_mode{}),
        Box::new(Op_mode::new()),
        Box::new(Var_mode::new()),
        Box::new(History_mode::new()),
        Box::new(Line_edit_mode{}),
    ])
}

pub fn interface() {
    let window = initscr();
    window.keypad(true);

    let mut ui = Rc::new(new_ui(window));

    let mut inputs = Vec::new();
    let mut op = String::new();

    print_command(&ui.window, "", 0);

    loop {
        let mut helper = ui.clone().build_helper();

        helper.add_escape_binding(bind_from_str("Q"));
        helper.add_escape_binding(bind_from_str(" "));
        helper.add_escape_binding(vec![KeyDC]);

        let out = helper.call_mode_by_next_binding(inputs);
        inputs = Vec::new();

        let ((_, tmp, _, _), _) = &out;

        std::mem::drop(helper);

        if !tmp.is_empty() {
            op = tmp.to_string();
            Rc::get_mut(&mut ui).unwrap().eval(op.clone());
            ui.print_stack();
        }

        match &out {
            ((_, o, _, true), Some(b))
                if b == &bind_from_str(" ") || b == &bind_from_str("\n") => {
                    if o.is_empty() {
                        Rc::get_mut(&mut ui).unwrap().eval(op.clone());
                        ui.print_stack();
                    }
                }

            ((_, _, _, true), Some(b)) if b == &vec![KeyDC] => {
                print_command(&ui.window, "", 0);
                continue;
            },
            ((_, _, _, true), _) => break,
            (_, Some(binds)) => inputs = binds.clone(),
            _ => {}
        }

    }

    endwin();
}

pub fn history_interface() {
    let window = initscr();
    window.keypad(true);

    let mut ui = Rc::new(new_ui(window));

    let mut inputs = Vec::new();
    let mut op = String::new();

    print_command(&ui.window, "", 0);

    loop {
        let mut helper = ui.clone().build_helper();

        helper.add_escape_binding(bind_from_str("Q"));
        helper.add_escape_binding(vec![KeyDC]);

        let ((op, loc), res) =
            helper.call_mode_by_name(
                "history".to_string(),
                HashMap::new(),
                inputs
            ).unwrap();

        inputs = Vec::new();
        mem::drop(helper);

        if let Some(b) = res {
            if b == bind_from_str("Q") {
                break;
            } else if b == vec![KeyDC] {
                print_command(&ui.window, "", 0);
                continue;
            } else {
                inputs = b;
            } 
        }

        Rc::get_mut(&mut ui).unwrap().eval(op.clone());
        ui.print_stack();
    }

    endwin();
}
