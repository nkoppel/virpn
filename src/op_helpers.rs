use crate::stack::Stack;
use crate::stack::Item::*;

pub type Res<T> = Result<T, String>;

pub type Op = Box<Fn(&mut Stack) -> Res<()>>;

pub fn op_1(f: &'static impl Fn(f64) -> f64) -> Op {
    Box::new(move |stack: &mut Stack| {
        if stack.is_empty() {
            return Err("Too few arguments!".to_string());
        }

        let mut new_stack = Stack::new();
        new_stack.push(stack.pop().unwrap());
        let g: Box<Fn(Vec<f64>) -> f64> = Box::new(move |s| f(s[0]));
        stack.push(new_stack.apply_map(&g));
        Ok(())
    })
}

pub fn op_2(f: &'static impl Fn(f64, f64) -> f64) -> Op {
    Box::new(move |stack: &mut Stack| {
        if stack.len() < 2 {
            return Err("Too few arguments!".to_string());
        }

        let mut new_stack = Stack::new();
        new_stack.push(stack.pop().unwrap());
        new_stack.push(stack.pop().unwrap());
        new_stack.rev();
        let g: Box<Fn(Vec<f64>) -> f64> = Box::new(move |s| f(s[0], s[1]));
        stack.push(new_stack.apply_map(&g));
        Ok(())
    })
}

pub fn fold_op(f: &'static impl Fn(f64, f64) -> f64, start: f64) -> Op {
    Box::new(move |stack: &mut Stack| {
        stack.push(Num(
            match stack.last() {
                None => start,
                Some(Num(n)) => f(start, *n),
                Some(List(s)) => Stack::apply_fold_vec(s, f, start)
            }
        ));
        Ok(())
    })
}
