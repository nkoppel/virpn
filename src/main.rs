mod io;
mod stack;
mod modes;
mod interface;

use crate::stack::*;
use crate::io::*;
use crate::modes::*;
use crate::interface::*;

use pancurses::{initscr, endwin};
use pancurses::Input::*;

fn main() {
    history_interface();
}
