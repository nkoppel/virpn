use crate::stack::Stack;
use crate::stack::Item::*;

pub type Res<T> = Result<T, String>;

pub fn op_1(f: &'static impl Fn(f64) -> f64) -> Box<Fn(&mut Stack) -> Res<()>> {
    Box::new(move |stack: &mut Stack| {
        if stack.is_empty() {
            return Err("Too few arguments!".to_string());
        }

        let mut new_stack = Stack::new();
        new_stack.add(stack.remove().unwrap());
        let g: Box<Fn(Vec<f64>) -> f64> = Box::new(move |s| f(s[0]));
        stack.add(new_stack.apply_map(&g));
        Ok(())
    })
}

pub fn op_2(f: &'static impl Fn(f64, f64) -> f64) -> Box<Fn(&mut Stack) -> Res<()>> {
    Box::new(move |stack: &mut Stack| {
        if stack.len() < 2 {
            return Err("Too few arguments!".to_string());
        }

        let mut new_stack = Stack::new();
        new_stack.add(stack.remove().unwrap());
        new_stack.add(stack.remove().unwrap());
        new_stack.rev();
        let g: Box<Fn(Vec<f64>) -> f64> = Box::new(move |s| f(s[0], s[1]));
        stack.add(new_stack.apply_map(&g));
        Ok(())
    })
}

pub fn fold_op(f: &'static impl Fn(f64, f64) -> f64, start: f64)
    -> Box<Fn(&mut Stack) -> Res<()>>
{
    Box::new(move |stack: &mut Stack| {
        stack.add(Num(
            match stack.stack.last() {
                None => start,
                Some(Num(n)) => f(start, *n),
                Some(Stack(s)) => s.apply_fold(f, start)
            }
        ));
        Ok(())
    })
}
