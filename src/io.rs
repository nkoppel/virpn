#[cfg(target_arch = "wasm32")]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Input {
    Character(char),
    KeyUp,
    KeyDown,
    KeyLeft,
    KeyRight,
    KeyDC,
    KeyBackspace,
}

#[cfg(target_arch = "wasm32")]
pub use Input::*;

#[cfg(not(target_arch = "wasm32"))]
pub use pancurses::{Input, Input::*, Window, initscr, endwin, noecho};

use std::collections::HashMap;

#[derive(Clone, Debug)]
enum BindTree<T> {
    Branch(HashMap<Input, BindTree<T>>),
    Leaf(T)
}

use BindTree::*;

#[derive(Clone, Debug)]
pub struct Bindings<T> {
    tree: BindTree<T>,
    buf: Vec<Input>,
    out_buf: Vec<Input>
}

impl<T> BindTree<T> where T: Clone {
    pub fn new() -> Self {
        Branch(HashMap::new())
    }

    pub fn insert<'a, I>(&mut self, mut key: I, val: T)
        where I: Iterator<Item=&'a Input>
    {
        if let Leaf(_) = self {
            *self = Branch(HashMap::new());
        }

        if let Branch(map) = self {
            if let Some(input) = key.next() {
                match map.get_mut(&input) {
                    Some(tree) => {
                        tree.insert(key, val);
                    }
                    None => {
                        let mut tree = Self::new();
                        tree.insert(key, val);
                        map.insert(*input, tree);
                    }
                }
            } else {
                *self = Leaf(val)
            }
        }
    }

    pub fn get<'a, I>(&self, mut key: I) -> (Option<T>, bool)
        where I: Iterator<Item=&'a Input>
    {
        match self {
            Branch(map) => {
                if let Some(i) = key.next() {
                    if map.contains_key(i) {
                        return map.get(i).unwrap().get(key);
                    } else {
                        return (None, false);
                    }
                }
                (None, true)
            }
            Leaf(t) => {
                if key.next() == None {
                    (Some(t.clone()), true)
                } else {
                    (None, false)
                }
            }
        }
    }
}

impl<T> Bindings<T> where T: Clone {
    pub fn new() -> Self {
        Bindings {
            tree: BindTree::new(),
            buf: Vec::new(),
            out_buf: Vec::new()
        }
    }

    pub fn from_vec(v: Vec<(Vec<Input>, T)>) -> Self {
        let mut out = Bindings::new();

        for (i, o) in v.into_iter() {
            out.tree.insert(i.iter(), o);
        }

        out
    }

    pub fn insert(&mut self, bind: Vec<Input>, output: T) {
        self.tree.insert(bind.iter(), output);
    }

    pub fn get_bind(&self) -> Vec<Input> {
        self.out_buf.clone()
    }

    pub fn add(&mut self, i: Input) -> Option<T> {
        self.buf.push(i);

        match self.tree.get(self.buf.iter()) {
            (Some(out), _) => {
                self.out_buf = std::mem::replace(&mut self.buf, Vec::new());
                Some(out)
            }
            (None, valid_prefix) => {
                if !valid_prefix {
                    self.buf.clear();

                    if self.buf.len() > 1 {
                        return self.add(i);
                    }
                }
                None
            }
        }
    }

    pub fn read_from_vec(&mut self, v: &[Input]) -> (Vec<Input>, T) {
        for c in v {
            match self.add(*c) {
                None => {}
                Some(out) => return (self.out_buf.clone(), out),
            }
        }

        panic!("Unknown binding: {:?}", v);
    }
}

pub fn bind_from_str(s: &str) -> Vec<Input> {
    let mut out = Vec::new();

    for c in s.chars() {
        out.push(Character(c));
    }

    out
}

#[cfg(not(target_arch = "wasm32"))]
use crate::stack::*;

pub const BOTTOM_BUFFER: i32 = 2;

#[cfg(not(target_arch = "wasm32"))]
pub fn print_stack(window: &Window, stack: &Stack) {
    let (starty, startx) = window.get_cur_yx();

    let width = window.get_max_x() as usize;
    let height = window.get_max_y() - BOTTOM_BUFFER;

    let s = stack.to_disp(width, height as usize);
    let mut lines = s.lines().rev();

    for y in (0..height).rev() {
        window.mv(y, 0);
        window.clrtoeol();
        if let Some(l) = lines.next() {
            if l.len() > width {
                window.addstr(&l[0..width].to_string());
            } else {
                window.addstr(l);
            }
        }
    }
    
    window.mv(starty, startx);
    window.refresh();
}

pub fn render_command(cmd: &str, cursor_loc: usize, width: usize)
    -> (String, usize)
{
    let len = cmd.len();

    if len >= width {
        let before_width = width * 2 / 3;
        let after_width  = width / 3;

        if cursor_loc > len {
            (
                format!("<{}", &cmd[len - width..]),
                width - 1
            )
        } else if len - cursor_loc < after_width {
            (
                format!("<{}", &cmd[len - width + 1..]),
                width - (len - cursor_loc)
            )
        } else if cursor_loc < before_width {
            (
                format!("{}>", &cmd[0..width - 1]),
                cursor_loc
            )
        } else {
            let start = cursor_loc - before_width + 1;
            let end = cursor_loc + after_width - 1;

            (
                format!("<{}>", &cmd[start..end]),
                before_width
            )
        }
    } else {
        (cmd.to_string(), cursor_loc)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn print_command(window: &Window, cmd: &str, cursor_loc: usize) {
    let width  = window.get_max_x() as usize;
    let height = window.get_max_y();

    window.mv(height - 2, 0);
    window.addstr(&"=".repeat(width));

    window.mv(height - 1, 0);
    window.clrtoeol();

    let (rend, loc) = render_command(cmd, cursor_loc, width);

    window.addstr(&rend);
    window.mv(height - 1, loc as i32);

    window.refresh();
}
