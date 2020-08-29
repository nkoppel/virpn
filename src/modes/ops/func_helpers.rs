use crate::modes::*;

pub type FuncOp = Rc<Fn(&mut Ui) -> ()>;

fn solver<'a, F>(op: &'static F, start: f64, end: f64) -> FuncOp
    where F: Fn(f64, f64, f64) -> bool
{
    Rc::new(move |ui: &mut Ui| {
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
        ui.eval(f.clone());
        stack = ui.get_stack();
        f_start = if let Some(n) = stack.pop_as_num() {n} else {return};

        stack.push(Num(end));
        ui.eval(f.clone());
        stack = ui.get_stack();
        f_end = if let Some(n) = stack.pop_as_num() {n} else {return};

        for _ in 0..100 {
            mid = (start + end) / 2.;

            stack.push(Num(mid));
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
    solver(op, -1_000_000., 1_000_000.)
}

pub fn range_solver<'a, F>(op: &'static F) -> FuncOp
    where F: Fn(f64, f64, f64) -> bool
{
    Rc::new(move |ui: &mut Ui| {
        let stack = ui.get_stack();

        let start = if let Some(n) = stack.pop_as_num() {n} else {return};
        let end   = if let Some(n) = stack.pop_as_num() {n} else {return};

        solver(op, start, end)(ui)
    })
}

fn depth_helper<F>(ui: &mut Ui, depth: usize, f: &mut F)
    where F: FnMut(&mut Ui)
{
    if depth > 0 {
        let stack = mem::replace(ui.get_stack(), Stack::new());
        let mut out = Stack::new();

        for i in stack {
            if let List(l) = i {
                *ui.get_stack() = Stack::from_vec(l);
                depth_helper(ui, depth - 1, f);

                out.push(List(
                    mem::replace(ui.get_stack(), Stack::new()).into_vec()
                ));
            } else {
                *ui.get_stack() = Stack::from_vec(vec![i]);
                f(ui);

                if let Some(i) = ui.get_stack().pop() {
                    out.push(i);
                }
            }
        }

        *ui.get_stack() = out;
    } else {
        let stack = mem::replace(ui.get_stack(), Stack::new());
        let mut out = Stack::new();

        for i in stack {
            *ui.get_stack() = Stack::from_vec(vec![i]);
            f(ui);

            if let Some(i) = ui.get_stack().pop() {
                out.push(i);
            }
        }

        *ui.get_stack() = out;
    }
}

pub fn map_depth(ui: &mut Ui) {
    let mut stack = ui.get_stack();

    if let Some(depth) = stack.pop_as_num() {
        if let Some(f) = stack.pop_as_func() {
            let mut f = |ui: &mut Ui| { ui.eval(f.clone()) };
            let depth = depth as usize;

            if depth == 0 {
                depth_helper(ui, depth, &mut f);
            } else if let Some(l) = stack.pop_as_list() {
                let mut tmp = mem::replace(stack, Stack::from_vec(l));

                depth_helper(ui, depth - 1, &mut f);
                stack = ui.get_stack();

                mem::swap(&mut tmp, stack);
                stack.push(List(tmp.into_vec()));
            }
        }
    }
}

pub fn fold_depth(ui: &mut Ui) {
    let mut stack = ui.get_stack();

    if let Some(mut state) = stack.pop() {
        if let Some(depth) = stack.pop_as_num() {
            if let Some(f) = stack.pop_as_func() {
                let mut f = |ui: &mut Ui| {
                    ui.get_stack().push(state.clone());
                    ui.eval("swap".to_string());
                    ui.eval(f.clone());

                    if let Some(s) = ui.get_stack().pop() {
                        state = s
                    }
                };
                let depth = depth as usize;

                if depth == 0 {
                    depth_helper(ui, depth, &mut f);
                    stack = ui.get_stack();

                    stack.clear();
                    stack.push(state);
                } else if let Some(l) = stack.pop_as_list() {
                    let mut tmp = mem::replace(stack, Stack::from_vec(l));

                    depth_helper(ui, depth - 1, &mut f);
                    stack = ui.get_stack();

                    mem::swap(&mut tmp, stack);
                    stack.push(state);
                }
            }
        }
    }
}
