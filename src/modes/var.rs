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

        ui.insert_mode(
            self.get_name(),
            Box::new(mem::replace(self, Var_mode::new()))
        );
    }

    fn eval_bindings(&self, mut ui: Ui_helper, _: HashMap<&str, &str>)
        -> ModeRes<(String, usize)>
    {
        let mut tmp = ui.get_next_binding();

        if let (_, Some(_)) = tmp {
            return ((String::new(), 0), tmp.1);
        }

        match tmp.0[0] {
            Character('Y') => {
                ui.print_output("del_yank", 8);
                return (("del_yank".to_string(), 8), None)
            },
            Character('P') => {
                ui.print_output("del_put", 7);
                return (("del_put" .to_string(), 7), None)
            },
            Character('y') => {
                ui.print_output("yank", 4);
                return (("yank".to_string(), 4), None)
            },
            Character('p') => {
                ui.print_output("put", 3);
                return (("put" .to_string(), 3), None)
            },
            Character('z') | Character('v') => {
                let setting = tmp.0 == vec![Character('z')];
                let mut op = if setting {"set "} else {"get "}.to_string();

                ui.print_output(&op, 4);

                for c in VAR_NAMES.chars() {
                    ui.add_escape_binding(vec![Character(c)]);
                }

                tmp = ui.get_next_binding();

                if let (_, Some(_)) = tmp {
                    return ((String::new(), 0), tmp.1);
                }

                if let Character(c) = tmp.0[0] {
                    op.push(c);
                    ui.print_output(&op, 5);

                    return ((op, 5), None);
                } else {
                    panic!()
                }
            }
            _ => panic!()
        }
    }
}
