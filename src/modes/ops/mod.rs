use regex::escape;
use crate::modes::*;

mod table; 
mod helpers;

use crate::modes::ops::helpers::Op;
use crate::modes::ops::table::gen_ops;

#[allow(non_camel_case_types)]
pub struct Op_mode{
    bindings: HashMap<Vec<Input>, String>,
    ops: HashMap<String, Op>,
}

impl Op_mode {
    pub fn new() -> Self {
        let mut bindings = HashMap::new();
        let mut ops = HashMap::new();

        for (name, binds, op) in gen_ops() {
            for bind in binds {
                bindings.insert(bind, name.clone());
            }
            ops.insert(name, op);
        }

        Op_mode {
            bindings,
            ops
        }
    }
}

impl Mode for Op_mode {
    fn get_bindings(&self) -> Vec<Vec<Input>> {
        self.bindings.keys().map(|x| x.clone()).collect()
    }

    fn get_operator_regex(&self) -> Regex {
        let mut names: Vec<String> =
            self.ops.keys().map(|x| escape(&x[..])).collect();

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

        ui.insert_mode("ops".to_string(), Box::new(Op_mode::new()));
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
