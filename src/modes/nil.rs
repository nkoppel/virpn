use crate::modes::*;

pub struct Nil_mode{}

impl Nil_mode {
    pub fn new() -> Self {
        Nil_mode{}
    }
}

impl Mode for Nil_mode {
    fn get_bindings(&self) -> Vec<Vec<Key>> {
        Vec::new()
    }

    fn get_operator_regex(&self) -> Regex {
        Regex::new("").unwrap()
    }

    fn copy(&self) -> Box<dyn Mode> {
        Box::new(Nil_mode{})
    }

    fn get_name(&self) -> String {
        "nil".to_string()
    }

    fn run(&mut self, modes: &ModeMap, stack: &mut Stack, ops: Vec<String>) {
        ()
    }

    // output: current string representation of mode, whether this mode is 
    // exiting, whether to pass all bindings to this mode
    fn eval_input(&mut self, modes: &ModeMap, bind: Vec<Key>)
        -> (String, bool, bool)
    {
        (String::new(), false, false)
    }
}
