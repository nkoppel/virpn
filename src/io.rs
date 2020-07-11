use pancurses::{Input, Input::*, Window};

use std::io;
use std::io::*;
use std::result::Result;

use std::collections::{HashMap, VecDeque};

pub struct Bindings<T> {
    maxlen: usize,
    escapes: Vec<Input>,
    binds: HashMap<Vec<Input>, T>,
    buf: Vec<Input>
}

impl<T> Bindings<T> where T: Clone {
    pub fn new() -> Self {
        Bindings {
            maxlen: 0,
            escapes: Vec::new(),
            binds: HashMap::new(),
            buf: Vec::new(),
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

    pub fn add(&mut self, i: Input) -> Option<Result<T, Input>> {
        if self.escapes.contains(&i) {
            self.buf.clear();
            return Some(Err(i));
        }

        self.buf.push(i);

        if self.buf.len() > self.maxlen {
            self.buf.clear();
            return None;
        }
        match self.binds.get(&self.buf) {
            Some(out) => {
                self.buf.clear();
                Some(Ok((*out).clone()))
            }
            None => return None
        }
    }

    pub fn read(&mut self, window: &Window)
        -> Result<(Vec<Input>, T), Input>
    {
        window.keypad(true);
        pancurses::noecho();

        let mut buf = Vec::new();
        let mut c = window.getch().unwrap();

        loop {
            match self.add(c) {
                None => {
                    c = window.getch().unwrap();

                    buf.push(c);
                }
                Some(Ok(out)) => return Ok((buf, out)),
                Some(Err(key)) => return Err(key)
            }
        }
    }
}

use crate::stack::*;

const BOTTOM_BUFFER: i32 = 2;

pub fn print_stack(window: &Window, stack: &Stack) {
    let width = window.get_max_x() as usize;
    let height = window.get_max_y() - BOTTOM_BUFFER;

    let s = stack.to_string(width);
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

    window.mv(0, 0);
    window.refresh();
}

pub fn print_command(window: &Window, cmd: &str, cursor_loc: usize) {
    let width  = window.get_max_x() as usize;
    let height = window.get_max_y();

    let len = cmd.len();

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
}
