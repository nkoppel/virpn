use std::collections::{HashMap, VecDeque};
use crate::stack::Stack;
use termion::event::Key;

enum Action {
    IntoMode(String, Key),
    Fun(String)
}

trait Mode {
    fn run(&mut self,
           modes: &mut HashMap<String, Box<dyn Mode>>,
           keys: VecDeque<Key>,
           stack: &mut Stack);
}

struct Number_mode
