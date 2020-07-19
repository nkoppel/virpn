use crate::modes::*;

pub struct Nil_mode{}

impl Nil_mode {
    pub fn new() -> Box<dyn LocalMode> {
        Box::new(Nil_mode{})
    }
}

impl LocalMode for Nil_mode {
    fn eval_bindings(&mut self, bind: Vec<Input>)
        -> (String, usize, Action)
    {
        (String::new(), 0, Continue)
    }
}
