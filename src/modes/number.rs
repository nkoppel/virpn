use crate::modes::*;

#[derive(Clone, Debug)]
pub struct Number_mode {
    buffer: String
}

impl Mode for Number_mode {
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
            vec![Character('m')]
        ]
    }

    fn get_operator_regex(&self) -> Regex {
        Regex::new(r"-?\d*.?\d\+").unwrap()
    }

    fn get_name(&self) -> String {
        "number".to_string()
    }

    fn eval_operators(&mut self, stack: &mut Stack, op: String) {
        match op.parse::<f64>() {
            Ok(f) => stack.push(Num(f)),
            Err(_) => ()
        }
    }

    fn eval_bindings(&mut self, bind: Vec<Input>)
        -> (String, Action)
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
            _ => panic!()
        }

        (self.buffer.clone(), Continue)
    }

    fn exit(&mut self) {
        self.buffer.clear()
    }
}
