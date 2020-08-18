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

    let n =
        match stack.pop() {
            Some(Num(n)) => n,
            Some(x) => {stack.push(x); return},
            None => return
        };

    let f =
        match stack.pop() {
            Some(Func(f)) => f,
            Some(x) => {stack.push(x); return},
            None => return
        };

    for _ in 0..n as usize {
        ui.eval(f.clone());
    }
}

pub fn gen_func_ops() -> Vec<(String, Vec<Vec<Input>>, FuncOp)> {
    vec![
        ("run",       vec!["ifr" ], Box::new(&run) as FuncOp),
        ("run_times", vec!["iftr"], Box::new(&run_times))
    ]
        .into_iter()
        .map(|(name, binds, op)| (
                name.to_string(),
                binds.into_iter().map(|bind| bind_from_str(bind)).collect(),
                op
            ))
        .collect()
}
