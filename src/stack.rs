#[derive(Clone, Debug)]
pub struct ProtoStack<T> {
    pub stack: Vec<T>,
    unpop_stack: Vec<T>,
}

#[derive(Clone, Debug)]
pub enum Item {
    Stack(ProtoStack<Item>),
    Num(f64),
}

pub type Stack = ProtoStack<Item>;

use Item::*;
use std::rc::Rc;

impl Stack {
    pub fn new() -> Self {
        Stack {
            stack: Vec::new(),
            unpop_stack: Vec::new()
        }
    }

    pub fn add(&mut self, i: Item) {
        self.stack.push(i);
    }

    pub fn remove(&mut self) -> Option<Item> {
        self.stack.pop()
    }

    pub fn pop(&mut self) {
        if let Some(i) =  self.stack.pop() {
            self.unpop_stack.push(i);
        }
    }

    pub fn unpop(&mut self) {
        if let Some(i) =  self.unpop_stack.pop() {
            self.stack.push(i);
        }
    }

    pub fn rev(&mut self) {
        let tmp = std::mem::replace(&mut self.stack, Vec::new());
        self.stack = tmp.into_iter().rev().collect();
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn unpop_is_empty(&self) -> bool {
        self.unpop_stack.is_empty()
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn apply_map(mut self, f: &impl Fn(Vec<f64>) -> f64) -> Item {
        let mut has_stack = false;

        for x in self.stack.iter() {
            match x {
                Stack(_) => has_stack = true,
                _ => {}
            }
        }

        if !has_stack {
            let input = self.stack.iter().map(|x|
                match x {
                    Num(n) => *n,
                    _ => panic!()
                }
            ).collect();
            return Num(f(input));
        }

        let mut has_empty = false;
        let mut rec_stack;
        let mut result = Stack::new();

        loop {
            rec_stack = Stack::new();
            for i in self.stack.iter_mut() {
                match i {
                    Num(_) => rec_stack.add(i.clone()),
                    Stack(s) => {
                        match s.remove() {
                            None => break,
                            Some(x) => rec_stack.add(x)
                        }
                    }
                }
            }

            result.add(rec_stack.apply_map(f));
        }

        Stack(result)
    }

    pub fn apply_fold(&self, f: &impl Fn(f64, f64) -> f64, mut state: f64)
        -> f64
    {
        for i in self.stack.iter() {
            match i {
                Num(n) => state = f(state, *n),
                Stack(s) => state = s.apply_fold(f, state)
            }
        }
        state
    }
}

impl Item {
    pub fn to_string(&self, indent: usize, term_width: usize) -> String {
        match self {
            Stack(s) => {
                let mut len = 0;
                let mut strs = Vec::new();

                for i in s.stack.iter() {
                    let tmp = i.to_string(indent + 2, term_width);
                    len += tmp.len();
                    strs.push(tmp);
                }

                if len + indent > term_width {
                    format!("[{}]", strs.join(" "))
                } else {
                    format!("[{}]", strs.join(&format!("\n{}", " ".repeat(indent))))
                }
            },
            Num(n) => n.to_string(),
        }
    }
}
