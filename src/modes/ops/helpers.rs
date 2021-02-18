use crate::modes::*;

pub type Op = Box<dyn Fn(&mut Stack) -> () + Sync + Send>;

pub fn basic<T>(f: &'static (impl Fn(&mut Stack) -> T + Sync + Send)) -> Op {
    Box::new(move |stack: &mut Stack| {f(stack);})
}

pub fn op_1(f: &'static (impl Fn(f64) -> f64 + Sync + Send)) -> Op {
    Box::new(move |stack: &mut Stack| {
        if stack.is_empty() {
            return;
        }

        let mut new_stack = Stack::new();
        new_stack.push(stack.pop().unwrap());
        let g: Box<dyn Fn(Vec<f64>) -> Item> = Box::new(move |s| Num(f(s[0])));
        stack.push(new_stack.apply_map(&g));
    })
}

pub fn op_2(f: &'static (impl Fn(f64, f64) -> f64 + Sync + Send)) -> Op {
    Box::new(move |stack: &mut Stack| {
        if stack.len() < 2 {
            return;
        }

        let mut new_stack = Stack::new();
        new_stack.push(stack.pop().unwrap());
        new_stack.push(stack.pop().unwrap());
        new_stack.rev();

        let g: Box<dyn Fn(Vec<f64>) -> Item> =
            Box::new(move |s| Num(f(s[0], s[1])));

        stack.push(new_stack.apply_map(&g));
    })
}

pub fn vec2_op(f: &'static (impl Fn(f64, f64) -> (f64, f64) + Sync + Send)) -> Op {
    Box::new(move |stack: &mut Stack| {
        stack.down();
        if stack.len() < 2 {
            return;
        }

        let i2 = stack.pop().unwrap();
        let i1 = stack.pop().unwrap();

        if let (Num(i1), Num(i2)) = (i1, i2) {
            let (o1, o2) = f(i1, i2);

            stack.push(Num(o1));
            stack.push(Num(o2));
        }

        stack.up();
    })
}

pub fn fold_op(f: &'static (impl Fn(f64, f64) -> f64 + Sync + Send), start: f64) -> Op {
    Box::new(move |stack: &mut Stack| {
        let n = stack.apply_fold(f, start);

        stack.clear();
        stack.push(Num(n));
    })
}

pub fn list_fold_op(f: &'static (impl Fn(f64, f64) -> f64 + Sync + Send), start: f64) -> Op {
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

pub fn chain(fs: Vec<Op>) -> Op {
    Box::new(move |stack: &mut Stack| {
        for f in &fs {
            f(stack)
        }
    })
}
