#[derive(Clone, Debug)]
pub enum Item {
    List(Vec<Item>),
    Num(f64),
}

#[derive(Clone, Debug)]
pub struct Stack {
    above: Vec<Vec<Item>>,
    curr: Vec<Item>,
}

pub use Item::*;

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
            curr: v.into_iter().map(|x| Num(x)).collect(),
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

    pub fn push(&mut self, i: Item) {
        self.curr.push(i);
    }

    pub fn pop(&mut self) -> Option<Item> {
        self.curr.pop()
    }

    pub fn up(&mut self) -> bool {
        if self.above.is_empty() {
            false
        } else {
            let mut tmp = self.above.pop().unwrap();
            std::mem::swap(&mut tmp, &mut self.curr);
            self.curr.push(List(tmp));
            true
        }
    }

    pub fn down(&mut self) -> bool {
        if self.curr.is_empty() {
            false
        } else {
            if let List(mut tmp) = self.curr.pop().unwrap() {
                std::mem::swap(&mut tmp, &mut self.curr);
                self.above.push(tmp);
                true
            } else {
                false
            }
        }
    }

    pub fn rev(&mut self) {
        let tmp = std::mem::replace(&mut self.curr, Vec::new());
        self.curr = tmp.into_iter().rev().collect();
    }

    pub fn last<'a>(&'a self) -> Option<&'a Item> {
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

    pub fn apply_map(mut self, f: &impl Fn(Vec<f64>) -> f64) -> Item {
        let mut has_stack = false;

        for x in self.curr.iter() {
            match x {
                List(_) => has_stack = true,
                _ => {}
            }
        }

        if !has_stack {
            let input = self.curr.iter().map(|x|
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
            for i in self.curr.iter_mut() {
                match i {
                    Num(_) => rec_stack.push(i.clone()),
                    List(s) => {
                        match s.pop() {
                            None => return List(result.into_vec()),
                            Some(x) => rec_stack.push(x)
                        }
                    }
                }
            }

            result.push(rec_stack.apply_map(f));
        }

    }

    pub fn apply_fold_vec(v: &Vec<Item>,
                         f: &impl Fn(f64, f64) -> f64,
                         mut state: f64) -> f64
    {
        for i in v.iter() {
            match i {
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

    pub fn to_string(&self, term_width: usize) -> String {
        let strs: Vec<String> =
            self.curr.iter() .map(|x| x.to_string(0, term_width)).collect();

        strs.join("\n")
    }
}

impl Item {
    pub fn to_string(&self, indent: usize, term_width: usize) -> String {
        match self {
            List(s) => {
                let mut len = 2;
                let mut strs = Vec::new();

                for i in s.iter() {
                    let tmp = i.to_string(indent + 2, term_width);
                    len += tmp.len() + 1;
                    strs.push(tmp);
                }

                let base_indent = " ".repeat(indent);

                if len + indent > term_width {
                    format!("[\n{0}  {1}\n{0}]", base_indent, strs.join(&format!("\n{}  ", base_indent)))
                } else {
                    format!("[{}]", strs.join(" "))
                }
            },
            Num(n) => n.to_string(),
        }
    }
}
