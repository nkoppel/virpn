use crate::modes::*;

pub type FuncOp = Box<Fn(&mut Ui) -> ()>;

fn solver<'a, F>(op: &'static F, start: f64, end: f64) -> FuncOp
    where F: Fn(f64, f64, f64) -> bool
{
    Box::new(move |ui: &mut Ui| {
        let mut tmp_stack = Stack::new();
        let mut stack = ui.get_stack();

        mem::swap(stack, &mut tmp_stack);

        let mut mid;

        let mut start = start;
        let mut end = end;

        let mut f_start;
        let mut f_end;
        let mut f_mid;

        let f = if let Some(f) = tmp_stack.pop_as_func() {f} else {return};

        stack.push(Num(start));
        mem::drop(stack);
        ui.eval(f.clone());
        stack = ui.get_stack();
        f_start = if let Some(n) = stack.pop_as_num() {n} else {return};

        stack.push(Num(end));
        mem::drop(stack);
        ui.eval(f.clone());
        stack = ui.get_stack();
        f_end = if let Some(n) = stack.pop_as_num() {n} else {return};

        for _ in 0..100 {
            mid = (start + end) / 2.;

            stack.push(Num(mid));
            mem::drop(stack);
            ui.eval(f.clone());
            stack = ui.get_stack();
            f_mid = if let Some(n) = stack.pop_as_num() {n} else {return};

            if op(f_start, f_mid, f_end) {
                start = mid;
                f_start = f_mid;
            } else {
                end = mid;
                f_end = f_mid;
            }
        }

        mem::swap(stack, &mut tmp_stack);

        stack.push(List(vec![Num(start), Num(f_start)]));
    })
}

pub fn million_solver<'a, F>(op: &'static F) -> FuncOp
    where F: Fn(f64, f64, f64) -> bool
{
    solver(op, -1000000., 1000000.)
}

pub fn range_solver<'a, F>(op: &'static F) -> FuncOp
    where F: Fn(f64, f64, f64) -> bool
{
    Box::new(move |ui: &mut Ui| {
        let stack = ui.get_stack();

        let start = if let Some(n) = stack.pop_as_num() {n} else {return};
        let end   = if let Some(n) = stack.pop_as_num() {n} else {return};

        mem::drop(stack);

        solver(op, start, end)(ui)
    })
}
