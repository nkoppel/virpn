mod io;
mod stack;
mod modes;
mod interface;

use crate::stack::*;
use crate::io::*;
use crate::modes::*;
use crate::interface::*;

use crate::modes::{
    number::Number_mode,
    ops::Op_mode,
    var::Var_mode
};

use pancurses::{initscr, endwin};
use pancurses::Input::*;

fn main() {
    history_interface();
}
