use regex::escape;
use crate::modes::*;

mod table; 
mod helpers;

mod func_table; 
mod func_helpers;

use crate::modes::ops::helpers::Op;
use crate::modes::ops::table::gen_ops;
use crate::modes::ops::func_table::*;

#[allow(non_camel_case_types)]
pub struct Op_mode{
    bindings: HashMap<Vec<Input>, String>,
    ops: HashMap<String, Op>,
    func_ops: HashMap<String, FuncOp>,
}

impl Op_mode {
    fn empty() -> Self {
        Op_mode {
            bindings: HashMap::new(),
            ops: HashMap::new(),
            func_ops: HashMap::new(),
        }
    }

    pub fn new() -> Self {
        let mut bindings = HashMap::new();
        let mut ops = HashMap::new();
        let mut func_ops = HashMap::new();

        for (name, binds, op) in gen_ops() {
            for bind in binds {
                bindings.insert(bind, name.clone());
            }
            ops.insert(name, op);
        }

        for (name, binds, op) in gen_func_ops() {
            for bind in binds {
                bindings.insert(bind, name.clone());
            }
            func_ops.insert(name, op);
        }

        Op_mode {
            bindings,
            ops,
            func_ops
        }
    }
}

impl Mode for Op_mode {
    fn get_bindings(&self) -> Vec<Vec<Input>> {
        self.bindings.keys().map(|x| x.clone()).collect()
    }

    fn get_operator_regex(&self) -> Regex {
        let mut names: Vec<&String> = self.ops.keys().collect();
        names.extend(self.func_ops.keys());

        let mut names: Vec<String> =
            names.into_iter().map(|x| escape(&x[..])).collect();

        // sort in order of descending length
        names.sort_by(|x, y| y.len().cmp(&x.len()));

        Regex::new(&format!("^{}", names.join("|^"))).unwrap()
    }

    fn get_name(&self) -> String {
        "ops".to_string()
    }

    fn eval_operators(&mut self, ui: &mut Ui, op: &str) {
        if let Some(f) = self.ops.get(op) {
            f(ui.get_stack());
        }
        else if let Some(_) = self.func_ops.get(op) {
            let f = self.func_ops.get(op).unwrap().clone();

            ui.insert_mode(
                "ops".to_string(),
                Box::new(mem::replace(self, Self::empty()))
            );

            f(ui);

            return;
        }

        ui.insert_mode(
            "ops".to_string(),
            Box::new(mem::replace(self, Self::empty()))
        );
    }

    fn eval_bindings(&self, mut ui: Ui_helper, _: HashMap<&str, &str>)
        -> ModeRes<(String, usize)>
    {
        let (bind, res) = ui.get_next_binding();
        let op = self.bindings.get(&bind).unwrap().to_string();
        let len = op.len();

        ui.print_output(&op, len);

        ((op, len), res)
    }
}
