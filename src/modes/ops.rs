#[derive(Clone, Debug)]
pub struct Op_mode {
    radian: bool,
    matrix: bool
}

impl Mode for Op_mode {
    fn get_bindings(&self) -> Vec<Vec<Key>> {
        // TODO compile bindings
    }

    fn get_operator_regex(&self) -> Regex {
        // TODO make regex
    }

    fn copy(&self) -> Box<dyn Mode> {
        Box::new(self.clone())
    }

    fn get_name(&self) -> String {
        return "op"
    }

    fn run(&mut self, modes: &ModeMap, stack: &mut Stack, ops: Vec<String>) {
        
    }

    // output: current string representation of mode, whether this mode is 
    // exiting, whether to pass all bindings to this mode
    fn eval_input(&mut self, modes: &ModeMap, bind: Vec<Key>)
        -> (String, bool, bool)
    {

    }
}
