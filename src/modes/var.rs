use crate::modes::*;

const VAR_NAMES: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[allow(non_camel_case_types)]
pub struct Var_mode {
    values: Vec<Item>
}

impl Var_mode {
    pub fn new() -> Self {
        Var_mode {values: vec![Num(0.); 52]}
    }
}

fn index_from_str(s: &str) -> usize {
    VAR_NAMES.find(s).unwrap()
}

impl Mode for Var_mode {
    fn get_bindings(&self) -> Vec<Vec<Input>> {
        vec![vec![Character('y')], vec![Character('p')]]
    }

    fn get_operator_regex(&self) -> Regex {
        Regex::new("^[sg]et +[[:alpha:]]").unwrap()
    }

    fn get_name(&self) -> String {
        "var".to_string()
    }

    fn eval_operators(&mut self, ui: &mut Ui, op: &str) {
        let word1 = &op[..3];
        let word2 = &op[4..];

        let stack = ui.get_stack();

        if word1 == "get" {
            stack.push(self.values[index_from_str(word2)].clone());
        } else if word1 == "set" {
            if let Some(i) = stack.pop() {
                self.values[index_from_str(word2)] = i;
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

        let setting = tmp.0 == vec![Character('y')];
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
}
