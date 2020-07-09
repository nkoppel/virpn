use pancurses::{Input, Input::*, Window};

use std::io;
use std::io::*;
use std::result::Result;

use std::collections::{HashMap, VecDeque};

pub struct Bindings<T> {
    maxlen: usize,
    escapes: Vec<Input>,
    binds: HashMap<Vec<Input>, T>
}

impl<T> Bindings<T> {
    pub fn new() -> Self {
        Bindings {
            maxlen: 0,
            escapes: Vec::new(),
            binds: HashMap::new()
        }
    }

    pub fn from_vec(v: Vec<(Vec<Input>, T)>, escapes: Vec<Input>) -> Self {
        let mut out = Bindings::new();

        out.escapes = escapes;

        for (i, o) in v {
            if i.len() > out.maxlen {
                out.maxlen = i.len();
            }

            out.binds.insert(i, o);
        }

        out
    }

    pub fn read<'a>(&'a self, window: &Window, queue: &mut VecDeque<Input>)
        -> Result<&'a T, Input>
    {
        window.keypad(true);
        pancurses::noecho();

        let mut buf = Vec::new();
        let mut c;

        while let None = self.binds.get(&buf) {
            match queue.pop_front() {
                None => c = window.getch().unwrap(),
                Some(x) => c = x
            }
            if self.escapes.contains(&c) {
                return Err(c)
            }
            buf.push(c);

            if buf.len() > self.maxlen {
                buf.clear();
            }
        }

        Ok(&self.binds.get(&buf).unwrap())
    }
}

use crate::stack::*;

const BOTTOM_BUFFER: i32 = 2;

pub fn print_stack(window: &Window, stack: &Stack) {
    let width = window.get_max_x() as usize;
    let height = window.get_max_y() - BOTTOM_BUFFER;
    let mut y = height - 1;

    let s = stack.to_string(width);
    let lines = s.lines().rev();

    for l in lines.take(height as usize) {
        window.mv(y, 0);
        window.addstr(l);
        y -= 1;
    }

    window.mv(0, 0);
    window.refresh();
}
