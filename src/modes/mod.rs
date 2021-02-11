pub use std::collections::{HashMap, VecDeque};
pub use std::rc::Rc;
pub use std::cell::Cell;

pub use crate::stack::Stack;
pub use crate::stack::{Item, Item::*};
pub use crate::io::*;

pub use pancurses::{Input, Input::*, Window};
pub use regex::Regex;

pub use std::mem;

// pub mod number;
// pub mod ops;
// pub mod var;
// pub mod history;
// pub mod line_edit;

pub type State = HashMap<String, Data>;

pub enum Data {
    Map(State),
    Array(Vec<Data>),
    Float(f64),
    Bool(bool),
    Int(i64),
    Uint(u64),
    Str(String),
    Itm(Item)
}

pub use Data::*;

pub trait Mode {
    // set of bindings used to enter this mode
    fn get_bindings(&self) -> Vec<Vec<Input>>;

    // returns compiled regex which matches operators used by mode
    fn get_operator_regex(&self) -> Regex;

    fn get_name(&self) -> String;

    fn eval_binding(&mut self, state: &mut State, bind: Vec<Input>)
        -> Vec<Message>;

    fn eval_operators(&mut self, run: &mut Ui, op: &str);

    fn exit(&mut self);
}

pub enum Message {
    Call(String, State),
    CallByBind(Vec<Input>, State),
    WrapText(String, String),
    EscBind(Vec<Input>),
    PressKeys(Vec<Input>),
    Print(String, usize),
    Return(String),
    NextKey(bool),
}

pub use Message::*;

pub struct Ui {
    operator_regexes: Vec<(Regex, String)>,
    bindings: Bindings<(bool, String)>,
    modes: HashMap<String, Box<dyn Mode>>,
    print: String,
    cursor: usize,
    stack: Stack,
    callstack: Vec<(String, State, Bindings<(bool, String)>, (String, String))>,
    nextkey: bool,
    window: Window,
}

impl Ui {
    pub fn new(window: Window) -> Self {
        Ui {
            operator_regexes: Vec::new(),
            bindings: Bindings::new(),
            modes: HashMap::new(),
            print: String::new(),
            cursor: 0,
            stack: Stack::new(),
            callstack: Vec::new(),
            nextkey: false,
            window,
        }
    }

    pub fn build(window: Window, modes: Vec<Box<dyn Mode>>) -> Self {
        let mut out = Ui::new(window);
        let mut binds = Vec::new();

        for mode in modes.into_iter() {
            let name = mode.get_name();
            let regex = mode.get_operator_regex();
            out.operator_regexes.push((regex, name.clone()));

            for bind in mode.get_bindings() {
                binds.push( (bind, (false, name.clone())) );
            }
            out.modes.insert(name, mode);
        }

        out.bindings = Bindings::from_vec(binds);

        out
    }

    pub fn get_mode(&self, mode: &str) -> Option<&Box<dyn Mode>> {
        match self.modes.get(mode) {
            None => None,
            Some(r) => Some(r)
        }
    }

    pub fn get_stack<'a>(&'a mut self) -> &'a mut Stack {
        &mut self.stack
    }

    pub fn tokenize(&self, mut ops: &str) -> Vec<(String, String)> {
        let mut out = Vec::new();
        let mut ran = true;

        while !ops.is_empty() && ran {
            ran = false;

            for (regex, m) in self.operator_regexes.iter() {
                if let Some(mat) = regex.find(&ops) {
                    let end = mat.end();

                    out.push( (m.to_string(), ops[..end].to_string()) );
                    ops = &ops[(end + 1).min(ops.len())..];

                    ran = true;
                    break;
                }
            }
        }

        out
    }

    pub fn eval(&mut self, exp: String) {
        let ops = self.tokenize(&exp);

        for (mode, op) in ops {
            if let Some(mut mode) = self.modes.remove(&mode) {
                mode.eval_operators(self, &op);
            } else {
                break;
            }
        }
    }

    fn get_wrap(&self) -> (String, String) {
        let mut left = String::new();
        let mut right = String::new();

        for (.., (l, _)) in &self.callstack {
            left += l;
        }

        for (.., (_, r)) in self.callstack.iter().rev() {
            right += r;
        }

        (left, right)
    }

    fn get_bindings<'a>(&'a mut self) -> &'a mut Bindings<(bool, String)> {
        if let Some((_, _, b, _)) = self.callstack.last_mut() {
            b
        } else {
            &mut self.bindings
        }
    }

    fn escape(&mut self, mode: &str) {
        while let Some((m, ..)) = self.callstack.last() {
            if m != mode {
                self.callstack.pop();
            } else {
                break;
            }
        }
    }

    fn run_mode(&mut self, bind: Vec<Input>) {
        if let Some((m, mut state, binds, wrap)) = self.callstack.pop() {
            let mut mode = self.modes.remove(&m).unwrap();

            self.eval_messages(mode.eval_binding(&mut state, bind));

            self.modes.insert(m.clone(), mode);
            self.callstack.push((m, state, binds, wrap));
        }
    }

    pub fn eval_messages(&mut self, messages: Vec<Message>) {
        for m in messages {
            match m {
                Call(mode, state) => {
                    let binds = self.get_bindings().clone();
                    self.callstack.push((mode, state, binds, (String::new(), String::new())));

                    let (l, r) = self.get_wrap();

                    self.cursor = l.len();
                    self.print = l + &r;
                    self.nextkey = false;
                }
                CallByBind(bind, state) => {
                    let mut binds = self.get_bindings().clone();
                    let (b, (esc, m)) = binds.read_from_vec(&bind);

                    if esc {
                        self.escape(&m);
                    } else {
                        self.eval_messages(vec![Call(m.clone(), state)]);
                    }

                    self.run_mode(b);
                }
                WrapText(left, right) => {
                    if let Some((.., (l, r))) = self.callstack.last_mut() {
                        *l = left;
                        *r = right;
                    }
                }
                EscBind(bind) => {
                    if let Some((mode, ..)) = self.callstack.last() {
                        let mode = mode.clone();
                        self.get_bindings().insert(bind, (true, mode));
                    }
                }
                PressKeys(keys) => {
                    for k in keys {
                        self.eval_key(k);
                    }
                }
                Print(s, cursor) => {
                    let (mut l, r) = self.get_wrap();
                    self.cursor = cursor + l.len();

                    l += &s;
                    l += &r;

                    self.print = l;
                    self.show();
                }
                Return(s) => {
                    self.callstack.pop();

                    if !s.is_empty() {
                        if let Some((_, state, ..)) = self.callstack.last_mut() {
                            state.insert("return".to_string(), Str(s));
                        } else {
                            self.eval(s);
                        }
                    }
                }
                NextKey(b) => {
                    self.nextkey = b;
                }
            }
        }
    }

    pub fn eval_key(&mut self, key: Input) {

    }

    pub fn show(&self) {
        print_stack(&self.window, &self.stack);
        print_command(&self.window, &self.print, self.cursor);
    }
}
