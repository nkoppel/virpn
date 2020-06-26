pub use std::collections::{HashMap, VecDeque};
pub use crate::stack::Stack;
pub use termion::event::{Key, Key::*};
pub use crate::stack::Item::*;
pub use regex::Regex;

pub mod manager;
pub mod number;

enum Action {
    IntoMode(String, Key),
    Output(String, bool)
}

trait Mode {
    // set of bindings used to enter this mode
    fn get_bindings(&self) -> Vec<Vec<Key>>;

    // returns compiled regex which matches operators used by mode
    fn get_operator_regex(&self) -> Regex;

    fn copy(&self) -> Box<dyn Mode>;

    fn get_name(&self) -> String;

    fn run(&mut self,
           modes: &HashMap<String, Box<dyn Mode>>,
           stack: &mut Stack,
           ops: Vec<String>);

    // output: current string representation of mode, whether this mode is 
    // exiting, whether to pass all bindings to this mode
    fn eval_input(&mut self,
                  modes: &HashMap<String, Box<dyn Mode>>,
                  bind: Vec<Key>
        ) -> (String, bool, bool);
}

type ModeMap = HashMap<String, Box<dyn Mode>>;
