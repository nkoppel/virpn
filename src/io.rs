use std::io;
use std::io::*;
use std::result::Result;
use termion::event::Key;
use termion::event::Key::*;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::cursor::DetectCursorPos;

use std::collections::HashMap;

pub struct Bindings<T> {
    maxlen: usize,
    escapes: Vec<Key>,
    binds: HashMap<Vec<Key>, T>
}

impl<T> Bindings<T> {
    pub fn new() -> Self {
        Bindings {
            maxlen: 0,
            escapes: Vec::new(),
            binds: HashMap::new()
        }
    }

    pub fn from_vec(v: Vec<(Vec<Key>, T)>, escapes: Vec<Key>) -> Self {
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
}

use std::collections::VecDeque;

pub fn read_with_bindings<'a, T>(bindings: &'a Bindings<T>, queue: &mut VecDeque<Key>)
    -> Result<&'a T, Key>
{
    let mut buf = Vec::new();

    let mut keys = stdin().keys();
    let _stdout = stdout().into_raw_mode().unwrap();
    let mut c;

    while let None = bindings.binds.get(&buf) {
        match queue.pop_front() {
            None => c = keys.next().unwrap().ok().unwrap(),
            Some(x) => c = x
        }
        if bindings.escapes.contains(&c) {
            return Err(c)
        }
        buf.push(c);

        if buf.len() > bindings.maxlen {
            buf.clear();
        }
    }

    Ok(&bindings.binds.get(&buf).unwrap())
}

pub fn reprint(s: &str, old_pos: usize, new_pos: usize) {
    let mut stdout = stdout().into_raw_mode().unwrap();

    let (width, height) = termion::terminal_size().ok().unwrap();
    let (x, y) = stdout.cursor_pos().ok().unwrap();

    let lines = old_pos as u16 / width;

    write!(stdout, "{}", termion::cursor::Goto(1, y - lines));
    write!(stdout, "{}", termion::clear::AfterCursor);
    std::mem::drop(stdout);
    print!("{}", s);
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    let (_, y) = stdout.cursor_pos().ok().unwrap();

    let lines = s.len() as u16 / width - new_pos as u16 / width;

    write!(stdout, "{}", termion::cursor::Goto(new_pos as u16 % width + 1, y - lines));

    stdout.flush();
}
