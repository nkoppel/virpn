use regex::escape;
use crate::modes::*;

mod table; 
mod helpers;

use crate::modes::ops::helpers::Op;
use crate::modes::ops::table::gen_ops;

pub struct Op_mode{
    radian: bool,
    matrix: bool,
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
            radian: true,
            matrix: false,
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
            self.bindings.values().map(|x| escape(&x[..])).collect();

        Regex::new(&format!("^{}$", names.join("$|^"))).unwrap()
    }

    fn get_name(&self) -> String {
        "ops".to_string()
    }

    fn eval_operators(&mut self, stack: &mut Stack, op: String) {
        if let Some(f) = self.ops.get(&op) {
            f(stack);
        }
    }

    fn eval_bindings(&mut self, bind: Vec<Input>)
        -> (String, Action)
    {
        (self.bindings.get(&bind).unwrap().to_string(), Exit)
    }

    fn exit(&mut self) {}
}
