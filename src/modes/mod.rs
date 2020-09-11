pub use std::collections::{HashMap, VecDeque};
pub use std::rc::Rc;
pub use std::cell::Cell;

pub use crate::stack::Stack;
pub use crate::stack::{Item, Item::*};
pub use crate::io::*;

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

pub trait Mode {
    // set of bindings used to enter this mode
    fn get_bindings(&self) -> Vec<Vec<Input>>;

    // returns compiled regex which matches operators used by mode
    fn get_operator_regex(&self) -> Regex;

    fn get_name(&self) -> String;

    fn eval_bindings(&self, ui: Ui_helper, init: HashMap<&str, &str>)
        -> ModeRes<(String, usize)>;

    fn eval_operators(&mut self, ui: &mut Ui, op: &str);
}

pub type ModeRes<T> = (T, Option<Vec<Input>>);

pub struct Ui {
    operator_regexes: Vec<(Regex, String)>,
    bindings: Bindings<(bool, String)>,
    modes: HashMap<String, Box<Mode>>,
    stack: Stack,
    pub window: Window
}

#[allow(non_camel_case_types)]
pub struct Ui_helper {
    ui: Rc<Ui>,
    init_bind: Vec<Input>,
    bindings: Bindings<(bool, String)>,
    mode: String,
    prev_surround: (String, String),
    surround: (String, String)
}

impl Ui {
    pub fn new(window: Window) -> Self {
        Ui {
            operator_regexes: Vec::new(),
            bindings: Bindings::new(),
            modes: HashMap::new(),
            stack: Stack::new(),
            window
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

    pub fn build_helper(self: Rc<Self>) -> Ui_helper {
        Ui_helper::new(self.clone(), &self.bindings)
    }

    pub fn get_mode(&self, mode: &str) -> Option<&Box<Mode>> {
        match self.modes.get(mode) {
            None => None,
            Some(r) => Some(r)
        }
    }

    pub fn get_stack<'a>(&'a mut self) -> &'a mut Stack {
        &mut self.stack
    }

    pub fn insert_mode(&mut self, name: String, mode: Box<dyn Mode>) {
        self.modes.insert(name, mode);
    }

    pub fn remove_mode(&mut self, name: &str) -> Option<Box<dyn Mode>> {
        self.modes.remove(name)
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
            if let Some(mut mode) = self.remove_mode(&mode) {
                mode.eval_operators(self, &op);
            } else {
                break;
            }
        }
    }

    pub fn print_stack(&self) {
        print_stack(&self.window, &self.stack);
    }
}

impl Ui_helper {
    pub fn new(ui: Rc<Ui>, binds: &Bindings<(bool, String)>) -> Self {
        Ui_helper {
            ui,
            init_bind: Vec::new(),
            bindings: binds.clone(),
            mode: String::new(),
            prev_surround: (String::new(), String::new()),
            surround: (String::new(), String::new())
        }
    }

    pub fn set_surrounding_text(&mut self, surr: (String, String)) {
        self.surround = surr;
    }

    pub fn add_escape_binding(&mut self, bind: Vec<Input>) {
        self.bindings.insert( bind, (true, self.mode.clone()) );
    }

    fn build_below(&self, mode: String) -> Self {
        let mut out = Ui_helper::new(self.ui.clone(), &self.bindings);

        let (pb, pa) = &self.prev_surround;
        let (sb, sa) = &self.surround;

        out.prev_surround = (pb.clone() + sb, sa.clone() + pa);
        out.mode = mode;

        out
    }

    fn is_my_binding(&mut self, bind: &Vec<Input>) -> bool {
        (self.bindings.read_from_vec(&bind).1).1 == self.mode
    }

    pub fn get_next_binding(&mut self) -> ModeRes<Vec<Input>> {
        let (bind, _) =
            if !self.init_bind.is_empty() {
                self.bindings.read_from_vec(
                    &mem::replace(&mut self.init_bind, Vec::new())
                )
            } else {
                self.bindings.read(&self.ui.window)
            };

        if self.is_my_binding(&bind) {
            (bind, None)
        } else {
            (bind.clone(), Some(bind))
        }
    }

    pub fn get_next_key(&self) -> Input {
        self.ui.window.getch().unwrap()
    }

    pub fn call_mode_by_name(&mut self,
                            name: String,
                            init: HashMap<&str, &str>,
                            bind: Vec<Input>)
        -> Option<ModeRes<(String, usize)>>
    {
        match self.ui.get_mode(&name) {
            None => None,
            Some(m) => {
                let mut below = self.build_below(name);
                below.init_bind = bind;
                Some(m.eval_bindings(below, init))
            }
        }
    }

    pub fn call_mode_by_next_binding(&mut self, buf: Vec<Input>)
        -> ModeRes<(String, String, usize, bool)>
    {
        let (bind, (esc, mode)) =
            if !buf.is_empty() {
                self.bindings.read_from_vec(&buf)
            } else if !self.init_bind.is_empty() {
                self.bindings.read_from_vec(
                    &mem::replace(&mut self.init_bind, Vec::new())
                )
            } else {
                self.bindings.read(&self.ui.window)
            };

        if esc {
            let tmp = (String::new(), String::new(), 0, true);

            return (tmp, Some(bind));
        }

        let m = self.ui.get_mode(&mode).unwrap();

        let mut below = self.build_below(mode.clone());

        below.init_bind = bind;

        let ((s, loc), res) = m.eval_bindings(below, HashMap::new());

        if let Some(bind) = res.clone() {
            let (_, (b, _)) = self.bindings.read_from_vec(&bind);

            ((mode, s, loc, b), res)
        } else {
            ((mode, s, loc, false), res)
        }
    }

    pub fn tokenize(&self, ops: &str) -> Vec<(String, String)> {
        self.ui.tokenize(ops)
    }

    pub fn print_output(&self, output: &str, loc: usize) {
        let (b, a) = &self.prev_surround;

        let s = format!("{}{}{}", b, output, a);

        print_command(&self.ui.window, &s, loc + b.len())
    }
}
