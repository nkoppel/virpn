#[derive(Clone, Debug)]
pub enum Item {
    List(Vec<Item>),
    Num(f64),
    Func(String),
}

#[derive(Clone, Debug)]
pub struct Stack {
    above: Vec<Vec<Item>>,
    curr: Vec<Item>,
}

pub use Item::*;

#[allow(dead_code)]
impl Stack {
    pub fn new() -> Self {
        Stack {
            above: Vec::new(),
            curr: Vec::new(),
        }
    }

    pub fn from_nums(v: Vec<f64>) -> Self {
        Stack {
            above: Vec::new(),
            curr: v.into_iter().map(Num).collect(),
        }
    }

    pub fn from_vec(v: Vec<Item>) -> Self {
        Stack {
            above: Vec::new(),
            curr: v,
        }
    }

    pub fn into_vec(self) -> Vec<Item> {
        self.curr
    }

    pub fn insert(&mut self, index: usize, i: Item) {
        self.curr.insert(index, i);
    }

    pub fn push(&mut self, i: Item) {
        self.curr.push(i);
    }

    pub fn pop(&mut self) -> Option<Item> {
        self.curr.pop()
    }

    pub fn pop_as_num(&mut self) -> Option<f64> {
        match self.pop() {
            Some(Num(n)) => return Some(n),
            Some(x) => self.push(x),
            None => {}
        }
        None
    }

    pub fn pop_as_list(&mut self) -> Option<Vec<Item>> {
        match self.pop() {
            Some(List(l)) => return Some(l),
            Some(x) => self.push(x),
            None => {}
        }
        None
    }

    pub fn pop_as_func(&mut self) -> Option<String> {
        match self.pop() {
            Some(Func(f)) => return Some(f),
            Some(x) => self.push(x),
            None => {}
        }
        None
    }

    pub fn up(&mut self) {
        let mut tmp = self.above.pop().unwrap_or_default();
        std::mem::swap(&mut tmp, &mut self.curr);
        self.curr.push(List(tmp));
    }

    pub fn down(&mut self) {
        let mut tmp =
            if let Some(List(_)) = self.curr.last() {
                if let Some(List(l)) = self.curr.pop() {
                    l
                } else {
                    panic!();
                }
            } else {
                Vec::new()
            };

        std::mem::swap(&mut tmp, &mut self.curr);
        self.above.push(tmp);
    }

    pub fn rev(&mut self) {
        let tmp = std::mem::replace(&mut self.curr, Vec::new());
        self.curr = tmp.into_iter().rev().collect();
    }

    pub fn last(&self) -> Option<&Item> {
        self.curr.last()
    }

    pub fn is_empty(&self) -> bool {
        self.curr.is_empty()
    }

    pub fn clear(&mut self) {
        self.curr.clear();
    }

    pub fn len(&self) -> usize {
        self.curr.len()
    }

    pub fn apply_map(self, f: &impl Fn(Vec<f64>) -> Item) -> Item {
        let mut has_list = false;

        for x in self.curr.iter() {
            if let List(_) = x {
                has_list = true;
                break;
            }
        }

        if !has_list {
            let mut input = Vec::new();

            for x in &self.curr {
                if let Num(n) = x {
                    input.push(*n);
                }
            }

            return f(input);
        }

        let mut iters: Vec<_> =
            self.curr.into_iter().map(|i| {
                let tmp: Box<dyn Iterator<Item = Item>> =
                    match i {
                        Num(_) | Func(_) => Box::new(std::iter::repeat(i)),
                        List(l) => Box::new(l.into_iter())
                    };

                tmp
            }).collect();

        let mut has_fn;
        let mut rec_stack;
        let mut result = Vec::new();

        loop {
            has_fn = false;
            rec_stack = Stack::new();

            for i in iters.iter_mut() {
                let tmp = i.next();
                match tmp {
                    Some(Func(_)) => {
                        has_fn = true;
                        rec_stack.push(tmp.unwrap());
                    }
                    Some(item) => rec_stack.push(item),
                    None => return List(result)
                }
            }

            if !has_fn {
                result.push(rec_stack.apply_map(f));
            }
        }
    }

    pub fn apply_fold_vec(v: &[Item],
                         f: &impl Fn(f64, f64) -> f64,
                         mut state: f64) -> f64
    {
        for i in v.iter() {
            match i {
                Func(_) => {},
                Num(n) => state = f(state, *n),
                List(s) => state = Stack::apply_fold_vec(s, f, state)
            }
        }
        state
    }

    pub fn apply_fold(&self, f: &impl Fn(f64, f64) -> f64, start: f64)
        -> f64
    {
        Stack::apply_fold_vec(&self.curr, f, start)
    }

    pub fn to_disp(&self, width: usize, height: usize) -> String {
        let strs: Vec<String> =
            self.curr.iter().map(|x| x.to_disp(0, width, height)).collect();

        strs.join("\n")
    }
}

fn show_number(n: f64) -> String {
    if n == 0. {
        if n.is_sign_positive() {
            "0".to_string()
        } else {
            "-0".to_string()
        }
    } else if n.abs() >= 1e12 || n.abs() <= 1e-12 {
        format!("{:e}", n)
    } else {
        format!("{}", n)
    }
}

impl Item {
    pub fn to_disp(&self, indent: usize, width: usize, height: usize)
        -> String
    {
        match self {
            List(s) => {
                let mut len = 2;
                let mut strs = Vec::new();

                for i in s.iter().rev() {
                    let tmp = i.to_disp(indent + 2, width, height);
                    len += tmp.len() + 1;
                    strs.push(tmp);

                    if len + indent > width && strs.len() > height {
                        break;
                    }
                }

                strs.reverse();

                let base_indent = " ".repeat(indent);

                if len + indent > width {
                    format!("[\n{0}  {1}\n{0}]", base_indent,
                        strs.join(&format!("\n{}  ", base_indent)))
                } else {
                    format!("[{}]", strs.join(" "))
                }
            },
            Num(n) => show_number(*n),
            Func(s) => format!("({})", s),
        }
    }
}

use std::vec::IntoIter;
use std::slice::{Iter, IterMut};

impl IntoIterator for Stack {
    type IntoIter = IntoIter<Item>;
    type Item = Item;

    fn into_iter(self) -> IntoIter<Item> {
        self.curr.into_iter()
    }
}

impl<'a> IntoIterator for &'a Stack {
    type Item = &'a Item;
    type IntoIter = Iter<'a, Item>;

    fn into_iter(self) -> Iter<'a, Item> {
        self.curr.iter()
    }
}

impl<'a> IntoIterator for &'a mut Stack {
    type Item = &'a mut Item;
    type IntoIter = IterMut<'a, Item>;

    fn into_iter(self) -> IterMut<'a, Item> {
        self.curr.iter_mut()
    }
}

use std::fmt;

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            List(s) => {
                let mut strs = Vec::new();

                for i in s.iter() {
                    strs.push(i.to_string());
                }

                write!(f, "[ {} ]", strs.join(" "))
            },
            Num(n) => write!(f, "{}", show_number(*n)),
            Func(s) => write!(f, "( {} )", s),
        }
    }
}


impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}
