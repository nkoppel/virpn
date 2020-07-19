use crate::modes::*;

#[derive(Clone, Debug)]
pub struct Number_global {}

#[derive(Clone, Debug)]
pub struct Number_mode {
    buffer: String
}

impl GlobalMode for Number_global {
    fn get_bindings(&self) -> Vec<Vec<Input>> {
        vec![
            vec![Character('a')],
            vec![Character('s')],
            vec![Character('d')],
            vec![Character('f')],
            vec![Character('g')],
            vec![Character('h')],
            vec![Character('j')],
            vec![Character('k')],
            vec![Character('l')],
            vec![Character(';')],
            vec![Character('n')],
            vec![Character('m')],
            vec![KeyBackspace]
        ]
    }

    fn get_operator_regex(&self) -> Regex {
        Regex::new(r"-?\d*.?\d+").unwrap()
    }

    fn get_name(&self) -> String {
        "number".to_string()
    }

    fn eval_operators(&mut self, ui: &mut Ui, op: String) {
        match op.parse::<f64>() {
            Ok(f) => ui.get_stack().push(Num(f)),
            Err(_) => ()
        }
    }

    fn build_local(self: Rc<Self>, init: String) -> Box<dyn LocalMode> {
        Box::new(Number_mode {
            buffer: init
        })
    }
}

impl LocalMode for Number_mode {
    fn eval_bindings(&mut self, bind: Vec<Input>)
        -> (String, usize, Action)
    {
        match bind[0] {
            Character('a') => {self.buffer.push('1')},
            Character('s') => {self.buffer.push('2')},
            Character('d') => {self.buffer.push('3')},
            Character('f') => {self.buffer.push('4')},
            Character('g') => {self.buffer.push('5')},
            Character('h') => {self.buffer.push('6')},
            Character('j') => {self.buffer.push('7')},
            Character('k') => {self.buffer.push('8')},
            Character('l') => {self.buffer.push('9')},
            Character(';') => {self.buffer.push('0')},
            Character('n') => {self.buffer.push('-')},
            Character('m') => {self.buffer.push('.')},
            KeyBackspace   => {self.buffer.pop();},
            _ => panic!()
        }

        if !self.buffer.is_empty() {
            let tmp = &self.buffer[..self.buffer.len() - 1];

            if tmp.len() > 1 || tmp == "." {
                if let Err(_) = self.buffer.parse::<f64>() {
                    self.buffer.pop();
                }
            }
        }

        (self.buffer.clone(), self.buffer.len(), Continue)
    }
}
