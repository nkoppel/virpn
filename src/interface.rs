use crate::modes::*;
use pancurses::{initscr, endwin, noecho};

pub fn interface() {
    let mut ui = Ui::build();
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
    let mut ui = Ui::build();

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
