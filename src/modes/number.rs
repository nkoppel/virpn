use crate::modes::*;

#[derive(Clone, Debug)]
pub struct Number_mode {
    buffer: String
}

impl Mode for Number_mode {
    fn get_bindings(&self) -> Vec<Vec<Key>> {
        vec![
            vec![Char('a')],
            vec![Char('s')],
            vec![Char('d')],
            vec![Char('f')],
            vec![Char('g')],
            vec![Char('h')],
            vec![Char('j')],
            vec![Char('k')],
            vec![Char('l')],
            vec![Char(';')],
            vec![Char('n')],
            vec![Char('m')]
        ]
    }

    fn get_operator_regex(&self) -> Regex {
        Regex::new(r"-?\d*.?\d\+").unwrap()
    }

    fn copy(&self) -> Box<dyn Mode> {
        Box::new(self.clone())
    }

    fn get_name(&self) -> String {
        "number".to_string()
    }

    fn run(&mut self, modes: &ModeMap, stack: &mut Stack, ops: Vec<String>) {
        match ops[0].parse::<f64>() {
            Ok(f) => stack.add(Num(f)),
            Err(_) => ()
        }
    }

    fn eval_input(&mut self, modes: &ModeMap, bind: Vec<Key>)
        -> (String, bool, bool)
    {
        match bind[0] {
            Char('a') => {self.buffer.push('1')},
            Char('s') => {self.buffer.push('2')},
            Char('d') => {self.buffer.push('3')},
            Char('f') => {self.buffer.push('4')},
            Char('g') => {self.buffer.push('5')},
            Char('h') => {self.buffer.push('6')},
            Char('j') => {self.buffer.push('7')},
            Char('k') => {self.buffer.push('8')},
            Char('l') => {self.buffer.push('9')},
            Char(';') => {self.buffer.push('0')},
            Char('n') => {self.buffer.push('-')},
            Char('m') => {self.buffer.push('.')},
            _ => panic!()
        }

        (self.buffer.clone(), false, false)
    }
}
