use pancurses::{Input, Input::*, Window};

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

    pub fn add(&mut self, i: Input) -> Option<T> {
        self.buf.push(i.clone());

        match self.tree.get(self.buf.iter()) {
            (Some(out), _) => {
                self.out_buf = std::mem::replace(&mut self.buf, Vec::new());
                Some(out.clone())
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

    pub fn read_from_vec(&mut self, v: &Vec<Input>) -> (Vec<Input>, T) {
        for c in v {
            match self.add(*c) {
                None => {}
                Some(out) => return (self.out_buf.clone(), out),
            }
        }

        panic!();
    }

    pub fn read(&mut self, window: &Window)
        -> (Vec<Input>, T)
    {
        window.keypad(true);
        pancurses::noecho();

        loop {
            match self.add(window.getch().unwrap()) {
                None => {}
                Some(out) => return (self.out_buf.clone(), out),
            }
        }
    }
}

pub fn bind_from_str(s: &str) -> Vec<Input> {
    let mut out = Vec::new();

    for c in s.chars() {
        out.push(Character(c));
    }

    out
}

use crate::stack::*;

const BOTTOM_BUFFER: i32 = 2;

pub fn print_stack(window: &Window, stack: &Stack) {
    let (starty, startx) = window.get_cur_yx();

    let width = window.get_max_x() as usize;
    let height = window.get_max_y() - BOTTOM_BUFFER;

    let s = stack.to_string(width, height as usize);
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

pub fn print_command(window: &Window, cmd: &str, cursor_loc: usize) {
    let width  = window.get_max_x() as usize;
    let height = window.get_max_y();

    let len = cmd.len();

    window.mv(height - 2, 0);
    window.addstr("=".repeat(width));

    window.mv(height - 1, 0);
    window.clrtoeol();

    if len >= width {
        let before_width = width * 2 / 3;
        let after_width  = width * 1 / 3;
        
        if len - cursor_loc < after_width {
            window.addstr(&format!("<{}", cmd[len - width + 1..].to_string()));
            window.mv(height - 1, (width - (len - cursor_loc)) as i32);
        } else if cursor_loc < before_width {
            window.addstr(&format!("{}>", cmd[0..width - 1].to_string()));
            window.mv(height - 1, cursor_loc as i32);
        } else {
            let start = cursor_loc - before_width + 1;
            let end = cursor_loc + after_width - 1;
            window.addstr(&format!("<{}>", cmd[start..end].to_string()));
            window.mv(height - 1, before_width as i32);
        }
    } else {
        window.addstr(cmd);
        window.mv(height - 1, cursor_loc as i32);
    }

    window.refresh();
}
