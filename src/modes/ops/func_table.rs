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

fn min (x: f64, _: f64, y: f64) -> bool { y > x }
fn max (x: f64, _: f64, y: f64) -> bool { y < x }
fn zero(_: f64, x: f64, y: f64) -> bool { (x > 0.) != (y > 0.) }

pub fn gen_func_ops() -> Vec<(String, Vec<Vec<Input>>, FuncOp)> {
    vec![
        ("run",        vec!["ifr" ], Rc::new(&run) as FuncOp),
        ("run_times",  vec!["iftr"], Rc::new(&run_times)),

        ("max",        vec!["irx" ], million_solver(&min)),
        ("min",        vec!["irn" ], million_solver(&max)),
        ("zero",       vec!["irz" ], million_solver(&zero)),

        ("range_max",  vec!["irrx" ], range_solver(&min)),
        ("range_min",  vec!["irrn" ], range_solver(&max)),
        ("range_zero", vec!["irrz" ], range_solver(&zero)),

        ("map_depth",  vec!["ifdm"], Rc::new(map_depth)),
        ("map"      ,  vec!["ifm" ], Rc::new(|ui: &mut Ui| {ui.get_stack().push(Num(0.)); map_depth(ui)})),
        ("map_nums" ,  vec!["ifnm"], Rc::new(|ui: &mut Ui| {ui.get_stack().push(Num(usize::max_value() as f64)); map_depth(ui)})),

        ("fold_depth",  vec!["ifdf"], Rc::new(fold_depth)),
        ("fold"      ,  vec!["iff" ], Rc::new(|ui: &mut Ui| {ui.get_stack().push(Num(0.)); fold_depth(ui)})),
        ("fold_nums" ,  vec!["ifnf"], Rc::new(|ui: &mut Ui| {ui.get_stack().push(Num(usize::max_value() as f64)); fold_depth(ui)})),
    ]
        .into_iter()
        .map(|(name, binds, op)| (
                name.to_string(),
                binds.into_iter().map(|bind| bind_from_str(bind)).collect(),
                op
            ))
        .collect()
}
