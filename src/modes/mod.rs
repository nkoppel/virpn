pub use std::collections::{HashMap, VecDeque};
pub use std::rc::Rc;
pub use std::cell::Cell;

pub use crate::stack::Stack;
pub use crate::stack::{Item, Item::*};

pub use pancurses::{Input, Input::*, Window};
pub use regex::Regex;

pub mod number;
pub mod nil;
// pub mod ops;
// pub mod var;

use crate::io::*;
use crate::modes::nil::Nil_mode;
use std::borrow::BorrowMut;

const KEY_ESC: Input = Character('\u{1b}');
const KEY_ENT: Input = Character('\n');
const KEY_SPC: Input = Character(' ');

#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    Continue,
    Req_own,
    Exit,
    Call(String, String),
    Run
}

pub use Action::*;

pub trait LocalMode {
    fn eval_bindings(&mut self, bind: Vec<Input>) -> (String, usize, Action);

    fn modify_str(&mut self, s: String, loc: usize) -> (String, usize) {
        (s, loc)
    }

    fn below_mode_exited(&mut self, out: (String, usize)) -> bool {false}
}

pub trait GlobalMode {
    // set of bindings used to enter this mode
    fn get_bindings(&self) -> Vec<Vec<Input>>;

    // returns compiled regex which matches operators used by mode
    fn get_operator_regex(&self) -> Regex;

    fn get_name(&self) -> String;

    fn eval_operators(&mut self, ui: &mut Ui, op: String);

    fn build_local(self: Rc<Self>, init: String) -> Box<dyn LocalMode>;
}

pub struct Ui {
    operator_regexes: Vec<(Regex, String)>,
    bindings: Bindings<String>,
    modes: HashMap<String, Rc<GlobalMode>>,
    mode_stack: Vec<(String, Box<dyn LocalMode>)>,
    stack: Stack
}

impl Ui {
    pub fn new() -> Self {
        Ui {
            operator_regexes: Vec::new(),
            bindings: Bindings::new(),
            modes: HashMap::new(),
            mode_stack: Vec::new(),
            stack: Stack::new()
        }
    }

    pub fn build(modes: Vec<Rc<dyn GlobalMode>>) -> Self {
        let mut out = Ui::new();
        let mut binds = Vec::new();

        for mode in modes.into_iter() {
            let name = mode.get_name();
            let regex = mode.get_operator_regex();
            out.operator_regexes.push((regex, name.clone()));

            for b in mode.get_bindings() {
                binds.push((b, name.clone()));
            }
            out.modes.insert(name, mode);
        }

        out.bindings =
            Bindings::from_vec(binds, vec![KEY_ESC, KeyDC, KEY_ENT, KEY_SPC]);

        out
    }

    fn get_mode(&mut self, m: &str) -> &mut Rc<dyn GlobalMode> {
        self.modes.get_mut(m).unwrap()
    }

    fn exit_mode(&mut self, prev_output: &(String, usize))
        -> (String, Box<dyn LocalMode>)
    {
        match self.mode_stack.pop() {
            Some((name, mut mode)) => {
                mode.below_mode_exited(prev_output.clone());
                (name, mode)
            }
            None => ("nil".to_string(), Nil_mode::new())
        }
    }

    fn print_with_mods(&mut self, window: &Window, out: (String, usize))
        -> String
    {
        let (mut s, mut loc) = out;

        for (_, m) in self.mode_stack.iter_mut().rev() {
            let tmp = m.modify_str(s, loc);
            s   = tmp.0;
            loc = tmp.1;
        }
        print_command(window, &s, loc);
        s
    }

    pub fn get_stack<'a>(&'a mut self) -> &'a mut Stack {
        &mut self.stack
    }

    pub fn eval(mut self: &mut Self, window: &Window, mut op: &str) {
        let mut ran = true;
        let mut mode = String::new();
        let mut o = String::new();

        while op.len() > 0 && ran {
            ran = false;

            for (r, m) in self.operator_regexes.iter() {
                if let Some(mat) = r.find(op) {
                    let range = mat.range();

                    if range.start == 0 {
                        o = op[range.clone()].to_string();
                        mode = m.to_string();
                        op = &op[range.end..];
                        ran = true;
                        break;
                    }
                }
            }

            if ran {
                if let Some(mut m) = self.modes.remove(&mode) {
                    let tmp = Rc::get_mut(&mut m).unwrap();
                    tmp.eval_operators(&mut self, o);
                    o = String::new();
                    self.modes.insert(mode.clone(), m);
                }
            }
        }
    }

    pub fn run(&mut self, window: &Window) {
        let mut run_on_mode_change = true;
        let mut prev_output = (String::new(), 0);

        let mut submode_name = "nil".to_string();
        let mut submode = Nil_mode::new();
        let mut submode_owns = false;

        window.keypad(true);
        pancurses::noecho();

        print_command(window, "", 0);

        loop {
            let mut inputs = Vec::new();

            if submode_owns {
                inputs.push(window.getch().unwrap());
            } else {
                match self.bindings.read(window) {
                    Err(KEY_ENT) => {
                        let op = self.print_with_mods(window, prev_output);
                        prev_output = (String::new(), 0);
                        self.eval(window, &op);
                        print_stack(window, &self.stack);
                        self.mode_stack.clear();
                        submode_name = "nil".to_string();
                        submode = Nil_mode::new();
                        submode_owns = false;
                    },
                    Err(KEY_ESC) | Err(KeyDC) => {
                        let tmp = self.exit_mode(&prev_output);
                        submode_name = tmp.0;
                        submode = tmp.1;
                        prev_output = (String::new(), 0);
                        self.print_with_mods(window, (String::new(), 0));
                        submode_owns = false;
                    },
                    Ok((i, sub)) => {
                        if sub != submode_name {
                            submode = self.get_mode(&sub).clone()
                                .build_local(String::new());

                            submode_name = sub.to_string();

                            if let Some((_, s)) = self.mode_stack.last_mut() {
                                let ret =
                                    s.below_mode_exited(prev_output.clone());

                                if ret {
                                    let tmp = self.exit_mode(&prev_output);
                                    submode_name = tmp.0;
                                    submode = tmp.1;
                                }
                            }
                        }
                        inputs = i;
                    }
                    Err(_) => {}
                }
            }

            if !inputs.is_empty() {
                let (op, loc, act) = submode.eval_bindings(inputs);

                self.print_with_mods(window, (op.clone(), loc));
                prev_output = (op, loc);

                match act {
                    Continue => submode_owns = false,
                    Req_own  => submode_owns = true,
                    Exit => {
                        let tmp = self.exit_mode(&prev_output);
                        submode_name = tmp.0;
                        submode = tmp.1;
                    }
                    Call(name, init) => {
                        self.mode_stack.push((submode_name, submode));
                        submode_name = name.clone();
                        submode = self.get_mode(&name).clone()
                            .build_local(init);
                    },
                    Run => {
                        self.mode_stack.push((submode_name, submode));
                        submode_name = "nil".to_string();
                        submode = Nil_mode::new();
                    }
                }
            }
        }
    }
}
