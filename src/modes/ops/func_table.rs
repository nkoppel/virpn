use crate::modes::*;
pub use crate::modes::ops::func_helpers::*;

fn run(ui: &mut Ui) {
    let stack = ui.get_stack();

    match stack.pop() {
        Some(Func(f)) => {ui.eval(f)},
        Some(x) => stack.push(x),
        None => {}
    }
}

fn run_times(ui: &mut Ui) {
    let stack = ui.get_stack();

    let n = if let Some(n) = stack.pop_as_num () {n} else {return};
    let f = if let Some(f) = stack.pop_as_func() {f} else {return};

    for _ in 0..n as usize {
        ui.eval(f.clone());
    }
}

fn integrate_rects(ui: &mut Ui) {
    let mut stack = ui.get_stack();

    let rect = if let Some(n) = stack.pop_as_num () {n as usize} else {return};
    let high = if let Some(n) = stack.pop_as_num () {n} else {return};
    let low  = if let Some(n) = stack.pop_as_num () {n} else {return};
    let f    = if let Some(f) = stack.pop_as_func() {f} else {return};

    let mut num = low;
    let delta = (high - low) / rect as f64;

    let mut multipliers = Vec::new();
    let mut inputs = Vec::new();

    for i in 0..rect+1 {
        if i == 0 || i == rect {
            multipliers.push(Num(1.));
        } else {
            multipliers.push(Num(((i % 2 + 1) * 2) as f64));
        }

        inputs.push(Num(num));

        num += delta;
    }

    stack.push(List(multipliers));
    stack.push(List(inputs));

    ui.eval(f);
    ui.eval("* sum_list".to_string());

    stack = ui.get_stack();

    let n = stack.pop_as_num().unwrap();
    stack.push(Num(n * (high - low) / (3. * rect as f64)));
}

fn integrate(ui: &mut Ui) {
    ui.get_stack().push(Num(100000.));
    integrate_rects(ui);
}

fn euler_approx(log: bool, ui: &mut Ui) {
    let stack = ui.get_stack();

    let steps = if let Some(n) = stack.pop_as_num () {n as usize} else {return};
    let endx  = if let Some(n) = stack.pop_as_num () {n} else {return};
    let start = if let Some(l) = stack.pop_as_list() {l} else {return};
    let func  = if let Some(f) = stack.pop_as_func() {f} else {return};

    let mut start = Stack::from_vec(start);

    let mut y = if let Some(n) = start.pop_as_num() {n} else {return};
    let mut x = if let Some(n) = start.pop_as_num() {n} else {return};

    let delta = (endx - x) / steps as f64;
    let mut out = Stack::new();

    for _ in 0..steps {
        ui.get_stack().push(Num(x));
        ui.get_stack().push(Num(y));
        ui.eval(func.clone());
        let slope = ui.get_stack().pop_as_num().unwrap();

        if log {
            out.push(List(vec![Num(x), Num(y), Num(slope)]));
        }
        x += delta;
        y += slope * delta;
    }

    if log {
        ui.get_stack().push(List(out.into_vec()));
    }
    ui.get_stack().push(List(vec![Num(x), Num(y)]));
}

fn derivative_at(ui: &mut Ui) {
    let stack = ui.get_stack();

    let x = if let Some(n) = stack.pop_as_num () {n} else {return};
    let f = if let Some(f) = stack.pop_as_func() {f} else {return};

    let d = (f64::EPSILON * 1000000000. * x).max(f64::EPSILON);
    let x2 = x + d;

    ui.get_stack().push(Num(x));
    ui.eval(f.clone());
    let y = ui.get_stack().pop_as_num().unwrap();

    ui.get_stack().push(Num(x2));
    ui.eval(f);
    let y2 = ui.get_stack().pop_as_num().unwrap();

    ui.get_stack().push(Num((y2 - y) / d));
}

fn sequence(ui: &mut Ui) {
    let stack = ui.get_stack();

    let n = if let Some(n) = stack.pop_as_num () {n} else {return};
    let x = if let Some(i) = stack.pop        () {i} else {return};
    let f = if let Some(f) = stack.pop_as_func() {f} else {return};

    stack.down();
    stack.push(x);

    for _ in 0..n as usize {
        let item = ui.get_stack().last().unwrap().clone();
        ui.get_stack().push(item);
        ui.eval(f.clone());
    }

    ui.get_stack().up()
}

fn zero(_: f64, x: f64, y: f64) -> bool { (x > 0.) != (y > 0.) }

pub fn gen_func_ops() -> Vec<(String, Vec<Vec<Input>>, FuncOp)> {
    vec![
        ("run",        vec!["ifr" ], Arc::new(&run) as FuncOp),
        ("run_times",  vec!["iftr"], Arc::new(&run_times)),

        ("max",        vec!["irx" ], Arc::new(|ui| optimize(true, ui))),
        ("min",        vec!["irn" ], Arc::new(|ui| optimize(false, ui))),
        ("zero",       vec!["irz" ], range_solver(&zero)),

        ("sequence", vec!["ifq"], Arc::new(sequence)),

        ("area",  vec!["ifa"], Arc::new(integrate)),
        ("slope", vec!["ifs"], Arc::new(derivative_at)),

        ("euler",        vec!["ife"] , Arc::new(|ui| euler_approx(false, ui))),
        ("euler_log",    vec!["ifle"], Arc::new(|ui| euler_approx(true, ui))),

        ("map_depth",  vec!["ifdm"], Arc::new(map_depth)),
        ("map"      ,  vec!["ifm" ], Arc::new(|ui: &mut Ui| {ui.get_stack().push(Num(0.)); map_depth(ui)})),
        ("map_nums" ,  vec!["ifnm"], Arc::new(|ui: &mut Ui| {ui.get_stack().push(Num(usize::max_value() as f64)); map_depth(ui)})),

        ("fold_depth",  vec!["ifdf"], Arc::new(fold_depth)),
        ("fold"      ,  vec!["iff" ], Arc::new(|ui: &mut Ui| {ui.get_stack().push(Num(0.)); fold_depth(ui)})),
        ("fold_nums" ,  vec!["ifnf"], Arc::new(|ui: &mut Ui| {ui.get_stack().push(Num(usize::max_value() as f64)); fold_depth(ui)})),
    ]
        .into_iter()
        .map(|(name, binds, op)| (
                name.to_string(),
                binds.into_iter().map(|bind| bind_from_str(bind)).collect(),
                op
            ))
        .collect()
}
