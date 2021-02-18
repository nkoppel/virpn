pub use std::collections::HashMap;
pub use std::sync::Arc;
pub use std::cell::Cell;

pub use crate::stack::Stack;
pub use crate::stack::{Item, Item::*};
pub use crate::data::*;
pub use crate::io::*;

use self::{
    number::Number_mode,
    ops::Op_mode,
    var::Var_mode,
    history::History_mode,
    line_edit::Line_edit_mode
};

#[cfg(not(target_arch = "wasm32"))]
pub use pancurses::{Input, Input::*, Window};

#[cfg(target_arch = "wasm32")]
pub use crate::terminal::{Input, Input::*, Window};

pub use regex::Regex;

pub use std::mem;

pub mod number;
pub mod ops;
pub mod var;
pub mod history;
pub mod line_edit;

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

    fn ret(&mut self, state: &mut State) -> String;
}

#[derive(Clone, Debug)]
pub enum Message {
    Call(String, State),
    CallByBind(Vec<Input>, State),
    WrapText(String, String),
    EscBind(Vec<Input>),
    PressKeys(Vec<Input>),
    Eval(String),
    Print(String, usize),
    AllowReplace(bool),
    Return,
    Exit,
    NextKey(bool),
}

pub use Message::*;

pub struct Ui {
    operator_regexes: Vec<(Regex, String)>,
    bindings: Bindings<(bool, String)>,
    modes: HashMap<String, Box<dyn Mode + Send + Sync>>,
    pub exit: bool,
    print: String,
    cursor: usize,
    stack: Stack,
    callstack: Vec<(String, State, bool, Bindings<(bool, String)>, (String, String))>,
    nextkey: bool,
}

impl Ui {
    pub fn new() -> Self {
        Ui {
            operator_regexes: Vec::new(),
            bindings: Bindings::new(),
            exit: false,
            modes: HashMap::new(),
            print: String::new(),
            cursor: 0,
            stack: Stack::new(),
            callstack: Vec::new(),
            nextkey: false,
        }
    }

    pub fn build_from_modes(modes: Vec<Box<dyn Mode + Send + Sync>>) -> Self {
        let mut out = Ui::new();
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

    pub fn build() -> Self {
        Self::build_from_modes(vec![
            Box::new(Number_mode{}),
            Box::new(Op_mode::new()),
            Box::new(Var_mode::new()),
            Box::new(History_mode::new()),
            Box::new(Line_edit_mode::new())
        ])
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

    pub fn insert_mode(&mut self, name: String, mode: Box<dyn Mode + Send + Sync>) {
        self.modes.insert(name, mode);
    }

    pub fn eval(&mut self, exp: String) {
        let ops = self.tokenize(&exp);

        for (m, op) in ops {
            if let Some(mut mode) = self.modes.remove(&m) {
                mode.eval_operators(self, &op);
            } else {
                break;
            }
        }
    }

    fn get_wrap(&self) -> (String, String) {
        let mut left = String::new();
        let mut right = String::new();
        let len = self.callstack.len();

        if len > 0 {
            for i in 0..len - 1 {
                left += &self.callstack[i].4.0;
                right += &self.callstack[len - 2 - i].4.1;
            }
        }

        (left, right)
    }

    fn get_bindings<'a>(&'a mut self) -> &'a mut Bindings<(bool, String)> {
        if let Some((.., b, _)) = self.callstack.last_mut() {
            b
        } else {
            &mut self.bindings
        }
    }

    fn run_mode(&mut self, bind: Vec<Input>) -> bool {
        if let Some((m, mut state, repl, binds, wrap)) = self.callstack.pop() {
            let mut mode = self.modes.remove(&m).expect(&format!("{}", m));
            let messages = mode.eval_binding(&mut state, bind);

            self.modes.insert(m.clone(), mode);
            self.callstack.push((m, state, repl, binds, wrap));

            self.eval_messages(messages);

            return true;
        }
        false
    }

    fn mode_return(&mut self, call: bool) {
        if let Some((m, mut state, ..)) = self.callstack.pop() {
            let mut mode = self.modes.remove(&m).unwrap();
            let s = mode.ret(&mut state);

            self.modes.insert(m, mode);

            if let Some((_, state, ..)) = self.callstack.last_mut() {
                state.insert("return".to_string(), Str(s));
                mem::drop(state);

                if call {
                    self.run_mode(Vec::new());
                }
            } else {
                self.eval(s);
                self.call_history();
            }
        }
    }

    fn escape(&mut self, mode: &str) {
        while let Some((m, ..)) = self.callstack.last() {
            if m != mode {
                self.mode_return(false);
            } else {
                break;
            }
        }
    }

    pub fn eval_messages(&mut self, messages: Vec<Message>) {
        for m in messages {
            match m {
                Call(mode, state) => {
                    let binds = self.get_bindings().clone();
                    self.callstack.push((mode, state, true, binds, (String::new(), String::new())));

                    let (l, r) = self.get_wrap();

                    self.cursor = l.len();
                    self.print = l + &r;
                    self.nextkey = false;
                }
                CallByBind(bind, state) => {
                    let (b, (esc, m)) =
                        self.get_bindings().read_from_vec(&bind);

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
                Eval(s) => {
                    self.eval(s);
                }
                Print(s, cursor) => {
                    let (mut l, r) = self.get_wrap();
                    self.cursor = cursor + l.len();

                    l += &s;
                    l += &r;

                    self.print = l;
                }
                AllowReplace(b) => {
                    if let Some((_, _, repl, ..)) = self.callstack.last_mut() {
                        *repl = b;
                    }
                }
                Return => {
                    self.mode_return(true);
                    self.nextkey = false;
                }
                Exit => {
                    self.exit = true;
                }
                NextKey(b) => {
                    self.nextkey = b;
                }
            }
        }
    }

    pub fn eval_key(&mut self, key: Input) {
        if self.nextkey {
            self.run_mode(vec![key]);
        } else if let Some((esc, m)) = self.get_bindings().add(key) {
            let bind = self.get_bindings().get_bind();

            if esc {
                self.escape(&m);
            }
            if !self.callstack.is_empty() {
                let (m2, _, repl, ..) = self.callstack.last().unwrap();

                if m == *m2 {
                    self.run_mode(bind);
                } else {
                    if *repl {
                        self.mode_return(true);
                    }
                    self.eval_messages(vec![
                        CallByBind(bind, HashMap::new())
                    ]);
                }
            } else {
                self.eval_messages(vec![
                    CallByBind(bind, HashMap::new())
                ]);
            }
        }
    }

    pub fn call_history(&mut self) {
        if self.callstack.is_empty() {
            self.callstack.push((
                "history".to_string(),
                HashMap::new(),
                false,
                self.bindings.clone(),
                (String::new(), String::new())
            ));

            self.eval_messages(vec![
                EscBind(vec![KeyUp]),
                EscBind(vec![KeyDown]),
                EscBind(bind_from_str("u")),
                EscBind(bind_from_str("R")),
                EscBind(bind_from_str(" ")),
                EscBind(bind_from_str("\n")),
                EscBind(bind_from_str("Q")),
            ])
        }
    }

    #[allow(dead_code)]
    pub fn debug_show(&mut self) {
        println!("{:?}", self.callstack.iter().map(|(n, ..)| n).collect::<Vec<_>>());
        println!("{:?}", self.stack);
        println!("{}, {:?}", self.cursor, self.print);
        println!();
    }

    pub fn show(&self, window: &Window) {
        print_stack(&window, &self.stack);
        print_command(&window, &self.print, self.cursor);
    }
}
