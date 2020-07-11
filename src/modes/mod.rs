pub use std::collections::{HashMap, VecDeque};
pub use crate::stack::Stack;
pub use pancurses::{Input, Input::*, Window};
pub use crate::stack::Item::*;
pub use regex::Regex;

pub mod number;
pub mod nil;

use crate::modes::nil::Nil_mode;
use crate::io::*;

const KeyEsc: Input = Character('\u{1b}');
const KeyEnt: Input = Character('\n');

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    Continue,
    Req_own,
    Exit
}

pub use Action::*;

trait Mode {
    // set of bindings used to enter this mode
    fn get_bindings(&self) -> Vec<Vec<Input>>;

    // returns compiled regex which matches operators used by mode
    fn get_operator_regex(&self) -> Regex;

    fn get_name(&self) -> String;

    fn eval_operators(&mut self, stack: &mut Stack, op: String);

    fn eval_bindings(&mut self, bind: Vec<Input>) -> (String, Action);

    // run before another mode is entered
    fn exit(&mut self);
}

pub struct Manager_mode {
    operator_regexes: Vec<(Regex, String)>,
    bindings: Bindings<String>,
    bindings_maxlen: usize,
    modes: HashMap<String, Box<dyn Mode>>,
    stack: Stack
}

impl Manager_mode {
    pub fn new() -> Manager_mode {
        Manager_mode {
            operator_regexes: Vec::new(),
            bindings: Bindings::new(),
            bindings_maxlen: 0,
            modes: HashMap::new(),
            stack: Stack::new()
        }
    }

    pub fn build(modes: &Vec<Box<dyn Mode>>) -> Manager_mode {
        let mut out = Manager_mode::new();
        let mut binds = Vec::new();

        for mode in modes {
            let name = mode.get_name();
            let regex = mode.get_operator_regex();
            out.operator_regexes.push((regex, name.clone()));

            for b in mode.get_bindings() {
                if b.len() > out.bindings_maxlen {
                    out.bindings_maxlen = b.len();
                }

                binds.push((b, name.clone()));
            }
        }

        out.bindings = Bindings::from_vec(binds, vec![KeyEsc, Character('\n')]);

        out
    }

    pub fn run_operator(&mut self, op: &str) {
        
    }

    pub fn run(&mut self, window: &Window) {
        let mut submode_owns = false;
        let mut submode = String::new();
        let mut prev_output = String::new();
        let mut inputs: Vec<Input> = Vec::new();

        window.keypad(true);
        pancurses::noecho();

        loop {
            let mut inputs;

            if submode_owns {
                inputs = vec![window.getch().unwrap()];
            } else {
                loop {
                    match self.bindings.read(&window) {
                        Err(KeyEsc) => {
                            self.modes.get_mut(&submode).unwrap().exit();
                            prev_output.clear();
                            print_command(&window, "", 0);
                        },
                        Err(KeyEnt) => {
                            self.modes.get_mut(&submode).unwrap().exit();
                            self.run_operator(&prev_output);
                            prev_output.clear();
                            print_command(&window, "", 0);
                        },
                        Ok((i, sub)) => {
                            if sub != submode {
                                self.modes.get_mut(&submode).unwrap().exit();
                                submode = sub.to_string();
                                submode_owns = false;
                            }
                            inputs = i;
                            break;
                        }
                        Err(_) => {}
                    }
                }
            }

            let (op, act) =
                self.modes.get_mut(&submode).unwrap().eval_bindings(inputs);

            prev_output = op.clone();
            print_command(&window, &op, op.len());

            if act == Exit {
                self.run_operator(&op);
                submode_owns = false;
            } else {
                submode_owns = act == Req_own;
            }
        }
    }
}
