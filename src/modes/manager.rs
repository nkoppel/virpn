use crate::modes::*;

pub struct Manager_mode {
    bindings: HashMap<Vec<Key>, String>,
    operator_regexes: HashMap<String, Regex>,
    maxlen: usize,
    buffer: Vec<Key>,
    submode: Box<dyn Mode>,
    submode_owns: bool,
    prev_output: String
}

impl Mode for Manager_mode {
    fn get_bindings(&self) -> Vec<Vec<Key>> {
        Vec::new()
    }

    fn get_operator_regex(&self) -> Regex {
        Regex::new(r".*").unwrap()
    }

    fn get_name(&self) -> String {
        "manager".to_string()
    }

    fn copy(&self) -> Box<dyn Mode> {
        Box::new(Manager_mode{
            bindings: self.bindings.clone(),
            operator_regexes: self.operator_regexes.clone(),
            maxlen: self.maxlen,
            buffer: self.buffer.clone(),
            submode: self.submode.copy(),
            submode_owns: self.submode_owns,
            prev_output: self.prev_output.clone()
        })
    }

    fn run(&mut self, modes: &ModeMap, stack: &mut Stack, ops: Vec<String>) {
        ()
    }

    fn eval_input(&mut self, modes: &ModeMap, bind: Vec<Key>)
        -> (String, bool, bool)
    {
        if !self.submode_owns {
            self.buffer.push(bind[0]);

            if self.buffer.len() > self.maxlen {
                self.buffer.clear();
                self.prev_output = String::new();
                return (String::new(), false, false);
            }

            if let Some(submode) = self.bindings.get(&self.buffer) {
                if &self.submode.get_name() != submode {
                    self.submode = (*modes.get(&submode[..]).unwrap()).copy();
                }
            } else {
                return (self.prev_output.clone(), false, false);
            }
        }

        let (s, exit, own) = self.submode.eval_input(modes, bind);

        self.prev_output = s.clone();

        if exit {
            self.submode_owns = false;
            return (s, true, false);
        } else {
            self.submode_owns = own;
            return (s, false, false);
        }
    }
}
