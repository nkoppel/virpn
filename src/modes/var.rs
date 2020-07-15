use crate::modes::*;

const NUM_VARS: usize = 52;
const VAR_NAMES: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub struct Var_mode {
    optype: String,
    values: Vec<Item>
}

impl Var_mode {
    pub fn new() -> Self {
        Var_mode {
            optype: String::new(),
            values: vec![Num(0.); 52]
        }
    }
}

fn index_from_str(s: &str) -> usize {
    VAR_NAMES.find(s).unwrap()
}

impl Mode for Var_mode {
    fn get_bindings(&self) -> Vec<Vec<Input>> {
        vec![vec![Character('z')], vec![Character('x')]]
    }

    fn get_operator_regex(&self) -> Regex {
        Regex::new("[sg]et +[[:alpha:]]").unwrap()
    }

    fn get_name(&self) -> String {
        "var".to_string()
    }

    fn eval_operators(&mut self, stack: &mut Stack, op: String) {
        let mut words = op.split(" ");

        let word1 = if let Some(w) = words.next() {w} else {return};
        let word2 = if let Some(w) = words.next() {w} else {return};

        if word1 == "get" {
            stack.push(self.values[index_from_str(word2)].clone());
        } else if word1 == "set" {
            if let Some(i) = stack.pop() {
                self.values[index_from_str(word2)] = i;
            }
        }
    }

    fn eval_bindings(&mut self, bind: Vec<Input>)
        -> (String, Action)
    {
        if let Some(Character(chr)) = bind.first() {
            if self.optype.is_empty() {
                if chr == &'x' {
                    self.optype = "get".to_string();
                    return (self.optype.clone(), Req_own);
                } else if chr == &'z' {
                    self.optype = "set".to_string();
                    return (self.optype.clone(), Req_own);
                }
            } else {
                if VAR_NAMES.contains(&chr.to_string()) {
                    let tmp =
                        std::mem::replace(&mut self.optype, String::new());

                    return (format!("{} {}", tmp, chr), Exit);
                }
            }
        }

        return (String::new(), Continue);
    }

    fn exit(&mut self) {
        self.optype.clear();
    }
}
