use crate::modes::*;

pub struct Nil_mode{}

impl Nil_mode {
    pub fn new() -> Self {
        Nil_mode{}
    }
}

impl Mode for Nil_mode {
    fn get_bindings(&self) -> Vec<Vec<Input>> {
        Vec::new()
    }

    fn get_operator_regex(&self) -> Regex {
        Regex::new("[[:^ascii:]]").unwrap()
    }

    fn get_name(&self) -> String {
        "nil".to_string()
    }

    fn eval_operators(&mut self, stack: &mut Stack, op: String) {
        ()
    }

    fn eval_bindings(&mut self, bind: Vec<Input>)
        -> (String, Action)
    {
        (String::new(), Continue)
    }

    fn exit(&mut self) {}
}
