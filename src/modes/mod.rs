pub use std::collections::{HashMap, VecDeque};
pub use crate::stack::Stack;
pub use pancurses::{Input, Input::*, Window};
pub use crate::stack::Item::*;
pub use regex::Regex;

pub mod number;
pub mod nil;

use crate::modes::nil::Nil_mode;

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

    fn eval_operators(&mut self, stack: &mut Stack, ops: &mut Vec<String>);

    fn eval_bindings(&mut self, bind: Vec<Input>) -> (String, Action);

    // run before another mode is entered
    fn exit(&mut self);
}

pub struct Manager_mode {
    operator_regexes: Vec<(Regex, String)>,
    bindings: HashMap<Vec<Input>, String>,
    bindings_maxlen: usize,
    modes: HashMap<String, Box<dyn Mode>>,
}

impl Manager_mode {
    pub fn new() -> Manager_mode {
        Manager_mode {
            operator_regexes: Vec::new(),
            bindings: HashMap::new(),
            bindings_maxlen: 0,
            modes: HashMap::new(),
        }
    }

    pub fn build(modes: &Vec<Box<dyn Mode>>) -> Manager_mode {
        let mut out = Manager_mode::new();

        for mode in modes {
            let name = mode.get_name();
            let regex = mode.get_operator_regex();
            out.operator_regexes.push((regex, name.clone()));

            for b in mode.get_bindings() {
                if b.len() > out.bindings_maxlen {
                    out.bindings_maxlen = b.len();
                }

                out.bindings.insert(b, name.clone());
            }
        }

        out
    }

    fn run(&mut self) -> String {
        let mut submode_owns = false;
        let mut submode = String::new();
        let mut key_buffer = Vec::new();
        let mut prev_output = String::new();

        if !submode_owns {
            key_buffer.push(bind[0]);

            if key_buffer.len() > self.bindings_maxlen {
                key_buffer.clear();
                prev_output = String::new();
                return String::new();
            }

            if let Some(sub) = self.bindings.get(&key_buffer) {
                self.modes.get_mut(&submode).unwrap().exit();
                submode = sub.to_string();
            } else {
                return prev_output.clone();
            }
        }

        let sub = self.modes.get_mut(&submode).unwrap();
        let (s, act) = sub.eval_bindings(key_buffer);

        key_buffer = Vec::new();

        prev_output = s.clone();

        if act == Exit {
            submode_owns = false;
            return s;
        } else {
            submode_owns = act == Req_own;
            return s;
        }
    }
}
