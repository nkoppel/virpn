pub use std::collections::{HashMap, VecDeque};
pub use crate::stack::Stack;
pub use termion::event::{Key, Key::*};
pub use crate::stack::Item::*;
pub use regex::Regex;

pub mod number;

pub enum Action {
    Continue,
    Req_own,
    Exit
}

pub use Action::*;

trait Mode {
    // set of bindings used to enter this mode
    fn get_bindings(&self) -> Vec<Vec<Key>>;

    // returns compiled regex which matches operators used by mode
    fn get_operator_regex(&self) -> Regex;

    fn get_name(&self) -> String;

    fn eval_operators(&mut self, stack: &mut Stack, ops: &mut Vec<String>);

    fn eval_bindings(&mut self, bind: Vec<Key>) -> (String, Action);

    // run before another mode is entered
    fn exit(&mut self);
}

pub struct Manager_mode {
    operator_regexes: Vec<(Regex, String)>,
    bindings: HashMap<Vec<Key>, String>,
    bindings_maxlen: usize,
    modes: HashMap<String, Box<dyn Mode>>;
}

impl Manager_mode {
    pub fn new() -> Manager_mode {
        Manager_mode {
            operator_regexes: Vec::new(),
            bindings: HashMap::new(),
            maxlen: 0,
            buffer: Vec::new(),
            submode: Box::new(Nil_mode::new()),
            submode_owns: false,
            prev_output: String::new()
        }
    }

    pub fn build(modes: &Vec<Box<dyn Mode>>) -> Manager_mode {
        let mut out = Manager_mode::new();

        for mode in modes {
            let name = mode.get_name();
            let regex = mode.get_operator_regex();
            out.operator_regexes.push((regex, name.clone()));

            for b in mode.get_bindings() {
                if b.len() > out.maxlen {
                    out.maxlen = b.len();
                }

                out.bindings.insert(b, name.clone());
            }
        }

        out
    }

    fn run(&mut self) {
        let mut submode_owns = false;
        let mut submode = Box::new(Nil_mode::new());
        let mut key_buffer = Vec::new();
        let mut prev_output = String::new();

        if !self.submode_owns {
            self.buffer.push(bind[0]);

            if self.buffer.len() > self.maxlen {
                self.buffer.clear();
                self.prev_output = String::new();
                return String::new();
            }

            if let Some(submode) = self.bindings.get(&self.buffer) {
                if &self.submode.get_name() != submode {
                    self.submode = (*modes.get(&submode[..]).unwrap()).copy();
                }
            } else {
                return self.prev_output.clone();
            }
        }

        let (s, act) = self.submode.eval_input(modes, bind);

        self.prev_output = s.clone();

        if act == Exit {
            self.submode_owns = false;
            return s;
        } else {
            self.submode_owns = act == Req_own;
            return s;
        }
    }
}
