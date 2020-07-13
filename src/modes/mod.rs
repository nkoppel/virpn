pub use std::collections::{HashMap, VecDeque};
pub use crate::stack::Stack;
pub use pancurses::{Input, Input::*, Window};
pub use crate::stack::Item::*;
pub use regex::Regex;

pub mod number;
pub mod nil;
pub mod ops;

use crate::modes::nil::Nil_mode;
use crate::io::*;

const KeyEsc: Input = Character('\u{1b}');
const KeyEnt: Input = Character('\n');
const KeySpc: Input = Character(' ');

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    Continue,
    Req_own,
    Exit
}

pub use Action::*;

pub trait Mode {
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

pub struct Ui {
    operator_regexes: Vec<(Regex, String)>,
    bindings: Bindings<String>,
    bindings_maxlen: usize,
    modes: HashMap<String, Box<dyn Mode>>,
    stack: Stack
}

impl Ui {
    pub fn new() -> Self {
        Ui {
            operator_regexes: Vec::new(),
            bindings: Bindings::new(),
            bindings_maxlen: 0,
            modes: HashMap::new(),
            stack: Stack::new()
        }
    }

    pub fn build(modes: Vec<Box<dyn Mode>>) -> Self {
        let mut out = Ui::new();
        let mut binds = Vec::new();

        for mode in modes.into_iter() {
            let name = mode.get_name();
            let regex = mode.get_operator_regex();
            out.operator_regexes.push((regex, name.clone()));

            for b in mode.get_bindings() {
                if b.len() > out.bindings_maxlen {
                    out.bindings_maxlen = b.len();
                }

                binds.push((b, name.clone()));
            }
            out.modes.insert(name, mode);
        }

        out.bindings = Bindings::from_vec(binds, vec![KeyEsc, KeyEnt, KeySpc]);

        out
    }

    fn get_mode(&mut self, m: &str) -> &mut Box<dyn Mode> {
        self.modes.get_mut(m).unwrap()
    }

    pub fn run_operator(&mut self, window: &Window, op: &str) {
        for (r, m) in self.operator_regexes.iter() {
            if r.is_match(op) {
                self.modes.get_mut(m).unwrap().eval_operators(&mut self.stack, op.to_string());
                print_stack(&window, &self.stack);
                break;
            }
        }
    }

    pub fn run(&mut self, window: &Window) {
        let mut run_on_mode_change = true;
        let mut submode_owns = false;
        let mut submode = "nil".to_string();
        let mut prev_output = String::new();
        let mut inputs: Vec<Input> = Vec::new();

        window.keypad(true);
        pancurses::noecho();

        print_command(&window, "", 0);

        loop {
            let mut inputs;

            if submode_owns {
                inputs = vec![window.getch().unwrap()];
            } else {
                loop {
                    match self.bindings.read(&window) {
                        Err(KeyEsc) => {
                            self.get_mode(&submode).exit();
                            prev_output.clear();
                            print_command(&window, "", 0);
                        },
                        Err(KeyEnt) | Err(KeySpc) => {
                            self.get_mode(&submode).exit();
                            self.run_operator(&window, &prev_output);
                            run_on_mode_change = false;
                        },
                        Ok((i, sub)) => {
                            if sub != submode {
                                self.get_mode(&submode).exit();
                                if run_on_mode_change {
                                    self.run_operator(&window, &prev_output);
                                }
                                submode = sub.to_string();
                                run_on_mode_change = true;
                                submode_owns = false;
                            }
                            inputs = i;
                            break;
                        }
                        Err(_) => {}
                    }
                }
            }

            let (op, act) = self.get_mode(&submode).eval_bindings(inputs);

            prev_output = op.clone();
            print_command(&window, &op, op.len());

            if act == Exit {
                self.run_operator(&window, &op);
                run_on_mode_change = false;
                submode_owns = false;
            } else {
                submode_owns = act == Req_own;
            }
        }
    }
}
