use crate::modes::*;

use crate::modes::{
    number::Number_mode,
    ops::Op_mode,
    var::Var_mode,
    history::History_mode,
    line_edit::Line_edit_mode
};

use pancurses::{initscr, endwin, noecho};

fn new_ui() -> Ui {
    Ui::build(vec![
        Box::new(Number_mode{}),
        Box::new(Op_mode::new()),
        Box::new(Var_mode::new()),
        Box::new(History_mode::new()),
        Box::new(Line_edit_mode::new()),
    ])
}

pub fn interface() {
    let mut ui = new_ui();
    let window = initscr();
    window.keypad(true);
    noecho();

    while !ui.exit {
        ui.show(&window);
        let k = window.getch().unwrap();

        ui.call_history();

        ui.eval_key(k);
    }

    endwin();
}

#[allow(dead_code)]
pub fn debug_interface() {
    let mut ui = new_ui();

    while !ui.exit {
        ui.debug_show();
        let window = initscr();
        window.keypad(true);
        let k = window.getch().unwrap();
        endwin();

        ui.call_history();

        ui.eval_key(k);
    }
}
