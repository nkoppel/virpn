use crate::stack::Stack;
use crate::stack::Item::*;

pub type Op = Box<Fn(&mut Stack) -> ()>;

pub fn basic<T>(f: &'static impl Fn(&mut Stack) -> T) -> Op {
    Box::new(move |stack: &mut Stack| {f(stack);})
}

pub fn op_1(f: &'static impl Fn(f64) -> f64) -> Op {
    Box::new(move |stack: &mut Stack| {
        if stack.is_empty() {
            return;
        }

        let mut new_stack = Stack::new();
        new_stack.push(stack.pop().unwrap());
        let g: Box<Fn(Vec<f64>) -> f64> = Box::new(move |s| f(s[0]));
        stack.push(new_stack.apply_map(&g));
    })
}

pub fn op_2(f: &'static impl Fn(f64, f64) -> f64) -> Op {
    Box::new(move |stack: &mut Stack| {
        if stack.len() < 2 {
            return;
        }

        let mut new_stack = Stack::new();
        new_stack.push(stack.pop().unwrap());
        new_stack.push(stack.pop().unwrap());
        new_stack.rev();
        let g: Box<Fn(Vec<f64>) -> f64> = Box::new(move |s| f(s[0], s[1]));
        stack.push(new_stack.apply_map(&g));
    })
}

pub fn fold_op(f: &'static impl Fn(f64, f64) -> f64, start: f64) -> Op {
    Box::new(move |stack: &mut Stack| {
        let n = stack.apply_fold(f, start);

        stack.clear();
        stack.push(Num(n));
    })
}

pub fn list_fold_op(f: &'static impl Fn(f64, f64) -> f64, start: f64) -> Op {
    Box::new(move |stack: &mut Stack| {
        match stack.pop() {
            Some(List(s)) => {
                let n = Stack::apply_fold_vec(&s, f, start);

                stack.push(Num(n));
            },
            Some(i) => stack.push(i),
            None => {}
        }
    })
}

pub fn constant(x: f64) -> Op {
    Box::new(move |stack: &mut Stack| {
        stack.push(Num(x));
    })
}
