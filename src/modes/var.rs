use crate::modes::*;

const VAR_NAMES: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[allow(non_camel_case_types)]
pub struct Var_mode {
    aux_stack: Vec<Item>,
    values: Vec<Item>
}

impl Var_mode {
    pub fn new() -> Self {
        Var_mode {
            aux_stack: Vec::new(),
            values: vec![Num(0.); 52]
        }
    }
}

fn index_from_str(s: &str) -> usize {
    VAR_NAMES.find(s).unwrap()
}

impl Mode for Var_mode {
    fn get_bindings(&self) -> Vec<Vec<Input>> {
        vec![
            vec![Character('v')],
            vec![Character('z')],
            vec![Character('y')],
            vec![Character('p')],
            vec![Character('Y')],
            vec![Character('P')],
        ]
    }

    fn get_operator_regex(&self) -> Regex {
        Regex::new(r"^[sg]et +[[:alpha:]]|^(del_|)(yank|put)").unwrap()
    }

    fn get_name(&self) -> String {
        "var".to_string()
    }

    fn eval_operators(&mut self, ui: &mut Ui, op: &str) {
        let op = op.trim();
        let stack = ui.get_stack();

        if op == "put" {
            if let Some(i) = self.aux_stack.last() {
                stack.push(i.clone());
            }
        } else if op == "yank" {
            if let Some(i) = stack.last() {
                self.aux_stack.push(i.clone());
            }
        } else if op == "del_put" {
            if let Some(i) = self.aux_stack.pop() {
                stack.push(i);
            }
        } else if op == "del_yank" {
            if let Some(i) = stack.pop() {
                self.aux_stack.push(i);
            }
        } else {
            let word1 = &op[..3];
            let word2 = &op[4..];

            if word1 == "get" {
                stack.push(self.values[index_from_str(word2)].clone());
            } else if word1 == "set" {
                if let Some(i) = stack.last() {
                    self.values[index_from_str(word2)] = i.clone();
                }
            }
        }
    }

    fn eval_binding(&mut self, state: &mut State, bind: Vec<Input>)
        -> Vec<Message>
    {
        let mut msg = Vec::new();
        let mut setting = Data::unwrap_bool_or(state.get("setting"), false);
        let mut name = Data::unwrap_bool_or(state.get("name"), false);

        msg.push(NextKey(false));

        if name {
            if let Character(c) = bind[0] {
                if VAR_NAMES.contains(c) {
                    let mut op = if setting {"set "} else {"get "}.to_string();
                    op.push(c);

                    msg.push(Print(op.clone(), 5));
                    name = false;

                    state.insert("op".to_string(), Str(op));
                }
            }

            msg.push(Return);
        } else {
            match bind[0] {
                Character('Y') => {
                    let op = "del_yank".to_string();
                    state.insert("op".to_string(), Str(op.clone()));
                    msg.push(Print(op, 8));
                    msg.push(Return);
                },
                Character('P') => {
                    let op = "del_put".to_string();
                    state.insert("op".to_string(), Str(op.clone()));
                    msg.push(Print(op, 7));
                    msg.push(Return);
                },
                Character('y') => {
                    state.insert("op".to_string(), Str("yank".to_string()));
                    msg.push(Print("yank".to_string(), 4));
                    msg.push(Return);
                },
                Character('p') => {
                    state.insert("op".to_string(), Str("put".to_string()));
                    msg.push(Print("put".to_string(), 3));
                    msg.push(Return);
                },
                Character('z') | Character('v') => {
                    let set = bind[0] == Character('z');
                    let op = if set {"set"} else {"get"}.to_string();

                    msg.push(Print(op, 3));
                    msg.push(NextKey(true));

                    name = true;
                    setting = set;
                }
                _ => panic!()
            }
        }

        state.insert("setting".to_string(), Bool(setting));
        state.insert("name".to_string(), Bool(name));

        msg
    }

    fn ret(&mut self, state: &mut State) -> String {
        Data::unwrap_string_or(state.get("op"), String::new())
    }
}
